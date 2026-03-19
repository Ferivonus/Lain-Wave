<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core'; 
  import FavoriteButton from '$lib/FavoriteButton.svelte';
  import SongStats from '$lib/SongStats.svelte';
  // Fonksiyonlarımızı merkezi store'dan alıyoruz
  import { playerState, sarkiCal, initializePlayer } from '../../store.svelte';

  // Favori şarkıları hesaplayan derived state
  let favoriSarkilar = $derived(
    playerState.sarkiListesi.filter(sarki => playerState.favoriler.includes(sarki.id))
  );

  // Veri Güvencesi: Eğer sayfa açıldığında store boşsa verileri çek
  onMount(async () => {
    if (playerState.sarkiListesi.length === 0 || playerState.favoriler.length === 0) {
      await initializePlayer();
    }
  });

  function favorileriCal() {
    if (favoriSarkilar.length > 0) {
      sarkiCal(favoriSarkilar[0]);
    }
  }
</script>

<div class="p-10 w-full min-h-full pb-32 flex flex-col relative min-w-0">
  
  <div class="flex gap-8 items-end mb-12 mt-4">
    <div class="w-56 h-56 bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 rounded-xl shadow-[0_10px_40px_rgba(99,102,241,0.4)] flex-shrink-0 border-4 border-white/10 overflow-hidden relative group flex items-center justify-center">
        <svg class="w-24 h-24 text-white drop-shadow-lg group-hover:scale-110 transition-transform duration-500" fill="currentColor" viewBox="0 0 24 24"><path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/></svg>
    </div>
    
    <div class="flex flex-col pb-2">
      <span class="text-sm font-semibold mb-2 text-white/80 tracking-widest uppercase italic">Lain Wave Collection</span>
      <h1 class="text-6xl font-black tracking-tight leading-tight mb-4 drop-shadow-md uppercase">
        Favori Müziklerim
      </h1>
      <div class="flex items-center gap-4">
        <p class="text-white/70 text-sm font-medium uppercase tracking-tight">
          {favoriSarkilar.length} Şarkı • Kişisel Arşivin
        </p>
        
        {#if favoriSarkilar.length > 0}
          <button onclick={favorileriCal} class="flex items-center gap-2 bg-pink-500 hover:bg-pink-400 text-white px-8 py-3 rounded-full font-bold shadow-lg transition-all hover:scale-105 ml-4 uppercase text-xs tracking-widest">
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
            Hepsini Çal
          </button>
        {/if}
      </div>
    </div>
  </div>

  {#if favoriSarkilar.length === 0}
    <div class="flex flex-col items-center justify-center flex-1 text-center mt-10 p-10 bg-white/5 border border-white/10 rounded-2xl border-dashed">
      <div class="text-7xl mb-6 opacity-30 drop-shadow-xl transform hover:scale-110 transition-transform duration-500 cursor-default">🤍</div>
      <h3 class="text-3xl font-black text-white mb-3 tracking-tight italic uppercase">Henüz Favoriniz Yok</h3>
      <p class="text-gray-400 font-medium text-lg max-w-md">Kütüphanedeki kalp ikonuna tıklayarak en sevdiğin parçaları burada toplayabilirsin.</p>
    </div>
  {:else}
    <div class="flex text-[10px] font-black text-white/30 border-b border-white/5 pb-3 mb-4 px-2 tracking-[0.2em] uppercase">
      <span class="w-12 text-center shrink-0">#</span>
      <span class="flex-1 min-w-0">BAŞLIK</span>
      <span class="w-48 shrink-0 text-right pr-4">İSTATİSTİK</span> 
      <span class="w-1/4 shrink-0 pl-6">ALBÜM</span>
      <span class="w-16 text-center shrink-0">DURUM</span>
    </div>

    <div class="flex flex-col gap-1.5">
      {#each favoriSarkilar as sarki, index}
        <div role="button" tabindex="0" 
             onclick={() => sarkiCal(sarki)} 
             onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(sarki)} 
             class="flex items-center text-sm p-2 rounded-xl hover:bg-white/5 transition-all duration-200 cursor-pointer group {playerState.aktifSarki?.id === sarki.id ? 'bg-white/10 border border-white/10 shadow-lg' : 'border border-transparent'}">
          
          <div class="w-12 text-center shrink-0">
             {#if playerState.aktifSarki?.id === sarki.id && playerState.suAnOynuyorMu}
                <div class="flex items-end justify-center gap-0.5 h-3">
                   <div class="w-1 bg-pink-500 animate-[bounce_1s_infinite]"></div>
                   <div class="w-1 bg-pink-500 animate-[bounce_1.2s_infinite]"></div>
                   <div class="w-1 bg-pink-500 animate-[bounce_0.8s_infinite]"></div>
                </div>
             {:else}
                <span class="text-white/30 group-hover:hidden font-mono text-xs">{index + 1}</span>
                <svg class="w-4 h-4 mx-auto hidden group-hover:block text-pink-400" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
             {/if}
          </div>
          
          <div class="flex-1 flex items-center gap-4 min-w-0">
            <div class="w-12 h-12 bg-black/30 rounded-lg overflow-hidden shrink-0 shadow-md">
              {#if sarki.kapak_yolu}
                <img src={convertFileSrc(sarki.kapak_yolu)} alt="Kapak" class="w-full h-full object-cover" />
              {:else}
                <div class="w-full h-full flex items-center justify-center text-white/20 bg-white/5">🎵</div>
              {/if}
            </div>
            
            <div class="flex flex-col min-w-0 gap-0.5 pr-2">
              <span class="font-bold text-sm truncate {playerState.aktifSarki?.id === sarki.id ? 'text-pink-400' : 'text-white'}">{sarki.isim}</span>
              <a href="/artist/{encodeURIComponent(sarki.sarkici)}" 
                 class="text-xs text-white/40 truncate font-medium hover:text-pink-400 transition-colors" 
                 onclick={(e) => e.stopPropagation()}>
                {sarki.sarkici}
              </a>
            </div>
          </div>

          <div class="w-48 shrink-0 flex items-center justify-end pr-4">
            <SongStats {sarki} />
          </div>
          
          <span class="w-1/4 text-white/40 truncate font-bold text-[10px] uppercase tracking-tighter pl-6 shrink-0">{sarki.album}</span>
          
          <div class="w-16 flex justify-center shrink-0" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="presentation">
            <FavoriteButton sarkiId={sarki.id} />
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>