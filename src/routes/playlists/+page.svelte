<script lang="ts">
  import { onMount } from 'svelte';
  // Merkezi fonksiyonlarımızı store'dan alıyoruz
  import { playerState, initializePlayer, yeniPlaylistOlustur, playlistSil } from '../../store.svelte';

  // Sayfa doğrudan açıldığında verilerin yüklendiğinden emin oluyoruz
  onMount(async () => {
    if (playerState.playlistler.length === 0) {
      await initializePlayer();
    }
  });

  // Arayüz tetikleyicisi
  async function handlePlaylistSil(id: string, event: Event) {
    event.preventDefault();
    event.stopPropagation();
    
    // ONAY PENCERESİ ARTIK BURADA
    if (confirm("Bu çalma listesini silmek istediğinize emin misiniz?")) {
      await playlistSil(id);
    }
  }

  const gradientler = [
    "from-indigo-600 to-purple-900",
    "from-pink-600 to-rose-900",
    "from-emerald-600 to-teal-900",
    "from-blue-600 to-cyan-900",
    "from-amber-600 to-orange-900",
    "from-fuchsia-600 to-pink-900",
    "from-red-600 to-rose-900"
  ];
</script>

<div class="p-10 w-full min-h-full pb-32 flex flex-col relative min-w-0">
  
  <div class="flex items-center justify-between mb-10">
    <div>
      <h1 class="text-5xl font-black tracking-tight text-white drop-shadow-md uppercase">Çalma Listeleri</h1>
      <p class="text-white/50 text-sm mt-2 font-medium">Kütüphanenizde toplam {playerState.playlistler.length} adet özel liste bulunuyor.</p>
    </div>
  </div>

  <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-6">
    
    <button 
      type="button" 
      onclick={yeniPlaylistOlustur} 
      class="group flex flex-col items-center justify-center bg-white/5 hover:bg-white/10 border-2 border-dashed border-white/20 hover:border-pink-400 rounded-xl aspect-square transition-all duration-300 cursor-pointer shadow-lg">
      <div class="w-16 h-16 rounded-full bg-white/5 group-hover:bg-pink-500/20 flex items-center justify-center transition-colors mb-4 shadow-inner">
        <svg class="w-8 h-8 text-white/40 group-hover:text-pink-400 transition-colors" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
      </div>
      <span class="text-sm font-bold text-white/50 group-hover:text-white transition-colors tracking-wide uppercase">Yeni Oluştur</span>
    </button>

    {#each playerState.playlistler as liste, i}
      <a href="/playlist/{liste.id}" class="group flex flex-col bg-black/20 hover:bg-[#321e30] rounded-xl p-4 transition-all duration-300 shadow-lg hover:shadow-[0_10px_30px_rgba(236,72,153,0.15)] border border-white/5 hover:border-pink-500/30 text-left relative">
        
        <div class="w-full aspect-square bg-gradient-to-br {gradientler[i % gradientler.length]} rounded-lg shadow-inner mb-4 flex items-center justify-center relative overflow-hidden transition-all duration-300 group-hover:-translate-y-1 group-hover:shadow-[0_8px_20px_rgba(0,0,0,0.4)]">
          <div class="absolute inset-0 bg-black/10 group-hover:bg-transparent transition-colors duration-300"></div>
          
          <svg class="w-16 h-16 text-white/40 group-hover:text-white group-hover:scale-110 transition-all duration-500 drop-shadow-lg" fill="currentColor" viewBox="0 0 24 24">
            <path d="M15 4v8.5c-.59-.35-1.27-.5-2-.5-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V4h-6zM3 4h8v2H3zm0 4h8v2H3zm0 4h4v2H3z"/>
          </svg>

          <div class="absolute bottom-2 right-2 w-11 h-11 bg-pink-500 rounded-full flex items-center justify-center opacity-0 translate-y-4 group-hover:opacity-100 group-hover:translate-y-0 transition-all duration-300 shadow-lg hover:scale-105 hover:bg-pink-400">
            <svg class="w-5 h-5 text-white ml-1" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
          </div>
        </div>

        <div class="flex items-start justify-between w-full">
          <div class="flex flex-col min-w-0 pr-2">
            <h3 class="text-white font-bold truncate text-base group-hover:text-pink-300 transition-colors">{liste.isim}</h3>
            <div class="flex items-center gap-1.5 mt-1 text-white/50">
              <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M9 18V5l12-2v13"></path><circle cx="6" cy="18" r="3"></circle><circle cx="18" cy="16" r="3"></circle></svg>
              <p class="text-xs font-medium">{liste.sarkilar.length} Şarkı</p>
            </div>
          </div>
          
          <button type="button" onclick={(e) => handlePlaylistSil(liste.id, e)} class="text-white/20 hover:text-red-500 transition-colors pt-1" title="Listeyi Sil">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path><line x1="10" y1="11" x2="10" y2="17"></line><line x1="14" y1="11" x2="14" y2="17"></line></svg>
          </button>
        </div>
      </a>
    {/each}

  </div>
</div>