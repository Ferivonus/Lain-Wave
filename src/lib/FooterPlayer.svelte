<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core';
  import FavoriteButton from './FavoriteButton.svelte';
  import { 
    playerState, 
    oncekiSarki, 
    oynatDuraklatToggle, 
    sonrakiSarki, 
    discordGuncelle 
  } from '../store.svelte';

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

  let zamanCubuguRef = $state<HTMLElement | null>(null);
  let zamanSurukleniyor = $state(false);

  function handleZamanPointerDown(e: PointerEvent) {
    zamanSurukleniyor = true;
    zamanCubuguRef?.setPointerCapture(e.pointerId);
    zamanAyarla(e);
  }

  function handleZamanPointerMove(e: PointerEvent) {
    if (zamanSurukleniyor) zamanAyarla(e);
  }

  function handleZamanPointerUp(e: PointerEvent) {
    zamanSurukleniyor = false;
    zamanCubuguRef?.releasePointerCapture(e.pointerId);
  }

  function zamanAyarla(e: PointerEvent | MouseEvent) {
    if (!playerState.aktifSarki || !playerState.audioRef || !playerState.toplamZaman || !zamanCubuguRef) return;
    const rect = zamanCubuguRef.getBoundingClientRect();
    const x = Math.max(0, Math.min(e.clientX - rect.left, rect.width));
    playerState.audioRef.currentTime = (x / rect.width) * playerState.toplamZaman;
  }

  function klavyeSarkiSar(olay: KeyboardEvent) {
    if (!playerState.aktifSarki || !playerState.audioRef || !playerState.toplamZaman) return;
    if (olay.key === 'ArrowRight') {
        playerState.audioRef.currentTime = Math.min(playerState.toplamZaman, playerState.audioRef.currentTime + 5);
    } else if (olay.key === 'ArrowLeft') {
        playerState.audioRef.currentTime = Math.max(0, playerState.audioRef.currentTime - 5);
    }
  }

  let sesCubuguRef = $state<HTMLElement | null>(null);
  let sesSurukleniyor = $state(false);
  let oncekiSesSeviyesi = $state<number>(1);

  function sesKapatAc() {
    if (!playerState.audioRef) return;
    if (playerState.sesSeviyesi > 0) {
      oncekiSesSeviyesi = playerState.sesSeviyesi;
      playerState.sesSeviyesi = 0;
      playerState.audioRef.volume = 0;
    } else {
      const yeniSes = oncekiSesSeviyesi > 0 ? oncekiSesSeviyesi : 1;
      playerState.sesSeviyesi = yeniSes;
      playerState.audioRef.volume = yeniSes;
    }
    localStorage.setItem('lainwave_ses', playerState.sesSeviyesi.toString());
  }

  function handleSesPointerDown(e: PointerEvent) {
    sesSurukleniyor = true;
    sesCubuguRef?.setPointerCapture(e.pointerId);
    sesAyarla(e);
  }

  function handleSesPointerMove(e: PointerEvent) {
    if (sesSurukleniyor) sesAyarla(e);
  }

  function handleSesPointerUp(e: PointerEvent) {
    sesSurukleniyor = false;
    sesCubuguRef?.releasePointerCapture(e.pointerId);
  }

  function sesAyarla(e: PointerEvent | MouseEvent) {
    if (!playerState.audioRef || !sesCubuguRef) return;
    const rect = sesCubuguRef.getBoundingClientRect();
    const x = Math.max(0, Math.min(e.clientX - rect.left, rect.width));
    const yeniSes = x / rect.width;
    playerState.audioRef.volume = yeniSes;
    playerState.sesSeviyesi = yeniSes;
    localStorage.setItem('lainwave_ses', yeniSes.toString());
  }

  function klavyeSesAyarla(olay: KeyboardEvent) {
    if (!playerState.audioRef) return;
    if (olay.key === 'ArrowRight' || olay.key === 'ArrowUp') {
        const yeniSes = Math.min(1, playerState.sesSeviyesi + 0.1);
        playerState.audioRef.volume = yeniSes;
        playerState.sesSeviyesi = yeniSes;
        localStorage.setItem('lainwave_ses', yeniSes.toString());
    } else if (olay.key === 'ArrowLeft' || olay.key === 'ArrowDown') {
        const yeniSes = Math.max(0, playerState.sesSeviyesi - 0.1);
        playerState.audioRef.volume = yeniSes;
        playerState.sesSeviyesi = yeniSes;
        localStorage.setItem('lainwave_ses', yeniSes.toString());
    }
  }

  function handleSesInput(e: Event) {
    const input = e.target as HTMLInputElement;
    let val = parseInt(input.value);
    if (isNaN(val)) return;
    val = Math.max(0, Math.min(100, val));
    const yeniSes = val / 100;
    if (playerState.audioRef) playerState.audioRef.volume = yeniSes;
    playerState.sesSeviyesi = yeniSes;
    localStorage.setItem('lainwave_ses', yeniSes.toString());
  }

  function toggleTekrarModu() {
    if (playerState.tekrarModu === 'kapali') {
        playerState.tekrarModu = 'liste';
    } else if (playerState.tekrarModu === 'liste') {
        playerState.tekrarModu = 'tek_sarki';
    } else {
        playerState.tekrarModu = 'kapali';
    }
  }
