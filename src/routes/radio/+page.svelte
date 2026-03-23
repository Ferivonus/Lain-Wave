<script lang="ts">
    import { onMount } from 'svelte';
    import { fade, fly, scale } from 'svelte/transition';
    import { playerState, sarkiCal, initializePlayer, sarkiSil } from '../../store.svelte';
    import type { Sarki } from '../../store.svelte';

    onMount(async () => {
        if (playerState.sarkiListesi.length === 0) {
            await initializePlayer();
        }
    });

    let seciliIstasyon = $state<{ isim: string, ikon: string, sarkilar: Sarki[], color: string } | null>(null);

    let mevcutTarzlar = $derived.by(() => {
        const tarzSeti = new Set<string>();
        playerState.sarkiListesi.forEach(s => {
            if (s.tarz) tarzSeti.add(s.tarz.trim());
        });
        return Array.from(tarzSeti);
    });

    const istasyonTanimlari = [
        { id: "pop", isim: "Pop", ikon: '<svg viewBox="0 0 24 24" fill="currentColor" class="w-12 h-12"><path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/></svg>', color: "var(--color-pop)", keywords: ["pop"] },
        { id: "rock", isim: "Rock", ikon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-12 h-12"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M17 8l-5-5-5 5M12 3v12"/></svg>', color: "var(--color-rock)", keywords: ["rock", "metal"] },
        { id: "lofi", isim: "Lo-Fi", ikon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-12 h-12"><path d="M18 8h1a4 4 0 0 1 0 8h-1M2 8h16v9a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V8zM6 1v3M10 1v3M14 1v3"/></svg>', color: "var(--color-lofi)", keywords: ["lofi", "lo-fi", "chill"] },
        { id: "cyber", isim: "Cyberpunk", ikon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-12 h-12"><rect x="3" y="11" width="18" height="10" rx="2"/><circle cx="12" cy="5" r="2"/><path d="M12 7v4M8 16h.01M16 16h.01"/></svg>', color: "var(--color-cyber)", keywords: ["cyberpunk", "electronic", "synthwave", "synth"] },
        { id: "ghibli", isim: "Ghibli", ikon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-12 h-12"><path d="M12 21V3M12 3c-4 0-7 3-7 7s7 11 7 11M12 3c4 0 7 3 7 7s-7 11-7 11"/></svg>', color: "var(--color-ghibli)", keywords: ["ghibli", "anime", "soundtrack"] },
        { id: "acoustic", isim: "Acoustic", ikon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-12 h-12"><path d="M12 20a8 8 0 1 0 0-16 8 8 0 0 0 0 16zM12 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4zM12 14v6M8 8l8 8"/></svg>', color: "var(--color-acoustic)", keywords: ["acoustic", "akustik"] },
        { id: "jazz", isim: "Jazz", ikon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-12 h-12"><path d="M9 18V5l12-2v13M6 18a3 3 0 1 1-6 0 3 3 0 0 1 6 0zM18 16a3 3 0 1 1-6 0 3 3 0 0 1 6 0z"/></svg>', color: "var(--color-jazz)", keywords: ["jazz", "blues"] },
        { id: "hiphop", isim: "Hip-Hop", ikon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-12 h-12"><path d="M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3z"/><path d="M19 10v2a7 7 0 0 1-14 0v-2M12 19v4M8 23h8"/></svg>', color: "var(--color-hiphop)", keywords: ["hip-hop", "rap", "hiphop"] },
        { id: "classic", isim: "Classical", ikon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-12 h-12"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>', color: "var(--color-classic)", keywords: ["classical", "klasik"] },
        { id: "podcast", isim: "Podcast", ikon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-12 h-12"><rect x="4" y="2" width="16" height="20" rx="2" ry="2"/><circle cx="12" cy="14" r="4"/><line x1="12" y1="6" x2="12.01" y2="6"/></svg>', color: "var(--color-podcast)", keywords: ["podcast", "oturum"] }
    ];

    let aktifIstasyonlar = $derived(
        istasyonTanimlari.map(ist => {
            const eslesmeVarMi = mevcutTarzlar.some(t => 
                ist.keywords.some(k => t.toLowerCase().includes(k))
            );
            return { ...ist, aktifMi: eslesmeVarMi };
        })
    );

    let digerTarzlar = $derived(
        mevcutTarzlar.filter(t => 
            !istasyonTanimlari.some(ist => 
                ist.keywords.some(k => t.toLowerCase().includes(k))
            )
        )
    );

    function istasyonAc(isim: string, ikon: string, color: string, keywords: string[], isAlternatif = false) {
        const uygunSarkilar = playerState.sarkiListesi.filter(s => {
            if (!s.tarz) return false;
            if (isAlternatif) return s.tarz === keywords[0];
            return keywords.some(k => s.tarz!.toLowerCase().includes(k));
        });
        
        if (uygunSarkilar.length > 0) {
            seciliIstasyon = { isim, ikon, sarkilar: uygunSarkilar, color };
        }
    }

    async function sarkiSilVeGuncelle(sarki: Sarki) {
        const onay = confirm(`"${sarki.isim}" kütüphaneden kalıcı olarak silinecek. Onaylıyor musun?`);
        if (!onay) return;

        const basarili = await sarkiSil(sarki);
        if (basarili && seciliIstasyon) {
            seciliIstasyon.sarkilar = seciliIstasyon.sarkilar.filter(s => s.id !== sarki.id);
            if (seciliIstasyon.sarkilar.length === 0) {
                seciliIstasyon = null;
            }
        }
    }

    function formatSure(saniye: number | undefined) {
        if (!saniye) return "0:00";
        const d = Math.floor(saniye / 60);
        const s = Math.floor(saniye % 60);
        return `${d}:${s.toString().padStart(2, '0')}`;
    }
</script>

<div class="p-8 lg:p-12 w-full min-h-full pb-32 flex flex-col relative overflow-hidden bg-transparent text-(--text-main) transition-colors duration-500 overflow-y-auto custom-scrollbar">
    
    <div class="absolute top-[-5%] right-[-5%] w-[400px] h-[400px] bg-(--accent) opacity-5 blur-[120px] rounded-full -z-10 animate-pulse-slow"></div>
    <div class="absolute bottom-[-5%] left-[-5%] w-[300px] h-[300px] bg-(--accent-sec) opacity-5 blur-[100px] rounded-full -z-10"></div>

    {#if !seciliIstasyon}
        <div in:fade={{ duration: 300 }}>
            <header class="mb-14" in:fly={{ y: -20, duration: 600 }}>
                <div class="flex items-center gap-3 mb-4">
                    <span class="flex h-2.5 w-2.5 relative">
                        <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-(--accent) opacity-75"></span>
                        <span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-(--accent)"></span>
                    </span>
                    <span class="text-[10px] font-black tracking-[0.4em] text-(--accent) uppercase">Live Stream Connected</span>
                </div>
                <h1 class="text-5xl lg:text-7xl font-black tracking-tighter uppercase leading-none italic drop-shadow-md">
                    Lain Radyo
                </h1>
                <p class="text-(--text-dim) mt-4 max-w-lg font-medium text-sm leading-relaxed">
                    Sistem kütüphaneni analiz etti. Frekansa tıklayarak yayın akışını inceleyebilir ve müziğe yön verebilirsin.
                </p>
            </header>

            <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-5 lg:gap-6">
                {#each aktifIstasyonlar as ist, i}
                    <button 
                        type="button"
                        aria-label="{ist.isim} frekansını aç"
                        class="relative aspect-square rounded-(--radius) overflow-hidden transition-all duration-500 border border-(--border) p-0 text-left group shadow-lg
                        {ist.aktifMi ? 'hover:-translate-y-2 hover:border-(--accent)/50 hover:shadow-2xl cursor-pointer' : 'opacity-20 grayscale cursor-not-allowed'}"
                        onclick={() => ist.aktifMi && istasyonAc(ist.isim, ist.ikon, ist.color, ist.keywords)}
                        disabled={!ist.aktifMi}
                        in:scale={{ duration: 400, start: 0.95, delay: i * 30 }}
                    >
                        <div class="absolute inset-0 bg-(--bg-card) group-hover:bg-(--bg-card-hover) transition-colors"></div>
                        <div class="absolute -bottom-10 -right-10 w-32 h-32 rounded-full blur-3xl opacity-0 group-hover:opacity-20 transition-opacity" style="background: {ist.color}"></div>

                        <div class="absolute inset-0 p-6 flex flex-col justify-between z-10">
                            <div class="transition-all duration-500 group-hover:scale-110 group-hover:-rotate-6 group-hover:-translate-x-2 drop-shadow-lg" style="color: {ist.color};">
                                {@html ist.ikon}
                            </div>
                            
                            <div class="max-w-[70%]">
                                <h3 class="text-lg font-black uppercase tracking-tight mb-1 group-hover:text-(--accent) transition-colors">{ist.isim}</h3>
                                <div class="flex items-center gap-2">
                                    <div class="w-1.5 h-1.5 rounded-full {ist.aktifMi ? 'bg-(--accent) animate-[pulse_1.5s_infinite]' : 'bg-zinc-600'}"></div>
                                    <p class="text-(--text-dim) text-[9px] font-black uppercase tracking-widest">
                                        {ist.aktifMi ? 'SİNYAL OK' : 'YAYIN YOK'}
                                    </p>
                                </div>
                            </div>
                        </div>
                    </button>
                {/each}
            </div>

            {#if digerTarzlar.length > 0}
                <div class="mt-16 pt-8 border-t border-(--border)" in:fade={{ delay: 300 }}>
                    <div class="flex items-center gap-4 mb-6">
                        <h2 class="text-[10px] font-black text-(--text-dim) uppercase tracking-[0.4em]">Alternatif Frekanslar</h2>
                        <div class="h-px flex-1 bg-(--border) opacity-50"></div>
                    </div>
                    <div class="flex flex-wrap gap-3">
                        {#each digerTarzlar as tarz}
                            <button 
                                type="button"
                                aria-label="{tarz} frekansını aç"
                                onclick={() => istasyonAc(tarz, '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-12 h-12"><path d="M12 20a8 8 0 1 0 0-16 8 8 0 0 0 0 16zM12 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4zM12 14v6M8 8l8 8"/></svg>', "var(--accent)", [tarz], true)}
                                class="px-5 py-2.5 rounded-xl bg-(--bg-surface) border border-(--border) text-(--text-dim) hover:text-white hover:border-(--accent) hover:bg-(--accent) transition-all text-[10px] font-black uppercase tracking-widest shadow-sm hover:shadow-[0_0_15px_var(--accent-glow)] active:scale-95"
                            >
                                {tarz}
                            </button>
                        {/each}
                    </div>
                </div>
            {/if}
        </div>
    
    {:else}
        <div in:fly={{ x: 20, duration: 400 }} class="flex flex-col h-full relative z-10">
            
            <button 
                type="button"
                aria-label="Frekanslara Dön"
                onclick={() => seciliIstasyon = null} 
                class="flex items-center gap-2 text-(--text-dim) hover:text-(--accent) transition-colors mb-8 group w-fit"
            >
                <svg class="w-5 h-5 group-hover:-translate-x-1 transition-transform" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path></svg>
                <span class="text-[10px] font-black uppercase tracking-[0.2em]">Frekanslara Dön</span>
            </button>

            <header class="flex items-end gap-6 mb-10 border-b border-(--border) pb-8 relative">
                <div class="drop-shadow-[0_0_20px_var(--accent-glow)]" style="color: {seciliIstasyon.color};">
                    {@html seciliIstasyon.ikon.replace('w-12 h-12', 'w-24 h-24 lg:w-32 lg:h-32')}
                </div>
                <div>
                    <h2 class="text-[10px] font-black tracking-[0.4em] uppercase opacity-70 mb-2" style="color: {seciliIstasyon.color}">Yayın Akışı Aktif</h2>
                    <h1 class="text-4xl lg:text-6xl font-black tracking-tighter uppercase italic">{seciliIstasyon.isim}</h1>
                    <p class="text-(--text-dim) font-bold text-xs uppercase tracking-widest mt-2">{seciliIstasyon.sarkilar.length} Veri Dosyası</p>
                </div>
            </header>

            <div class="flex flex-col gap-2">
                {#each seciliIstasyon.sarkilar as sarki, index}
                    <div class="flex items-center gap-4 p-3 pr-6 rounded-xl border border-transparent hover:border-(--border) bg-transparent hover:bg-(--bg-surface) transition-all group">
                        
                        <div class="w-10 text-center font-mono text-[10px] font-bold text-(--text-dim) group-hover:hidden">
                            {(index + 1).toString().padStart(2, '0')}
                        </div>
                        
                        <button 
                            type="button"
                            aria-label="Şarkıyı Oynat"
                            onclick={() => sarkiCal(sarki)}
                            class="w-10 h-10 hidden group-hover:flex items-center justify-center text-(--accent) focus:outline-none"
                        >
                            <svg class="w-6 h-6 fill-current" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
                        </button>

                        <button 
                            type="button" 
                            class="flex-1 min-w-0 cursor-pointer text-left focus:outline-none" 
                            onclick={() => sarkiCal(sarki)}
                        >
                            <h4 class="text-sm font-black truncate {playerState.aktifSarki?.id === sarki.id ? 'text-(--accent)' : 'text-(--text-main)'}">{sarki.isim}</h4>
                            <p class="text-[10px] font-bold text-(--text-dim) uppercase tracking-widest truncate mt-0.5">{sarki.sarkici}</p>
                        </button>

                        <div class="text-[10px] font-mono text-(--text-dim) font-bold">
                            {formatSure(sarki.sure)}
                        </div>

                        <button 
                            type="button"
                            aria-label="Kütüphaneden Sil"
                            onclick={(e) => { e.stopPropagation(); sarkiSilVeGuncelle(sarki); }}
                            class="ml-4 opacity-0 group-hover:opacity-100 text-red-500 hover:text-red-400 transition-opacity p-2 rounded-lg hover:bg-red-500/10 active:scale-95 focus:outline-none"
                            title="Kütüphaneden Sil"
                        >
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg>
                        </button>
                    </div>
                {/each}
            </div>
        </div>
    {/if}

</div>

<style>
    .animate-pulse-slow {
        animation: pulse-slow 8s cubic-bezier(0.4, 0, 0.6, 1) infinite;
    }

    @keyframes pulse-slow {
        0%, 100% { opacity: 0.05; transform: scale(1); }
        50% { opacity: 0.1; transform: scale(1.05); }
    }

    .custom-scrollbar::-webkit-scrollbar { width: 4px; }
    .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--accent); }
</style>