<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import FavoriteButton from '$lib/FavoriteButton.svelte';
  import SongStats from '$lib/SongStats.svelte';
  import { 
    playerState, 
    sarkiCal, 
    initializePlayer, 
    siraGuncelle, 
    handleSarkiSil,
    handlePlaylistEkle
  } from '../../store.svelte';
  import { fade } from 'svelte/transition';

  onMount(async () => {
    if(playerState.sarkiListesi.length === 0) {
         await initializePlayer();
    }
  });


  let suruklenenIndex = $state<number | null>(null);
  let uzerindeGezinilenIndex = $state<number | null>(null);

  function dragBasla(event: DragEvent, index: number) {
    suruklenenIndex = index;
    if (event.dataTransfer) event.dataTransfer.effectAllowed = "move";
  }

  function dragUzerinde(event: DragEvent, index: number) {
    event.preventDefault(); 
    uzerindeGezinilenIndex = index;
  }

  async function drop(event: DragEvent, hedefIndex: number) {
    event.preventDefault();
    if (suruklenenIndex !== null && suruklenenIndex !== hedefIndex) {
      let yeniListe = [...playerState.sarkiListesi];
      const [suruklenenSarki] = yeniListe.splice(suruklenenIndex, 1); 
      yeniListe.splice(hedefIndex, 0, suruklenenSarki); 
      await siraGuncelle(yeniListe);
    }
    dragBitir();
  }

  function dragBitir() {
    suruklenenIndex = null;
    uzerindeGezinilenIndex = null;
  }
</script>

