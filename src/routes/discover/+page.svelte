<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc, invoke } from '@tauri-apps/api/core';
  import FavoriteButton from '$lib/FavoriteButton.svelte';
  import { playerState, sarkiCal, initializePlayer, sarkiSil, sarkiPlaylisteEkle, type Sarki } from '../../store.svelte';
  import { fade, fly, scale, slide } from 'svelte/transition';

  onMount(async () => {
    if (playerState.sarkiListesi.length === 0) {
      await initializePlayer();
    }
  });

  // --- KÜTÜPHANE STATE'LERİ ---
  let kategoriler = $derived.by(() => {
    const map = new Map();
    playerState.sarkiListesi.forEach(s => {
      if (!s.tarz) return;
      const t = s.tarz.trim();
      map.set(t, (map.get(t) || 0) + 1);
    });
    return Array.from(map.entries()).map(([isim, adet]) => ({ isim, adet }));
  });

  let enCokDinlenenler = $derived(
    [...playerState.sarkiListesi]
      .sort((a, b) => (b.dinlenme_sayisi || 0) - (a.dinlenme_sayisi || 0))
      .slice(0, 5)
  );

  let yeniEklenenler = $derived(
    [...playerState.sarkiListesi]
      .slice(-10) 
      .reverse()
  );

  const tarzIkonlari: Record<string, string> = {
    "Pop": "✨", "Rock": "🎸", "Lofi": "☕", "Cyberpunk": "🤖", 
    "Ghibli": "🌳", "Electronic": "⚡", "Jazz": "🎷", "Podcast": "🎙️"
  };

  // --- YOUTUBE ARAMA & İNDİRME ---
  let aramaSorgusu = $state("");
  let aramaYapiliyor = $state(false);
  let aramaSonuclari = $state<any[]>([]);
  let indirmeUrl = $state("");
  let indiriliyor = $state(false);
  let indirmeMesaji = $state("");

  async function muzikAra() {
      if (!aramaSorgusu.trim()) return;

      if (aramaSorgusu.includes("http://") || aramaSorgusu.includes("https://")) {
          await youtubeIndir(aramaSorgusu);
          return;
      }

      aramaYapiliyor = true;
      aramaSonuclari = [];
      indirmeMesaji = "Ağda frekanslar taranıyor...";

      try {
          const sonuclar = await invoke<any[]>('youtube_arama', { sorgu: aramaSorgusu });
          aramaSonuclari = sonuclar;
          indirmeMesaji = sonuclar.length > 0 ? `${sonuclar.length} sinyal tespit edildi.` : "Sinyal bulunamadı.";
      } catch (e) {
          indirmeMesaji = "Tarama başarısız: " + e;
      } finally {
          aramaYapiliyor = false;
      }
  }

  async function youtubeIndir(gelenUrl?: string) {
      const urlToDownload = gelenUrl || indirmeUrl;
      if (!urlToDownload.trim()) return;

      indiriliyor = true;
      indirmeMesaji = "Veri akışı sağlanıyor, arşive indiriliyor...";

      try {
          await invoke<Sarki>('youtube_indir', { url: urlToDownload, tarz: "Pop" });
          indirmeMesaji = "Veri başarıyla arşive eklendi.";
          aramaSorgusu = "";
          indirmeUrl = "";
          aramaSonuclari = [];
          await initializePlayer();
      } catch (e) {
          indirmeMesaji = "Bağlantı koptu: " + e;
      } finally {
          indiriliyor = false;
          setTimeout(() => { if (!indiriliyor && !aramaYapiliyor) indirmeMesaji = ""; }, 5000);
      }
  }

  async function handleSarkiSil(sarki: Sarki, event: Event) {
    event.stopPropagation();
    if (confirm(`"${sarki.isim}" kütüphaneden silinecek. Onaylıyor musun?`)) {
        await sarkiSil(sarki);
    }
  }

  async function handlePlaylistEkle(sarkiId: string, event: Event) {
    const select = event.target as HTMLSelectElement;
    if (select.value) {
        await sarkiPlaylisteEkle(sarkiId, select.value);
        select.value = ""; 
    }
  }

  function editModaliAc(sarki: Sarki, event: Event) {
      event.stopPropagation();
      playerState.duzenlenecekSarki = sarki;
      playerState.isEditModalOpen = true;
  }
