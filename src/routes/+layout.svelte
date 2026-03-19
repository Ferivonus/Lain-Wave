<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/state'; 
  import '../app.css';
  // Fonksiyonlarımızı store'dan içe aktarıyoruz
  import { playerState, oynatDuraklatToggle, initializePlayer, yeniPlaylistOlustur } from '../store.svelte';
  
  import Sidebar from '$lib/Sidebar.svelte';
  import RightPanel from '$lib/RightPanel.svelte';
  import AddMusicModal from '$lib/AddMusicModal.svelte';
  import FooterPlayer from '$lib/FooterPlayer.svelte';
  
  let { children } = $props();
  let aktifYol = $derived(page.url.pathname as string);
  let sagMenuAcik = $state(false);

  onMount(async () => {
    // 1. Sadece sesi ayarla
    const kayitliSes = localStorage.getItem('lainwave_ses');
    if (kayitliSes) {
      const ses = parseFloat(kayitliSes);
      playerState.sesSeviyesi = ses;
      if (playerState.audioRef) playerState.audioRef.volume = ses;
    }
    
    // 2. Koca bir veri çekme bloğunu tek satıra indirdik!
    await initializePlayer();
  });

  // Klavye kısayolu UI/UX ile ilgili olduğu için layout içinde kalması mantıklı
  function klavyeKisaYollari(e: KeyboardEvent) {
    const hedef = e.target as HTMLElement;
    if (['INPUT', 'TEXTAREA', 'BUTTON'].includes(hedef.tagName)) return;
    if (e.key === ' ' || e.code === 'Space') {
      e.preventDefault(); 
      oynatDuraklatToggle();
    }
  }
</script>

<svelte:window onkeydown={klavyeKisaYollari} />

<div class="h-screen w-full flex flex-col font-sans overflow-hidden bg-[#261825] text-white selection:bg-pink-500 relative">
  
  {#if playerState.isAddMusicModalOpen}
    <AddMusicModal />
  {/if}

  <div class="flex-1 flex overflow-hidden relative">
    <Sidebar {aktifYol} onYeniPlaylist={yeniPlaylistOlustur} />

    <main class="flex-1 min-w-0 flex flex-col bg-gradient-to-br from-[#d972b3] to-[#8d6288] overflow-y-auto custom-scrollbar relative">
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
  :global(.custom-scrollbar::-webkit-scrollbar) { width: 4px; }
  :global(.custom-scrollbar::-webkit-scrollbar-track) { background: transparent; }
  :global(.custom-scrollbar::-webkit-scrollbar-thumb) { background: rgba(255,255,255,0.2); border-radius: 4px; }
  :global(.custom-scrollbar::-webkit-scrollbar-thumb:hover) { background: rgba(255,255,255,0.4); }
</style>
