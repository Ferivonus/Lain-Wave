<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import FavoriteButton from '$lib/FavoriteButton.svelte';
  import SongStats from '$lib/SongStats.svelte';
  import { 
    playerState, 
    sarkiCal, 
    type Sarki, 
    initializePlayer, 
    sarkiPlaylisteEkle, 
    sarkiSil, 
    siraGuncelle 
  } from '../../store.svelte';
  import { fade, fly } from 'svelte/transition';

  onMount(async () => {
    if(playerState.sarkiListesi.length === 0) {
         await initializePlayer();
    }
  });

  async function handlePlaylistEkle(sarkiId: string, event: Event) {
    const selectElement = event.target as HTMLSelectElement;
    const playlistId = selectElement.value;
    if (!playlistId) return;

    const basarili = await sarkiPlaylisteEkle(sarkiId, playlistId);
    if(basarili) {
        selectElement.value = ""; 
    }
  }

  async function handleSarkiSil(sarki: Sarki, event: MouseEvent | KeyboardEvent) {
    event.preventDefault();
    event.stopPropagation();
    
    const mesaj = `"${sarki.isim}" adlı parçayı kütüphaneden ve diskten KALICI olarak silmek istediğinize emin misiniz?\n\nBu işlem geri alınamaz.`;
    
    if (confirm(mesaj)) {
        try {
            await sarkiSil(sarki);
        } catch (hata) {
            alert("Silme işlemi sırasında bir hata oluştu.");
        }
    }
  }

  let suruklenenIndex = $state<number | null>(null);
  let uzerindeGezinilenIndex = $state<number | null>(null);

  function dragBasla(event: DragEvent, index: number) {
    suruklenenIndex = index;
    if (event.dataTransfer) {
        event.dataTransfer.effectAllowed = "move";
    }
  }

  function dragUzerinde(event: DragEvent, index: number) {
    event.preventDefault(); 
    uzerindeGezinilenIndex = index;
  }

  async function drop(event: DragEvent, hedefIndex: number) {
    event.preventDefault();
    if (suruklenenIndex !== null && suruklenenIndex !== hedefIndex) {
      let yeniListe = [...playerState.sarkiListesi];
      const [suruklenenSarki] = yeniListe.splice(suruklenenIndex, 1); 
      yeniListe.splice(hedefIndex, 0, suruklenenSarki); 
      await siraGuncelle(yeniListe);
    }
    dragBitir();
  }

  function dragBitir() {
    suruklenenIndex = null;
    uzerindeGezinilenIndex = null;
  }
</script>

