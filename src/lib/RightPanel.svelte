<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { playerState } from '../store.svelte';
  
  let { sagMenuAcik, onClose } = $props<{ 
    sagMenuAcik: boolean, 
    onClose: () => void 
  }>();

  let sarki = $derived(playerState.aktifSarki);
</script>

<aside class="{sagMenuAcik ? 'flex absolute right-0 top-0 bottom-0 h-full shadow-[0_0_50px_rgba(0,0,0,0.8)] z-50' : 'hidden xl:flex'} w-80 shrink-0 bg-[#341d32] flex-col z-10 border-l border-white/5 relative overflow-hidden transition-all duration-300">
  
  {#if playerState.suAnOynuyorMu}
    <div class="absolute inset-0 bg-gradient-to-b from-pink-500/10 to-transparent blur-3xl -z-10 transition-opacity duration-1000"></div>
  {/if}

  <div class="pointer-events-none absolute inset-0 scanlines opacity-10 z-0"></div>
  
  <div class="p-6 flex flex-col h-full z-10">
    <div class="flex justify-between text-[11px] font-bold text-gray-400 mb-6 border-b border-white/10 pb-4 relative tracking-widest shrink-0">
      {#if sagMenuAcik}
        <button onclick={onClose} class="xl:hidden absolute -top-4 -right-4 p-2 text-white/50 hover:text-pink-400 transition-colors" aria-label="Kapat">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
        </button>
      {/if}
      <button type="button" aria-label="Geri" class="hover:text-white cursor-pointer transition-colors">BACK</button>
      <button type="button" aria-label="Sıradaki" class="hover:text-white cursor-pointer transition-colors">UP NEXT</button>
      <button type="button" aria-label="Şarkı Sözleri" class="text-white border-b-2 border-pink-500 pb-4 -mb-4">LYRICS</button>
    </div>
    
    <div class="flex-1 flex flex-col gap-3 text-center text-sm font-medium overflow-y-auto custom-scrollbar pr-2">
      {#if sarki}
        
        <div class="w-full aspect-square bg-black/40 rounded-xl mb-2 shadow-[0_10px_30px_rgba(0,0,0,0.4)] overflow-hidden border border-white/10 relative group shrink-0">
            {#if sarki.kapak_yolu}
              <img src={convertFileSrc(sarki.kapak_yolu)} alt="Kapak" class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-700" />
            {:else}
              <div class="w-full h-full flex items-center justify-center opacity-30 text-white">
                <svg class="w-16 h-16" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"></path></svg>
              </div>
            {/if}
            <div class="absolute inset-0 bg-pink-500 mix-blend-overlay opacity-0 group-hover:opacity-20 transition-opacity duration-300 pointer-events-none"></div>
        </div>
        
        <h2 class="text-xl font-black text-white tracking-tight truncate glitch-text" data-text={sarki.isim}>
          {sarki.isim}
        </h2>
        <p class="text-pink-400 font-bold text-[11px] uppercase tracking-widest truncate">{sarki.sarkici}</p>
        
        <div class="flex flex-wrap justify-center gap-2 mt-1 mb-4">
          {#if sarki.yil}
            <span class="px-2 py-0.5 bg-white/5 border border-white/10 rounded text-[9px] font-bold text-white/60 uppercase tracking-widest">{sarki.yil}</span>
          {/if}
          {#if sarki.kalite}
            <span class="px-2 py-0.5 bg-pink-500/10 border border-pink-500/20 rounded text-[9px] font-bold text-pink-400 uppercase tracking-widest">{sarki.kalite}</span>
          {/if}
        </div>
        
        <div class="mt-2 text-left w-full relative z-10">
          <div class="text-[9px] font-bold text-white/30 tracking-[0.2em] uppercase mb-3 border-b border-white/10 pb-2">
            Sistem Kayıtları
          </div>
          
          {#if sarki.notlar && sarki.notlar.trim() !== ""}
            <div class="font-mono text-xs leading-relaxed text-white/70 whitespace-pre-wrap terminal-text pb-6">
              {sarki.notlar}
            </div>
          {:else}
            <div class="flex flex-col items-center justify-center py-6 opacity-30">
              <svg class="w-8 h-8 mb-2 text-white" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path></svg>
              <p class="font-mono text-[10px] uppercase tracking-widest text-center">Metin verisi bulunamadı.</p>
            </div>
          {/if}
        </div>

      {:else}
        <div class="flex-1 flex flex-col items-center justify-center opacity-30 mt-10">
          <svg class="w-16 h-16 mb-4 text-white" fill="none" stroke="currentColor" stroke-width="1" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
          <p class="font-mono text-[10px] uppercase tracking-widest text-center">Veri Akışı Bekleniyor...</p>
        </div>
      {/if}
    </div>
  </div>
</aside>

<style>
  .custom-scrollbar::-webkit-scrollbar {
    width: 4px;
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(236, 72, 153, 0.3);
  }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover {
    background: rgba(236, 72, 153, 0.6);
  }

  .glitch-text {
    position: relative;
  }
  .glitch-text::before,
  .glitch-text::after {
    content: attr(data-text);
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: transparent;
  }
  .glitch-text::before {
    left: 2px;
    text-shadow: -1px 0 red;
    clip: rect(24px, 550px, 90px, 0);
    animation: glitch-anim-2 3s infinite linear alternate-reverse;
  }
  .glitch-text::after {
    left: -2px;
    text-shadow: -1px 0 blue;
    clip: rect(85px, 550px, 140px, 0);
    animation: glitch-anim 2.5s infinite linear alternate-reverse;
  }

  @keyframes glitch-anim {
    0% { clip: rect(10px, 9999px, 83px, 0); }
    20% { clip: rect(62px, 9999px, 12px, 0); }
    40% { clip: rect(23px, 9999px, 98px, 0); }
    60% { clip: rect(85px, 9999px, 42px, 0); }
    80% { clip: rect(14px, 9999px, 73px, 0); }
    100% { clip: rect(53px, 9999px, 29px, 0); }
  }
  @keyframes glitch-anim-2 {
    0% { clip: rect(65px, 9999px, 100px, 0); }
    20% { clip: rect(10px, 9999px, 45px, 0); }
    40% { clip: rect(80px, 9999px, 12px, 0); }
    60% { clip: rect(25px, 9999px, 85px, 0); }
    80% { clip: rect(95px, 9999px, 32px, 0); }
    100% { clip: rect(40px, 9999px, 68px, 0); }
  }

  .terminal-text {
    text-shadow: 0 0 5px rgba(255, 255, 255, 0.3);
  }

  .scanlines {
    background: linear-gradient(
      to bottom,
      rgba(255,255,255,0),
      rgba(255,255,255,0) 50%,
      rgba(0,0,0,0.15) 50%,
      rgba(0,0,0,0.15)
    );
    background-size: 100% 4px;
  }
</style>