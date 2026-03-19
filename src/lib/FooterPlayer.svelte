<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core';
  import FavoriteButton from './FavoriteButton.svelte';
  // YENİ: discordGuncelle fonksiyonunu import ettik
  import { playerState, oncekiSarki, oynatDuraklatToggle, sonrakiSarki, discordGuncelle } from '../store.svelte';

  let { sagMenuAcik, onToggleRightPanel } = $props<{ 
    sagMenuAcik: boolean, 
    onToggleRightPanel: () => void 
  }>();

  function formatZaman(saniye: number) {
    if (isNaN(saniye) || saniye === 0) return "0:00";
    const dk = Math.floor(saniye / 60);
    const sn = Math.floor(saniye % 60);
    return `${dk}:${sn < 10 ? '0' : ''}${sn}`;
  }

  function sarkiSar(olay: MouseEvent) {
    if (!playerState.aktifSarki || !playerState.audioRef || !playerState.toplamZaman) return;
    const cubuk = olay.currentTarget as HTMLElement;
    const yuzde = Math.max(0, Math.min(1, olay.offsetX / cubuk.clientWidth));
    playerState.audioRef.currentTime = yuzde * playerState.toplamZaman;
  }

  function sesAyarla(olay: MouseEvent) {
    if (!playerState.audioRef) return;
    const cubuk = olay.currentTarget as HTMLElement;
    const yeniSes = Math.max(0, Math.min(1, olay.offsetX / cubuk.clientWidth));
    playerState.audioRef.volume = yeniSes;
    playerState.sesSeviyesi = yeniSes;
    localStorage.setItem('lainwave_ses', yeniSes.toString());
  }
</script>

