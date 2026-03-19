<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { playerState, sarkiCal } from '../store.svelte';
  import { fade, fly, slide } from 'svelte/transition';

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
  class="{sagMenuAcik ? 'flex absolute right-0 top-0 bottom-0 h-full shadow-2xl z-50' : 'hidden xl:flex'} 
  w-80 shrink-0 bg-[var(--bg-surface)] flex-col z-10 border-l border-[var(--border)] relative overflow-hidden transition-all duration-500"
>
  
  {#if playerState.suAnOynuyorMu}
    <div 
      class="absolute inset-0 opacity-10 blur-[120px] -z-10 transition-opacity duration-1000"
      style="background: radial-gradient(circle at top right, var(--accent), transparent);"
    ></div>
  {/if}

  <div class="p-6 flex flex-col h-full z-10 relative">
    
    <div class="flex items-center justify-between mb-8 border-b border-[var(--border)] pb-2 relative shrink-0">
      <div class="flex gap-5">
        <button 
          type="button"
          onclick={() => aktifSekme = 'detay'}
          class="text-[10px] font-black tracking-[0.2em] uppercase transition-all pb-2 -mb-[10px] 
          {aktifSekme === 'detay' ? 'text-[var(--accent)] border-b-2 border-[var(--accent)]' : 'text-[var(--text-dim)] hover:text-[var(--text-main)]'}"
        >
          DETAYLAR
        </button>
        <button 
          type="button"
          onclick={() => aktifSekme = 'siradaki'}
          class="text-[10px] font-black tracking-[0.2em] uppercase transition-all pb-2 -mb-[10px] 
          {aktifSekme === 'siradaki' ? 'text-[var(--accent)] border-b-2 border-[var(--accent)]' : 'text-[var(--text-dim)] hover:text-[var(--text-main)]'}"
        >
          SIRADAKİ
        </button>
      </div>

      {#if sagMenuAcik}
        <button 
          type="button"
          onclick={onClose} 
          class="xl:hidden p-1 text-[var(--text-dim)] hover:text-[var(--accent)] transition-colors"
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
    
    <div class="flex-1 overflow-y-auto custom-scrollbar pr-1">
      
      {#if aktifSekme === 'detay'}
        <div in:fade={{ duration: 300 }}>
          {#if sarki}
            <div class="w-full aspect-square bg-[var(--bg-card)] rounded-[var(--radius)] mb-6 shadow-2xl overflow-hidden border border-[var(--border)] relative group">
                {#if sarki.kapak_yolu}
                  <img src={convertFileSrc(sarki.kapak_yolu)} alt="{sarki.isim} Kapak" class="w-full h-full object-cover transition-transform duration-700 group-hover:scale-105" />
                {:else}
                  <div class="w-full h-full flex items-center justify-center bg-gradient-to-br from-[var(--bg-main)] to-[var(--bg-surface)]">
                    <svg class="w-20 h-20 text-[var(--text-dim)] opacity-20" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                      <path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/>
                    </svg>
                  </div>
                {/if}
                <div class="absolute inset-0 bg-[var(--accent)] mix-blend-overlay opacity-0 group-hover:opacity-10 transition-opacity pointer-events-none"></div>
            </div>
            
            <div class="mb-8 text-center xl:text-left">
              <h2 class="text-xl font-bold text-[var(--text-main)] tracking-tight leading-tight mb-1">{sarki.isim}</h2>
              <p class="text-[var(--accent)] font-bold text-xs uppercase tracking-widest">{sarki.sarkici}</p>
              
              <div class="flex flex-wrap justify-center xl:justify-start gap-2 mt-4">
                {#if sarki.yil}
                  <span class="px-2 py-1 bg-[var(--bg-card)] border border-[var(--border)] rounded-md text-[9px] font-black text-[var(--text-dim)] uppercase">{sarki.yil}</span>
                {/if}
                {#if sarki.kalite}
                  <span class="px-2 py-1 bg-[var(--accent)]/10 border border-[var(--accent)]/20 rounded-md text-[9px] font-black text-[var(--accent)] uppercase">{sarki.kalite}</span>
                {/if}
              </div>
            </div>
            
            <div class="mt-8">
              <div class="text-[9px] font-black text-[var(--text-dim)] tracking-[0.2em] uppercase mb-4 border-b border-[var(--border)] pb-2">
                SİSTEM KAYITLARI
              </div>
              
              {#if sarki.notlar && sarki.notlar.trim() !== ""}
                <div class="text-sm leading-relaxed text-[var(--text-main)]/70 whitespace-pre-wrap pb-10 font-medium italic">
                  "{sarki.notlar}"
                </div>
              {:else}
                <div class="flex flex-col items-center justify-center py-12 opacity-20 text-[var(--text-dim)]">
                  <svg class="w-8 h-8 mb-3" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" aria-hidden="true"><path d="M4 6h16M4 12h16M4 18h7"></path></svg>
                  <p class="text-[10px] uppercase font-bold tracking-widest">Açıklama Mevcut Değil</p>
                </div>
              {/if}
            </div>

          {:else}
            <div class="flex flex-col items-center justify-center h-full opacity-20 py-20">
              <svg class="w-16 h-16 mb-4" fill="none" stroke="currentColor" stroke-width="1" viewBox="0 0 24 24" aria-hidden="true"><path d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
              <p class="text-[10px] uppercase font-bold tracking-[0.3em] text-center">Veri Akışı Bekleniyor</p>
            </div>
          {/if}
        </div>

      {:else if aktifSekme === 'siradaki'}
        <div in:fade={{ duration: 300 }} class="space-y-3">
          <div class="text-[9px] font-black text-[var(--text-dim)] tracking-[0.2em] uppercase mb-4 border-b border-[var(--border)] pb-2">
            SIRADAKİ PARÇALAR
          </div>
          {#if siradakiSarkilar.length > 0}
            {#each siradakiSarkilar as item}
              <button 
                type="button"
                onclick={() => sarkiCal(item)}
                class="w-full flex items-center gap-3 p-2 rounded-xl hover:bg-[var(--bg-card-hover)] border border-transparent hover:border-[var(--border)] transition-all group text-left"
                aria-label="{item.isim} parçasını çal"
              >
                <div class="w-12 h-12 rounded-lg bg-[var(--bg-card)] overflow-hidden shrink-0 border border-[var(--border)]">
                  {#if item.kapak_yolu}
                    <img src={convertFileSrc(item.kapak_yolu)} alt="" class="w-full h-full object-cover" />
                  {:else}
                    <div class="w-full h-full flex items-center justify-center text-[var(--text-dim)] opacity-20">🎵</div>
                  {/if}
                </div>
                <div class="flex-1 min-w-0">
                  <p class="text-xs font-bold text-[var(--text-main)] truncate group-hover:text-[var(--accent)] transition-colors">{item.isim}</p>
                  <p class="text-[10px] text-[var(--text-dim)] truncate">{item.sarkici}</p>
                </div>
              </button>
            {/each}
          {:else}
            <div class="py-20 text-center opacity-20">
              <p class="text-[10px] font-bold uppercase tracking-widest">Kuyrukta başka parça yok</p>
            </div>
          {/if}
        </div>
      {/if}
    </div>
  </div>
</aside>

<style>
  .custom-scrollbar::-webkit-scrollbar { width: 4px; }
  .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
  .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--accent); }
  
  button { cursor: pointer; outline: none; }
</style>