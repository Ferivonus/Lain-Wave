<script lang="ts">
    import { playerState, type Sarki } from '../store.svelte';

    let { sarki } = $props<{ sarki: Sarki }>();

    function formatSure(saniye?: number) {
        if (saniye === undefined || saniye === null || isNaN(saniye)) return "--:--";
        const dk = Math.floor(saniye / 60);
        const sn = Math.floor(saniye % 60);
        return `${dk}:${sn < 10 ? '0' : ''}${sn}`;
    }

    function formatDinlenme(sayi?: number) {
        if (!sayi) return "0";
        if (sayi >= 1000000) return (sayi / 1000000).toFixed(1).replace(/\.0$/, '') + "M";
        if (sayi >= 1000) return (sayi / 1000).toFixed(1).replace(/\.0$/, '') + "K";
        return sayi.toString();
    }

    function duzenleModaliAc(e: MouseEvent) {
        e.preventDefault();
        e.stopPropagation();
        playerState.duzenlenecekSarki = sarki;
        playerState.isEditModalOpen = true;
    }
</script>

<div class="flex items-center justify-end gap-3 lg:gap-5 text-(--text-dim) font-medium shrink-0 text-[10px] lg:text-[11px] transition-all duration-500">
    
    <button 
        type="button" 
        onclick={duzenleModaliAc}
        class="opacity-0 group-hover:opacity-100 focus-visible:opacity-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-(--accent) p-2 rounded-lg hover:bg-(--accent)/10 hover:text-(--accent) transition-all shrink-0"
        aria-label="Verileri Düzenle"
        title="Verileri Düzenle"
    >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path>
        </svg>
    </button>

    {#if sarki.tarz && sarki.tarz.trim() !== ""}
        <div 
            class="hidden xl:flex items-center px-2.5 py-1 rounded-md bg-(--bg-card) border border-(--border) text-(--text-dim) uppercase tracking-widest font-black max-w-[90px] shrink-0 cursor-default hover:border-(--accent)/30 hover:text-(--text-main) transition-all shadow-sm" 
            title="Tür: {sarki.tarz}"
        >
            <span class="truncate">{sarki.tarz}</span>
        </div>
    {/if}

    {#if sarki.yil}
        <div 
            class="hidden 2xl:flex items-center gap-1.5 px-2.5 py-1 rounded-md font-bold tracking-tight text-(--accent-sec) bg-(--accent-sec)/5 border border-(--accent-sec)/10 cursor-default shrink-0 shadow-sm" 
            title="Çıkış Yılı: {sarki.yil}"
        >
            <svg class="w-3 h-3 opacity-60" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
                <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
                <line x1="16" y1="2" x2="16" y2="6"></line>
                <line x1="8" y1="2" x2="8" y2="6"></line>
            </svg>
            <span class="font-mono">{sarki.yil}</span>
        </div>
    {/if}

    {#if sarki.kalite && sarki.kalite.trim() !== ""}
        {@const kaliteBuyuk = sarki.kalite.trim().toUpperCase().split(' ')[0]} 
        <div 
            class="hidden lg:flex items-center px-2 py-0.5 rounded-md font-black tracking-[0.15em] uppercase border transition-all duration-300 cursor-default shrink-0
            {kaliteBuyuk === 'FLAC' || kaliteBuyuk === 'WAV'
                ? 'border-(--accent)/40 text-(--accent) bg-(--accent)/10 shadow-[0_0_10px_var(--accent-glow)]/10' 
                : 'border-(--border) text-(--text-dim) bg-(--bg-card)'}" 
            title="Ses Kalitesi: {sarki.kalite}">
            {kaliteBuyuk}
        </div>
    {/if}

    <div 
        class="flex items-center gap-2 min-w-[55px] justify-end group cursor-default hover:text-(--accent) transition-all shrink-0" 
        title="{sarki.dinlenme_sayisi || 0} Kez Dinlendi"
    >
        <svg class="w-4 h-4 opacity-50 group-hover:opacity-100 group-hover:scale-110 transition-all" fill="currentColor" viewBox="0 0 24 24">
            <path d="M8 5v14l11-7z"/>
        </svg>
        <span class="font-mono font-black tracking-tighter text-[11px] lg:text-[12px]">{formatDinlenme(sarki.dinlenme_sayisi)}</span>
    </div>

    <div 
        class="flex items-center gap-2 min-w-[50px] justify-end group cursor-default hover:text-(--text-main) transition-all shrink-0" 
        title="Süre: {formatSure(sarki.sure)}"
    >
        <svg class="w-4 h-4 opacity-40 group-hover:opacity-100 transition-opacity" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
            <circle cx="12" cy="12" r="10"></circle>
            <polyline points="12 6 12 12 16 14"></polyline>
        </svg>
        <span class="font-mono font-black tracking-tighter text-[11px] lg:text-[12px]">{formatSure(sarki.sure)}</span>
    </div>

</div>