<div class="p-8 lg:p-12 w-full min-h-full pb-32 flex flex-col relative min-w-0 bg-transparent text-(--text-main) transition-colors duration-500 overflow-y-auto custom-scrollbar">
  
  <header class="flex flex-col md:flex-row gap-8 items-center md:items-end mb-12 mt-4" in:fade>
    <div class="w-48 h-48 lg:w-56 lg:h-56 bg-linear-to-br from-(--accent) via-(--accent-sec) to-(--bg-surface) rounded-(--radius) shadow-2xl shrink-0 border border-white/10 flex items-center justify-center relative group">
        <div class="absolute inset-0 bg-black/10 group-hover:bg-transparent transition-colors duration-500"></div>
        <span class="text-7xl lg:text-8xl drop-shadow-2xl group-hover:scale-110 transition-transform duration-700 select-none">🎧</span>
    </div>
    
    <div class="flex flex-col text-center md:text-left pb-2 min-w-0 flex-1">
      <span class="text-[10px] font-black mb-3 text-(--accent) tracking-[0.4em] uppercase italic">Local Frequency Archive</span>
      <h1 class="text-5xl lg:text-7xl font-black tracking-tighter leading-none mb-6 uppercase italic drop-shadow-md truncate">Kütüphane</h1>
      <div class="flex flex-col sm:flex-row items-center gap-6">
        <p class="text-(--text-dim) text-xs lg:text-sm font-bold uppercase tracking-widest">
          {playerState.sarkiListesi.length} Veri Bloğu • Çevrimiçi
        </p>
        <button type="button" onclick={() => playerState.isAddMusicModalOpen = true} class="flex items-center gap-3 bg-(--text-main) text-(--bg-main) hover:bg-(--accent) hover:text-white px-10 py-3.5 rounded-full font-black shadow-xl transition-all active:scale-95 uppercase text-[10px] lg:text-xs tracking-widest">
            Yeni Parça Ekle
        </button>
      </div>
    </div>
  </header>

  {#if playerState.sarkiListesi.length === 0}
    <div class="flex flex-col items-center justify-center flex-1 p-16 bg-(--bg-card) border border-(--border) rounded-(--radius) border-dashed opacity-50" in:fade>
      <span class="text-6xl mb-4">📡</span>
      <h3 class="text-xl font-bold uppercase tracking-widest">Sinyal Yok</h3>
    </div>
  {:else}
    <div class="flex items-center text-[10px] font-black text-(--text-dim) border-b border-(--border) pb-3 mb-4 px-6 tracking-[0.2em] uppercase shrink-0">
      <span class="w-10 shrink-0">#</span>
      <span class="flex-1 min-w-0 ml-4">KİMLİK & BİLGİ</span>
      <span class="w-40 lg:w-64 2xl:w-80 shrink-0 hidden md:block text-right pr-4">METRİKLER</span> 
      <span class="w-48 sm:w-56 shrink-0 text-right">YÖNETİM</span> 
    </div>

    <div class="flex flex-col gap-1.5">
      {#each playerState.sarkiListesi as sarki, index}
        <div 
          role="button" 
          tabindex="0" 
          draggable="true"
          class="flex items-center p-2.5 px-6 rounded-2xl transition-all duration-300 group cursor-pointer border-t-2
                 {playerState.aktifSarki?.id === sarki.id ? 'bg-(--accent)/10 shadow-inner border-transparent' : 'border-transparent hover:bg-(--bg-card-hover)'}
                 {uzerindeGezinilenIndex === index ? 'border-(--accent)! bg-(--accent)/5' : ''}"
          onclick={() => sarkiCal(sarki)}
          onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(sarki)}
          ondragstart={(e) => dragBasla(e, index)}
          ondragover={(e) => dragUzerinde(e, index)}
          ondrop={(e) => drop(e, index)}
          ondragend={dragBitir}
        >
          <div class="w-10 shrink-0 flex items-center justify-start font-mono text-xs text-(--text-dim)/40 relative">
             {#if playerState.aktifSarki?.id === sarki.id && playerState.suAnOynuyorMu}
                <div class="flex items-end gap-0.5 h-3">
                    <div class="w-1 bg-(--accent) animate-bounce"></div>
                    <div class="w-1 bg-(--accent) animate-[bounce_1.2s_infinite]"></div>
                </div>
             {:else}
                <span class="group-hover:hidden">{index + 1}</span>
                <svg class="w-4 h-4 hidden group-hover:block text-(--accent)" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
             {/if}
          </div>
          
          <div class="flex-1 min-w-0 flex items-center gap-4 ml-4">
            <div class="w-11 h-11 bg-(--bg-card) rounded-lg overflow-hidden shrink-0 border border-(--border) shadow-sm">
              {#if sarki.kapak_yolu}
                <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover transition-transform group-hover:scale-110" />
              {:else}
                <div class="w-full h-full flex items-center justify-center text-[10px] opacity-20 bg-(--bg-surface) italic font-bold">LW</div>
              {/if}
            </div>
            <div class="flex flex-col min-w-0 flex-1">
              <span class="font-black truncate text-sm lg:text-base tracking-tight {playerState.aktifSarki?.id === sarki.id ? 'text-(--accent)' : 'text-(--text-main)'}">
                {sarki.isim}
              </span>
              <div class="flex items-center gap-2 mt-0.5 overflow-hidden">
                  <span class="text-[10px] text-(--text-dim) truncate font-bold uppercase tracking-widest opacity-80 group-hover:text-(--accent) transition-colors">
                    {sarki.sarkici}
                  </span>
                  {#if sarki.album}
                      <span class="w-1 h-1 rounded-full bg-(--border) shrink-0 hidden sm:block"></span>
                      <span class="text-[9px] text-(--text-dim)/50 uppercase font-bold truncate hidden sm:block">
                        {sarki.album}
                      </span>
                  {/if}
              </div>
            </div>
          </div>

          <div class="w-40 lg:w-64 2xl:w-80 shrink-0 hidden md:flex items-center justify-end pr-4">
            <SongStats {sarki} />
          </div>

          <div class="w-48 sm:w-56 shrink-0 flex items-center justify-end gap-2 md:gap-3" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="presentation">
            
            <FavoriteButton sarkiId={sarki.id} />
            
            <button 
                type="button"
                onclick={(e) => { e.stopPropagation(); playerState.duzenlenecekSarki = sarki; playerState.isEditModalOpen = true; }} 
                class="p-2 text-(--text-dim)/60 hover:text-(--accent) hover:bg-(--accent)/10 rounded-lg transition-all"
                aria-label="Düzenle"
                title="Bilgileri Düzenle"
            >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><path d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg>
            </button>

            <select 
                aria-label="Listeye Ekle" 
                onchange={(e) => handlePlaylistEkle(sarki.id, e)} 
                class="bg-(--bg-surface) text-[9px] text-(--text-dim) rounded-lg px-2 py-1.5 outline-none border border-(--border) hover:border-(--accent)/50 cursor-pointer w-20 sm:w-24 font-bold uppercase transition-all focus:border-(--accent) opacity-70 hover:opacity-100"
            >
              <option value="">➕ LİSTE</option>
              {#each playerState.playlistler as pl}
                {#if !pl.sarkilar.includes(sarki.id)}
                  <option value={pl.id}>{pl.isim.toUpperCase()}</option>
                {/if}
              {/each}
            </select>

            <button 
                type="button" 
                aria-label="Sil" 
                title="Kalıcı Olarak Sil"
                onclick={(e) => handleSarkiSil(sarki, e)} 
                class="p-2 text-(--text-dim)/60 hover:text-red-500 hover:bg-red-500/10 rounded-lg transition-all"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"></path></svg>
            </button>

          </div>
          
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  @keyframes bounce { 0%, 100% { height: 4px; } 50% { height: 12px; } }
  .custom-scrollbar::-webkit-scrollbar { width: 4px; }
  .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--accent); }
</style>