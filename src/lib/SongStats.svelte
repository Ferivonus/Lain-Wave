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
        if (sayi >= 1000000) return (sayi / 1000000).toFixed(1) + "M";
        if (sayi >= 1000) return (sayi / 1000).toFixed(1) + "K";
        return sayi.toString();
    }
</script>

<div class="flex items-center justify-end gap-3 lg:gap-4 text-white/40 font-medium shrink-0 text-[11px] lg:text-xs">
    
    {#if sarki.tarz && sarki.tarz.trim() !== ""}
        <div class="hidden xl:flex px-2.5 py-0.5 rounded-md bg-white/5 border border-white/10 text-white/50 hover:text-white/80 hover:bg-white/10 transition-colors uppercase tracking-wider text-[10px] max-w-[100px] truncate cursor-default" title="Tür: {sarki.tarz}">
            <span class="truncate">{sarki.tarz}</span>
        </div>
    {/if}

    {#if sarki.yil}
        <div class="hidden xl:flex items-center gap-1 px-1.5 py-0.5 rounded text-[9px] font-bold tracking-widest text-purple-300/80 bg-purple-500/10 border border-purple-500/20 hover:text-purple-300 hover:bg-purple-500/20 transition-all cursor-default" title="Çıkış Yılı: {sarki.yil}">
            <svg class="w-3 h-3 opacity-70" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect><line x1="16" y1="2" x2="16" y2="6"></line><line x1="8" y1="2" x2="8" y2="6"></line><line x1="3" y1="10" x2="21" y2="10"></line></svg>
            {sarki.yil}
        </div>
    {/if}

    {#if sarki.kalite && sarki.kalite.trim() !== ""}
        {@const kaliteBuyuk = sarki.kalite.toUpperCase()}
        {@const isHighRes = kaliteBuyuk === 'FLAC' || kaliteBuyuk === 'WAV'}
        <div class="hidden {isHighRes ? 'sm:flex' : 'lg:flex'} items-center px-1.5 py-0.5 rounded text-[9px] font-black tracking-widest uppercase border transition-all duration-300 cursor-default
            {isHighRes 
                ? 'border-pink-500/50 text-pink-400 bg-pink-500/10 shadow-[0_0_8px_rgba(236,72,153,0.15)] hover:shadow-[0_0_15px_rgba(236,72,153,0.4)]' 
                : 'border-white/10 text-white/30 bg-white/5 hover:text-white/50 hover:border-white/20'}" 
            title="Ses Kalitesi: {kaliteBuyuk}">
            {kaliteBuyuk}
        </div>
    {/if}

    <div class="hidden md:flex items-center gap-1.5 hover:text-pink-400 transition-colors min-w-[45px] justify-end group cursor-default" title="{sarki.dinlenme_sayisi || 0} Kez Dinlendi">
        <svg class="w-3.5 h-3.5 opacity-60 group-hover:opacity-100 group-hover:scale-110 transition-all duration-300" fill="currentColor" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
        <span class="font-mono">{formatDinlenme(sarki.dinlenme_sayisi)}</span>
    </div>

    <div class="flex items-center gap-1.5 hover:text-white transition-colors min-w-[45px] justify-end group cursor-default" title="Süre: {formatSure(sarki.sure)}">
        <svg class="w-3.5 h-3.5 opacity-60 group-hover:opacity-100 group-hover:rotate-12 transition-all duration-300" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><circle cx="12" cy="12" r="10"></circle><polyline points="12 6 12 12 16 14"></polyline></svg>
        <span class="font-mono text-white/70 group-hover:text-white transition-colors">{formatSure(sarki.sure)}</span>
    </div>

</div>