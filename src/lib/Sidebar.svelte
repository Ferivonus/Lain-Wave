<script lang="ts">
  import { playerState, playlistSil } from '../store.svelte';
  
  let { aktifYol, onYeniPlaylist } = $props<{ 
    aktifYol: string, 
    onYeniPlaylist: () => void 
  }>();

  // Olayın sayfa yönlendirmesini tetiklemesini engellemek için sarmalayıcı (wrapper)
  async function handlePlaylistSil(id: string, event: Event) {
    event.preventDefault(); // href'e gitmesini engeller
    event.stopPropagation();
    
    // ONAY PENCERESİ ARTIK BURADA
    if (confirm("Bu çalma listesini tamamen silmek istediğinize emin misiniz?")) {
        await playlistSil(id);
    }
  }
</script>

<aside class="w-20 lg:w-64 bg-[#41273f] flex flex-col py-6 border-r border-white/5 shadow-2xl z-10 flex-shrink-0 transition-all duration-300">
  
  <div class="flex items-center justify-center lg:justify-start px-0 lg:px-6 mb-6">
    <div class="w-10 h-10 bg-pink-500 rounded-lg flex items-center justify-center shrink-0">
      <svg class="w-6 h-6 text-white" fill="currentColor" viewBox="0 0 24 24"><path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/></svg>
    </div>
    <h1 class="hidden lg:block text-xl font-black tracking-widest uppercase text-pink-300 drop-shadow-md ml-3">LAIN WAVE</h1>
  </div>
  
  <div class="px-3 lg:px-6 mb-6">
    <button type="button" aria-label="Müzik Ekle" onclick={() => playerState.isAddMusicModalOpen = true} class="w-full bg-[#6366f1] hover:bg-[#4f46e5] text-white rounded-xl py-3 flex items-center justify-center gap-2 font-bold shadow-lg transition-all hover:scale-[1.02]">
      <svg class="w-5 h-5 shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
      <span class="hidden lg:block">Müzik Ekle</span>
    </button>
  </div>

  <nav class="flex flex-col gap-1 mb-6 px-3 lg:px-2">
    <a href="/" class="flex items-center justify-center lg:justify-start gap-4 px-0 lg:px-4 py-3 lg:py-2 rounded-lg text-sm font-medium transition-colors {aktifYol === '/' ? 'bg-white/10 text-white' : 'text-gray-300 hover:text-white hover:bg-white/5'}">
      <svg class="w-6 h-6 lg:w-5 lg:h-5 shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path><polyline points="9 22 9 12 15 12 15 22"></polyline></svg> 
      <span class="hidden lg:block">Ana Sayfa</span>
    </a>

    <a href="/library" class="flex items-center justify-center lg:justify-start gap-4 px-0 lg:px-4 py-3 lg:py-2 rounded-lg text-sm font-medium transition-colors {aktifYol === '/library' ? 'bg-white/10 text-white' : 'text-gray-300 hover:text-white hover:bg-white/5'}">
      <svg class="w-6 h-6 lg:w-5 lg:h-5 shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><line x1="9" y1="3" x2="9" y2="21"></line></svg>
      <span class="hidden lg:block">Kütüphane</span>
    </a>

    <a href="/search" class="flex items-center justify-center lg:justify-start gap-4 px-0 lg:px-4 py-3 lg:py-2 rounded-lg text-sm font-medium transition-colors {aktifYol === '/search' ? 'bg-white/10 text-white' : 'text-gray-300 hover:text-white hover:bg-white/5'}">
      <svg class="w-6 h-6 lg:w-5 lg:h-5 shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg> 
      <span class="hidden lg:block">Ara / Keşfet</span>
    </a>
    <a href="/discover" class="flex items-center justify-center lg:justify-start gap-4 px-0 lg:px-4 py-3 lg:py-2 rounded-lg text-sm font-medium transition-colors {aktifYol === '/discover' ? 'bg-white/10 text-white' : 'text-gray-300 hover:text-white hover:bg-white/5'}">
      <svg class="w-6 h-6 lg:w-5 lg:h-5 shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"></polygon></svg>
      <span class="hidden lg:block">Trendler</span>
    </a>
    <a href="/favorites" class="flex items-center justify-center lg:justify-start gap-4 px-0 lg:px-4 py-3 lg:py-2 rounded-lg text-sm font-medium transition-colors {aktifYol === '/favorites' ? 'bg-white/10 text-white' : 'text-gray-300 hover:text-white hover:bg-white/5'}">
      <svg class="w-6 h-6 lg:w-5 lg:h-5 shrink-0" fill="currentColor" viewBox="0 0 24 24"><path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/></svg>
      <span class="hidden lg:block">Favoriler</span>
    </a>
    <a href="/artists" class="flex items-center justify-center lg:justify-start gap-4 px-0 lg:px-4 py-3 lg:py-2 rounded-lg text-sm font-medium transition-colors {aktifYol === '/artists' ? 'bg-white/10 text-white' : 'text-gray-300 hover:text-white hover:bg-white/5'}">
      <svg class="w-6 h-6 lg:w-5 lg:h-5 shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
        <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path>
        <circle cx="9" cy="7" r="4"></circle>
        <path d="M23 21v-2a4 4 0 0 0-3-3.87"></path>
        <path d="M16 3.13a4 4 0 0 1 0 7.75"></path>
      </svg>
      <span class="hidden lg:block">Sanatçılar</span>
    </a>
    <a href="/radio" class="flex items-center justify-center lg:justify-start gap-4 px-0 lg:px-4 py-3 lg:py-2 rounded-lg text-sm font-medium transition-colors {aktifYol === '/radio' ? 'bg-white/10 text-white' : 'text-gray-300 hover:text-white hover:bg-white/5'}">
      <svg class="w-6 h-6 lg:w-5 lg:h-5 shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><circle cx="12" cy="12" r="2"></circle><path d="M16.24 7.76a6 6 0 0 1 0 8.49m-8.48-.01a6 6 0 0 1 0-8.49m11.31-2.82a10 10 0 0 1 0 14.14m-14.14 0a10 10 0 0 1 0-14.14"></path></svg>
      <span class="hidden lg:block">Lain Radyo</span>
    </a>
    <a href="/podcasts" class="flex items-center justify-center lg:justify-start gap-4 px-0 lg:px-4 py-3 lg:py-2 rounded-lg text-sm font-medium transition-colors {aktifYol === '/podcasts' ? 'bg-white/10 text-white' : 'text-gray-300 hover:text-white hover:bg-white/5'}">
      <svg class="w-6 h-6 lg:w-5 lg:h-5 shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"></path><path d="M19 10v2a7 7 0 0 1-14 0v-2"></path><line x1="12" y1="19" x2="12" y2="23"></line><line x1="8" y1="23" x2="16" y2="23"></line></svg>
      <span class="hidden lg:block">Podcast'ler</span>
    </a>
  </nav>

  <div class="hidden lg:flex items-center justify-between px-6 mb-2 group">
    <a href="/playlists" class="text-xs font-bold text-gray-500 tracking-widest uppercase hover:text-white transition-colors">Playlists</a>
    <button type="button" aria-label="Yeni Liste Oluştur" onclick={onYeniPlaylist} class="text-gray-400 hover:text-white transition-colors" title="Yeni Liste">
      <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
    </button>
  </div>
  
  <div class="hidden lg:flex flex-col gap-1 flex-1 overflow-y-auto custom-scrollbar px-2">
    {#if playerState.playlistler.length === 0}
      <span class="text-xs text-gray-600 px-4 mt-2 italic">Henüz liste yok...</span>
    {/if}
    {#each playerState.playlistler as liste}
      <div class="flex items-center group/item px-2 rounded-lg {aktifYol === `/playlist/${liste.id}` ? 'bg-white/10' : 'hover:bg-white/5'} transition-colors">
        
        <a href="/playlist/{liste.id}" class="flex-1 flex items-center gap-3 py-2 text-sm font-medium transition-colors text-left truncate {aktifYol === `/playlist/${liste.id}` ? 'text-white' : 'text-gray-400 hover:text-white'}">
          <svg class="w-4 h-4 text-pink-500/50 group-hover/item:text-pink-400 shrink-0" fill="currentColor" viewBox="0 0 24 24"><path d="M15 4v8.5c-.59-.35-1.27-.5-2-.5-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V4h-6zM3 4h8v2H3zm0 4h8v2H3zm0 4h4v2H3z"/></svg> 
          <span class="truncate pr-2">{liste.isim}</span>
        </a>

        <button 
            type="button" 
            aria-label="Listeyi Sil" 
            title="Listeyi Sil" 
            onclick={(e) => handlePlaylistSil(liste.id, e)} 
            class="opacity-0 group-hover/item:opacity-100 p-1.5 text-white/30 hover:text-red-500 transition-all shrink-0">
          <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path><line x1="10" y1="11" x2="10" y2="17"></line><line x1="14" y1="11" x2="14" y2="17"></line></svg>
        </button>

      </div>
    {/each}
  </div>

  <a href="/settings" class="flex items-center justify-center lg:justify-start gap-4 px-0 lg:px-6 py-4 mt-auto text-sm font-medium transition-colors border-t border-white/5 {aktifYol === '/settings' ? 'text-white bg-white/5' : 'text-gray-400 hover:text-white hover:bg-white/5'}">
    <svg class="w-6 h-6 lg:w-5 lg:h-5 shrink-0 opacity-70" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><circle cx="12" cy="12" r="3"></circle><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path></svg> 
    <span class="hidden lg:block">Settings</span>
  </a>
</aside>