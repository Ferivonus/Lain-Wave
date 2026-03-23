<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import FavoriteButton from '$lib/FavoriteButton.svelte';
  import SongStats from '$lib/SongStats.svelte';
  import { playerState, initializePlayer, sarkiPlaylisteEkle, sarkiCal, sarkiSil, type Sarki } from '../../store.svelte';
  import { fade, scale } from 'svelte/transition';

  let aramaMetni = $state("");

  onMount(async () => {
    if (playerState.sarkiListesi.length === 0) {
      await initializePlayer();
    }
  });

  let aramaSonuclari = $derived(
    aramaMetni.trim() === ""
      ? [] 
      : playerState.sarkiListesi.filter((sarki) => {
          const aranan = aramaMetni.toLowerCase();
          return (
            sarki.isim.toLowerCase().includes(aranan) ||
            sarki.sarkici.toLowerCase().includes(aranan) ||
            (sarki.album && sarki.album.toLowerCase().includes(aranan))
          );
        })
  );

  let enPopulerler = $derived(
    [...playerState.sarkiListesi]
      .sort((a, b) => (b.dinlenme_sayisi || 0) - (a.dinlenme_sayisi || 0))
      .slice(0, 5)
  );

  let tumSarkilarSirali = $derived(
    [...playerState.sarkiListesi].sort((a, b) => (b.dinlenme_sayisi || 0) - (a.dinlenme_sayisi || 0))
  );

  async function handlePlaylistEkle(sarkiId: string, event: Event) {
    const selectElement = event.target as HTMLSelectElement;
    const playlistId = selectElement.value;
    if (!playlistId) return;

    const basarili = await sarkiPlaylisteEkle(sarkiId, playlistId);
    if (basarili) {
      selectElement.value = "";
    }
  }

  async function handleSarkiSil(sarki: Sarki, event: MouseEvent | KeyboardEvent) {
    event.preventDefault();
    event.stopPropagation();
    
    const mesaj = `DİKKAT: "${sarki.isim}" adlı parçayı kütüphaneden ve diskten KALICI olarak silmek istediğinize emin misiniz?\n\nBu işlem geri alınamaz.`;
    
    if (confirm(mesaj)) {
        try {
            await sarkiSil(sarki);
        } catch (hata) {
            alert("Silme işlemi sırasında bir hata oluştu.");
        }
    }
  }

  function editModaliAc(sarki: Sarki, event: Event) {
    event.stopPropagation();
    playerState.duzenlenecekSarki = sarki;
    playerState.isEditModalOpen = true;
  }

  const kategoriler = [
    { isim: "Pop", renk: "var(--color-pop)", ikon: '<svg width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m12 3-1.9 5.8a2 2 0 0 1-1.3 1.3L3 12l5.8 1.9a2 2 0 0 1 1.3 1.3L12 21l1.9-5.8a2 2 0 0 1 1.3-1.3L21 12l-5.8-1.9a2 2 0 0 1-1.3-1.3Z"/></svg>' },
    { isim: "Rock", renk: "var(--color-rock)", ikon: '<svg width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z"/></svg>' },
    { isim: "Lo-Fi", renk: "var(--color-lofi)", ikon: '<svg width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M17 8h1a4 4 0 1 1 0 8h-1"/><path d="M3 8h14v9a4 4 0 0 1-4 4H7a4 4 0 0 1-4-4Z"/><line x1="6" y1="2" x2="6" y2="4"/><line x1="10" y1="2" x2="10" y2="4"/><line x1="14" y1="2" x2="14" y2="4"/></svg>' },
    { isim: "Cyberpunk", renk: "var(--color-cyber)", ikon: '<svg width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="4" y="4" width="16" height="16" rx="2" ry="2"/><rect x="9" y="9" width="6" height="6"/><line x1="9" y1="1" x2="9" y2="4"/><line x1="15" y1="1" x2="15" y2="4"/><line x1="9" y1="20" x2="9" y2="23"/><line x1="15" y1="20" x2="15" y2="23"/><line x1="20" y1="9" x2="23" y2="9"/><line x1="20" y1="14" x2="23" y2="14"/><line x1="1" y1="9" x2="4" y2="9"/><line x1="1" y1="14" x2="4" y2="14"/></svg>' },
    { isim: "Jazz", renk: "var(--color-jazz)", ikon: '<svg width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><circle cx="12" cy="12" r="3"/></svg>' },
    { isim: "Classical", renk: "var(--color-classic)", ikon: '<svg width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>' }
  ];
