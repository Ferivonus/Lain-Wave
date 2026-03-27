<script lang="ts">
  import { convertFileSrc, invoke } from '@tauri-apps/api/core';
  import { playerState, sarkiCal } from '../store.svelte';
  import { fade, fly, slide } from 'svelte/transition';
  import { tick } from 'svelte';

  let { sagMenuAcik, onClose } = $props<{ 
    sagMenuAcik: boolean, 
    onClose: () => void 
  }>();

  let aktifSekme = $state<'detay' | 'siradaki' | 'sozler'>('detay');
  let sarki = $derived(playerState.aktifSarki);
  
  let siradakiSarkilar = $derived.by(() => {
    if (!sarki) return [];
    const index = playerState.sarkiListesi.findIndex(s => s.id === sarki.id);
    if (index === -1) return [];
    return playerState.sarkiListesi.slice(index + 1, index + 11);
  });

  function duzenleModaliAc() {
    if (!sarki) return;
    playerState.duzenlenecekSarki = sarki;
    playerState.isEditModalOpen = true;
  }

  interface LyricLine { start: number; end: number; text: string; }
  interface AvailableLanguage { dil: string; yol: string; }

  let rawLyrics = $state<LyricLine[]>([]);
  let activeLyricIndex = $state<number>(-1);
  let sozlerYukleniyor = $state(false);
  let sozScrollContainer: HTMLDivElement | null = $state(null);
  let mevcutDiller = $state<AvailableLanguage[]>([]);
  let secilenDilYolu = $state<string>("");

  $effect(() => {
    if (sarki?.sozler_yolu) {
      dilleriGetir(sarki.sozler_yolu);
    } else {
      rawLyrics = [];
      mevcutDiller = [];
    }
  });

  async function dilleriGetir(yol: string) {
    try {
      mevcutDiller = await invoke('mevcut_soz_dillerini_bul', { yol });
      if (mevcutDiller.length > 0) {
        const varolan = mevcutDiller.find(d => d.yol === yol);
        secilenDilYolu = varolan ? varolan.yol : mevcutDiller[0].yol;
        loadLyrics(secilenDilYolu);
      }
    } catch (e) {
      secilenDilYolu = yol;
      loadLyrics(yol);
    }
  }

  function dilDegistir(yol: string) {
    secilenDilYolu = yol;
    loadLyrics(yol);
  }

  $effect(() => {
    const currentTime = playerState.suAnkiZaman;

    if (aktifSekme === 'sozler' && rawLyrics.length > 0) {
      let newIndex = rawLyrics.findIndex(line => currentTime >= line.start && currentTime <= line.end);

      if (newIndex === -1) {
          let closest = -1;
          for (let i = 0; i < rawLyrics.length; i++) {
              if (currentTime > rawLyrics[i].start) closest = i;
              else break;
          }
          newIndex = closest;
      }

      if (newIndex !== activeLyricIndex) {
        activeLyricIndex = newIndex;
        scrollToActiveLyric();
      }
    }
  });

  function parseTime(timeStr: string): number {
    const parts = timeStr.replace(',', '.').split(':');
    if (parts.length < 2) return 0;
    let h = 0, m = 0, s = 0;
    if (parts.length === 3) {
      h = parseFloat(parts[0]); m = parseFloat(parts[1]); s = parseFloat(parts[2]);
    } else {
      m = parseFloat(parts[0]); s = parseFloat(parts[1]);
    }
    return (h * 3600) + (m * 60) + s;
  }

  async function loadLyrics(yol: string) {
    if (!yol) return;
    sozlerYukleniyor = true;
    try {
      const content = await invoke<string>('sarki_sozu_oku', { yol });
      const blocks = content.replace(/\r\n/g, '\n').trim().split(/\n\s*\n/);
      rawLyrics = blocks.map(block => {
        const lines = block.split('\n').filter(l => l.trim());
        if (lines.length >= 2) {
          const timeLineIndex = lines[0].includes('-->') ? 0 : 1;
          const timeMatch = lines[timeLineIndex].split(' --> ');
          const text = lines.slice(timeLineIndex + 1).join(' ');
          if (timeMatch.length === 2) {
             return { start: parseTime(timeMatch[0]), end: parseTime(timeMatch[1]), text };
          }
        }
        return null;
      }).filter(l => l !== null) as LyricLine[];
    } catch (err) {
      rawLyrics = [];
    } finally {
      sozlerYukleniyor = false;
    }
  }

  async function scrollToActiveLyric() {
    await tick();
    if (sozScrollContainer && activeLyricIndex !== -1) {
      const activeEl = sozScrollContainer.children[activeLyricIndex] as HTMLElement;
      if (activeEl) {
        activeEl.scrollIntoView({ behavior: 'smooth', block: 'center' });
      }
    }
  }
</script>