<footer class="h-20 lg:h-24 bg-[#2b1b2a] border-t border-white/5 flex items-center justify-between px-4 lg:px-6 z-20 shadow-[0_-10px_20px_rgba(0,0,0,0.3)] relative backdrop-blur-md bg-opacity-95">
  
  <div class="flex items-center gap-3 lg:gap-4 w-1/3 min-w-0">
    <div class="w-12 h-12 lg:w-14 lg:h-14 bg-gradient-to-br from-pink-400 to-purple-600 rounded-md shadow-lg border border-white/10 flex items-center justify-center text-xl flex-shrink-0 overflow-hidden">
      {#if playerState.aktifSarki?.kapak_yolu}
        <img src={convertFileSrc(playerState.aktifSarki.kapak_yolu)} alt="Albüm Kapağı" class="w-full h-full object-cover" />
      {:else if playerState.aktifSarki}
        <svg class="w-6 h-6 text-white/50" fill="currentColor" viewBox="0 0 24 24"><path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/></svg>
      {/if}
    </div>
    
    <div class="flex flex-col min-w-0 truncate">
      <span class="text-sm font-bold text-white truncate">{playerState.aktifSarki?.isim || "Hazır"}</span>
      {#if playerState.aktifSarki}
        <a href="/artist/{encodeURIComponent(playerState.aktifSarki.sarkici)}" class="text-xs text-gray-400 truncate hover:text-pink-400 transition-colors">
          {playerState.aktifSarki.sarkici}
        </a>
      {:else}
        <span class="text-xs text-gray-400 truncate">Bir şarkı seçin</span>
      {/if}       
    </div>
    
    <div class="hidden lg:flex items-center ml-2">
      {#if playerState.aktifSarki}
          <FavoriteButton sarkiId={playerState.aktifSarki.id} />
      {/if}
    </div>
  </div>

  <div class="flex flex-col items-center w-1/3 lg:max-w-md">
    <div class="flex items-center gap-4 lg:gap-6 mb-1 lg:mb-2">
      <button type="button" aria-label="Karışık" class="hidden sm:block text-gray-400 hover:text-white transition-colors">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><polyline points="16 3 21 3 21 8"></polyline><line x1="4" y1="20" x2="21" y2="3"></line><polyline points="21 16 21 21 16 21"></polyline><line x1="15" y1="15" x2="21" y2="21"></line><line x1="4" y1="4" x2="9" y2="9"></line></svg>
      </button>
      <button type="button" aria-label="Önceki" onclick={oncekiSarki} class="text-gray-400 hover:text-white transition-colors">
        <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24"><path d="M6 6h2v12H6zm3.5 6l8.5 6V6z"/></svg>
      </button>
      <button type="button" aria-label="Oynat" onclick={oynatDuraklatToggle} class="w-10 h-10 bg-white text-black rounded-full flex items-center justify-center hover:scale-105 transition-transform shadow-[0_0_15px_rgba(255,255,255,0.2)]">
        {#if playerState.suAnOynuyorMu} 
          <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/></svg>
        {:else} 
          <svg class="w-5 h-5 ml-1" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
        {/if}
      </button>
      <button type="button" aria-label="Sonraki" onclick={sonrakiSarki} class="text-gray-400 hover:text-white transition-colors">
        <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24"><path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z"/></svg>
      </button>
      <button type="button" aria-label="Tekrar" class="hidden sm:block text-gray-400 hover:text-white transition-colors">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><polyline points="17 1 21 5 17 9"></polyline><path d="M3 11V9a4 4 0 0 1 4-4h14"></path><polyline points="7 23 3 19 7 15"></polyline><path d="M21 13v2a4 4 0 0 1-4 4H3"></path></svg>
      </button>
    </div>
    
    <div class="hidden sm:flex items-center gap-3 w-full text-xs text-gray-400 font-medium">
      <span class="w-8 text-right">{formatZaman(playerState.suAnkiZaman)}</span>
      <button type="button" aria-label="Sar" onclick={sarkiSar} class="flex-1 h-1.5 bg-gray-700 rounded-full overflow-hidden cursor-pointer hover:h-2 transition-all relative block outline-none border-none p-0">
        <div class="absolute top-0 left-0 h-full bg-pink-400 rounded-full transition-all duration-100 ease-linear pointer-events-none" style="width: {(playerState.suAnkiZaman / (playerState.toplamZaman || 1)) * 100}%"></div>
      </button>
      <span class="w-8">{formatZaman(playerState.toplamZaman)}</span>
    </div>
  </div>

  <div class="flex items-center justify-end gap-4 w-1/3 text-gray-400">
    <svg class="hidden sm:block w-5 h-5 shrink-0" fill="currentColor" viewBox="0 0 24 24"><path d="M3 9v6h4l5 5V4L7 9H3zm13.5 3c0-1.77-1.02-3.29-2.5-4.03v8.05c1.48-.73 2.5-2.25 2.5-4.02zM14 3.23v2.06c2.89.86 5 3.54 5 6.71s-2.11 5.85-5 6.71v2.06c4.01-.91 7-4.49 7-8.77s-2.99-7.86-7-8.77z"/></svg>
    <button type="button" aria-label="Ses" onclick={sesAyarla} class="hidden sm:block w-16 lg:w-24 h-1.5 bg-gray-700 rounded-full cursor-pointer relative overflow-hidden outline-none border-none p-0 group shrink-0">
      <div class="absolute top-0 left-0 h-full bg-white group-hover:bg-pink-400 rounded-full transition-all pointer-events-none" style="width: {playerState.sesSeviyesi * 100}%"></div>
    </button>
    
    <button type="button" aria-label="Sözler ve Sıradaki" onclick={onToggleRightPanel} class="{sagMenuAcik ? 'text-pink-400 bg-white/5' : 'hover:text-white hover:bg-white/5'} p-2 rounded-lg transition-all ml-0 lg:ml-2">
      <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" viewBox="0 0 24 24">
        <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
        <line x1="15" y1="3" x2="15" y2="21"></line>
      </svg>
    </button>
  </div>
</footer>

<audio 
  bind:this={playerState.audioRef} 
  ontimeupdate={() => {
      const zaman = playerState.audioRef?.currentTime || 0;
      playerState.suAnkiZaman = zaman;
      const sonKayitli = parseFloat(localStorage.getItem('lainwave_zaman') || '0');
      if (Math.abs(zaman - sonKayitli) > 1) {
          localStorage.setItem('lainwave_zaman', zaman.toString());
      }
  }} 
  onloadedmetadata={() => {
      playerState.toplamZaman = playerState.audioRef?.duration || 0;
      const kayitliZaman = localStorage.getItem('lainwave_zaman');
      if (kayitliZaman && !playerState.suAnOynuyorMu && playerState.audioRef) {
           playerState.audioRef.currentTime = parseFloat(kayitliZaman);
      }
  }} 
  onseeked={() => {
      // YENİ EKLENEN KISIM: İlerleme çubuğu tıklandığında discordGuncelle çağrılıyor
      if (playerState.suAnOynuyorMu) {
          discordGuncelle();
      }
  }}
  onended={sonrakiSarki}>
</audio>