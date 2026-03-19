<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { page } from '$app/state'; 
  import FavoriteButton from '$lib/FavoriteButton.svelte';
  import SongStats from '$lib/SongStats.svelte';
  // Fonksiyonlarımızı merkezi store'dan alıyoruz
  import { playerState, sarkiCal, initializePlayer, playlisttenSarkiCikar } from '../../../store.svelte';

  let playlistId = $derived(page.params.id);
  
  let aktifPlaylist = $derived(playerState.playlistler?.find(p => p.id === playlistId));

  let gosterilenSarkilar = $derived(
    playerState.sarkiListesi.filter(sarki => aktifPlaylist?.sarkilar?.includes(sarki.id))
  );

  // Veri Kontrolü: Tek satıra düştü
  onMount(async () => {
    if (playerState.sarkiListesi.length === 0 || playerState.playlistler.length === 0) {
      await initializePlayer();
    }
  });

  let playlistKapakGorseli = $derived(
    gosterilenSarkilar.length > 0 && gosterilenSarkilar[0].kapak_yolu 
      ? convertFileSrc(gosterilenSarkilar[0].kapak_yolu) 
      : null
  );

  function listeyiCal() {
    if (gosterilenSarkilar.length > 0) {
      sarkiCal(gosterilenSarkilar[0]);
    }
  }

  // DÜZELTİLDİ: Onay penceresi (confirm) buraya eklendi
  async function handleListedenCikar(sarkiId: string, isim: string, event: Event) {
    event.stopPropagation();
    if (!playlistId) return; 
    
    if (confirm(`"${isim}" adlı şarkıyı bu listeden çıkarmak istediğinize emin misiniz?`)) {
        await playlisttenSarkiCikar(playlistId, sarkiId);
    }
  }
</script>

<div class="p-10 w-full min-h-full pb-32 flex flex-col relative min-w-0">
  
  <div class="flex gap-8 items-end mb-10 mt-4">
    <div class="w-56 h-56 bg-gradient-to-br from-indigo-900 to-purple-900 rounded-xl shadow-[0_10px_40px_rgba(0,0,0,0.3)] flex-shrink-0 border-4 border-white/10 overflow-hidden relative group flex items-center justify-center">
      {#if playlistKapakGorseli}
        <img src={playlistKapakGorseli} alt="Playlist Kapağı" class="w-full h-full object-cover opacity-60 group-hover:scale-110 transition-transform duration-700" />
      {/if}
      <div class="absolute inset-0 bg-black/20"></div>
      <svg class="absolute w-24 h-24 text-white/50 drop-shadow-lg group-hover:scale-110 transition-transform duration-500 z-10" fill="currentColor" viewBox="0 0 24 24">
        <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 14.5c-2.49 0-4.5-2.01-4.5-4.5S9.51 7.5 12 7.5s4.5 2.01 4.5 4.5-2.01 4.5-4.5 4.5zm0-5.5c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1z"/>
      </svg>
    </div>
    
    <div class="flex flex-col pb-2">
      <span class="text-sm font-semibold mb-2 text-white/80 tracking-widest uppercase">Lain Wave Playlist</span>
      <h1 class="text-6xl font-black tracking-tight leading-tight mb-6 drop-shadow-md uppercase">
        {aktifPlaylist?.isim || "Yükleniyor..."}
      </h1>
      <div class="flex items-center gap-4">
        <p class="text-white/70 text-sm font-medium">
          {gosterilenSarkilar.length} Şarkı
        </p>
        
        {#if gosterilenSarkilar.length > 0}
          <button onclick={listeyiCal} class="flex items-center gap-2 bg-pink-500 hover:bg-pink-400 text-white px-6 py-2 rounded-full font-bold shadow-lg transition-all hover:scale-105 ml-4">
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
            Oynat
          </button>
        {/if}
      </div>
    </div>
  </div>

  {#if gosterilenSarkilar.length === 0}
    <div class="flex flex-col items-center justify-center flex-1 text-center mt-10 p-10 bg-white/5 border border-white/10 rounded-2xl border-dashed">
      <div class="text-7xl mb-6 opacity-50 drop-shadow-xl transform hover:scale-110 transition-transform duration-500 cursor-default">💽</div>
      <h3 class="text-3xl font-black text-white mb-3 tracking-tight">Bu Liste Boş</h3>
      <p class="text-gray-300 font-medium text-lg max-w-md">"Kütüphane" veya "Ara" sekmesine giderek bu listeye müzik ekleyebilirsiniz.</p>
    </div>
  {:else}
    <div class="flex text-xs font-semibold text-white/50 border-b border-white/10 pb-2 mb-4 px-2 mt-4">
      <span class="w-12 text-center shrink-0">#</span>
      <span class="flex-1 min-w-0">TITLE</span>
      <span class="w-48 shrink-0 text-right pr-4">STATS</span> 
      <span class="w-1/4 shrink-0 pl-4">ALBUM</span>
      <span class="w-20 text-center shrink-0">ACTIONS</span>
    </div>

    <div class="flex flex-col gap-2">
      {#each gosterilenSarkilar as sarki, index}
        <div role="button" tabindex="0" onclick={() => sarkiCal(sarki)} onkeydown={(e) => e.key === 'Enter' && sarkiCal(sarki)} class="flex items-center text-sm p-2 rounded-lg hover:bg-white/10 transition-all duration-200 cursor-pointer group {playerState.aktifSarki?.id === sarki.id ? 'bg-white/15 shadow-inner' : ''}">
          <span class="w-12 text-center text-white/50 group-hover:hidden font-medium shrink-0">{index + 1}</span>
          <span class="w-12 text-center hidden group-hover:flex items-center justify-center text-white shrink-0">
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
          </span>
          
          <div class="flex-1 flex items-center gap-4 min-w-0">
            <div class="w-10 h-10 bg-black/30 rounded overflow-hidden shrink-0 shadow-sm">
              {#if sarki.kapak_yolu}
                <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover" />
              {:else}
                <div class="w-full h-full flex items-center justify-center text-xs text-white/30">
                  <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/></svg>
                </div>
              {/if}
            </div>
            
            <div class="flex flex-col min-w-0 gap-0.5 pr-2">
              <span class="font-bold truncate {playerState.aktifSarki?.id === sarki.id ? 'text-pink-400' : 'text-white'}">{sarki.isim}</span>
              <a href="/artist/{encodeURIComponent(sarki.sarkici)}" class="text-xs text-white/50 truncate font-medium hover:text-pink-400 transition-colors" onclick={(e) => e.stopPropagation()}>
                {sarki.sarkici}
              </a>
            </div>
          </div>

          <div class="w-48 shrink-0 flex items-center justify-end pr-4">
            <SongStats {sarki} />
          </div>

          <span class="w-1/4 text-white/50 truncate font-medium pl-4 shrink-0">{sarki.album}</span>
          
          <div class="w-20 text-center shrink-0 flex items-center justify-center gap-4" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="button" tabindex="0">
            <FavoriteButton sarkiId={sarki.id} />
            
            <button type="button" aria-label="Listeden Çıkar" title="Listeden Kaldır" onclick={(e) => handleListedenCikar(sarki.id, sarki.isim, e)} class="text-white/30 hover:text-red-500 transition-colors">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path><line x1="10" y1="11" x2="10" y2="17"></line><line x1="14" y1="11" x2="14" y2="17"></line></svg>
            </button>
          </div>

        </div>
      {/each}
    </div>
  {/if}
</div>