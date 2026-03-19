<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { playerState, sarkiCal } from '../store.svelte';
  import { fade, fly } from 'svelte/transition';

  let { sagMenuAcik, onClose } = $props<{ 
    sagMenuAcik: boolean, 
    onClose: () => void 
  }>();

  let aktifSekme = $state<'detay' | 'siradaki'>('detay');
  let sarki = $derived(playerState.aktifSarki);
  
  let siradakiSarkilar = $derived.by(() => {
    if (!sarki) return [];
    const index = playerState.sarkiListesi.findIndex(s => s.id === sarki.id);
    if (index === -1) return [];
    return playerState.sarkiListesi.slice(index + 1, index + 11);
  });
</script>

<aside 
  class="{sagMenuAcik ? 'flex absolute right-0 top-0 bottom-0 h-full shadow-[-20px_0_50px_rgba(0,0,0,0.5)] z-50' : 'hidden xl:flex'} 
  w-80 shrink-0 bg-[var(--bg-surface)]/95 backdrop-blur-xl flex-col z-10 border-l border-[var(--border)] relative overflow-hidden transition-all duration-500"
>
  
  {#if playerState.suAnOynuyorMu}
    <div 
      class="absolute inset-0 opacity-15 blur-[100px] -z-10 transition-opacity duration-1000 mix-blend-screen"
      style="background: radial-gradient(circle at top right, var(--accent), transparent 70%);"
    ></div>
  {/if}

  <div class="p-6 flex flex-col h-full z-10 relative">
    
    <div class="flex items-center justify-between mb-8 border-b border-[var(--border)] pb-2 relative shrink-0">
      <div class="flex gap-5 relative">
        <button 
          type="button"
          onclick={() => aktifSekme = 'detay'}
          class="text-[10px] font-black tracking-[0.2em] uppercase transition-all pb-2 -mb-[10px] relative
          {aktifSekme === 'detay' ? 'text-[var(--accent)]' : 'text-[var(--text-dim)] hover:text-[var(--text-main)]'}"
        >
          DETAYLAR
          {#if aktifSekme === 'detay'}
            <div class="absolute bottom-0 left-0 w-full h-[2px] bg-[var(--accent)] rounded-t-md shadow-[0_0_10px_var(--accent-glow)]" in:fade={{ duration: 200 }}></div>
          {/if}
        </button>
        <button 
          type="button"
          onclick={() => aktifSekme = 'siradaki'}
          class="text-[10px] font-black tracking-[0.2em] uppercase transition-all pb-2 -mb-[10px] relative
          {aktifSekme === 'siradaki' ? 'text-[var(--accent)]' : 'text-[var(--text-dim)] hover:text-[var(--text-main)]'}"
        >
          SIRADAKİ
          {#if aktifSekme === 'siradaki'}
            <div class="absolute bottom-0 left-0 w-full h-[2px] bg-[var(--accent)] rounded-t-md shadow-[0_0_10px_var(--accent-glow)]" in:fade={{ duration: 200 }}></div>
          {/if}
        </button>
      </div>

      {#if sagMenuAcik}
        <button 
          type="button"
          onclick={onClose} 
          class="xl:hidden p-1.5 rounded-lg text-[var(--text-dim)] hover:bg-[var(--bg-card)] hover:text-[var(--accent)] transition-all active:scale-95 -mr-2"
          aria-label="Paneli Kapat"
          title="Kapat"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      {/if}
    </div>
    
    <div class="flex-1 overflow-y-auto custom-scrollbar pr-1 relative">
      
      {#if aktifSekme === 'detay'}
        <div in:fly={{ x: -10, duration: 300 }} out:fade={{ duration: 150 }} class="absolute w-full pb-6 pr-1">
          {#if sarki}
            <div class="w-full aspect-square bg-[var(--bg-card)] rounded-2xl mb-6 shadow-2xl overflow-hidden border border-[var(--border)] relative group">
                {#if sarki.kapak_yolu}
                  <img src={convertFileSrc(sarki.kapak_yolu)} alt="{sarki.isim} Kapak" class="w-full h-full object-cover transition-transform duration-700 group-hover:scale-105" />
                {:else}
                  <div class="w-full h-full flex items-center justify-center bg-gradient-to-br from-[var(--bg-surface)] to-[var(--bg-main)]">
                    <svg class="w-24 h-24 text-[var(--text-dim)] opacity-10" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                      <path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/>
                    </svg>
                  </div>
                {/if}
                <div class="absolute inset-0 bg-gradient-to-t from-[var(--bg-main)]/50 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500 pointer-events-none"></div>
            </div>
            
            <div class="mb-8 text-center xl:text-left">
              <h2 class="text-xl font-black text-[var(--text-main)] tracking-tight leading-tight mb-1 truncate px-2 xl:px-0" title={sarki.isim}>{sarki.isim}</h2>
              <a href="/artist/{encodeURIComponent(sarki.sarkici)}" class="text-[var(--accent)] font-bold text-[10px] uppercase tracking-widest hover:text-[var(--text-main)] transition-colors line-clamp-1 px-2 xl:px-0">{sarki.sarkici}</a>
              
              <div class="flex flex-wrap justify-center xl:justify-start gap-2 mt-4">
                {#if sarki.yil}
                  <span class="px-2.5 py-1 bg-[var(--bg-surface)] border border-[var(--border)] rounded-md text-[9px] font-black text-[var(--text-dim)] uppercase tracking-widest shadow-sm">{sarki.yil}</span>
                {/if}
                {#if sarki.kalite}
                  <span class="px-2.5 py-1 bg-[var(--accent)]/10 border border-[var(--accent)]/20 rounded-md text-[9px] font-black text-[var(--accent)] uppercase tracking-widest shadow-sm truncate">{sarki.kalite}</span>
                {/if}
              </div>
            </div>
            
            <div class="mt-8">
              <div class="text-[9px] font-black text-[var(--text-dim)] tracking-[0.3em] uppercase mb-4 border-b border-[var(--border)] pb-2 flex items-center justify-between">
                <span>SİSTEM KAYITLARI</span>
              </div>
              
              {#if sarki.notlar && sarki.notlar.trim() !== ""}
                <div class="text-xs leading-relaxed text-[var(--text-main)]/80 whitespace-pre-wrap font-medium p-4 bg-[var(--bg-card)] rounded-xl border border-[var(--border)] shadow-inner">
                  {sarki.notlar}
                </div>
              {:else}
                <div class="flex flex-col items-center justify-center py-10 opacity-30 text-[var(--text-dim)] bg-[var(--bg-card)]/50 rounded-xl border border-dashed border-[var(--border)]">
                  <svg class="w-6 h-6 mb-2" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" aria-hidden="true"><path d="M4 6h16M4 12h16M4 18h7"></path></svg>
                  <p class="text-[9px] uppercase font-black tracking-widest">Açıklama Mevcut Değil</p>
                </div>
              {/if}
            </div>

          {:else}
            <div class="flex flex-col items-center justify-center h-full opacity-20 py-32">
              <svg class="w-16 h-16 mb-4 animate-pulse" fill="none" stroke="currentColor" stroke-width="1" viewBox="0 0 24 24" aria-hidden="true"><path d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
              <p class="text-[9px] uppercase font-black tracking-[0.4em] text-center">Veri Akışı Bekleniyor</p>
            </div>
          {/if}
        </div>

      {:else if aktifSekme === 'siradaki'}
        <div in:fly={{ x: 10, duration: 300 }} out:fade={{ duration: 150 }} class="absolute w-full space-y-3 pb-6 pr-1">
          <div class="text-[9px] font-black text-[var(--text-dim)] tracking-[0.3em] uppercase mb-4 border-b border-[var(--border)] pb-2">
            SIRADAKİ PARÇALAR
          </div>
          {#if siradakiSarkilar.length > 0}
            <div class="flex flex-col gap-2">
              {#each siradakiSarkilar as item}
                <button 
                  type="button"
                  onclick={() => sarkiCal(item)}
                  class="w-full flex items-center gap-3 p-2.5 rounded-xl bg-[var(--bg-card)] hover:bg-[var(--bg-card-hover)] border border-[var(--border)] hover:border-[var(--accent)]/30 transition-all duration-300 group text-left shadow-sm active:scale-[0.98]"
                  aria-label="{item.isim} parçasını çal"
                >
                  <div class="w-11 h-11 rounded-lg bg-[var(--bg-surface)] overflow-hidden shrink-0 border border-white/5 relative">
                    {#if item.kapak_yolu}
                      <img src={convertFileSrc(item.kapak_yolu)} alt="" class="w-full h-full object-cover transition-transform group-hover:scale-110" />
                    {:else}
                      <div class="w-full h-full flex items-center justify-center text-[var(--text-dim)] opacity-20">🎵</div>
                    {/if}
                    <div class="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
                        <svg class="w-5 h-5 text-white" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
                    </div>
                  </div>
                  <div class="flex-1 min-w-0 pr-1">
                    <p class="text-[13px] font-bold text-[var(--text-main)] truncate group-hover:text-[var(--accent)] transition-colors leading-tight mb-0.5">{item.isim}</p>
                    <p class="text-[10px] font-bold text-[var(--text-dim)] truncate uppercase tracking-widest opacity-80">{item.sarkici}</p>
                  </div>
                </button>
              {/each}
            </div>
          {:else}
            <div class="py-24 text-center opacity-20 flex flex-col items-center justify-center">
              <svg class="w-12 h-12 mb-3" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="8" x2="12" y2="12"></line><line x1="12" y1="16" x2="12.01" y2="16"></line></svg>
              <p class="text-[9px] font-black uppercase tracking-[0.3em]">Kuyrukta Başka Parça Yok</p>
            </div>
          {/if}
        </div>
      {/if}
      
    </div>
  </div>
</aside>

<style>
  .custom-scrollbar::-webkit-scrollbar { width: 3px; }
  .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
  .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--accent); }
  
  button { outline: none; }
  button:focus-visible {
      outline: 2px solid var(--accent);
      border-radius: 4px;
      outline-offset: 2px;
  }
</style>