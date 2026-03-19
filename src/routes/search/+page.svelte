<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import FavoriteButton from '$lib/FavoriteButton.svelte';
  import SongStats from '$lib/SongStats.svelte';
  import { playerState, initializePlayer, sarkiPlaylisteEkle, sarkiCal } from '../../store.svelte';
  // Fonksiyonlarımızı merkezi store'dan içe aktarıyoruz

  let aramaMetni = $state("");

  // Sayfa açıldığında kütüphanenin boş olmadığından emin olalım
  onMount(async () => {
    if (playerState.sarkiListesi.length === 0) {
      await initializePlayer(); // Tüm veri çekme işlemini store'a devrettik
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
            sarki.album.toLowerCase().includes(aranan)
          );
        })
  );

  // Arayüz olayını (event) yakalayıp store'daki asıl fonksiyona iletiyoruz
  async function handlePlaylistEkle(sarkiId: string, event: Event) {
    const selectElement = event.target as HTMLSelectElement;
    const playlistId = selectElement.value;
    
    const basarili = await sarkiPlaylisteEkle(sarkiId, playlistId);
    if (basarili) {
      selectElement.value = ""; // İşlem başarılıysa seçimi sıfırla
    }
  }

  const kategoriler = [
    { isim: "Siberpunk", renk: "from-fuchsia-600 to-purple-900" },
    { isim: "Gece Sürüşü", renk: "from-blue-600 to-indigo-900" },
    { isim: "Synthwave", renk: "from-pink-500 to-rose-900" },
    { isim: "Lo-Fi Beats", renk: "from-emerald-500 to-teal-900" },
    { isim: "J-Pop", renk: "from-red-500 to-orange-900" },
    { isim: "Acoustic", renk: "from-amber-600 to-yellow-900" },
    { isim: "Metal", renk: "from-zinc-600 to-neutral-900" },
    { isim: "Podcast", renk: "from-cyan-600 to-blue-900" },
  ];
</script>

<div class="w-full min-h-full pb-32 flex flex-col relative min-w-0">
  
  <div class="sticky top-0 z-20 px-10 pt-10 pb-6 bg-gradient-to-b from-[#261825]/90 via-[#261825]/70 to-transparent backdrop-blur-xl">
    <div class="relative group max-w-4xl mx-auto">
      <div class="absolute inset-y-0 left-0 flex items-center pl-6 pointer-events-none transition-colors group-focus-within:text-pink-400 text-white/50">
        <svg class="w-7 h-7" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" viewBox="0 0 24 24">
          <circle cx="11" cy="11" r="8"></circle>
          <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
        </svg>
      </div>
      
      <input
        type="text"
        bind:value={aramaMetni}
        placeholder="Ne dinlemek istersin?"
        class="w-full bg-black/30 text-white placeholder-white/40 text-xl font-bold rounded-full py-5 pl-16 pr-14 outline-none border border-white/10 focus:border-pink-400/60 focus:bg-black/50 transition-all duration-300 shadow-[0_10px_30px_rgba(0,0,0,0.3)] focus:shadow-[0_10px_40px_rgba(236,72,153,0.2)]"
      />

      {#if aramaMetni.length > 0}
        <button 
          type="button" 
          onclick={() => aramaMetni = ""} 
          class="absolute inset-y-0 right-0 flex items-center pr-5 text-white/40 hover:text-white transition-colors"
          aria-label="Aramayı Temizle">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" viewBox="0 0 24 24"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
        </button>
      {/if}
    </div>
  </div>

  <div class="px-10 flex-1 min-w-0">
    {#if aramaMetni.trim() === ""}
      <div class="max-w-6xl mx-auto mt-4">
        <h2 class="text-2xl font-black text-white mb-6 tracking-tight drop-shadow-md uppercase italic">Hepsine Göz At</h2>
        
        <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
          {#each kategoriler as kategori}
            <div class="relative aspect-[3/2] rounded-xl overflow-hidden cursor-pointer group shadow-lg hover:shadow-2xl transition-all duration-300 transform hover:-translate-y-1">
              <div class="absolute inset-0 bg-gradient-to-br {kategori.renk} opacity-90 group-hover:opacity-100 transition-opacity"></div>
              <span class="absolute top-4 left-4 text-xl font-bold text-white tracking-wide drop-shadow-lg z-10">{kategori.isim}</span>
              <div class="absolute -bottom-4 -right-4 w-24 h-24 bg-black/20 rounded-lg transform rotate-12 group-hover:rotate-6 transition-transform duration-300 shadow-xl backdrop-blur-sm border border-white/10"></div>
            </div>
          {/each}
        </div>
      </div>

    {:else if aramaSonuclari.length === 0}
      <div class="flex flex-col items-center justify-center h-64 text-center mt-10">
        <div class="text-7xl mb-6 opacity-50 drop-shadow-xl transform hover:scale-110 transition-transform duration-500 cursor-default">🔍</div>
        <h3 class="text-3xl font-black text-white mb-3 tracking-tight">Sonuç Bulunamadı</h3>
        <p class="text-gray-300 font-medium text-lg">"{aramaMetni}" için kütüphanede eşleşen bir şey yok.</p>
      </div>

    {:else}
      <div class="max-w-6xl mx-auto">
        <div class="flex items-center justify-between mb-6 px-2 mt-4">
          <h2 class="text-sm font-bold text-white/50 tracking-widest uppercase">
            En İyi Eşleşmeler ({aramaSonuclari.length})
          </h2>
        </div>

        <div class="flex text-[10px] font-black text-white/30 border-b border-white/5 pb-3 mb-4 px-2 tracking-[0.2em] uppercase">
          <span class="w-12 text-center shrink-0">#</span>
          <span class="flex-1 min-w-0">BAŞLIK</span>
          <span class="w-48 shrink-0 text-right pr-4">İSTATİSTİK</span> 
          <span class="w-1/4 shrink-0 pl-6">ALBÜM</span>
          <span class="w-48 text-center shrink-0">İŞLEMLER</span>
        </div>

        <div class="flex flex-col gap-1.5">
          {#each aramaSonuclari as sarki, index}
            <div role="button" tabindex="0" onclick={() => sarkiCal(sarki)} onkeydown={(e) => e.key === 'Enter' && sarkiCal(sarki)} class="flex items-center text-sm p-2 rounded-xl hover:bg-white/5 transition-all duration-200 cursor-pointer group {playerState.aktifSarki?.id === sarki.id ? 'bg-white/10 border border-white/10' : 'border border-transparent'}">
              
              <span class="w-12 text-center shrink-0">
                <span class="text-white/30 group-hover:hidden font-mono text-xs">{index + 1}</span>
                <svg class="w-4 h-4 mx-auto hidden group-hover:block text-pink-400" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
              </span>
              
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

              <div class="w-48 shrink-0 flex items-center justify-end gap-3 pr-2" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()} role="presentation">
                <FavoriteButton sarkiId={sarki.id} />
                <select aria-label="Playliste Ekle" onchange={(e) => handlePlaylistEkle(sarki.id, e)} class="bg-black/50 text-[10px] text-white/70 rounded-lg px-2 py-1.5 outline-none border border-white/5 hover:border-pink-500/50 cursor-pointer w-28 focus:border-pink-400 transition-all font-bold uppercase tracking-tighter">
                  <option value="">➕ EKLE...</option>
                  {#each playerState.playlistler as pl}
                    {#if !pl.sarkilar.includes(sarki.id)}
                      <option value={pl.id}>{pl.isim.toUpperCase()}</option>
                    {/if}
                  {/each}
                </select>
              </div>
              
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</div>