<aside class="{sagMenuAcik ? 'flex absolute right-0 top-0 bottom-0 h-full shadow-[-20px_0_50px_rgba(0,0,0,0.5)] z-50' : 'hidden xl:flex'} w-80 shrink-0 bg-(--bg-surface)/95 backdrop-blur-xl flex-col z-10 border-l border-(--border) relative overflow-hidden transition-all duration-500">
  
  {#if playerState.suAnOynuyorMu}
    <div class="absolute inset-0 opacity-10 blur-[120px] -z-10 transition-opacity duration-1000 mix-blend-screen"
      style="background: radial-gradient(circle at top right, var(--accent), transparent 70%);"></div>
  {/if}

  <div class="p-6 flex flex-col h-full z-10 relative">
    <div class="flex items-center justify-between mb-8 border-b border-(--border) pb-2 relative shrink-0">
      <div class="flex gap-4 relative">
        {#each ['detay', 'siradaki', 'sozler'] as sekme}
          <button type="button" onclick={() => aktifSekme = sekme as any} 
            class="text-[9px] font-black tracking-[0.2em] uppercase transition-all pb-2 -mb-2.5 relative 
            {aktifSekme === sekme ? 'text-(--accent)' : 'text-(--text-dim) hover:text-(--text-main)'}">
            {sekme === 'detay' ? 'DETAY' : sekme === 'siradaki' ? 'KUYRUK' : 'SÖZLER'}
            {#if aktifSekme === sekme}
              <div class="absolute bottom-0 left-0 w-full h-0.5 bg-(--accent) rounded-t-md shadow-[0_0_10px_var(--accent-glow)]" in:fade></div>
            {/if}
          </button>
        {/each}
      </div>

      {#if sagMenuAcik}
        <button 
          type="button" 
          onclick={onClose} 
          aria-label="Paneli Kapat"
          class="xl:hidden p-1.5 rounded-lg text-(--text-dim) hover:bg-(--bg-card) hover:text-(--accent) transition-all -mr-2"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
        </button>
      {/if}
    </div>
    
    <div class="flex-1 overflow-y-auto custom-scrollbar relative pr-1">
      
      {#if aktifSekme === 'detay'}
        <div in:fly={{ x: -10, duration: 300 }} out:fade={{ duration: 150 }} class="absolute w-full pb-6">
          {#if sarki}
            <div class="w-full aspect-square bg-(--bg-card) rounded-2xl mb-6 shadow-2xl overflow-hidden border border-(--border) relative group">
                {#if sarki.kapak_yolu}
                  <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover transition-transform duration-700 group-hover:scale-105" />
                {:else}
                  <div class="w-full h-full flex items-center justify-center bg-linear-to-br from-(--bg-surface) to-(--bg-main)">
                    <svg class="w-20 h-20 text-(--text-dim) opacity-10" fill="currentColor" viewBox="0 0 24 24"><path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/></svg>
                  </div>
                {/if}
                <button 
                  onclick={duzenleModaliAc} 
                  aria-label="Metadatayı Düzenle"
                  title="Metadatayı Düzenle"
                  class="absolute top-4 right-4 p-2.5 bg-black/50 backdrop-blur-md text-white rounded-xl opacity-0 group-hover:opacity-100 transition-all hover:scale-110 active:scale-90 border border-white/10"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><path d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg>
                </button>
            </div>
            
            <div class="mb-8">
              <h2 class="text-xl font-black text-(--text-main) tracking-tight leading-tight mb-1 truncate" title={sarki.isim}>{sarki.isim}</h2>
              <p class="text-(--accent) font-bold text-[11px] uppercase tracking-[0.15em] opacity-90">{sarki.sarkici}</p>
              
              <div class="flex flex-wrap gap-2 mt-4">
                {#if sarki.tarz}<span class="px-2 py-0.5 bg-(--bg-card) border border-(--border) rounded-md text-[8px] font-black text-(--text-dim) uppercase tracking-widest">{sarki.tarz}</span>{/if}
                {#if sarki.yil}<span class="px-2 py-0.5 bg-(--accent-sec)/10 border border-(--accent-sec)/20 rounded-md text-[8px] font-black text-(--accent-sec) uppercase tracking-widest">{sarki.yil}</span>{/if}
              </div>
            </div>
            
            <div class="mt-8 space-y-4">
              <div class="text-[9px] font-black text-(--text-dim) tracking-[0.3em] uppercase border-b border-(--border) pb-2">SİSTEM KAYITLARI</div>
              <div class="font-mono text-[10px] p-4 bg-black/20 rounded-xl border border-(--border) text-(--text-dim) space-y-4">
                <div>
                    <span class="text-(--accent) opacity-50 block mb-1"># DOSYA YOLU:</span>
                    <div class="truncate opacity-60">{sarki.yol}</div>
                </div>
                <div>
                    <span class="text-(--accent) opacity-50 block mb-1"># İŞLEM NOTU:</span>
                    <div class="text-(--text-main)/80 italic line-clamp-4">{sarki.notlar || 'Veri girişi yok...'}</div>
                </div>
              </div>
            </div>
          {:else}
            <div class="py-32 text-center opacity-20"><p class="text-[9px] uppercase font-black tracking-[0.4em]">Sinyal Bekleniyor</p></div>
          {/if}
        </div>

      {:else if aktifSekme === 'siradaki'}
        <div in:fly={{ x: 10, duration: 300 }} out:fade={{ duration: 150 }} class="absolute w-full space-y-3 pb-6">
          <div class="text-[9px] font-black text-(--text-dim) tracking-[0.3em] uppercase mb-4 border-b border-(--border) pb-2">SIRADAKİ ANALİZİ</div>
          {#if siradakiSarkilar.length > 0}
            <div class="flex flex-col gap-2">
              {#each siradakiSarkilar as item}
                <button onclick={() => sarkiCal(item)} class="w-full flex items-center gap-3 p-2 rounded-xl bg-(--bg-card)/40 hover:bg-(--bg-card-hover) border border-transparent hover:border-(--accent)/20 transition-all group text-left">
                  <div class="w-10 h-10 rounded-lg overflow-hidden shrink-0 border border-white/5 relative">
                    {#if item.kapak_yolu}
                      <img src={convertFileSrc(item.kapak_yolu)} alt="" class="w-full h-full object-cover" />
                    {:else}
                      <div class="w-full h-full bg-(--bg-surface) flex items-center justify-center text-(--text-dim) opacity-20 text-xs">🎵</div>
                    {/if}
                  </div>
                  <div class="flex-1 min-w-0 pr-1">
                    <p class="text-[12px] font-bold text-(--text-main) truncate group-hover:text-(--accent) transition-colors leading-tight">{item.isim}</p>
                    <p class="text-[9px] font-bold text-(--text-dim) truncate uppercase tracking-tighter opacity-60">{item.sarkici}</p>
                  </div>
                </button>
              {/each}
            </div>
          {:else}
            <div class="py-24 text-center opacity-20 border border-dashed border-(--border) rounded-2xl"><p class="text-[8px] font-black uppercase tracking-[0.2em]">Kuyruk Sonu</p></div>
          {/if}
        </div>

      {:else if aktifSekme === 'sozler'}
        <div in:fade class="flex flex-col h-full absolute w-full pb-10">
          {#if mevcutDiller.length > 1}
            <div class="flex gap-2 mb-4 overflow-x-auto pb-2 shrink-0 no-scrollbar" in:slide>
              {#each mevcutDiller as dilObj}
                <button onclick={() => dilDegistir(dilObj.yol)}
                  class="px-3 py-1 rounded-full text-[8px] font-black tracking-widest border transition-all shrink-0
                  {secilenDilYolu === dilObj.yol ? 'bg-(--accent) border-(--accent) text-white shadow-lg shadow-(--accent)/20' : 'bg-white/5 border-white/10 text-white/40 hover:text-white'}">
                  {dilObj.dil.toUpperCase()}
                </button>
              {/each}
            </div>
          {/if}

          {#if sozlerYukleniyor}
            <div class="flex-1 flex flex-col items-center justify-center opacity-50 mt-10">
                <div class="w-8 h-8 border-2 border-(--accent) border-t-transparent rounded-full animate-spin mb-4"></div>
                <p class="text-[9px] uppercase font-black tracking-[0.2em]">Veri Okunuyor...</p>
            </div>
          {:else if rawLyrics.length > 0}
            <div class="flex-1 overflow-y-auto custom-scrollbar scroll-smooth space-y-6 text-center px-4 py-[40vh] relative mask-fade" bind:this={sozScrollContainer}>
               {#each rawLyrics as line, index}
                 <p class="transform-gpu transition-all duration-700 ease-[cubic-bezier(0.22,1,0.36,1)] origin-center leading-relaxed text-base
                    {index === activeLyricIndex ? 'text-(--text-main) font-black scale-125 opacity-100 drop-shadow-[0_0_15px_rgba(255,255,255,0.3)]' : 
                     index < activeLyricIndex ? 'text-(--text-dim) font-bold opacity-20 scale-95 blur-[1px]' : 
                     'text-(--text-dim) font-bold opacity-40 scale-100 hover:opacity-100 hover:scale-105'}"
                 >
                    {line.text}
                 </p>
               {/each}
            </div>
          {:else}
            <div class="flex-1 flex flex-col items-center justify-center text-center py-20 opacity-20">
                <div class="w-12 h-12 bg-(--accent)/10 text-(--accent) rounded-full flex items-center justify-center mx-auto mb-4 border border-(--accent)/20">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"></path></svg>
                </div>
                <h3 class="text-[10px] font-black uppercase tracking-widest text-(--text-main)">Senkronize Veri Yok</h3>
                <p class="text-[9px] mt-2 uppercase leading-relaxed px-6 tracking-tighter">Bu parça için dijital veri akışı henüz tanımlanmadı.</p>
            </div>
          {/if}
        </div>
      {/if}
      
    </div>
  </div>
</aside>


<style>
  .custom-scrollbar::-webkit-scrollbar { width: 2px; }
  .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
  .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--accent); }
  
  .no-scrollbar::-webkit-scrollbar { display: none; }
  
  .mask-fade {
    mask-image: linear-gradient(to bottom, transparent 0%, black 15%, black 85%, transparent 100%);
    -webkit-mask-image: linear-gradient(to bottom, transparent 0%, black 15%, black 85%, transparent 100%);
  }

  p { 
    will-change: transform, opacity, filter; 
  }

  button { outline: none; }
</style>