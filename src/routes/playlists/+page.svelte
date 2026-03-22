<script lang="ts">
  import { onMount } from 'svelte';
  import { fade, fly, scale } from 'svelte/transition';
  import { playerState, initializePlayer, yeniPlaylistOlustur, playlistSil, sarkiCal, type Playlist } from '../../store.svelte';

  onMount(async () => {
    if (playerState.playlistler.length === 0) {
      await initializePlayer();
    }
  });

  async function handlePlaylistSil(id: string, isim: string, event: Event) {
    event.preventDefault();
    event.stopPropagation();
    
    if (confirm(`DİKKAT: "${isim}" adlı çalma listesini silmek istediğinize emin misiniz?\n\nBu işlem geri alınamaz (İçindeki şarkılar kütüphanede kalmaya devam eder).`)) {
      await playlistSil(id);
    }
  }

  function listeyiCal(liste: Playlist, event: Event) {
    event.preventDefault();
    event.stopPropagation();

    if (liste.sarkilar && liste.sarkilar.length > 0) {
        const ilkSarkiId = liste.sarkilar[0];
        const sarki = playerState.sarkiListesi.find(s => s.id === ilkSarkiId);
        if (sarki) {
            sarkiCal(sarki);
        } else {
            alert("Şarkı kütüphanede bulunamadı.");
        }
    } else {
        alert("Bu çalma listesi henüz boş. Önce birkaç şarkı eklemelisin.");
    }
  }

  const temaGradientleri = [
    "background: linear-gradient(135deg, var(--accent) 0%, var(--bg-surface) 100%);",
    "background: linear-gradient(135deg, var(--accent-sec) 0%, var(--bg-card) 100%);",
    "background: linear-gradient(135deg, var(--bg-card-hover) 0%, var(--accent) 100%);",
    "background: linear-gradient(135deg, var(--bg-surface) 0%, var(--accent-sec) 100%);"
  ];
</script>

