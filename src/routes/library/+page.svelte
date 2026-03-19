<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import FavoriteButton from '$lib/FavoriteButton.svelte';
  import SongStats from '$lib/SongStats.svelte';
  // Fonksiyonlarımızı store'dan içe aktarıyoruz
  import { 
    playerState, 
    sarkiCal, 
    type Sarki, 
    initializePlayer, 
    sarkiPlaylisteEkle, 
    sarkiSil, 
    siraGuncelle 
  } from '../../store.svelte';

  onMount(async () => {
    // Verileri getirme işlemi artık store.svelte.ts içerisindeki initializePlayer ile yapılıyor
    if(playerState.sarkiListesi.length === 0) {
         await initializePlayer();
    }
  });

  // --- UI TETİKLEYİCİLERİ (Event Handlers) ---

  async function handlePlaylistEkle(sarkiId: string, event: Event) {
    const selectElement = event.target as HTMLSelectElement;
    const playlistId = selectElement.value;
    const basarili = await sarkiPlaylisteEkle(sarkiId, playlistId);
    if(basarili) {
        selectElement.value = ""; // Seçimi sıfırla
    }
  }

  // DÜZELTİLDİ: Onay penceresi (confirm) buraya eklendi ve async yapıldı
  async function handleSarkiSil(sarki: Sarki, event: Event) {
    event.stopPropagation();
    if (confirm(`"${sarki.isim}" adlı parçayı kütüphaneden tamamen silmek istediğinize emin misiniz?`)) {
        await sarkiSil(sarki);
    }
  }

  // --- SÜRÜKLE BIRAK (DRAG & DROP) ---
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
      
      // İşlemi store'a devret
      await siraGuncelle(yeniListe);
    }
    dragBitir();
  }

  function dragBitir() {
    suruklenenIndex = null;
    uzerindeGezinilenIndex = null;
  }
</script>