</script>

<div class="p-8 lg:p-12 w-full min-h-full pb-32 flex flex-col relative min-w-0 bg-transparent text-[var(--text-main)] transition-colors duration-500 overflow-y-auto custom-scrollbar">
  
  <section class="relative w-full h-72 rounded-[var(--radius)] overflow-hidden mb-12 shadow-2xl border border-[var(--border)] group shrink-0" in:fade>
    <div class="absolute inset-0 bg-gradient-to-r from-[var(--accent)] via-[var(--accent-sec)] to-[var(--bg-main)] opacity-60 z-10"></div>
    <div class="absolute inset-0 bg-[url('https://images.unsplash.com/photo-1614613535308-eb5fbd3d2c17?q=80&w=2070')] bg-cover bg-center mix-blend-overlay group-hover:scale-105 transition-transform duration-1000"></div>
    <div class="absolute inset-0 p-10 flex flex-col justify-center z-20">
      <div class="flex items-center gap-3 mb-4">
        <span class="w-10 h-[2px] bg-white/50"></span>
        <span class="text-[10px] font-black tracking-[0.4em] text-white/90 uppercase">Lain Wave Intelligence</span>
      </div>
      <h1 class="text-5xl lg:text-7xl font-black text-white mb-4 tracking-tighter italic leading-none drop-shadow-2xl">KEŞFET</h1>
      <p class="text-white/80 max-w-lg font-medium text-sm leading-relaxed">Sistem kütüphaneni analiz etti. Yeni veri akışları yakalayabilir veya mevcut arşivi inceleyebilirsin.</p>
    </div>
  </section>

  <section class="mb-16 bg-[var(--bg-card)] border border-[var(--border)] rounded-[var(--radius)] p-8 shadow-xl relative overflow-hidden group shrink-0">
      <div class="absolute top-0 right-0 w-64 h-64 bg-[var(--accent)]/5 blur-[80px] rounded-full pointer-events-none"></div>
      
      <div class="flex items-center gap-4 mb-6 relative z-10">
          <div class="w-12 h-12 rounded-2xl bg-red-500/10 text-red-500 flex items-center justify-center border border-red-500/20 shrink-0">
              <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M15.5 14h-.79l-.28-.27A6.471 6.471 0 0 0 16 9.5 6.5 6.5 0 1 0 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/></svg>
          </div>
          <div>
              <h2 class="text-xl font-black uppercase tracking-tight italic">Ağ Tarayıcısı</h2>
              <p class="text-[10px] text-[var(--text-dim)] font-bold tracking-widest uppercase mt-1">YouTube üzerinden müzik arayın veya URL yapıştırın</p>
          </div>
      </div>

      <div class="flex flex-col md:flex-row gap-4 relative z-10 mb-4">
          <input 
              type="text" 
              bind:value={aramaSorgusu}
              onkeydown={(e) => e.key === 'Enter' && !aramaYapiliyor && muzikAra()}
              placeholder="Şarkı veya sanatçı adı yazın..." 
              class="flex-1 bg-[var(--bg-surface)] border border-[var(--border)] rounded-xl px-6 py-4 outline-none text-sm text-[var(--text-main)] focus:border-red-500/50 transition-colors placeholder:text-[var(--text-dim)]/50 font-mono"
              disabled={aramaYapiliyor || indiriliyor}
          />
          <button 
              type="button"
              onclick={muzikAra}
              disabled={aramaYapiliyor || indiriliyor || !aramaSorgusu.trim()}
              aria-label="Aramayı Başlat"
              class="bg-red-500 hover:bg-red-600 text-white font-black uppercase tracking-[0.2em] text-[10px] px-10 py-4 rounded-xl transition-all shadow-lg active:scale-95 disabled:opacity-50 shrink-0 min-w-[160px]"
          >
              {#if aramaYapiliyor || indiriliyor}
                  <svg class="w-5 h-5 animate-spin" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
              {:else} Ağı Tara {/if}
          </button>
      </div>
      
      {#if indirmeMesaji}
          <div class="mb-2 text-[10px] font-mono font-bold uppercase tracking-widest {indirmeMesaji.includes('başarı') ? 'text-[var(--accent)]' : 'text-red-400'}" in:slide>> {indirmeMesaji}</div>
      {/if}

      {#if aramaSonuclari.length > 0}
          <div class="flex flex-col gap-2 mt-6 max-h-[400px] overflow-y-auto custom-scrollbar pr-2" in:fade>
              <h3 class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-[0.3em] mb-2 border-b border-[var(--border)] pb-2">Bulunan Sinyaller (Tıklayarak İndir)</h3>
              {#each aramaSonuclari as sonuc}
                  <button 
                      type="button"
                      onclick={() => youtubeIndir(sonuc.webpage_url)}
                      disabled={indiriliyor}
                      aria-label="{sonuc.title} parçasını indir"
                      class="flex items-center gap-4 p-3 bg-[var(--bg-surface)] border border-[var(--border)] hover:border-red-500/30 rounded-xl group transition-all text-left w-full min-w-0"
                  >
                      <div class="w-16 h-10 bg-black rounded-lg overflow-hidden shrink-0 relative">
                          <img src={sonuc.thumbnail} alt="" class="w-full h-full object-cover opacity-70 group-hover:opacity-100 transition-opacity" />
                      </div>
                      <div class="flex-1 min-w-0">
                          <p class="text-xs font-bold text-[var(--text-main)] truncate group-hover:text-red-400 transition-colors">{sonuc.title}</p>
                          <div class="flex items-center gap-2 mt-1">
                              <span class="text-[9px] font-black text-[var(--text-dim)] uppercase truncate max-w-[120px]">{sonuc.channel}</span>
                              <span class="w-1 h-1 bg-[var(--border)] rounded-full"></span>
                              <span class="text-[9px] font-mono text-[var(--text-dim)]">{sonuc.duration_string}</span>
                          </div>
                      </div>
                      <div class="p-2 text-[var(--text-dim)] group-hover:text-red-500 shrink-0 transition-colors">
                          <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true"><path stroke-linecap="round" stroke-linejoin="round" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path></svg>
                      </div>
                  </button>
              {/each}
          </div>
      {/if}
  </section>

  <section class="mb-16 shrink-0">
    <h2 class="text-xs font-black text-[var(--text-dim)] mb-6 uppercase tracking-[0.4em] flex items-center gap-4">FREKANS GRUPLARI <div class="h-px flex-1 bg-[var(--border)] opacity-50"></div></h2>
    <div class="flex gap-4 overflow-x-auto pb-4 no-scrollbar">
      {#each kategoriler as kat, i}
        <a href="/search?q={kat.isim}" class="flex-shrink-0 w-36 h-44 bg-[var(--bg-card)] border border-[var(--border)] rounded-[var(--radius)] p-5 flex flex-col justify-between hover:bg-[var(--bg-card-hover)] hover:border-[var(--accent)]/50 transition-all group shadow-lg" in:scale={{ duration: 400, delay: i * 50 }}>
          <span class="text-4xl group-hover:scale-110 transition-transform">{tarzIkonlari[kat.isim] || "🎵"}</span>
          <div class="min-w-0">
            <p class="font-black text-sm uppercase tracking-tight group-hover:text-[var(--accent)] transition-colors truncate">{kat.isim}</p>
            <p class="text-[9px] font-bold text-[var(--text-dim)] uppercase">{kat.adet} Parça</p>
          </div>
        </a>
      {/each}
    </div>
  </section>

  <div class="grid grid-cols-1 lg:grid-cols-12 gap-12">
    
    <div class="lg:col-span-5 flex flex-col min-w-0">
      <h2 class="text-xl font-black text-[var(--text-main)] mb-8 flex items-center gap-4 uppercase italic tracking-tight"><span class="text-[var(--accent)] text-3xl font-serif">#</span> Zirvedekiler</h2>
      <div class="flex flex-col gap-3">
        {#each enCokDinlenenler as sarki, index}
          <div 
            role="button" tabindex="0"
            onclick={() => sarkiCal(sarki)}
            onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(sarki)}
            aria-label="{sarki.isim} çal"
            class="flex items-center gap-4 p-4 rounded-2xl transition-all group cursor-pointer border {playerState.aktifSarki?.id === sarki.id ? 'bg-[var(--accent)]/10 border-[var(--accent)]/30' : 'bg-[var(--bg-card)] border-[var(--border)] hover:bg-[var(--bg-card-hover)] hover:border-[var(--accent)]/30'}"
            in:fly={{ x: -20, duration: 400, delay: index * 50 }}
          >
            <div class="w-6 text-center shrink-0">
               {#if playerState.aktifSarki?.id === sarki.id && playerState.suAnOynuyorMu}
                  <div class="flex items-end justify-center gap-0.5 h-3.5 mb-1">
                     <div class="w-1 bg-[var(--accent)] animate-[bounce_1s_infinite]"></div>
                     <div class="w-1 bg-[var(--accent)] animate-[bounce_1.2s_infinite]"></div>
                     <div class="w-1 bg-[var(--accent)] animate-[bounce_0.8s_infinite]"></div>
                  </div>
               {:else}
                  <span class="text-lg font-black text-[var(--text-dim)]/20 group-hover:hidden italic">{index + 1}</span>
                  <svg class="w-4 h-4 mx-auto hidden group-hover:block text-[var(--accent)]" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M8 5v14l11-7z"/></svg>
               {/if}
            </div>
            <div class="w-12 h-12 rounded-xl overflow-hidden shrink-0 border border-[var(--border)] bg-[var(--bg-surface)]">
              {#if sarki.kapak_yolu}
                <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover group-hover:scale-110 transition-transform" />
              {:else}
                <div class="w-full h-full flex items-center justify-center text-[var(--text-dim)]/30 italic font-black text-xs">LW</div>
              {/if}
            </div>
            <div class="flex-1 min-w-0">
              <span class="font-bold text-[var(--text-main)] truncate block text-sm group-hover:text-[var(--accent)] transition-colors">{sarki.isim}</span>
              <button onclick={(e) => { e.stopPropagation(); }} class="text-[10px] text-[var(--text-dim)] font-bold uppercase tracking-widest truncate block opacity-80 hover:text-[var(--accent)] text-left w-full">
                {sarki.sarkici}
              </button>
            </div>
            <div class="shrink-0 flex items-center gap-1" onclick={(e) => e.stopPropagation()} role="presentation">
              <select aria-label="Listeye Ekle" onchange={(e) => handlePlaylistEkle(sarki.id, e)} class="bg-[var(--bg-surface)] text-[9px] text-[var(--text-dim)] rounded-lg px-1 py-1 outline-none border border-[var(--border)] w-16 focus:border-[var(--accent)] opacity-0 group-hover:opacity-100 hidden sm:block transition-all cursor-pointer font-bold">
                <option value="">➕</option>
                {#each playerState.playlistler as pl}
                  {#if !pl.sarkilar.includes(sarki.id)}<option value={pl.id}>{pl.isim.toUpperCase()}</option>{/if}
                {/each}
              </select>
              <button type="button" aria-label="Düzenle" title="Bilgileri Düzenle" onclick={(e) => editModaliAc(sarki, e)} class="text-[var(--text-dim)]/30 hover:text-[var(--accent)] p-1 opacity-0 group-hover:opacity-100 transition-all">
                  <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true"><path d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg>
              </button>
              <FavoriteButton sarkiId={sarki.id} />
              <button type="button" aria-label="Sil" title="Sil" onclick={(e) => handleSarkiSil(sarki, e)} class="text-[var(--text-dim)]/30 hover:text-red-500 p-1 opacity-0 group-hover:opacity-100 transition-all">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true"><path d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg>
              </button>
            </div>
          </div>
        {/each}
      </div>
    </div>

    <div class="lg:col-span-7 flex flex-col min-w-0">
      <h2 class="text-xl font-black text-[var(--text-main)] mb-8 flex items-center gap-4 uppercase italic tracking-tight"><span class="text-[var(--accent-sec)] text-3xl font-serif">/</span> Son Eklenenler</h2>
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        {#each yeniEklenenler as sarki, i}
          <div 
            role="button" tabindex="0"
            onclick={() => sarkiCal(sarki)}
            onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(sarki)}
            aria-label="{sarki.isim} çal"
            class="flex items-center gap-3 p-3 rounded-2xl transition-all cursor-pointer group border {playerState.aktifSarki?.id === sarki.id ? 'bg-[var(--accent)]/10 border-[var(--accent)]/30' : 'bg-[var(--bg-card)] border-[var(--border)] hover:bg-[var(--bg-card-hover)] hover:border-[var(--accent-sec)]/30'}"
            in:fly={{ y: 10, duration: 400, delay: i * 30 }}
          >
            <div class="w-10 h-10 rounded-xl overflow-hidden shrink-0 relative border border-[var(--border)]">
               {#if sarki.kapak_yolu}
                  <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover group-hover:scale-110 transition-transform" />
               {:else}
                  <div class="w-full h-full bg-[var(--bg-surface)] flex items-center justify-center text-[var(--text-dim)]/20 text-xs">🎵</div>
               {/if}
               <div class="absolute inset-0 bg-[var(--accent)]/10 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center"><svg class="w-4 h-4 text-white" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M8 5v14l11-7z"/></svg></div>
            </div>
            <div class="flex-1 min-w-0">
              <span class="text-xs font-bold text-[var(--text-main)] truncate block leading-tight group-hover:text-[var(--accent)] transition-colors">{sarki.isim}</span>
              <span class="text-[9px] text-[var(--text-dim)] font-bold truncate uppercase tracking-widest opacity-80 block">{sarki.sarkici}</span>
            </div>
            <div onclick={(e) => e.stopPropagation()} role="presentation" class="shrink-0 flex items-center gap-1">
               <button type="button" aria-label="Düzenle" title="Düzenle" onclick={(e) => editModaliAc(sarki, e)} class="text-[var(--text-dim)]/30 hover:text-[var(--accent)] p-1 opacity-0 group-hover:opacity-100 transition-all">
                  <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true"><path d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg>
               </button>
               <FavoriteButton sarkiId={sarki.id} />
               <button type="button" aria-label="Sil" title="Sil" onclick={(e) => handleSarkiSil(sarki, e)} class="text-[var(--text-dim)]/30 hover:text-red-500 p-1 opacity-0 group-hover:opacity-100 transition-all">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true"><path d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg>
               </button>
            </div>
          </div>
        {/each}
      </div>

      <div class="mt-8 p-6 rounded-[var(--radius)] bg-gradient-to-br from-[var(--bg-card)] to-[var(--bg-surface)] border border-[var(--border)] flex items-center justify-between shadow-xl shrink-0" in:fade={{ delay: 500 }}>
        <div class="flex flex-col">
          <span class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-[0.3em] mb-2">Toplam Veri Akışı</span>
          <div class="flex items-end gap-2 leading-none">
             <span class="text-4xl font-black text-[var(--text-main)] italic tracking-tighter">{playerState.sarkiListesi.length}</span>
             <span class="text-[10px] font-bold text-[var(--accent)] uppercase tracking-widest mb-1">Dizin</span>
          </div>
        </div>
        <div class="w-12 h-12 rounded-full border border-[var(--border)] flex items-center justify-center text-[var(--accent)] bg-[var(--bg-surface)] shadow-inner">
          <svg class="w-5 h-5 animate-pulse" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M17 8l-5-5-5 5M12 3v12"></path></svg>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  @keyframes bounce { 0%, 100% { height: 4px; } 50% { height: 14px; } }
  .no-scrollbar::-webkit-scrollbar { display: none; }
  .no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
  .custom-scrollbar::-webkit-scrollbar { width: 4px; }
  .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--accent); }
</style>