</script>

<footer class="h-20 lg:h-24 bg-[var(--bg-surface)] border-t border-[var(--border)] flex items-center justify-between px-4 lg:px-8 z-40 shadow-[0_-10px_30px_rgba(0,0,0,0.3)] relative transition-all duration-500 backdrop-blur-md">
  
  <div class="flex items-center gap-4 w-1/3 min-w-0">
    <div class="w-12 h-12 lg:w-14 lg:h-14 bg-[var(--bg-card)] rounded-xl shadow-lg border border-[var(--border)] flex-shrink-0 overflow-hidden group relative">
      {#if playerState.aktifSarki?.kapak_yolu}
        <img src={convertFileSrc(playerState.aktifSarki.kapak_yolu)} alt="Albüm Kapağı" class="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110" />
      {:else}
        <div class="w-full h-full flex items-center justify-center bg-gradient-to-br from-[var(--accent)]/20 to-[var(--accent-sec)]/20">
          <svg class="w-6 h-6 text-[var(--text-dim)]" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/></svg>
        </div>
      {/if}
    </div>
    
    <div class="flex flex-col min-w-0">
      <span class="text-sm font-bold text-[var(--text-main)] truncate leading-tight">
        {playerState.aktifSarki?.isim || "Lain Wave"}
      </span>
      {#if playerState.aktifSarki}
        <a href="/artist/{encodeURIComponent(playerState.aktifSarki.sarkici)}" class="text-[11px] text-[var(--text-dim)] font-bold uppercase tracking-widest truncate hover:text-[var(--accent)] transition-colors mt-0.5">
          {playerState.aktifSarki.sarkici}
        </a>
      {:else}
        <span class="text-[10px] text-[var(--text-dim)] font-bold uppercase tracking-widest truncate mt-0.5">Sistem Hazır</span>
      {/if}       
    </div>
    
    <div class="hidden md:flex items-center ml-2">
      {#if playerState.aktifSarki}
          <FavoriteButton sarkiId={playerState.aktifSarki.id} />
      {/if}
    </div>
  </div>

  <div class="flex flex-col items-center w-1/3 lg:max-w-xl">
    <div class="flex items-center gap-4 md:gap-6 mb-2">
      <button 
        type="button" 
        aria-label="Karışık Çal" 
        disabled={!playerState.aktifSarki}
        class="hidden sm:block text-[var(--text-dim)] hover:text-[var(--accent)] transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24" aria-hidden="true"><polyline points="16 3 21 3 21 8"></polyline><line x1="4" y1="20" x2="21" y2="3"></line><polyline points="21 16 21 21 16 21"></polyline></svg>
      </button>

      <button 
        type="button" 
        aria-label="Önceki Şarkı" 
        onclick={oncekiSarki} 
        disabled={!playerState.aktifSarki}
        class="text-[var(--text-main)] hover:scale-110 transition-transform active:scale-95 disabled:opacity-30 disabled:hover:scale-100 disabled:cursor-not-allowed shrink-0"
      >
        <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M6 6h2v12H6zm3.5 6l8.5 6V6z"/></svg>
      </button>

      <button 
        type="button"
        onclick={oynatDuraklatToggle} 
        aria-label={playerState.suAnOynuyorMu ? "Duraklat" : "Oynat"}
        disabled={!playerState.aktifSarki}
        class="w-10 h-10 lg:w-12 lg:h-12 shrink-0 bg-[var(--text-main)] text-[var(--bg-main)] rounded-full flex items-center justify-center hover:scale-105 transition-all shadow-[0_5px_15px_rgba(0,0,0,0.3)] active:scale-95 disabled:opacity-50 disabled:hover:scale-100 disabled:cursor-not-allowed"
      >
        {#if playerState.suAnOynuyorMu} 
          <svg class="w-5 h-5 lg:w-6 lg:h-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/></svg>
        {:else} 
          <svg class="w-5 h-5 lg:w-6 lg:h-6 ml-1" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M8 5v14l11-7z"/></svg>
        {/if}
      </button>

      <button 
        type="button" 
        aria-label="Sonraki Şarkı" 
        onclick={sonrakiSarki} 
        disabled={!playerState.aktifSarki}
        class="text-[var(--text-main)] hover:scale-110 transition-transform active:scale-95 disabled:opacity-30 disabled:hover:scale-100 disabled:cursor-not-allowed shrink-0"
      >
        <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z"/></svg>
      </button>

      <button 
        type="button" 
        aria-label="Tekrar Modu" 
        onclick={toggleTekrarModu}
        disabled={!playerState.aktifSarki}
        class="hidden sm:block transition-all disabled:opacity-30 disabled:cursor-not-allowed 
               {playerState.tekrarModu !== 'kapali' ? 'text-[var(--accent)] drop-shadow-[0_0_5px_var(--accent-glow)]' : 'text-[var(--text-dim)] hover:text-[var(--text-main)]'}"
        title={playerState.tekrarModu === 'tek_sarki' ? "Şarkıyı Tekrarla" : playerState.tekrarModu === 'liste' ? "Listeyi Tekrarla" : "Tekrar Kapalı"}
      >
        {#if playerState.tekrarModu === 'tek_sarki'}
          <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" viewBox="0 0 24 24">
            <polyline points="17 1 21 5 17 9"></polyline>
            <path d="M3 11V9a4 4 0 0 1 4-4h14"></path>
            <polyline points="7 23 3 19 7 15"></polyline>
            <path d="M21 13v2a4 4 0 0 1-4 4H3"></path>
            <path d="M11 10h1v4"></path>
          </svg>
        {:else}
          <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" viewBox="0 0 24 24">
            <polyline points="17 1 21 5 17 9"></polyline>
            <path d="M3 11V9a4 4 0 0 1 4-4h14"></path>
            <polyline points="7 23 3 19 7 15"></polyline>
            <path d="M21 13v2a4 4 0 0 1-4 4H3"></path>
          </svg>
        {/if}
      </button>
    </div>
    
    <div class="hidden sm:flex items-center gap-3 w-full text-[10px] font-bold text-[var(--text-dim)] uppercase tracking-widest">
      <span class="w-10 text-right shrink-0">{formatZaman(playerState.suAnkiZaman)}</span>
      <div 
        bind:this={zamanCubuguRef}
        role="slider" 
        tabindex="0" 
        aria-valuemin="0"
        aria-valuemax={playerState.toplamZaman || 100}
        aria-valuenow={playerState.suAnkiZaman}
        onpointerdown={handleZamanPointerDown}
        onpointermove={handleZamanPointerMove}
        onpointerup={handleZamanPointerUp}
        onpointercancel={handleZamanPointerUp}
        onkeydown={klavyeSarkiSar}
        aria-label="Şarkı ilerleme çubuğu"
        class="flex-1 h-1.5 bg-[var(--border)] rounded-full cursor-pointer relative group transition-all hover:h-2 touch-none {!playerState.aktifSarki ? 'opacity-50 pointer-events-none' : ''}"
      >
        <div 
          class="absolute top-0 left-0 h-full bg-[var(--accent)] rounded-full transition-all duration-100 ease-linear pointer-events-none" 
          style="width: {(playerState.suAnkiZaman / (playerState.toplamZaman || 1)) * 100}%"
        >
          <div class="absolute right-0 top-1/2 -translate-y-1/2 w-3 h-3 bg-white rounded-full shadow-lg opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none"></div>
        </div>
      </div>
      <span class="w-10 shrink-0">{formatZaman(playerState.toplamZaman)}</span>
    </div>
  </div>

  <div class="flex items-center justify-end gap-4 w-1/3 text-[var(--text-dim)] min-w-0">
    <div class="hidden md:flex items-center gap-3 group">
      
      <button 
        type="button" 
        aria-label="Sesi Aç/Kapat"
        onclick={sesKapatAc}
        class="shrink-0 transition-transform hover:scale-110 active:scale-90 group-hover:text-[var(--text-main)]"
      >
        <svg class="w-5 h-5 transition-colors" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          {#if playerState.sesSeviyesi === 0}
            <path d="M16.5 12c0-1.77-1.02-3.29-2.5-4.03v2.21l2.45 2.45c.03-.2.05-.41.05-.63zm2.5 0c0 .94-.2 1.82-.54 2.64l1.51 1.51C20.63 14.91 21 13.5 21 12c0-4.28-2.99-7.86-7-8.77v2.06c2.89.86 5 3.54 5 6.71zM4.27 3L3 4.27 7.73 9H3v6h4l5 5v-6.73l4.25 4.25c-.67.52-1.42.93-2.25 1.18v2.06c1.38-.31 2.63-.95 3.69-1.81L19.73 21 21 19.73l-9-9L4.27 3zM12 4L9.91 6.09 12 8.18V4z"/>
          {:else if playerState.sesSeviyesi < 0.5}
            <path d="M3 9v6h4l5 5V4L7 9H3zm13.5 3c0-1.77-1.02-3.29-2.5-4.03v8.05c1.48-.73 2.5-2.25 2.5-4.02z"/>
          {:else}
            <path d="M3 9v6h4l5 5V4L7 9H3zm13.5 3c0-1.77-1.02-3.29-2.5-4.03v8.05c1.48-.73 2.5-2.25 2.5-4.02zM14 3.23v2.06c2.89.86 5 3.54 5 6.71s-2.11 5.85-5 6.71v2.06c4.01-.91 7-4.49 7-8.77s-2.99-7.86-7-8.77z"/>
          {/if}
        </svg>
      </button>
      
      <div 
        bind:this={sesCubuguRef}
        role="slider" 
        tabindex="0"
        aria-valuemin="0"
        aria-valuemax="100"
        aria-valuenow={Math.round(playerState.sesSeviyesi * 100)}
        onpointerdown={handleSesPointerDown}
        onpointermove={handleSesPointerMove}
        onpointerup={handleSesPointerUp}
        onpointercancel={handleSesPointerUp}
        onkeydown={klavyeSesAyarla}
        aria-label="Ses seviyesi ayarı"
        class="w-16 lg:w-24 h-1.5 bg-[var(--border)] rounded-full cursor-pointer relative group hover:h-2 transition-all touch-none shrink-0"
      >
        <div class="absolute top-0 left-0 h-full bg-[var(--text-dim)] group-hover:bg-[var(--accent)] rounded-full transition-all pointer-events-none" style="width: {playerState.sesSeviyesi * 100}%">
          <div class="absolute right-0 top-1/2 -translate-y-1/2 w-3 h-3 bg-white rounded-full shadow-lg opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none"></div>
        </div>
      </div>

      <div class="flex items-center text-[10px] font-mono font-bold bg-[var(--bg-surface)] border border-[var(--border)] rounded-md px-1 py-1 focus-within:border-[var(--accent)] focus-within:text-[var(--accent)] transition-colors w-12 justify-center ml-1">
          <input 
              type="number" 
              min="0" 
              max="100" 
              value={Math.round(playerState.sesSeviyesi * 100)} 
              oninput={handleSesInput}
              class="w-6 bg-transparent outline-none text-right appearance-none m-0 p-0 leading-none custom-number-input"
          />
          <span class="opacity-50 ml-0.5 leading-none">%</span>
      </div>
    </div>
    
    <button 
      type="button"
      onclick={onToggleRightPanel} 
      class="p-2.5 rounded-xl transition-all {sagMenuAcik ? 'text-[var(--accent)] bg-[var(--accent)]/10' : 'hover:text-[var(--text-main)] hover:bg-[var(--bg-card)]'} shrink-0"
      aria-label="Sağ paneli değiştir"
    >
      <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true">
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
      if (Math.round(zaman) % 2 === 0) {
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
      if (playerState.suAnOynuyorMu) discordGuncelle();
  }}
  onended={sonrakiSarki}
>
</audio>

<style>
  button:focus-visible, div[role="slider"]:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 4px;
    border-radius: var(--radius);
  }

  .custom-number-input::-webkit-inner-spin-button, 
  .custom-number-input::-webkit-outer-spin-button { 
      -webkit-appearance: none;
      appearance: none; 
      margin: 0; 
  }
  .custom-number-input {
      -moz-appearance: textfield;
      appearance: textfield;
  }
</style>