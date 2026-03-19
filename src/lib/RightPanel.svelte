<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { playerState, sarkiCal } from '../store.svelte';
  import { fade, fly, scale } from 'svelte/transition';

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
</script>

<aside 
  class="{sagMenuAcik ? 'flex absolute right-0 top-0 bottom-0 h-full shadow-[-20px_0_50px_rgba(0,0,0,0.5)] z-50' : 'hidden xl:flex'} 
  w-80 shrink-0 bg-[var(--bg-surface)]/95 backdrop-blur-xl flex-col z-10 border-l border-[var(--border)] relative overflow-hidden transition-all duration-500"
>
  
  {#if playerState.suAnOynuyorMu}
    <div 
      class="absolute inset-0 opacity-10 blur-[120px] -z-10 transition-opacity duration-1000 mix-blend-screen"
      style="background: radial-gradient(circle at top right, var(--accent), transparent 70%);"
    ></div>
  {/if}

  <div class="p-6 flex flex-col h-full z-10 relative">
    
    <div class="flex items-center justify-between mb-8 border-b border-[var(--border)] pb-2 relative shrink-0">
      <div class="flex gap-4 relative">
        {#each ['detay', 'siradaki', 'sozler'] as sekme}
          <button 
            type="button"
            onclick={() => aktifSekme = sekme as any}
            class="text-[9px] font-black tracking-[0.2em] uppercase transition-all pb-2 -mb-[10px] relative
            {aktifSekme === sekme ? 'text-[var(--accent)]' : 'text-[var(--text-dim)] hover:text-[var(--text-main)]'}"
          >
            {sekme === 'detay' ? 'DETAY' : sekme === 'siradaki' ? 'KUYRUK' : 'SÖZLER'}
            {#if aktifSekme === sekme}
              <div class="absolute bottom-0 left-0 w-full h-[2px] bg-[var(--accent)] rounded-t-md shadow-[0_0_10px_var(--accent-glow)]" in:fade></div>
            {/if}
          </button>
        {/each}
      </div>

      {#if sagMenuAcik}
        <button 
          type="button"
          onclick={onClose} 
          class="xl:hidden p-1.5 rounded-lg text-[var(--text-dim)] hover:bg-[var(--bg-card)] hover:text-[var(--accent)] transition-all active:scale-95 -mr-2"
          aria-label="Kapat"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
        </button>
      {/if}
    </div>
    
    <div class="flex-1 overflow-y-auto custom-scrollbar pr-1 relative">
      
      {#if aktifSekme === 'detay'}
        <div in:fly={{ x: -10, duration: 300 }} out:fade={{ duration: 150 }} class="absolute w-full pb-6 pr-1">
          {#if sarki}
            <div class="w-full aspect-square bg-[var(--bg-card)] rounded-2xl mb-6 shadow-2xl overflow-hidden border border-[var(--border)] relative group">
                {#if sarki.kapak_yolu}
                  <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover transition-transform duration-700 group-hover:scale-105" />
                {:else}
                  <div class="w-full h-full flex items-center justify-center bg-gradient-to-br from-[var(--bg-surface)] to-[var(--bg-main)]">
                    <svg class="w-20 h-20 text-[var(--text-dim)] opacity-10" fill="currentColor" viewBox="0 0 24 24"><path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/></svg>
                  </div>
                {/if}
                <button 
                  onclick={duzenleModaliAc}
                  class="absolute top-4 right-4 p-2.5 bg-black/50 backdrop-blur-md text-white rounded-xl opacity-0 group-hover:opacity-100 transition-all hover:scale-110 active:scale-90 border border-white/10"
                  title="Metadatayı Düzenle"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><path d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg>
                </button>
            </div>
            
            <div class="mb-8">
              <h2 class="text-xl font-black text-[var(--text-main)] tracking-tight leading-tight mb-1 truncate" title={sarki.isim}>{sarki.isim}</h2>
              <p class="text-[var(--accent)] font-bold text-[11px] uppercase tracking-[0.15em] opacity-90">{sarki.sarkici}</p>
              
              <div class="flex flex-wrap gap-2 mt-4">
                {#if sarki.tarz}
                  <span class="px-2 py-0.5 bg-[var(--bg-card)] border border-[var(--border)] rounded-md text-[8px] font-black text-[var(--text-dim)] uppercase tracking-widest">{sarki.tarz}</span>
                {/if}
                {#if sarki.yil}
                  <span class="px-2 py-0.5 bg-[var(--accent-sec)]/10 border border-[var(--accent-sec)]/20 rounded-md text-[8px] font-black text-[var(--accent-sec)] uppercase tracking-widest">{sarki.yil}</span>
                {/if}
              </div>
            </div>
            
            <div class="mt-8 space-y-4">
              <div class="text-[9px] font-black text-[var(--text-dim)] tracking-[0.3em] uppercase border-b border-[var(--border)] pb-2">SİSTEM KAYITLARI</div>
              
              <div class="font-mono text-[10px] leading-relaxed p-4 bg-black/20 rounded-xl border border-[var(--border)] text-[var(--text-dim)] overflow-hidden">
                <div class="flex gap-2 mb-1">
                    <span class="text-[var(--accent)] opacity-50">#</span>
                    <span class="text-white/40 uppercase tracking-tighter">Dosya Yolu:</span>
                </div>
                <div class="truncate opacity-60 mb-3">{sarki.yol}</div>

                <div class="flex gap-2 mb-1">
                    <span class="text-[var(--accent)] opacity-50">#</span>
                    <span class="text-white/40 uppercase tracking-tighter">İşlem Notu:</span>
                </div>
                {#if sarki.notlar}
                    <div class="text-[var(--text-main)]/80 italic line-clamp-4">{sarki.notlar}</div>
                {:else}
                    <div class="opacity-20 uppercase tracking-widest text-[8px]">Veri girişi yok...</div>
                {/if}
              </div>
            </div>

          {:else}
            <div class="flex flex-col items-center justify-center h-full opacity-20 py-32 text-center">
              <svg class="w-12 h-12 mb-4 animate-pulse" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24"><path d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
              <p class="text-[9px] uppercase font-black tracking-[0.4em]">Sinyal Bekleniyor</p>
            </div>
          {/if}
        </div>

      {:else if aktifSekme === 'siradaki'}
        <div in:fly={{ x: 10, duration: 300 }} out:fade={{ duration: 150 }} class="absolute w-full space-y-3 pb-6 pr-1">
          <div class="text-[9px] font-black text-[var(--text-dim)] tracking-[0.3em] uppercase mb-4 border-b border-[var(--border)] pb-2">KUYRUK ANALİZİ</div>
          {#if siradakiSarkilar.length > 0}
            <div class="flex flex-col gap-2">
              {#each siradakiSarkilar as item}
                <button 
                  type="button"
                  onclick={() => sarkiCal(item)}
                  class="w-full flex items-center gap-3 p-2 rounded-xl bg-[var(--bg-card)]/40 hover:bg-[var(--bg-card-hover)] border border-transparent hover:border-[var(--accent)]/20 transition-all duration-300 group text-left"
                >
                  <div class="w-10 h-10 rounded-lg overflow-hidden shrink-0 border border-white/5 relative">
                    {#if item.kapak_yolu}
                      <img src={convertFileSrc(item.kapak_yolu)} alt="" class="w-full h-full object-cover" />
                    {:else}
                      <div class="w-full h-full bg-[var(--bg-surface)] flex items-center justify-center text-[var(--text-dim)] opacity-20 text-xs">🎵</div>
                    {/if}
                  </div>
                  <div class="flex-1 min-w-0 pr-1">
                    <p class="text-[12px] font-bold text-[var(--text-main)] truncate group-hover:text-[var(--accent)] transition-colors leading-tight">{item.isim}</p>
                    <p class="text-[9px] font-bold text-[var(--text-dim)] truncate uppercase tracking-tighter opacity-60">{item.sarkici}</p>
                  </div>
                </button>
              {/each}
            </div>
          {:else}
            <div class="py-24 text-center opacity-20 flex flex-col items-center justify-center border border-dashed border-[var(--border)] rounded-2xl">
              <p class="text-[8px] font-black uppercase tracking-[0.2em]">Kuyruk Sonu</p>
            </div>
          {/if}
        </div>

      {:else if aktifSekme === 'sozler'}
        <div in:fly={{ y: 10, duration: 300 }} class="absolute w-full text-center py-20 pr-1">
            <div class="w-12 h-12 bg-[var(--accent)]/10 text-[var(--accent)] rounded-full flex items-center justify-center mx-auto mb-4">
                <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"></path></svg>
            </div>
            <h3 class="text-[10px] font-black uppercase tracking-widest text-[var(--text-main)]">Söz Modülü</h3>
            <p class="text-[9px] text-[var(--text-dim)] mt-2 uppercase leading-relaxed px-6">Bu parça için senkronize veri akışı henüz tanımlanmadı.</p>
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
</style>