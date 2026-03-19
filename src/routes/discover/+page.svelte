<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import FavoriteButton from '$lib/FavoriteButton.svelte';
  import SongStats from '$lib/SongStats.svelte';
  import { playerState, sarkiCal, initializePlayer, sarkiSil, sarkiPlaylisteEkle, type Sarki } from '../../store.svelte';
  import { fade, fly, scale } from 'svelte/transition';

  onMount(async () => {
    if (playerState.sarkiListesi.length === 0) {
      await initializePlayer();
    }
  });

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

  async function handleSarkiSil(sarki: Sarki, event: MouseEvent | KeyboardEvent) {
    event.preventDefault();
    event.stopPropagation();
    
    const mesaj = `"${sarki.isim}" adlı parçayı kütüphaneden ve diskten KALICI olarak silmek istediğinize emin misiniz?\n\nBu işlem geri alınamaz.`;
    
    if (confirm(mesaj)) {
        try {
            await sarkiSil(sarki);
        } catch (hata) {
            alert("Silme işlemi sırasında bir hata oluştu.");
        }
    }
  }

  async function handlePlaylistEkle(sarkiId: string, event: Event) {
    const selectElement = event.target as HTMLSelectElement;
    const playlistId = selectElement.value;
    if (!playlistId) return;

    const basarili = await sarkiPlaylisteEkle(sarkiId, playlistId);
    if(basarili) {
        selectElement.value = ""; 
    }
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
      <h1 class="text-5xl lg:text-7xl font-black text-white mb-4 tracking-tighter italic leading-none drop-shadow-2xl">
        KEŞFET
      </h1>
      <p class="text-white/80 max-w-lg font-medium text-sm leading-relaxed">
        Sistem kütüphaneni analiz etti. Mevcut frekansların ve en yeni veri blokların aşağıda listelenmiştir.
      </p>
    </div>
  </section>

  <section class="mb-16">
    <h2 class="text-xs font-black text-[var(--text-dim)] mb-6 uppercase tracking-[0.4em] flex items-center gap-4">
        Frekans Grupları <div class="h-px flex-1 bg-[var(--border)]"></div>
    </h2>
    <div class="flex gap-4 overflow-x-auto pb-4 custom-scrollbar-h no-scrollbar">
      {#each kategoriler as kat, i}
        <a 
          href="/search?q={kat.isim}"
          class="flex-shrink-0 w-36 h-44 bg-[var(--bg-card)] border border-[var(--border)] rounded-[var(--radius)] p-5 flex flex-col justify-between hover:bg-[var(--bg-card-hover)] hover:border-[var(--accent)]/50 transition-all group shadow-lg"
          in:scale={{ duration: 400, delay: i * 50 }}
        >
          <span class="text-4xl group-hover:scale-110 transition-transform">{tarzIkonlari[kat.isim] || "🎵"}</span>
          <div>
            <p class="font-black text-sm uppercase tracking-tight group-hover:text-[var(--accent)] transition-colors">{kat.isim}</p>
            <p class="text-[9px] font-bold text-[var(--text-dim)] uppercase">{kat.adet} Parça</p>
          </div>
        </a>
      {/each}
    </div>
  </section>

  <div class="grid grid-cols-1 lg:grid-cols-12 gap-12">
    
    <div class="lg:col-span-5 flex flex-col">
      <h2 class="text-xl font-black text-[var(--text-main)] mb-8 flex items-center gap-4 uppercase italic tracking-tight">
        <span class="text-[var(--accent)] text-3xl font-serif">#</span> Zirvedekiler
      </h2>
      
      <div class="flex flex-col gap-3">
        {#each enCokDinlenenler as sarki, index}
          <div 
            role="button" tabindex="0"
            onclick={() => sarkiCal(sarki)}
            onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(sarki)}
            aria-label="{sarki.isim} parçasını çal"
            class="flex items-center gap-4 p-4 rounded-2xl transition-all group cursor-pointer shadow-sm border {playerState.aktifSarki?.id === sarki.id ? 'bg-[var(--accent)]/10 border-[var(--accent)]/30' : 'bg-[var(--bg-card)] border-[var(--border)] hover:bg-[var(--bg-card-hover)] hover:border-[var(--accent)]/30'}"
            in:fly={{ x: -20, duration: 400, delay: index * 50 }}
          >
            <div class="w-8 text-center shrink-0">
               {#if playerState.aktifSarki?.id === sarki.id && playerState.suAnOynuyorMu}
                  <div class="flex items-end justify-center gap-0.5 h-3.5 mb-1">
                     <div class="w-1 bg-[var(--accent)] animate-[bounce_1s_infinite]"></div>
                     <div class="w-1 bg-[var(--accent)] animate-[bounce_1.2s_infinite]"></div>
                     <div class="w-1 bg-[var(--accent)] animate-[bounce_0.8s_infinite]"></div>
                  </div>
               {:else}
                  <span class="text-xl font-black text-[var(--text-dim)]/20 group-hover:hidden transition-colors font-serif italic">
                      {index + 1}
                  </span>
                  <svg class="w-4 h-4 mx-auto hidden group-hover:block text-[var(--accent)]" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M8 5v14l11-7z"/></svg>
               {/if}
            </div>
            
            <div class="w-12 h-12 rounded-xl overflow-hidden shadow-lg shrink-0 border border-[var(--border)] bg-[var(--bg-surface)]">
              {#if sarki.kapak_yolu}
                <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover transition-transform group-hover:scale-110" />
              {:else}
                <div class="w-full h-full flex items-center justify-center text-[var(--text-dim)]/30 italic font-black text-xs">LW</div>
              {/if}
            </div>

            <div class="flex-1 min-w-0 pr-2">
              <span class="font-bold text-[var(--text-main)] truncate block text-sm group-hover:text-[var(--accent)] transition-colors">{sarki.isim}</span>
              <a href="/artist/{encodeURIComponent(sarki.sarkici)}" onclick={(e) => e.stopPropagation()} class="text-[10px] text-[var(--text-dim)] font-bold uppercase tracking-widest truncate block opacity-80 hover:text-[var(--accent)] transition-colors">{sarki.sarkici}</a>
            </div>

            <div class="shrink-0 flex items-center gap-2" role="presentation" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
              <select 
                  aria-label="Listeye Ekle" 
                  onchange={(e) => handlePlaylistEkle(sarki.id, e)} 
                  class="bg-[var(--bg-surface)] text-[9px] text-[var(--text-dim)] rounded-lg px-1.5 py-1 outline-none border border-[var(--border)] hover:border-[var(--accent)]/50 cursor-pointer w-20 focus:border-[var(--accent)] transition-all font-bold uppercase opacity-0 group-hover:opacity-100 hidden sm:block"
              >
                <option value="">➕ EKLE</option>
                {#each playerState.playlistler as pl}
                  {#if !pl.sarkilar.includes(sarki.id)}
                    <option value={pl.id}>{pl.isim.toUpperCase()}</option>
                  {/if}
                {/each}
              </select>
              
              <FavoriteButton sarkiId={sarki.id} />
              
              <button 
                  type="button" 
                  aria-label="Sil" 
                  title="Kalıcı Olarak Sil" 
                  onclick={(e) => handleSarkiSil(sarki, e)} 
                  class="text-[var(--text-dim)]/30 hover:text-red-500 transition-all p-1 opacity-0 group-hover:opacity-100"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
              </button>
            </div>
          </div>
        {/each}
      </div>
    </div>

    <div class="lg:col-span-7 flex flex-col">
      <h2 class="text-xl font-black text-[var(--text-main)] mb-8 flex items-center gap-4 uppercase italic tracking-tight">
        <span class="text-[var(--accent-sec)] text-3xl font-serif">/</span> Son Eklenenler
      </h2>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        {#each yeniEklenenler as sarki, i}
          <div 
            role="button" tabindex="0"
            onclick={() => sarkiCal(sarki)}
            onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(sarki)}
            aria-label="{sarki.isim} çal"
            class="flex items-center gap-4 p-3 rounded-2xl transition-all cursor-pointer group shadow-sm border {playerState.aktifSarki?.id === sarki.id ? 'bg-[var(--accent)]/10 border-[var(--accent)]/30' : 'bg-[var(--bg-card)] border-[var(--border)] hover:bg-[var(--bg-card-hover)] hover:border-[var(--accent-sec)]/30'}"
            in:fly={{ y: 10, duration: 400, delay: i * 30 }}
          >
            <div class="w-12 h-12 rounded-xl overflow-hidden shrink-0 relative border border-[var(--border)]">
               {#if sarki.kapak_yolu}
                  <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-500" />
               {:else}
                  <div class="w-full h-full bg-[var(--bg-surface)] flex items-center justify-center text-[var(--text-dim)]/20 text-xs">🎵</div>
               {/if}
               <div class="absolute inset-0 bg-[var(--accent)]/10 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
                  <svg class="w-5 h-5 text-white" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
               </div>
            </div>
            
            <div class="flex-1 min-w-0 pr-2">
              <span class="text-sm font-bold text-[var(--text-main)] truncate block leading-tight group-hover:text-[var(--accent)] transition-colors">{sarki.isim}</span>
              <a href="/artist/{encodeURIComponent(sarki.sarkici)}" onclick={(e) => e.stopPropagation()} class="text-[9px] text-[var(--text-dim)] font-bold truncate uppercase tracking-widest opacity-80 hover:text-[var(--accent)] transition-colors block">{sarki.sarkici}</a>
            </div>

            <div onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="presentation" class="shrink-0 flex items-center gap-2 pr-2">
               
               <select 
                   aria-label="Listeye Ekle" 
                   onchange={(e) => handlePlaylistEkle(sarki.id, e)} 
                   class="bg-[var(--bg-surface)] text-[9px] text-[var(--text-dim)] rounded-lg px-1.5 py-1 outline-none border border-[var(--border)] hover:border-[var(--accent)]/50 cursor-pointer w-20 focus:border-[var(--accent)] transition-all font-bold uppercase opacity-0 group-hover:opacity-100 hidden md:block"
               >
                 <option value="">➕ EKLE</option>
                 {#each playerState.playlistler as pl}
                   {#if !pl.sarkilar.includes(sarki.id)}
                     <option value={pl.id}>{pl.isim.toUpperCase()}</option>
                   {/if}
                 {/each}
               </select>

               <FavoriteButton sarkiId={sarki.id} />
               
               <button 
                  type="button" 
                  aria-label="Sil" 
                  onclick={(e) => handleSarkiSil(sarki, e)} 
                  class="text-[var(--text-dim)]/30 hover:text-red-500 transition-all p-1 opacity-0 group-hover:opacity-100"
               >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
               </button>
            </div>
          </div>
        {/each}
      </div>

      <div class="mt-8 p-8 rounded-[var(--radius)] bg-gradient-to-br from-[var(--bg-card)] to-[var(--bg-surface)] border border-[var(--border)] flex items-center justify-between shadow-xl" in:fade={{ delay: 500 }}>
        <div class="flex flex-col">
          <span class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-[0.3em] mb-2">Toplam Veri Akışı</span>
          <div class="flex items-end gap-2 leading-none">
             <span class="text-5xl font-black text-[var(--text-main)] italic tracking-tighter">{playerState.sarkiListesi.length}</span>
             <span class="text-[11px] font-bold text-[var(--accent)] uppercase tracking-widest mb-1">Indexli Parça</span>
          </div>
        </div>
        <div class="w-14 h-14 rounded-full border border-[var(--border)] flex items-center justify-center text-[var(--accent)] bg-[var(--bg-surface)] shadow-inner">
          <svg class="w-6 h-6 animate-pulse" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M17 8l-5-5-5 5M12 3v12"></path>
          </svg>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  @keyframes bounce {
    0%, 100% { height: 4px; }
    50% { height: 14px; }
  }

  .no-scrollbar::-webkit-scrollbar { display: none; }
  .no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }

  .custom-scrollbar::-webkit-scrollbar { width: 4px; }
  .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--accent); }
</style>