<div class="p-10 w-full min-h-full pb-32 flex flex-col">
  <div class="flex gap-8 items-end mb-12">
    <div class="w-56 h-56 bg-gradient-to-br from-pink-400 to-purple-600 rounded-xl shadow-[0_10px_30px_rgba(236,72,153,0.3)] flex-shrink-0 border-4 border-white/10 overflow-hidden relative group">
      <div class="absolute inset-0 flex items-center justify-center text-8xl opacity-80 group-hover:scale-110 transition-transform duration-500">🎵</div>
      <div class="absolute inset-0 bg-black/10"></div>
    </div>
    
    <div class="flex flex-col pb-2">
      <span class="text-sm font-semibold mb-2 text-white/80 tracking-widest uppercase">Lain Wave Database</span>
      <h1 class="text-6xl font-black tracking-tight leading-tight mb-4 drop-shadow-md uppercase">TÜM MÜZİKLER</h1>
      <p class="text-white/70 text-sm font-medium">{playerState.sarkiListesi.length} songs • Oynatılmaya hazır</p>
    </div>
  </div>

  {#if playerState.sarkiListesi.length === 0}
    <div class="flex flex-col items-center justify-center mt-10 p-10 bg-white/5 border border-white/10 rounded-2xl border-dashed">
      <div class="text-6xl mb-4 opacity-50">🎧</div>
      <h3 class="text-2xl font-bold text-white mb-2">Kütüphaneniz Bomboş</h3>
      <p class="text-gray-400 mb-8 max-w-md text-center">Hemen bilgisayarınızdan veya internetten parçalar aktararak kendi Lain Wave arşivinizi oluşturmaya başlayın.</p>
      <button type="button" aria-label="Müzik Ekle" onclick={() => playerState.isAddMusicModalOpen = true} class="bg-[#6366f1] hover:bg-[#4f46e5] text-white px-8 py-3 rounded-full font-bold shadow-lg transition-all hover:scale-105 flex items-center gap-2">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
        Müzik Ekle
      </button>
    </div>
  {:else}
    <div class="flex text-xs font-semibold text-white/50 border-b border-white/10 pb-2 mb-4 px-2">
      <span class="w-8"></span> 
      <span class="w-8 text-center shrink-0">#</span>
      <span class="flex-1 min-w-0">TITLE</span>
      <span class="w-32 shrink-0">STATS</span> 
      <span class="w-1/4 shrink-0 pl-4">ALBUM</span>
      <span class="w-48 text-center shrink-0">ACTIONS</span> 
    </div>

    <div class="flex flex-col gap-2">
      {#each playerState.sarkiListesi as sarki, index}
        <div 
          role="button" 
          tabindex="0" 
          draggable="true"
          ondragstart={(e) => dragBasla(e, index)}
          ondragover={(e) => dragUzerinde(e, index)}
          ondrop={(e) => drop(e, index)}
          ondragend={dragBitir}
          onclick={() => sarkiCal(sarki)} 
          onkeydown={(e) => e.key === 'Enter' && sarkiCal(sarki)} 
          class="flex items-center text-sm p-2 rounded-lg transition-all duration-200 cursor-pointer group 
                 {playerState.aktifSarki?.id === sarki.id ? 'bg-white/15 shadow-inner' : 'hover:bg-white/10'}
                 {uzerindeGezinilenIndex === index ? 'border-t-2 border-pink-400 bg-white/5' : 'border-t-2 border-transparent'}"
        >
          
          <div class="w-8 text-white/20 hover:text-white cursor-grab active:cursor-grabbing flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity" title="Sürükle">
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M9 3v2H7V3h2zm0 8v2H7v-2h2zm0 8v2H7v-2h2zm6-16v2h-2V3h2zm0 8v2h-2v-2h2zm0 8v2h-2v-2h2z"/></svg>
          </div>

          <span class="w-8 text-center text-white/50 group-hover:hidden shrink-0 font-medium">{index + 1}</span>
          <span class="w-8 text-center hidden group-hover:flex items-center justify-center text-white shrink-0">
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
          </span>
          
          <div class="flex-1 flex items-center gap-4 min-w-0">
            <div class="w-10 h-10 bg-black/30 rounded overflow-hidden shrink-0 shadow-sm">
              {#if sarki.kapak_yolu}
                <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover" />
              {:else}
                <div class="w-full h-full flex items-center justify-center text-xs text-white/30">🎵</div>
              {/if}
            </div>
            <div class="flex flex-col min-w-0 gap-0.5">
              <span class="font-bold truncate {playerState.aktifSarki?.id === sarki.id ? 'text-pink-300' : 'text-white'}">{sarki.isim}</span>
              <span class="text-xs text-white/50 truncate font-medium">{sarki.sarkici}</span>
            </div>
          </div>

          <div class="w-32 shrink-0 flex items-center">
            <SongStats {sarki} />
          </div>

          <span class="w-1/4 text-white/50 truncate font-medium pl-4 shrink-0">{sarki.album}</span>

          <div class="w-48 shrink-0 flex items-center justify-end gap-3 pr-2" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="button" tabindex="0">
            
            <FavoriteButton sarkiId={sarki.id} />
            
            <select aria-label="Playliste Ekle" onchange={(e) => handlePlaylistEkle(sarki.id, e)} class="bg-black/50 text-xs text-white/70 rounded-md px-1 py-1.5 outline-none border border-white/10 hover:border-pink-500/50 cursor-pointer w-24 focus:border-pink-400 transition-colors font-medium">
              <option value="">➕ Ekle...</option>
              {#each playerState.playlistler as pl}
                {#if !pl.sarkilar.includes(sarki.id)}
                  <option value={pl.id}>{pl.isim}</option>
                {/if}
              {/each}
            </select>

            <button type="button" aria-label="Şarkıyı Sil" title="Kütüphaneden Sil" onclick={(e) => handleSarkiSil(sarki, e)} class="text-white/30 hover:text-red-500 transition-colors pl-1">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path><line x1="10" y1="11" x2="10" y2="17"></line><line x1="14" y1="11" x2="14" y2="17"></line></svg>
            </button>

          </div>
          
        </div>
      {/each}
    </div>
  {/if}
</div>