<div class="p-8 lg:p-12 w-full min-h-full pb-32 flex flex-col relative min-w-0 bg-transparent text-[var(--text-main)] transition-colors duration-500 overflow-y-auto custom-scrollbar">
  
  <header class="mb-12" in:fly={{ y: -20, duration: 600 }}>
    <div class="flex items-center gap-3 mb-4">
      <span class="w-10 h-[2px] bg-[var(--accent)]/50 shrink-0"></span>
      <span class="text-[10px] font-black tracking-[0.4em] text-[var(--text-dim)] uppercase truncate">Koleksiyonlar</span>
    </div>
    <h1 class="text-5xl lg:text-7xl font-black italic tracking-tighter uppercase leading-none drop-shadow-lg truncate">
      Çalma Listeleri
    </h1>
    
    <div class="flex flex-col md:flex-row md:items-center gap-6 mt-6">
      <p class="text-[var(--text-dim)] text-[10px] lg:text-xs font-bold uppercase tracking-[0.3em] truncate flex-1">
          Kütüphanende {playerState.playlistler.length} adet özel frekans bloğu tanımlı
      </p>
      
      <div class="flex flex-wrap items-center gap-3 shrink-0">
        <button 
          type="button" 
          onclick={() => playerState.isAddMusicModalOpen = true} 
          class="flex items-center gap-2 bg-[var(--bg-card)] border border-[var(--border)] text-[var(--text-dim)] hover:text-[var(--accent)] hover:border-[var(--accent)]/50 hover:bg-[var(--accent)]/5 px-6 py-3 rounded-full font-black shadow-lg transition-all active:scale-95 uppercase text-[10px] tracking-widest shrink-0"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"></path></svg>
          Yeni Parça Ekle
        </button>

        <button 
          type="button" 
          onclick={yeniPlaylistOlustur} 
          class="flex items-center gap-2 bg-[var(--text-main)] text-[var(--bg-main)] hover:bg-[var(--accent)] hover:text-white px-8 py-3 rounded-full font-black shadow-xl transition-all active:scale-95 uppercase text-[10px] tracking-widest shrink-0"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="3" viewBox="0 0 24 24"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
          Yeni Liste
        </button>
      </div>
    </div>
  </header>

  <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-6 lg:gap-8 auto-rows-fr">
    
    <button 
      type="button" 
      onclick={yeniPlaylistOlustur} 
      aria-label="Yeni çalma listesi oluştur veya içe aktar"
      class="group flex flex-col items-center justify-center bg-[var(--bg-card)] hover:bg-[var(--bg-card-hover)] border-2 border-dashed border-[var(--border)] hover:border-[var(--accent)]/50 rounded-[var(--radius)] p-4 lg:p-5 transition-all duration-300 shadow-lg active:scale-95 h-full w-full min-h-[220px]"
      in:fade
    >
      <div class="w-16 h-16 rounded-full bg-[var(--bg-surface)] group-hover:bg-[var(--accent)]/10 flex items-center justify-center transition-all mb-4 border border-[var(--border)] group-hover:scale-110 shadow-inner group-hover:border-[var(--accent)]/50 shrink-0">
        <svg class="w-8 h-8 text-[var(--text-dim)] group-hover:text-[var(--accent)] transition-colors" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true">
            <line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
      </div>
      <span class="text-[11px] font-black text-[var(--text-dim)] group-hover:text-[var(--text-main)] transition-colors tracking-[0.2em] uppercase mt-2 text-center w-full truncate">Yeni / İçe Aktar</span>
    </button>

    {#each playerState.playlistler as liste, i}
      <a 
        href="/playlist/{liste.id}" 
        in:scale={{ duration: 400, start: 0.95, delay: i * 30 }}
        class="group flex flex-col bg-[var(--bg-card)] hover:bg-[var(--bg-card-hover)] rounded-[var(--radius)] p-4 lg:p-5 transition-all duration-500 shadow-lg border border-[var(--border)] hover:border-[var(--accent)]/30 text-left relative overflow-hidden h-full w-full min-w-0"
      >
        <div 
          class="w-full aspect-square shrink-0 rounded-[calc(var(--radius)-0.5rem)] mb-5 flex items-center justify-center relative overflow-hidden transition-all duration-500 group-hover:shadow-xl border border-white/10"
          style="{temaGradientleri[i % temaGradientleri.length]} opacity: 0.8;"
        >
          <div class="absolute inset-0 bg-black/20 group-hover:bg-transparent transition-colors duration-500"></div>
          
          <svg class="w-20 h-20 text-white/40 group-hover:text-white group-hover:scale-110 transition-all duration-700 drop-shadow-2xl" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path d="M15 4v8.5c-.59-.35-1.27-.5-2-.5-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V4h-6zM3 4h8v2H3zm0 4h8v2H3zm0 4h4v2H3z"/>
          </svg>

          <button 
            type="button"
            aria-label="Listeyi Oynat"
            onclick={(e) => listeyiCal(liste, e)}
            class="absolute bottom-3 right-3 w-12 h-12 bg-white text-black rounded-full flex items-center justify-center opacity-0 translate-y-4 group-hover:opacity-100 group-hover:translate-y-0 transition-all duration-300 shadow-2xl hover:scale-110 border-none"
          >
            <svg class="w-6 h-6 fill-current ml-1" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
          </button>
        </div>

        <div class="flex items-start justify-between w-full min-w-0 mt-auto">
          
          <div class="flex flex-col min-w-0 flex-1 pr-2">
            <h3 class="text-[var(--text-main)] font-black truncate text-base lg:text-lg group-hover:text-[var(--accent)] transition-colors uppercase tracking-tight leading-tight w-full">
                {liste.isim}
            </h3>
            <div class="flex items-center gap-2 mt-2 text-[var(--text-dim)]/70">
              <svg class="w-3 h-3 shrink-0" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true"><path d="M9 18V5l12-2v13"></path><circle cx="6" cy="18" r="3"></circle><circle cx="18" cy="16" r="3"></circle></svg>
              <p class="text-[9px] font-black uppercase tracking-[0.2em] truncate w-full">{liste.sarkilar.length} Parça</p>
            </div>
          </div>
          
          <button 
            type="button" 
            onclick={(e) => handlePlaylistSil(liste.id, liste.isim, e)} 
            class="text-[var(--text-dim)]/30 hover:text-red-500 hover:bg-red-500/10 rounded-lg transition-all p-1.5 shrink-0 opacity-0 group-hover:opacity-100 -mr-1" 
            aria-label="{liste.isim} listesini sil"
            title="Listeyi Sil"
          >
            <svg class="w-4 h-4 sm:w-5 sm:h-5" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true">
                <polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
            </svg>
          </button>
          
        </div>
      </a>
    {/each}

  </div>
</div>

<style>
  .custom-scrollbar::-webkit-scrollbar { width: 4px; }
  .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
  .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--accent); }

  a:focus-visible, button:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 4px;
    border-radius: var(--radius);
  }
</style>