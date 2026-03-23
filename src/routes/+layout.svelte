<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/state'; 
  import { invoke } from '@tauri-apps/api/core';
  import '../app.css';
  import { playerState, oynatDuraklatToggle, initializePlayer, yeniPlaylistOlustur, type Ayarlar } from '../store.svelte';
  
  import Sidebar from '$lib/Sidebar.svelte';
  import RightPanel from '$lib/RightPanel.svelte';
  import AddMusicModal from '$lib/AddMusicModal.svelte';
  import FooterPlayer from '$lib/FooterPlayer.svelte';
  import EditSongModal from '$lib/EditSongModal.svelte';
  import CreatePlaylistModal from '$lib/CreatePlaylistModal.svelte';
  import { fade, scale } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';
  
  let { children } = $props();
  let aktifYol = $derived(page.url.pathname as string);
  let sagMenuAcik = $state(false);

  let isChecking = $state(true);
  let isWelcomeScreenOpen = $state(false);
  let tempUsername = $state("");
  let isSaving = $state(false);

  onMount(async () => {
    const kayitliSes = localStorage.getItem('lainwave_ses');
    if (kayitliSes) {
      const ses = parseFloat(kayitliSes);
      playerState.sesSeviyesi = ses;
      if (playerState.audioRef) playerState.audioRef.volume = ses;
    }
    
    await initializePlayer();

    if (!playerState.username || playerState.username.trim() === "") {
        isWelcomeScreenOpen = true;
    }
    
    isChecking = false;
  });

  async function kaydetUsername() {
    if (!tempUsername.trim() || isSaving) return;
    isSaving = true;

    try {
        const ayarlar = await invoke<Ayarlar>('ayarlari_getir');
        ayarlar.kullanici_adi = tempUsername.trim();
        await invoke('ayarlari_kaydet', { ayarlar });
        
        playerState.username = tempUsername.trim();
        isWelcomeScreenOpen = false;
    } catch (e) {
        alert("Bağlantı hatası. Sistem profili oluşturulamadı.");
    } finally {
        isSaving = false;
    }
  }

  function klavyeKisaYollari(e: KeyboardEvent) {
    if (isWelcomeScreenOpen || isChecking) return;

    const hedef = e.target as HTMLElement;
    
    if (
      ['INPUT', 'TEXTAREA', 'SELECT', 'BUTTON'].includes(hedef.tagName) || 
      hedef.isContentEditable || 
      hedef.getAttribute('role') === 'button' ||
      hedef.getAttribute('role') === 'dialog' ||
      hedef.closest('button')
    ) {
      return;
    }

    if (e.key === ' ' || e.code === 'Space') {
      e.preventDefault(); 
      oynatDuraklatToggle();
    }
  }

  function focusAcilista(node: HTMLInputElement) {
      node.focus();
  }
</script>

<svelte:window onkeydown={klavyeKisaYollari} />

{#if isChecking}
  <div class="fixed inset-0 bg-[#050505] z-9999 flex items-center justify-center">
      <span class="animate-pulse text-white/50 font-black tracking-[0.5em] uppercase text-xs">Sistem Başlatılıyor...</span>
  </div>
{/if}

{#if isWelcomeScreenOpen && !isChecking}
  <div 
      class="fixed inset-0 z-9999 flex items-center justify-center bg-[#050505]/95 backdrop-blur-2xl p-4"
      in:fade={{ duration: 400 }}
  >
      <div 
          class="bg-[#0a0a0a] w-full max-w-md rounded-3xl shadow-[0_0_80px_rgba(0,0,0,0.8)] border border-white/10 p-10 flex flex-col items-center text-center relative overflow-hidden"
          in:scale={{ start: 0.95, duration: 600, easing: cubicOut, delay: 200 }}
      >
          <div class="absolute top-0 right-0 w-64 h-64 bg-white/5 blur-[80px] -z-10 rounded-full pointer-events-none"></div>
          
          <div class="w-20 h-20 bg-white/5 border border-white/10 text-white rounded-full flex items-center justify-center mb-8 shadow-inner">
              <svg class="w-10 h-10" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path></svg>
          </div>
          
          <h2 class="text-3xl font-black uppercase italic tracking-tighter mb-2 text-white">Sisteme Giriş</h2>
          <p class="text-[10px] font-bold text-white/50 uppercase tracking-widest mb-8 leading-relaxed">Lain Wave ağına bağlanmak için<br>bir kimlik belirleyin</p>
          
          <div class="w-full space-y-4">
              <input 
                  type="text" 
                  bind:value={tempUsername}
                  onkeydown={(e) => e.key === 'Enter' && kaydetUsername()}
                  use:focusAcilista
                  placeholder="Sistem Hitap Adı" 
                  class="w-full bg-white/5 border border-white/10 rounded-2xl p-5 text-center text-sm font-black outline-none focus:border-white/40 transition-all placeholder:text-white/20 text-white tracking-widest uppercase"
              />
              <button 
                  type="button"
                  onclick={kaydetUsername}
                  disabled={!tempUsername.trim() || isSaving}
                  class="w-full bg-white text-black font-black py-5 rounded-2xl shadow-[0_10px_30px_rgba(255,255,255,0.2)] hover:scale-105 active:scale-95 transition-all disabled:opacity-30 disabled:hover:scale-100 uppercase tracking-[0.2em] text-[11px]"
              >
                  {#if isSaving} Bağlanıyor... {:else} Ağa Bağlan {/if}
              </button>
              
              <div class="pt-4 border-t border-white/5">
                  <p class="text-[8px] font-bold text-white/30 uppercase tracking-widest leading-relaxed">
                      Bu kimlik sadece cihazınızda yerel olarak saklanır ve sistemi kişiselleştirmek için kullanılır.<br>İstemiyorsanız rastgele bir isim belirleyebilirsiniz.
                  </p>
              </div>
          </div>
      </div>
  </div>
{/if}

<div class="{playerState.currentTheme} h-screen w-full flex flex-col font-sans overflow-hidden bg-(--bg-main) text-(--text-main) relative transition-colors duration-500">

  <div class="flex-1 flex overflow-hidden relative">
    <Sidebar {aktifYol} onYeniPlaylist={yeniPlaylistOlustur} />

    <main 
      class="flex-1 min-w-0 flex flex-col overflow-y-auto custom-scrollbar relative"
      style="background: var(--bg-gradient);"
    >
      {@render children()}
    </main>

    <RightPanel {sagMenuAcik} onClose={() => { sagMenuAcik = false; }} />
  </div>

  <FooterPlayer 
    {sagMenuAcik} 
    onToggleRightPanel={() => { sagMenuAcik = !sagMenuAcik; }} 
  />

  <AddMusicModal /> 
  <EditSongModal />
  <CreatePlaylistModal />
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