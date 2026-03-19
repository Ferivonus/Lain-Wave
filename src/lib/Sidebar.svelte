<script lang="ts">
    import { playerState, playlistSil } from '../store.svelte';
    import { fade, slide } from 'svelte/transition';
    
    let { aktifYol, onYeniPlaylist } = $props<{ 
        aktifYol: string, 
        onYeniPlaylist: () => void 
    }>();

    async function handlePlaylistSil(id: string, event: Event) {
        event.preventDefault(); 
        event.stopPropagation();
        
        if (confirm("Bu çalma listesini tamamen silmek istediğinize emin misiniz?")) {
            await playlistSil(id);
        }
    }
</script>

<aside class="w-20 lg:w-64 bg-[var(--bg-surface)] flex flex-col py-6 border-r border-[var(--border)] z-20 flex-shrink-0 transition-all duration-500 relative h-full">
    
    <div class="flex items-center justify-center lg:justify-start px-0 lg:px-6 mb-8 shrink-0">
        <div class="w-10 h-10 bg-[var(--accent)] rounded-xl flex items-center justify-center shrink-0 shadow-lg shadow-[var(--accent)]/20">
            <svg class="w-6 h-6 text-white" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/>
            </svg>
        </div>
        <h1 class="hidden lg:block text-lg font-black tracking-widest uppercase text-[var(--text-main)] ml-3 italic">
            LAIN WAVE
        </h1>
    </div>
    
    <div class="px-3 lg:px-6 mb-8 shrink-0">
        <button 
            type="button" 
            onclick={() => playerState.isAddMusicModalOpen = true} 
            class="w-full bg-[var(--accent)] hover:opacity-90 text-white rounded-2xl py-3.5 flex items-center justify-center gap-2 font-bold shadow-xl transition-all active:scale-95 group"
            aria-label="Müzik Aktar"
        >
            <svg class="w-5 h-5 shrink-0 group-hover:rotate-90 transition-transform duration-300" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true">
                <line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line>
            </svg>
            <span class="hidden lg:block text-[10px] uppercase tracking-[0.2em]">Müzik Aktar</span>
        </button>
    </div>

    <nav class="flex flex-col gap-1 px-3 lg:px-4 flex-1 overflow-y-auto custom-scrollbar pr-1">
        
        <a href="/" class="flex items-center justify-center lg:justify-start gap-4 px-4 py-3 rounded-xl text-sm font-bold transition-all group {aktifYol === '/' ? 'bg-[var(--accent)]/10 text-[var(--accent)]' : 'text-[var(--text-dim)] hover:text-[var(--text-main)] hover:bg-[var(--bg-card)]'}">
            <svg class="w-6 h-6 lg:w-5 lg:h-5 shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24" aria-hidden="true"><path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path><polyline points="9 22 9 12 15 12 15 22"></polyline></svg> 
            <span class="hidden lg:block uppercase tracking-wider text-[11px]">Ana Sayfa</span>
        </a>

        <a href="/library" class="flex items-center justify-center lg:justify-start gap-4 px-4 py-3 rounded-xl text-sm font-bold transition-all {aktifYol === '/library' ? 'bg-[var(--accent)]/10 text-[var(--accent)]' : 'text-[var(--text-dim)] hover:text-[var(--text-main)] hover:bg-[var(--bg-card)]'}">
            <svg class="w-6 h-6 lg:w-5 lg:h-5 shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24" aria-hidden="true"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><line x1="9" y1="3" x2="9" y2="21"></line></svg>
            <span class="hidden lg:block uppercase tracking-wider text-[11px]">Kütüphane</span>
        </a>

        <a href="/search" class="flex items-center justify-center lg:justify-start gap-4 px-4 py-3 rounded-xl text-sm font-bold transition-all {aktifYol === '/search' ? 'bg-[var(--accent)]/10 text-[var(--accent)]' : 'text-[var(--text-dim)] hover:text-[var(--text-main)] hover:bg-[var(--bg-card)]'}">
            <svg class="w-6 h-6 lg:w-5 lg:h-5 shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24" aria-hidden="true"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg> 
            <span class="hidden lg:block uppercase tracking-wider text-[11px]">Ara</span>
        </a>

        <a href="/discover" class="flex items-center justify-center lg:justify-start gap-4 px-4 py-3 rounded-xl text-sm font-bold transition-all {aktifYol === '/discover' ? 'bg-[var(--accent)]/10 text-[var(--accent)]' : 'text-[var(--text-dim)] hover:text-[var(--text-main)] hover:bg-[var(--bg-card)]'}">
            <svg class="w-6 h-6 lg:w-5 lg:h-5 shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24" aria-hidden="true"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"></polygon></svg>
            <span class="hidden lg:block uppercase tracking-wider text-[11px]">Trendler</span>
        </a>

        <a href="/favorites" class="flex items-center justify-center lg:justify-start gap-4 px-4 py-3 rounded-xl text-sm font-bold transition-all {aktifYol === '/favorites' ? 'bg-[var(--accent)]/10 text-[var(--accent)]' : 'text-[var(--text-dim)] hover:text-[var(--text-main)] hover:bg-[var(--bg-card)]'}">
            <svg class="w-6 h-6 lg:w-5 lg:h-5 shrink-0" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/></svg>
            <span class="hidden lg:block uppercase tracking-wider text-[11px]">Favoriler</span>
        </a>

        <a href="/artists" class="flex items-center justify-center lg:justify-start gap-4 px-4 py-3 rounded-xl text-sm font-bold transition-all {aktifYol === '/artists' ? 'bg-[var(--accent)]/10 text-[var(--accent)]' : 'text-[var(--text-dim)] hover:text-[var(--text-main)] hover:bg-[var(--bg-card)]'}">
            <svg class="w-6 h-6 lg:w-5 lg:h-5 shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24" aria-hidden="true"><path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path><circle cx="9" cy="7" r="4"></circle></svg>
            <span class="hidden lg:block uppercase tracking-wider text-[11px]">Sanatçılar</span>
        </a>

        <a href="/radio" class="flex items-center justify-center lg:justify-start gap-4 px-4 py-3 rounded-xl text-sm font-bold transition-all {aktifYol === '/radio' ? 'bg-[var(--accent)]/10 text-[var(--accent)]' : 'text-[var(--text-dim)] hover:text-[var(--text-main)] hover:bg-[var(--bg-card)]'}">
            <svg class="w-6 h-6 lg:w-5 lg:h-5 shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24" aria-hidden="true"><circle cx="12" cy="12" r="2"></circle><path d="M16.24 7.76a6 6 0 0 1 0 8.49m-8.48-.01a6 6 0 0 1 0-8.49m11.31-2.82a10 10 0 0 1 0 14.14m-14.14 0a10 10 0 0 1 0-14.14"></path></svg>
            <span class="hidden lg:block uppercase tracking-wider text-[11px]">Radyo</span>
        </a>

        <a href="/podcasts" class="flex items-center justify-center lg:justify-start gap-4 px-4 py-3 rounded-xl text-sm font-bold transition-all {aktifYol === '/podcasts' ? 'bg-[var(--accent)]/10 text-[var(--accent)]' : 'text-[var(--text-dim)] hover:text-[var(--text-main)] hover:bg-[var(--bg-card)]'}">
            <svg class="w-6 h-6 lg:w-5 lg:h-5 shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24" aria-hidden="true"><path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"></path><path d="M19 10v2a7 7 0 0 1-14 0v-2"></path></svg>
            <span class="hidden lg:block uppercase tracking-wider text-[11px]">Podcast</span>
        </a>

        <div class="mt-6 mb-2 px-4 hidden lg:flex items-center justify-between shrink-0">
            <a href="/playlists" class="text-[10px] font-black text-[var(--text-dim)] tracking-[0.2em] uppercase hover:text-[var(--accent)] transition-colors">Listelerim</a>
            <button type="button" onclick={onYeniPlaylist} class="text-[var(--text-dim)] hover:text-[var(--accent)] p-1" title="Yeni Liste"><svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg></button>
        </div>

        {#each playerState.playlistler as liste}
            <div class="flex items-center group px-4 rounded-xl transition-all {aktifYol === `/playlist/${liste.id}` ? 'bg-[var(--accent)]/10' : 'hover:bg-[var(--bg-card)]'}">
                <a href="/playlist/{liste.id}" class="flex-1 py-2.5 text-[11px] font-bold truncate flex items-center gap-4 {aktifYol === `/playlist/${liste.id}` ? 'text-[var(--accent)]' : 'text-[var(--text-dim)]'}">
                    <svg class="w-5 h-5 lg:w-3.5 lg:h-3.5 shrink-0 opacity-50" fill="currentColor" viewBox="0 0 24 24"><path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/></svg>
                    <span class="hidden lg:block">{liste.isim}</span>
                </a>
                
                <button type="button" onclick={(e) => handlePlaylistSil(liste.id, e)} class="hidden lg:block opacity-0 group-hover:opacity-100 text-red-500 p-1 transition-opacity shrink-0" aria-label="Sil">
                    <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"></path></svg>
                </button>
            </div>
        {/each}
    </nav>

    <div class="px-3 lg:px-4 mt-auto pt-4 border-t border-[var(--border)] shrink-0">
        <a href="/settings" class="flex items-center justify-center lg:justify-start gap-4 px-4 py-3 rounded-xl text-sm font-bold transition-all {aktifYol === '/settings' ? 'bg-[var(--accent)]/10 text-[var(--accent)]' : 'text-[var(--text-dim)] hover:text-[var(--text-main)] hover:bg-[var(--bg-card)]'}">
            <svg class="w-6 h-6 lg:w-5 lg:h-5 shrink-0 opacity-70" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24" aria-hidden="true"><circle cx="12" cy="12" r="3"></circle><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path></svg> 
            <span class="hidden lg:block uppercase tracking-wider text-[11px]">Ayarlar</span>
        </a>
    </div>
</aside>

<style>
    .custom-scrollbar::-webkit-scrollbar { width: 3px; }
    .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
    .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--accent); }
    
    a, button { transition: all 0.2s ease-in-out; }
</style>