</script>

<div class="w-full min-h-full pb-32 flex flex-col relative bg-transparent text-(--text-main) transition-colors duration-500 custom-scrollbar overflow-y-auto">
  
  <div class="sticky top-0 z-30 px-8 lg:px-12 pt-10 pb-6 bg-linear-to-b from-(--bg-main) via-(--bg-main)/90 to-transparent backdrop-blur-md">
    <div class="relative group max-w-4xl mx-auto">
      <div class="absolute inset-y-0 left-0 flex items-center pl-6 pointer-events-none text-(--text-dim) group-focus-within:text-(--accent) transition-colors">
        <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
          <circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line>
        </svg>
      </div>
      
      <input
        type="text"
        bind:value={aramaMetni}
        placeholder="Şarkı, sanatçı veya albüm ara..."
        class="w-full bg-(--bg-surface) text-(--text-main) placeholder-(--text-dim)/50 text-lg lg:text-xl font-black rounded-2xl py-5 pl-16 pr-14 outline-none border border-(--border) focus:border-(--accent)/50 transition-all shadow-2xl focus:shadow-[0_10px_30px_var(--accent-glow)]"
      />

      {#if aramaMetni.length > 0}
        <button 
          type="button"
          onclick={() => aramaMetni = ""} 
          class="absolute inset-y-0 right-0 flex items-center pr-6 text-(--text-dim) hover:text-(--accent) transition-colors"
          aria-label="Temizle">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
        </button>
      {/if}
    </div>
  </div>

  <div class="px-8 lg:px-12 flex-1 max-w-7xl mx-auto w-full">
    {#if aramaMetni.trim() === ""}
      <div class="mt-4" in:fade>
        <h2 class="text-xs font-black text-(--text-dim) mb-6 uppercase tracking-[0.3em] flex items-center gap-4">
          Frekans Kategorileri <div class="h-px flex-1 bg-(--border)"></div>
        </h2>
        <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 gap-4 lg:gap-6 mb-16">
          {#each kategoriler as kategori, i}
            <a href="/search?q={kategori.isim}" 
               class="relative aspect-square rounded-(--radius) bg-(--bg-card) hover:bg-(--bg-card-hover) border border-(--border) p-5 cursor-pointer hover:-translate-y-1 hover:shadow-xl transition-all duration-300 flex flex-col justify-end group overflow-hidden"
               style="border-bottom-color: {kategori.renk};"
               in:scale={{ duration: 400, delay: i * 50 }}>
              <div class="absolute -bottom-10 -right-10 w-32 h-32 rounded-full blur-3xl opacity-10 group-hover:opacity-30 transition-opacity" style="background: {kategori.renk}"></div>
              
              <div class="absolute top-4 right-4 opacity-50 group-hover:opacity-100 group-hover:scale-110 transition-all duration-500" style="color: {kategori.renk}">
                  {@html kategori.ikon}
              </div>
              
              <span class="relative z-10 text-lg lg:text-xl font-black tracking-tighter uppercase italic drop-shadow-md group-hover:translate-x-1 transition-transform" style="color: {kategori.renk}">
                  {kategori.isim}
              </span>
            </a>
          {/each}
        </div>

        <h2 class="text-xs font-black text-(--text-dim) mb-6 uppercase tracking-[0.3em] flex items-center gap-4">
          Tüm Kayıtlar <span class="text-[9px] opacity-50 tracking-widest">(Popülerliğe Göre)</span> <div class="h-px flex-1 bg-[var(--border)]"></div>
        </h2>

        <div class="flex items-center text-[10px] font-black text-(--text-dim) border-b border-[var(--border)] pb-3 mb-4 px-4 sm:px-6 tracking-[0.2em] uppercase shrink-0">
          <span class="w-8 sm:w-10 shrink-0">#</span>
          <span class="flex-1 min-w-0 ml-2 sm:ml-4">KİMLİK & BİLGİ</span>
          <span class="w-auto max-w-50 xl:max-w-70 shrink-0 hidden lg:flex justify-end pr-4">METRİKLER</span> 
          <span class="w-32 sm:w-48 xl:w-56 shrink-0 text-right pr-2">İŞLEMLER</span> 
        </div>

        <div class="flex flex-col gap-2">
          {#each tumSarkilarSirali as sarki, index}
            <div 
              role="button" tabindex="0" 
              onclick={() => sarkiCal(sarki)} 
              onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(sarki)} 
              aria-label="{sarki.isim} çal"
              class="flex items-center p-2.5 px-4 sm:px-6 rounded-2xl hover:bg-[var(--bg-card-hover)] transition-all duration-300 cursor-pointer group border-t-2
              {playerState.aktifSarki?.id === sarki.id ? 'bg-[var(--accent)]/10 shadow-inner border-transparent' : 'border-transparent'}"
            >
              
              <div class="w-8 sm:w-10 shrink-0 flex items-center justify-start font-mono text-xs text-[var(--text-dim)]/40 relative">
                 {#if playerState.aktifSarki?.id === sarki.id && playerState.suAnOynuyorMu}
                    <div class="flex items-end gap-0.5 h-3">
                       <div class="w-1 bg-[var(--accent)] animate-[bounce_1s_infinite]"></div>
                       <div class="w-1 bg-[var(--accent)] animate-[bounce_1.2s_infinite]"></div>
                    </div>
                 {:else}
                    <span class="group-hover:hidden">{index + 1}</span>
                    <svg class="w-4 h-4 hidden group-hover:block text-[var(--accent)]" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
                 {/if}
              </div>
              
              <div class="flex-1 min-w-0 flex items-center gap-3 sm:gap-4 ml-2 sm:ml-4">
                <div class="w-10 h-10 sm:w-11 sm:h-11 bg-[var(--bg-card)] rounded-lg overflow-hidden shrink-0 shadow-md border border-[var(--border)]">
                  {#if sarki.kapak_yolu}
                    <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover transition-transform group-hover:scale-110" />
                  {:else}
                    <div class="w-full h-full flex items-center justify-center text-[var(--text-dim)]/20 bg-[var(--bg-surface)] font-black text-[10px] italic">LW</div>
                  {/if}
                </div>
                
                <div class="flex flex-col min-w-0 flex-1 pr-2">
                  <span class="font-black truncate text-sm lg:text-base tracking-tight {playerState.aktifSarki?.id === sarki.id ? 'text-[var(--accent)]' : 'text-[var(--text-main)]'}">
                    {sarki.isim}
                  </span>
                  <div class="flex items-center gap-2 mt-0.5 overflow-hidden">
                      <a href="/artist/{encodeURIComponent(sarki.sarkici)}" onclick={(e) => e.stopPropagation()} class="text-[10px] text-[var(--text-dim)] truncate font-bold uppercase tracking-widest opacity-80 group-hover:text-[var(--accent)] transition-colors inline-block max-w-max text-left">
                        {sarki.sarkici}
                      </a>
                      {#if sarki.album}
                          <span class="w-1 h-1 rounded-full bg-[var(--border)] shrink-0 hidden sm:block"></span>
                          <span class="text-[9px] text-[var(--text-dim)]/50 uppercase font-bold truncate hidden sm:block">
                            {sarki.album}
                          </span>
                      {/if}
                  </div>
                </div>
              </div>

              <div class="w-auto max-w-[200px] xl:max-w-[280px] shrink-0 hidden lg:flex items-center justify-end pr-4 pl-2 min-w-0">
                <SongStats {sarki} />
              </div>

              <div class="w-32 sm:w-48 xl:w-56 shrink-0 flex items-center justify-end gap-1 sm:gap-2 pr-2" role="presentation" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
                
                <select aria-label="Listeye Ekle" onchange={(e) => handlePlaylistEkle(sarki.id, e)} class="bg-[var(--bg-surface)] text-[9px] text-[var(--text-dim)] rounded-lg px-1.5 sm:px-2 py-1.5 outline-none border border-[var(--border)] hover:border-[var(--accent)]/50 cursor-pointer w-16 sm:w-20 font-bold uppercase transition-all focus:border-[var(--accent)] opacity-70 hover:opacity-100 hidden sm:block truncate">
                  <option value="">➕ LİSTE</option>
                  {#each playerState.playlistler as pl}
                    {#if !pl.sarkilar.includes(sarki.id)}<option value={pl.id}>{pl.isim.toUpperCase()}</option>{/if}
                  {/each}
                </select>

                <button type="button" aria-label="Düzenle" title="Bilgileri Düzenle" onclick={(e) => editModaliAc(sarki, e)} class="p-1.5 sm:p-2 text-[var(--text-dim)]/50 hover:text-[var(--accent)] hover:bg-[var(--accent)]/10 rounded-lg transition-all hidden sm:block shrink-0">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><path d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg>
                </button>

                <FavoriteButton sarkiId={sarki.id} />
                
                <button type="button" aria-label="Sil" title="Kalıcı Olarak Sil" onclick={(e) => handleSarkiSil(sarki, e)} class="text-[var(--text-dim)]/50 hover:text-red-500 hover:bg-red-500/10 rounded-lg transition-all p-1.5 sm:p-2 shrink-0">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
                </button>
              </div>
            </div>
          {/each}
        </div>
      </div>

    {:else if aramaSonuclari.length === 0}
      <div class="flex flex-col mt-10" in:fade>
        <div class="mb-12 text-center py-10 bg-[var(--bg-card)] border border-[var(--border)] rounded-[var(--radius)] border-dashed">
          <div class="text-5xl mb-4 opacity-30">🔍</div>
          <h3 class="text-2xl font-black text-[var(--text-main)] mb-2 tracking-tight">"{aramaMetni}" bulunamadı</h3>
          <p class="text-[var(--text-dim)] text-sm font-medium">Yazım hatası yapmış olabilir misin? Kütüphanende böyle bir veri yok.</p>
        </div>

        <div>
          <h4 class="text-[10px] font-black text-[var(--accent)] mb-6 uppercase tracking-[0.3em] flex items-center gap-4">
            Alternatif Olarak Bunları Dinleyebilirsin <div class="h-px flex-1 bg-[var(--border)]"></div>
          </h4>
          
          <div class="flex items-center text-[10px] font-black text-[var(--text-dim)] border-b border-[var(--border)] pb-3 mb-4 px-4 sm:px-6 tracking-[0.2em] uppercase shrink-0">
            <span class="w-8 sm:w-10 shrink-0">#</span>
            <span class="flex-1 min-w-0 ml-2 sm:ml-4">KİMLİK & BİLGİ</span>
            <span class="w-auto max-w-[200px] xl:max-w-[280px] shrink-0 hidden lg:flex justify-end pr-4">METRİKLER</span> 
            <span class="w-32 sm:w-48 xl:w-56 shrink-0 text-right pr-2">İŞLEMLER</span> 
          </div>

          <div class="flex flex-col gap-2">
            {#each enPopulerler as sarki, index}
              <div 
                role="button" tabindex="0" 
                onclick={() => sarkiCal(sarki)} 
                onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(sarki)} 
                aria-label="{sarki.isim} çal"
                class="flex items-center p-2.5 px-4 sm:px-6 rounded-2xl hover:bg-[var(--bg-card-hover)] transition-all duration-300 cursor-pointer group border-t-2
                {playerState.aktifSarki?.id === sarki.id ? 'bg-[var(--accent)]/10 shadow-inner border-transparent' : 'border-transparent'}"
              >
                
                <div class="w-8 sm:w-10 shrink-0 flex items-center justify-start font-mono text-xs text-[var(--text-dim)]/40 relative">
                   {#if playerState.aktifSarki?.id === sarki.id && playerState.suAnOynuyorMu}
                      <div class="flex items-end gap-0.5 h-3">
                         <div class="w-1 bg-[var(--accent)] animate-[bounce_1s_infinite]"></div>
                         <div class="w-1 bg-[var(--accent)] animate-[bounce_1.2s_infinite]"></div>
                      </div>
                   {:else}
                      <span class="group-hover:hidden">{index + 1}</span>
                      <svg class="w-4 h-4 hidden group-hover:block text-[var(--accent)]" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
                   {/if}
                </div>
                
                <div class="flex-1 min-w-0 flex items-center gap-3 sm:gap-4 ml-2 sm:ml-4">
                  <div class="w-10 h-10 sm:w-11 sm:h-11 bg-[var(--bg-card)] rounded-lg overflow-hidden shrink-0 shadow-md border border-[var(--border)]">
                    {#if sarki.kapak_yolu}
                      <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover transition-transform group-hover:scale-110" />
                    {:else}
                      <div class="w-full h-full flex items-center justify-center text-[var(--text-dim)]/20 bg-[var(--bg-surface)] font-black text-[10px] italic">LW</div>
                    {/if}
                  </div>
                  
                  <div class="flex flex-col min-w-0 flex-1 pr-2">
                    <span class="font-black truncate text-sm lg:text-base tracking-tight {playerState.aktifSarki?.id === sarki.id ? 'text-[var(--accent)]' : 'text-[var(--text-main)]'}">
                      {sarki.isim}
                    </span>
                    <div class="flex items-center gap-2 mt-0.5 overflow-hidden">
                        <a href="/artist/{encodeURIComponent(sarki.sarkici)}" onclick={(e) => e.stopPropagation()} class="text-[10px] text-[var(--text-dim)] truncate font-bold uppercase tracking-widest opacity-80 group-hover:text-[var(--accent)] transition-colors inline-block max-w-max text-left">
                          {sarki.sarkici}
                        </a>
                        {#if sarki.album}
                            <span class="w-1 h-1 rounded-full bg-[var(--border)] shrink-0 hidden sm:block"></span>
                            <span class="text-[9px] text-[var(--text-dim)]/50 uppercase font-bold truncate hidden sm:block">
                              {sarki.album}
                            </span>
                        {/if}
                    </div>
                  </div>
                </div>

                <div class="w-auto max-w-[200px] xl:max-w-[280px] shrink-0 hidden lg:flex items-center justify-end pr-4 pl-2 min-w-0">
                  <SongStats {sarki} />
                </div>

                <div class="w-32 sm:w-48 xl:w-56 shrink-0 flex items-center justify-end gap-1 sm:gap-2 pr-2" role="presentation" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
                  
                  <select aria-label="Listeye Ekle" onchange={(e) => handlePlaylistEkle(sarki.id, e)} class="bg-[var(--bg-surface)] text-[9px] text-[var(--text-dim)] rounded-lg px-1.5 sm:px-2 py-1.5 outline-none border border-[var(--border)] hover:border-[var(--accent)]/50 cursor-pointer w-16 sm:w-20 font-bold uppercase transition-all focus:border-[var(--accent)] opacity-70 hover:opacity-100 hidden sm:block truncate">
                    <option value="">➕ LİSTE</option>
                    {#each playerState.playlistler as pl}
                      {#if !pl.sarkilar.includes(sarki.id)}<option value={pl.id}>{pl.isim.toUpperCase()}</option>{/if}
                    {/each}
                  </select>

                  <button type="button" aria-label="Düzenle" title="Bilgileri Düzenle" onclick={(e) => editModaliAc(sarki, e)} class="p-1.5 sm:p-2 text-[var(--text-dim)]/50 hover:text-[var(--accent)] hover:bg-[var(--accent)]/10 rounded-lg transition-all hidden sm:block shrink-0">
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><path d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg>
                  </button>

                  <FavoriteButton sarkiId={sarki.id} />
                  
                  <button type="button" aria-label="Sil" title="Kalıcı Olarak Sil" onclick={(e) => handleSarkiSil(sarki, e)} class="text-[var(--text-dim)]/50 hover:text-red-500 hover:bg-red-500/10 rounded-lg transition-all p-1.5 sm:p-2 shrink-0">
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
                  </button>
                </div>
              </div>
            {/each}
          </div>
        </div>
      </div>

    {:else}
      <div class="mt-4" in:fade>
        
        <div class="flex items-center text-[10px] font-black text-[var(--text-dim)] border-b border-[var(--border)] pb-3 mb-4 px-4 sm:px-6 tracking-[0.2em] uppercase shrink-0">
          <span class="w-8 sm:w-10 shrink-0">#</span>
          <span class="flex-1 min-w-0 ml-2 sm:ml-4">KİMLİK & BİLGİ</span>
          <span class="w-auto max-w-[200px] xl:max-w-[280px] shrink-0 hidden lg:flex justify-end pr-4">METRİKLER</span> 
          <span class="w-32 sm:w-48 xl:w-56 shrink-0 text-right pr-2">İŞLEMLER</span> 
        </div>

        <div class="flex flex-col gap-2">
          {#each aramaSonuclari as sarki, index}
            <div 
              role="button" tabindex="0" 
              onclick={() => sarkiCal(sarki)} 
              onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(sarki)} 
              aria-label="{sarki.isim} çal"
              class="flex items-center p-2.5 px-4 sm:px-6 rounded-2xl hover:bg-[var(--bg-card-hover)] transition-all duration-300 cursor-pointer group border-t-2
              {playerState.aktifSarki?.id === sarki.id ? 'bg-[var(--accent)]/10 shadow-inner border-transparent' : 'border-transparent'}"
            >
              
              <div class="w-8 sm:w-10 shrink-0 flex items-center justify-start font-mono text-xs text-[var(--text-dim)]/40 relative">
                 {#if playerState.aktifSarki?.id === sarki.id && playerState.suAnOynuyorMu}
                    <div class="flex items-end gap-0.5 h-3">
                       <div class="w-1 bg-[var(--accent)] animate-[bounce_1s_infinite]"></div>
                       <div class="w-1 bg-[var(--accent)] animate-[bounce_1.2s_infinite]"></div>
                    </div>
                 {:else}
                    <span class="group-hover:hidden">{index + 1}</span>
                    <svg class="w-4 h-4 hidden group-hover:block text-[var(--accent)]" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
                 {/if}
              </div>
              
              <div class="flex-1 min-w-0 flex items-center gap-3 sm:gap-4 ml-2 sm:ml-4">
                <div class="w-10 h-10 sm:w-11 sm:h-11 bg-[var(--bg-card)] rounded-lg overflow-hidden shrink-0 shadow-md border border-[var(--border)]">
                  {#if sarki.kapak_yolu}
                    <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover transition-transform group-hover:scale-110" />
                  {:else}
                    <div class="w-full h-full flex items-center justify-center text-[var(--text-dim)]/20 bg-[var(--bg-surface)] font-black text-[10px] italic">LW</div>
                  {/if}
                </div>
                
                <div class="flex flex-col min-w-0 flex-1 pr-2">
                  <span class="font-black truncate text-sm lg:text-base tracking-tight {playerState.aktifSarki?.id === sarki.id ? 'text-[var(--accent)]' : 'text-[var(--text-main)]'}">
                    {sarki.isim}
                  </span>
                  <div class="flex items-center gap-2 mt-0.5 overflow-hidden">
                      <a href="/artist/{encodeURIComponent(sarki.sarkici)}" onclick={(e) => e.stopPropagation()} class="text-[10px] text-[var(--text-dim)] truncate font-bold uppercase tracking-widest opacity-80 group-hover:text-[var(--accent)] transition-colors inline-block max-w-max text-left">
                        {sarki.sarkici}
                      </a>
                      {#if sarki.album}
                          <span class="w-1 h-1 rounded-full bg-[var(--border)] shrink-0 hidden sm:block"></span>
                          <span class="text-[9px] text-[var(--text-dim)]/50 uppercase font-bold truncate hidden sm:block">
                            {sarki.album}
                          </span>
                      {/if}
                  </div>
                </div>
              </div>

              <div class="w-auto max-w-[200px] xl:max-w-[280px] shrink-0 hidden lg:flex items-center justify-end pr-4 pl-2 min-w-0">
                <SongStats {sarki} />
              </div>

              <div class="w-32 sm:w-48 xl:w-56 shrink-0 flex items-center justify-end gap-1 sm:gap-2 pr-2" role="presentation" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
                
                <select aria-label="Listeye Ekle" onchange={(e) => handlePlaylistEkle(sarki.id, e)} class="bg-[var(--bg-surface)] text-[9px] text-[var(--text-dim)] rounded-lg px-1.5 sm:px-2 py-1.5 outline-none border border-[var(--border)] hover:border-[var(--accent)]/50 cursor-pointer w-16 sm:w-20 font-bold uppercase transition-all focus:border-[var(--accent)] opacity-70 hover:opacity-100 hidden sm:block truncate">
                  <option value="">➕ LİSTE</option>
                  {#each playerState.playlistler as pl}
                    {#if !pl.sarkilar.includes(sarki.id)}<option value={pl.id}>{pl.isim.toUpperCase()}</option>{/if}
                  {/each}
                </select>

                <button type="button" aria-label="Düzenle" title="Bilgileri Düzenle" onclick={(e) => editModaliAc(sarki, e)} class="p-1.5 sm:p-2 text-[var(--text-dim)]/50 hover:text-[var(--accent)] hover:bg-[var(--accent)]/10 rounded-lg transition-all hidden sm:block shrink-0">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><path d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg>
                </button>

                <FavoriteButton sarkiId={sarki.id} />
                
                <button type="button" aria-label="Sil" title="Kalıcı Olarak Sil" onclick={(e) => handleSarkiSil(sarki, e)} class="text-[var(--text-dim)]/50 hover:text-red-500 hover:bg-red-500/10 rounded-lg transition-all p-1.5 sm:p-2 shrink-0">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
                </button>
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  @keyframes bounce {
    0%, 100% { height: 4px; }
    50% { height: 14px; }
  }

  input::selection {
    background: var(--accent);
    color: white;
  }

  .custom-scrollbar::-webkit-scrollbar { width: 4px; }
  .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
  .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--accent); }
</style>