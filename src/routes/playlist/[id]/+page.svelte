<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { page } from '$app/state'; 
  import FavoriteButton from '$lib/FavoriteButton.svelte';
  import SongStats from '$lib/SongStats.svelte';
  import ExportPlaylistButton from '$lib/ExportPlaylistButton.svelte';
  import { 
    playerState, 
    sarkiCal, 
    type Sarki, 
    initializePlayer, 
    playlisttenSarkiCikar, 
    sarkiSil,
    siraGuncelle
  } from '../../../store.svelte';
  import { fade } from 'svelte/transition';

  let playlistId = $derived(page.params.id);
  let aktifPlaylist = $derived(playerState.playlistler?.find(p => p.id === playlistId));

  let gosterilenSarkilar = $derived(
    playerState.sarkiListesi.filter(sarki => aktifPlaylist?.sarkilar?.includes(sarki.id))
  );

  let playlistKapakGorseli = $derived(
    gosterilenSarkilar.length > 0 && gosterilenSarkilar[0].kapak_yolu 
      ? convertFileSrc(gosterilenSarkilar[0].kapak_yolu) 
      : null
  );

  onMount(async () => {
    if (playerState.sarkiListesi.length === 0 || playerState.playlistler.length === 0) {
      await initializePlayer();
    }
  });

  function listeyiCal() {
    if (gosterilenSarkilar.length > 0) {
      sarkiCal(gosterilenSarkilar[0]);
    }
  }

  async function handleListedenCikar(sarkiId: string, isim: string, event: Event) {
    event.preventDefault();
    event.stopPropagation();
    if (!playlistId) return; 
    
    if (confirm(`"${isim}" adlı şarkıyı bu listeden çıkarmak istediğinize emin misiniz?`)) {
        await playlisttenSarkiCikar(playlistId, sarkiId);
    }
  }

  async function handleKalicSarkiSil(sarki: Sarki, event: MouseEvent | KeyboardEvent) {
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

  let suruklenenIndex = $state<number | null>(null);
  let uzerindeGezinilenIndex = $state<number | null>(null);

  function dragBasla(event: DragEvent, index: number) {
    suruklenenIndex = index;
    if (event.dataTransfer) {
        event.dataTransfer.effectAllowed = "move";
    }
  }

  function dragUzerinde(event: DragEvent, index: number) {
    event.preventDefault(); 
    uzerindeGezinilenIndex = index;
  }

  async function drop(event: DragEvent, hedefIndex: number) {
    event.preventDefault();
    if (suruklenenIndex !== null && suruklenenIndex !== hedefIndex && aktifPlaylist) {
      let yeniSira = [...aktifPlaylist.sarkilar];
      const [suruklenenSarkiId] = yeniSira.splice(suruklenenIndex, 1); 
      yeniSira.splice(hedefIndex, 0, suruklenenSarkiId); 
      
      aktifPlaylist.sarkilar = yeniSira;
      
      const sarkiYeniListe = yeniSira.map(id => playerState.sarkiListesi.find(s => s.id === id)).filter(Boolean) as Sarki[];
      if (playerState.aktifSarki && sarkiYeniListe.some(s => s.id === playerState.aktifSarki?.id)) {
          await siraGuncelle(sarkiYeniListe);
      }
    }
    dragBitir();
  }

  function dragBitir() {
    suruklenenIndex = null;
    uzerindeGezinilenIndex = null;
  }
</script>

<div class="p-8 lg:p-12 w-full min-h-full pb-32 flex flex-col relative min-w-0 bg-transparent text-[var(--text-main)] transition-colors duration-500 overflow-y-auto custom-scrollbar">
  
  <header class="flex flex-col md:flex-row gap-8 items-center md:items-end mb-12 mt-4" in:fade>
    <div class="w-52 h-52 lg:w-64 lg:h-64 bg-[var(--bg-card)] rounded-[var(--radius)] shadow-2xl flex-shrink-0 border border-white/10 overflow-hidden relative group flex items-center justify-center">
        {#if playlistKapakGorseli}
          <img src={playlistKapakGorseli} alt="Playlist Kapağı" class="w-full h-full object-cover transition-transform duration-700 group-hover:scale-110 opacity-80" />
          <div class="absolute inset-0 bg-black/20 group-hover:bg-transparent transition-colors duration-500"></div>
        {:else}
          <div class="absolute inset-0 bg-gradient-to-br from-[var(--accent)]/20 to-[var(--bg-surface)]"></div>
        {/if}
        <svg class="absolute w-24 h-24 text-white/30 drop-shadow-2xl group-hover:scale-110 transition-transform duration-500 z-10 pointer-events-none" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 14.5c-2.49 0-4.5-2.01-4.5-4.5S9.51 7.5 12 7.5s4.5 2.01 4.5 4.5-2.01 4.5-4.5 4.5zm0-5.5c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1z"/>
        </svg>
    </div>
    
    <div class="flex flex-col text-center md:text-left pb-2 min-w-0 flex-1">
      <span class="text-[10px] font-black mb-3 text-[var(--accent)] tracking-[0.4em] uppercase italic truncate">User Playlist Collection</span>
      <h1 class="text-4xl lg:text-7xl font-black tracking-tighter leading-none mb-6 uppercase italic drop-shadow-md truncate">
        {aktifPlaylist?.isim || "Yükleniyor..."}
      </h1>
      
      <div class="flex flex-col sm:flex-row items-center gap-6 mt-2">
        <p class="text-[var(--text-dim)] text-xs lg:text-sm font-bold uppercase tracking-widest whitespace-nowrap truncate">
          {gosterilenSarkilar.length} Benzersiz Kayıt • Sıralı Liste
        </p>
        
        <div class="flex items-center gap-3">
            {#if gosterilenSarkilar.length > 0 && aktifPlaylist}
                <ExportPlaylistButton {aktifPlaylist} sarkilar={gosterilenSarkilar} />
            {/if}

            {#if gosterilenSarkilar.length > 0}
              <button 
                type="button"
                onclick={listeyiCal} 
                class="flex items-center gap-3 bg-[var(--text-main)] text-[var(--bg-main)] hover:bg-[var(--accent)] hover:text-white px-10 py-2.5 rounded-lg font-black shadow-xl transition-all active:scale-95 uppercase text-[10px] lg:text-xs tracking-widest shrink-0"
                aria-label="Listeyi oynat"
              >
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
                Listeyi Çal
              </button>
            {/if}
        </div>
      </div>
    </div>
  </header>

  {#if gosterilenSarkilar.length === 0}
    <div class="flex flex-col items-center justify-center flex-1 text-center mt-10 p-16 bg-[var(--bg-card)] border border-[var(--border)] rounded-[var(--radius)] border-dashed" in:fade>
      <div class="text-6xl mb-8 opacity-20 filter grayscale">💽</div>
      <h3 class="text-2xl font-bold mb-3 tracking-tight uppercase">Sinyal Bulunamadı</h3>
      <p class="text-[var(--text-dim)] font-medium text-sm max-w-sm leading-relaxed uppercase tracking-wider">
        Bu çalma listesi henüz kütüphaneden veri almamış. "Keşfet" veya "Kütüphane" sekmelerinden şarkı ekleyebilirsin.
      </p>
    </div>
  {:else}
    <div class="flex items-center text-[10px] font-black text-[var(--text-dim)] border-b border-[var(--border)] pb-3 mb-4 px-6 tracking-[0.2em] uppercase shrink-0">
      <span class="w-12 shrink-0">#</span>
      <span class="flex-1 min-w-0 ml-4">KİMLİK & BİLGİ</span>
      <span class="w-40 lg:w-64 2xl:w-80 shrink-0 hidden md:block text-right pr-4">METRİKLER</span> 
      <span class="w-52 sm:w-64 shrink-0 text-right">YÖNETİM</span> 
    </div>

    <div class="flex flex-col gap-1.5">
      {#each gosterilenSarkilar as sarki, index}
        <div 
            role="button" 
            tabindex="0" 
            draggable="true"
            ondragstart={(e) => dragBasla(e, index)}
            ondragover={(e) => dragUzerinde(e, index)}
            ondrop={(e) => drop(e, index)}
            ondragend={dragBitir}
            onclick={() => sarkiCal(sarki)} 
            onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(sarki)} 
            aria-label="{sarki.isim} çal"
            class="flex items-center p-2.5 px-6 rounded-2xl transition-all duration-300 group cursor-pointer border-t-2
                   {playerState.aktifSarki?.id === sarki.id ? 'bg-[var(--accent)]/10 shadow-inner border-transparent' : 'border-transparent hover:bg-[var(--bg-card-hover)]'}
                   {uzerindeGezinilenIndex === index ? '!border-[var(--accent)] bg-[var(--accent)]/5' : ''}"
        >
          <div class="w-12 shrink-0 flex items-center justify-start relative">
             <div class="mr-2 text-[var(--text-dim)]/20 hover:text-[var(--accent)] cursor-grab active:cursor-grabbing opacity-0 group-hover:opacity-100 transition-all shrink-0" title="Sıralamayı Değiştir">
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M9 3v2H7V3h2zm0 8v2H7v-2h2zm0 8v2H7v-2h2zm6-16v2h-2V3h2zm0 8v2h-2v-2h2zm0 8v2h-2v-2h2z"/></svg>
             </div>
             <div class="font-mono text-xs text-[var(--text-dim)]/40 flex items-center justify-center w-4">
                 {#if playerState.aktifSarki?.id === sarki.id && playerState.suAnOynuyorMu}
                    <div class="flex items-end gap-0.5 h-3">
                        <div class="w-1 bg-[var(--accent)] animate-[bounce_1s_infinite]"></div>
                        <div class="w-1 bg-[var(--accent)] animate-[bounce_1.2s_infinite]"></div>
                    </div>
                 {:else}
                    <span class="group-hover:hidden">{index + 1}</span>
                    <svg class="w-4 h-4 hidden group-hover:block text-[var(--accent)] absolute left-6" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
                 {/if}
             </div>
          </div>
          
          <div class="flex-1 min-w-0 flex items-center gap-4 ml-4">
            <div class="w-11 h-11 bg-[var(--bg-card)] rounded-lg overflow-hidden shrink-0 shadow-md border border-[var(--border)]">
              {#if sarki.kapak_yolu}
                <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover transition-transform group-hover:scale-110" />
              {:else}
                <div class="w-full h-full flex items-center justify-center text-[10px] opacity-20 bg-[var(--bg-surface)] italic font-bold">LW</div>
              {/if}
            </div>
            
            <div class="flex flex-col min-w-0 flex-1 pr-2">
              <span class="font-black text-sm lg:text-base truncate tracking-tight {playerState.aktifSarki?.id === sarki.id ? 'text-[var(--accent)]' : 'text-[var(--text-main)]'}">
                {sarki.isim}
              </span>
              <div class="flex items-center gap-2 mt-0.5 overflow-hidden">
                <button onclick={(e) => { e.stopPropagation();  }} class="text-[10px] text-[var(--text-dim)] truncate font-bold uppercase tracking-widest hover:text-[var(--accent)] transition-colors inline-block max-w-max opacity-80 text-left">
                  {sarki.sarkici}
                </button>
                {#if sarki.album}
                  <span class="w-1 h-1 rounded-full bg-[var(--border)] shrink-0 hidden sm:block"></span>
                  <span class="text-[9px] text-[var(--text-dim)]/50 uppercase font-bold truncate hidden sm:block">
                    {sarki.album}
                  </span>
                {/if}
              </div>
            </div>
          </div>

          <div class="w-40 lg:w-64 2xl:w-80 shrink-0 hidden md:flex items-center justify-end pr-4">
            <SongStats {sarki} />
          </div>
          
          <div class="w-52 sm:w-64 flex items-center justify-end gap-2 shrink-0" role="presentation" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
            <FavoriteButton sarkiId={sarki.id} />
            
            <button 
                type="button" 
                aria-label="Düzenle" 
                title="Bilgileri Düzenle" 
                onclick={(e) => { e.stopPropagation(); playerState.duzenlenecekSarki = sarki; playerState.isEditModalOpen = true; }} 
                class="p-2 text-[var(--text-dim)]/60 hover:text-[var(--accent)] hover:bg-[var(--accent)]/10 rounded-lg transition-all"
            >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><path d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg>
            </button>

            <button 
                type="button" 
                aria-label="Listeden Çıkar" 
                title="Sadece Bu Listeden Kaldır" 
                onclick={(e) => handleListedenCikar(sarki.id, sarki.isim, e)} 
                class="p-2 text-[var(--text-dim)]/60 hover:text-orange-400 hover:bg-orange-400/10 rounded-lg transition-all"
            >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
            </button>

            <button 
                type="button" 
                aria-label="Kalıcı Sil" 
                title="Kütüphaneden ve Diskten Sil" 
                onclick={(e) => handleKalicSarkiSil(sarki, e)} 
                class="p-2 text-[var(--text-dim)]/60 hover:text-red-500 hover:bg-red-500/10 rounded-lg transition-all"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  @keyframes bounce {
    0%, 100% { height: 4px; }
    50% { height: 14px; }
  }

  .custom-scrollbar::-webkit-scrollbar { width: 4px; }
  .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--accent); }

  .cursor-grab:active { cursor: grabbing; }
</style>