<div class="p-8 lg:p-12 w-full min-h-full pb-32 flex flex-col relative min-w-0 bg-transparent text-[var(--text-main)] transition-colors duration-500 overflow-y-auto custom-scrollbar">
  
  <header class="flex flex-col md:flex-row gap-8 items-center md:items-end mb-12 mt-4" in:fade>
    <div class="w-48 h-48 lg:w-56 lg:h-56 bg-gradient-to-br from-[var(--accent)] via-[var(--accent-sec)] to-[var(--bg-surface)] rounded-[var(--radius)] shadow-2xl flex-shrink-0 border border-white/10 overflow-hidden relative group flex items-center justify-center">
        <div class="absolute inset-0 bg-black/10 group-hover:bg-transparent transition-colors duration-500"></div>
        <span class="text-7xl lg:text-8xl drop-shadow-2xl group-hover:scale-110 transition-transform duration-700 select-none">🎧</span>
    </div>
    
    <div class="flex flex-col text-center md:text-left pb-2 min-w-0">
      <span class="text-[10px] font-black mb-3 text-[var(--accent)] tracking-[0.4em] uppercase italic">Local Frequency Archive</span>
      <h1 class="text-5xl lg:text-7xl font-black tracking-tighter leading-none mb-6 uppercase italic drop-shadow-md">
        Kütüphane
      </h1>
      <div class="flex flex-col sm:flex-row items-center gap-6">
        <p class="text-[var(--text-dim)] text-xs lg:text-sm font-bold uppercase tracking-widest whitespace-nowrap">
          {playerState.sarkiListesi.length} Kayıtlı Veri Bloğu • Çevrimiçi
        </p>
        
        <button 
            type="button"
            onclick={() => playerState.isAddMusicModalOpen = true} 
            class="flex items-center gap-3 bg-[var(--text-main)] text-[var(--bg-main)] hover:bg-[var(--accent)] hover:text-white px-10 py-3.5 rounded-full font-black shadow-xl transition-all active:scale-95 uppercase text-[10px] lg:text-xs tracking-widest"
            aria-label="Yeni Müzik Ekle"
        >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="3" viewBox="0 0 24 24" aria-hidden="true"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
            Yeni Parça Ekle
        </button>
      </div>
    </div>
  </header>

  {#if playerState.sarkiListesi.length === 0}
    <div class="flex flex-col items-center justify-center flex-1 text-center mt-10 p-16 bg-[var(--bg-card)] border border-[var(--border)] rounded-[var(--radius)] border-dashed" in:fade>
      <div class="text-6xl mb-8 opacity-20 filter grayscale">📡</div>
      <h3 class="text-2xl font-bold mb-3 tracking-tight uppercase">Sinyal Bulunamadı</h3>
      <p class="text-[var(--text-dim)] font-medium text-sm max-w-sm leading-relaxed uppercase tracking-wider">
        Arşivin şu an boş görünüyor. Hemen yerel diskinden parçalar aktararak kendi Lain Wave kütüphaneni oluşturmaya başla.
      </p>
    </div>
  {:else}
    <div class="flex text-[10px] font-black text-[var(--text-dim)] border-b border-[var(--border)] pb-3 mb-4 px-4 tracking-[0.2em] uppercase">
      <span class="w-8 shrink-0"></span> 
      <span class="w-10 text-center shrink-0">#</span>
      <span class="flex-1 min-w-0 ml-4">BAŞLIK</span>
      <span class="w-32 shrink-0 hidden md:block">İSTATİSTİK</span> 
      <span class="w-1/4 shrink-0 pl-6 hidden lg:block">ALBÜM</span>
      <span class="w-48 text-center shrink-0">İŞLEMLER</span> 
    </div>

    <div class="flex flex-col gap-1.5">
      {#each playerState.sarkiListesi as sarki, index}
        <div 
          role="button" 
          tabindex="0" 
          draggable="true"
          ondragstart={(e) => dragBasla(e, index)}
          ondragover={(e) => dragUzerinde(e, index)}
          ondrop={(e) => drop(e, index)}
          ondragend={dragBitir}
          onclick={() => sarkiCal(sarki)} 
          onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(sarki)} 
          aria-label="{sarki.isim} parçasını çal"
          class="flex items-center text-sm p-2.5 rounded-2xl transition-all duration-300 cursor-pointer group 
                 {playerState.aktifSarki?.id === sarki.id ? 'bg-[var(--accent)]/10 shadow-inner' : 'hover:bg-[var(--bg-card-hover)]'}
                 {uzerindeGezinilenIndex === index ? 'border-t-2 border-[var(--accent)] bg-[var(--accent)]/5' : 'border-t-2 border-transparent'}"
        >
          
          <div class="w-8 text-[var(--text-dim)]/20 hover:text-[var(--accent)] cursor-grab active:cursor-grabbing flex items-center justify-center opacity-0 group-hover:opacity-100 transition-all shrink-0" title="Sıralamayı Değiştir">
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M9 3v2H7V3h2zm0 8v2H7v-2h2zm0 8v2H7v-2h2zm6-16v2h-2V3h2zm0 8v2h-2v-2h2zm0 8v2h-2v-2h2z"/></svg>
          </div>

          <div class="w-10 text-center shrink-0">
             {#if playerState.aktifSarki?.id === sarki.id && playerState.suAnOynuyorMu}
                <div class="flex items-end justify-center gap-0.5 h-3.5 mb-1">
                   <div class="w-1 bg-[var(--accent)] animate-[bounce_1s_infinite]"></div>
                   <div class="w-1 bg-[var(--accent)] animate-[bounce_1.2s_infinite]"></div>
                   <div class="w-1 bg-[var(--accent)] animate-[bounce_0.8s_infinite]"></div>
                </div>
             {:else}
                <span class="text-[var(--text-dim)]/40 group-hover:hidden font-mono text-xs">{index + 1}</span>
                <svg class="w-4 h-4 mx-auto hidden group-hover:block text-[var(--accent)]" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M8 5v14l11-7z"/></svg>
             {/if}
          </div>
          
          <div class="flex-1 flex items-center gap-4 min-w-0 ml-4">
            <div class="w-11 h-11 bg-[var(--bg-card)] rounded-lg overflow-hidden shrink-0 shadow-md border border-[var(--border)]">
              {#if sarki.kapak_yolu}
                <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover transition-transform group-hover:scale-110" />
              {:else}
                <div class="w-full h-full flex items-center justify-center text-xs text-[var(--text-dim)]/30 bg-[var(--bg-surface)]">🎵</div>
              {/if}
            </div>
            <div class="flex flex-col min-w-0">
              <span class="font-bold truncate text-sm {playerState.aktifSarki?.id === sarki.id ? 'text-[var(--accent)]' : 'text-[var(--text-main)]'}">{sarki.isim}</span>
              <a href="/artist/{encodeURIComponent(sarki.sarkici)}" onclick={(e) => e.stopPropagation()} class="text-[11px] text-[var(--text-dim)] truncate font-bold uppercase tracking-tight opacity-70 group-hover:text-[var(--accent)] transition-colors inline-block max-w-max">{sarki.sarkici}</a>
            </div>
          </div>

          <div class="w-32 shrink-0 hidden md:flex items-center">
            <SongStats {sarki} />
          </div>

          <span class="w-1/4 text-[var(--text-dim)] truncate font-black text-[10px] uppercase tracking-tighter pl-6 shrink-0 hidden lg:block opacity-60">
            {sarki.album || "Single"}
          </span>

          <div class="w-48 shrink-0 flex items-center justify-end gap-3 pr-2" 
               onclick={(e) => e.stopPropagation()} 
               onkeydown={(e) => e.stopPropagation()} 
               role="presentation">
            
            <FavoriteButton sarkiId={sarki.id} />
            
            <select 
                aria-label="Listeye Ekle" 
                onchange={(e) => handlePlaylistEkle(sarki.id, e)} 
                class="bg-[var(--bg-surface)] text-[10px] text-[var(--text-dim)] rounded-lg px-2 py-1.5 outline-none border border-[var(--border)] hover:border-[var(--accent)]/50 cursor-pointer w-24 focus:border-[var(--accent)] transition-all font-bold uppercase hidden sm:block opacity-0 group-hover:opacity-100"
            >
              <option value="">➕ EKLE</option>
              {#each playerState.playlistler as pl}
                {#if !pl.sarkilar.includes(sarki.id)}
                  <option value={pl.id}>{pl.isim.toUpperCase()}</option>
                {/if}
              {/each}
            </select>

            <button 
                type="button" 
                aria-label="Kütüphaneden Sil" 
                title="Sil" 
                onclick={(e) => handleSarkiSil(sarki, e)} 
                class="text-[var(--text-dim)]/30 hover:text-red-500 transition-all p-1 opacity-0 group-hover:opacity-100"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
            </button>

          </div>
          
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  @keyframes bounce {
    0%, 100% { height: 4px; }
    50% { height: 14px; }
  }

  .custom-scrollbar::-webkit-scrollbar { width: 4px; }
  .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--accent); }

  .cursor-grab:active { cursor: grabbing; }
</style>