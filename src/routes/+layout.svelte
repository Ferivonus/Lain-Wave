<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/state'; 
  import '../app.css';
  import { playerState, oynatDuraklatToggle, initializePlayer, yeniPlaylistOlustur } from '../store.svelte';
  
  import Sidebar from '$lib/Sidebar.svelte';
  import RightPanel from '$lib/RightPanel.svelte';
  import AddMusicModal from '$lib/AddMusicModal.svelte';
  import FooterPlayer from '$lib/FooterPlayer.svelte';
  
  let { children } = $props();
  let aktifYol = $derived(page.url.pathname as string);
  let sagMenuAcik = $state(false);

  onMount(async () => {
    const kayitliSes = localStorage.getItem('lainwave_ses');
    if (kayitliSes) {
      const ses = parseFloat(kayitliSes);
      playerState.sesSeviyesi = ses;
      if (playerState.audioRef) playerState.audioRef.volume = ses;
    }
    
    await initializePlayer();
  });

  function klavyeKisaYollari(e: KeyboardEvent) {
    const hedef = e.target as HTMLElement;
    
    if (
      ['INPUT', 'TEXTAREA', 'SELECT', 'BUTTON'].includes(hedef.tagName) || 
      hedef.isContentEditable || 
      hedef.getAttribute('role') === 'button' ||
      hedef.getAttribute('role') === 'dialog'
    ) {
      return;
    }

    if (e.key === ' ' || e.code === 'Space') {
      e.preventDefault(); 
      oynatDuraklatToggle();
    }
  }
</script>

<svelte:window onkeydown={klavyeKisaYollari} />

<div class="{playerState.currentTheme} h-screen w-full flex flex-col font-sans overflow-hidden bg-[var(--bg-main)] text-[var(--text-main)] relative transition-colors duration-500">

  {#if playerState.isAddMusicModalOpen}
    <AddMusicModal />
  {/if}

  <div class="flex-1 flex overflow-hidden relative">
    <Sidebar {aktifYol} onYeniPlaylist={yeniPlaylistOlustur} />

    <main 
      class="flex-1 min-w-0 flex flex-col overflow-y-auto custom-scrollbar relative"
      style="background: var(--bg-gradient);"
    >
      {@render children()}
    </main>

    <RightPanel {sagMenuAcik} onClose={() => sagMenuAcik = false} />
  </div>

  <FooterPlayer 
    {sagMenuAcik} 
    onToggleRightPanel={() => sagMenuAcik = !sagMenuAcik} 
  />

</div>

<style>
  :global(::selection) {
    background-color: var(--selection-bg);
    color: white;
  }
  
  :global(.custom-scrollbar::-webkit-scrollbar) { width: 4px; }
  :global(.custom-scrollbar::-webkit-scrollbar-track) { background: transparent; }
  :global(.custom-scrollbar::-webkit-scrollbar-thumb) { 
    background: var(--border); 
    border-radius: 4px; 
  }
  :global(.custom-scrollbar::-webkit-scrollbar-thumb:hover) { 
    background: var(--accent); 
  }
</style>