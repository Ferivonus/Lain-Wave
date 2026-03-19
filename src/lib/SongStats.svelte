<script lang="ts">
    import type { Sarki } from '../store.svelte';

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
</script>

<div class="flex items-center justify-end gap-3 lg:gap-5 text-[var(--text-dim)] font-medium shrink-0 text-[10px] lg:text-[11px] transition-colors duration-500">
    
    {#if sarki.tarz && sarki.tarz.trim() !== ""}
        <div 
            class="hidden xl:flex px-2 py-0.5 rounded-md bg-[var(--bg-card)] border border-[var(--border)] text-[var(--text-dim)] uppercase tracking-wider max-w-[90px] truncate cursor-default hover:text-[var(--text-main)] transition-colors" 
            title="Tür: {sarki.tarz}"
        >
            <span class="truncate">{sarki.tarz}</span>
        </div>
    {/if}

    {#if sarki.yil}
        <div 
            class="hidden xl:flex items-center gap-1.5 px-2 py-0.5 rounded-md font-bold tracking-tight text-[var(--accent-sec)] bg-[var(--accent-sec)]/10 border border-[var(--accent-sec)]/20 cursor-default" 
            title="Çıkış Yılı: {sarki.yil}"
        >
            <svg class="w-3 h-3 opacity-80" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true">
                <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
                <line x1="16" y1="2" x2="16" y2="6"></line>
                <line x1="8" y1="2" x2="8" y2="6"></line>
            </svg>
            <span>{sarki.yil}</span>
        </div>
    {/if}

    {#if sarki.kalite && sarki.kalite.trim() !== ""}
        {@const kaliteBuyuk = sarki.kalite.trim().toUpperCase()}
        {@const isHighRes = kaliteBuyuk === 'FLAC' || kaliteBuyuk === 'WAV'}
        <div 
            class="hidden {isHighRes ? 'sm:flex' : 'lg:flex'} items-center px-1.5 py-0.5 rounded-md font-black tracking-widest uppercase border transition-all duration-300 cursor-default
            {isHighRes 
                ? 'border-[var(--accent)]/40 text-[var(--accent)] bg-[var(--accent)]/10 shadow-sm' 
                : 'border-[var(--border)] text-[var(--text-dim)] bg-[var(--bg-card)]'}" 
            title="Ses Kalitesi: {kaliteBuyuk}">
            {kaliteBuyuk}
        </div>
    {/if}

    <div 
        class="hidden md:flex items-center gap-1.5 min-w-[42px] justify-end group cursor-default hover:text-[var(--accent)] transition-colors" 
        title="{sarki.dinlenme_sayisi || 0} Kez Dinlendi"
    >
        <svg class="w-3.5 h-3.5 opacity-50 group-hover:opacity-100 transition-opacity" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path d="M8 5v14l11-7z"/>
        </svg>
        <span class="font-mono font-bold tracking-tighter">{formatDinlenme(sarki.dinlenme_sayisi)}</span>
    </div>

    <div 
        class="flex items-center gap-1.5 min-w-[42px] justify-end group cursor-default hover:text-[var(--text-main)] transition-colors" 
        title="Süre: {formatSure(sarki.sure)}"
    >
        <svg class="w-3.5 h-3.5 opacity-50 group-hover:opacity-100 transition-opacity" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true">
            <circle cx="12" cy="12" r="10"></circle>
            <polyline points="12 6 12 12 16 14"></polyline>
        </svg>
        <span class="font-mono font-bold tracking-tighter">{formatSure(sarki.sure)}</span>
    </div>

</div>