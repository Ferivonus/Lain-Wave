<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import FavoriteButton from '$lib/FavoriteButton.svelte';
  // Merkezi store fonksiyonlarımızı alıyoruz
  import { playerState, sarkiCal, initializePlayer } from '../../store.svelte';

  // Sayfa yüklendiğinde verilerin tam olduğundan emin olalım
  onMount(async () => {
    if (playerState.sarkiListesi.length === 0) {
      await initializePlayer(); // Veri çekme işlemini store'a devrettik
    }
  });

  // En çok dinlenen 5 şarkı (Dinlenme sayısına göre azalan sıralama)
  let enCokDinlenenler = $derived(
    [...playerState.sarkiListesi]
      .sort((a, b) => (b.dinlenme_sayisi || 0) - (a.dinlenme_sayisi || 0))
      .slice(0, 5)
  );

  // Son eklenen 10 şarkı (Listeyi ters çevirip ilk 10'u alıyoruz)
  let yeniEklenenler = $derived(
    [...playerState.sarkiListesi]
      .reverse()
      .slice(0, 10)
  );
</script>

<div class="p-10 w-full min-h-full pb-32 flex flex-col relative min-w-0">
  
  <div class="relative w-full h-64 rounded-3xl overflow-hidden mb-12 shadow-2xl border border-white/10 group">
    <div class="absolute inset-0 bg-gradient-to-r from-purple-900 via-pink-600 to-orange-500 opacity-80"></div>
    <div class="absolute inset-0 bg-[url('https://images.unsplash.com/photo-1614613535308-eb5fbd3d2c17?q=80&w=2070')] bg-cover bg-center mix-blend-overlay group-hover:scale-105 transition-transform duration-700"></div>
    
    <div class="absolute inset-0 p-10 flex flex-col justify-center">
      <span class="text-xs font-black tracking-[0.5em] text-white/70 uppercase mb-2">Haftalık Keşif</span>
      <h1 class="text-5xl font-black text-white mb-4 tracking-tighter italic">TRENDLER VE YENİLİKLER</h1>
      <p class="text-white/80 max-w-lg font-medium">Lain Wave algoritması kütüphaneni senin için analiz etti. İşte bu aralar en çok kulak verdiğin parçalar.</p>
    </div>
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-12 gap-10">
    
    <div class="lg:col-span-5 flex flex-col">
      <h2 class="text-2xl font-black text-white mb-6 flex items-center gap-3 uppercase italic tracking-tight">
        <span class="text-pink-500 text-3xl">#</span> Zirvedekiler
      </h2>
      
      <div class="flex flex-col gap-4">
        {#each enCokDinlenenler as sarki, index}
          <div 
            role="button" tabindex="0"
            onclick={() => sarkiCal(sarki)}
            onkeydown={(e) => e.key === 'Enter' && sarkiCal(sarki)}
            class="flex items-center gap-4 p-4 rounded-2xl bg-white/5 border border-white/5 hover:bg-white/10 hover:border-pink-500/30 transition-all group cursor-pointer"
          >
            <span class="text-2xl font-black text-white/20 group-hover:text-pink-500 transition-colors w-8">{index + 1}</span>
            
            <div class="w-14 h-14 rounded-lg overflow-hidden shadow-lg shrink-0">
              {#if sarki.kapak_yolu}
                <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover" />
              {:else}
                <div class="w-full h-full bg-black/40 flex items-center justify-center text-white/20 italic font-black">LW</div>
              {/if}
            </div>

            <div class="flex flex-col min-w-0 flex-1">
              <span class="font-bold text-white truncate">{sarki.isim}</span>
              <span class="text-xs text-white/40 truncate">{sarki.sarkici}</span>
            </div>

            <div class="flex flex-col items-end gap-1">
              <span class="text-[10px] font-bold text-pink-400 uppercase tracking-widest">{sarki.kalite || 'MP3'}</span>
              <div class="flex items-center gap-1 text-white/20">
                <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
                <span class="text-xs font-bold">{sarki.dinlenme_sayisi || 0}</span>
              </div>
            </div>
          </div>
        {/each}
      </div>
    </div>

    <div class="lg:col-span-7 flex flex-col">
      <h2 class="text-2xl font-black text-white mb-6 flex items-center gap-3 uppercase italic tracking-tight">
        <span class="text-blue-500 text-3xl">/</span> Son Eklenenler
      </h2>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        {#each yeniEklenenler as sarki}
          <div 
            role="button" tabindex="0"
            onclick={() => sarkiCal(sarki)}
            onkeydown={(e) => e.key === 'Enter' && sarkiCal(sarki)}
            class="flex items-center gap-4 p-3 rounded-xl bg-black/20 hover:bg-black/40 border border-white/5 transition-all cursor-pointer group"
          >
            <div class="w-12 h-12 rounded-md overflow-hidden shrink-0 relative">
               {#if sarki.kapak_yolu}
                  <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover" />
               {:else}
                  <div class="w-full h-full bg-white/5 flex items-center justify-center text-xs">🎵</div>
               {/if}
               <div class="absolute inset-0 bg-pink-500/0 group-hover:bg-pink-500/20 transition-colors flex items-center justify-center">
                  <svg class="w-5 h-5 text-white opacity-0 group-hover:opacity-100 scale-50 group-hover:scale-100 transition-all" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
               </div>
            </div>
            
            <div class="flex flex-col min-w-0 flex-1">
              <span class="text-sm font-bold text-white truncate">{sarki.isim}</span>
              <span class="text-[10px] text-white/40 truncate uppercase tracking-tighter">{sarki.sarkici}</span>
            </div>

            <div class="flex items-center gap-2" onclick={(e) => e.stopPropagation()} role="presentation">
               <FavoriteButton sarkiId={sarki.id} />
            </div>
          </div>
        {/each}
      </div>

      <div class="mt-8 p-6 rounded-3xl bg-gradient-to-br from-white/5 to-transparent border border-white/10 flex items-center justify-between">
        <div class="flex flex-col">
          <span class="text-[10px] font-black text-white/30 uppercase tracking-[0.2em] mb-1">Toplam Kütüphane</span>
          <span class="text-4xl font-black text-white italic">{playerState.sarkiListesi.length} <span class="text-sm not-italic font-medium text-white/40">Parça</span></span>
        </div>
        <div class="w-12 h-12 rounded-full border-2 border-white/10 flex items-center justify-center text-white/20">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="17 8 12 3 7 8"></polyline><line x1="12" y1="3" x2="12" y2="15"></line></svg>
        </div>
      </div>
    </div>

  </div>
</div>