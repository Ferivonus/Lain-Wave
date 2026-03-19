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
        { id: "pop", isim: "Pop", ikon: "✨", color: "var(--accent)", keywords: ["pop"] },
        { id: "rock", isim: "Rock", ikon: "🎸", color: "#ef4444", keywords: ["rock", "metal"] },
        { id: "lofi", isim: "Lo-Fi", ikon: "☕", color: "#8b5cf6", keywords: ["lofi", "lo-fi", "chill"] },
        { id: "cyber", isim: "Cyberpunk", ikon: "🤖", color: "#06b6d4", keywords: ["cyberpunk", "electronic", "synthwave", "synth"] },
        { id: "ghibli", isim: "Ghibli", ikon: "🌳", color: "#83c5be", keywords: ["ghibli", "anime", "soundtrack"] },
        { id: "acoustic", isim: "Acoustic", ikon: "🎸", color: "#f59e0b", keywords: ["acoustic", "akustik"] },
        { id: "jazz", isim: "Jazz", ikon: "🎷", color: "#10b981", keywords: ["jazz", "blues"] },
        { id: "hiphop", isim: "Hip-Hop", ikon: "🎤", color: "#6366f1", keywords: ["hip-hop", "rap", "hiphop"] },
        { id: "classic", isim: "Classical", ikon: "🎻", color: "#71717a", keywords: ["classical", "klasik"] },
        { id: "podcast", isim: "Podcast", ikon: "🎙️", color: "#ec4899", keywords: ["podcast", "oturum"] }
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

<div class="p-8 lg:p-12 w-full min-h-full pb-32 flex flex-col relative overflow-hidden bg-transparent text-[var(--text-main)] transition-colors duration-500 overflow-y-auto custom-scrollbar">
    
    <div class="absolute top-[-5%] right-[-5%] w-[400px] h-[400px] bg-[var(--accent)] opacity-5 blur-[120px] rounded-full -z-10 animate-pulse-slow"></div>
    <div class="absolute bottom-[-5%] left-[-5%] w-[300px] h-[300px] bg-[var(--accent-sec)] opacity-5 blur-[100px] rounded-full -z-10"></div>

    {#if !seciliIstasyon}
        <div in:fade={{ duration: 300 }}>
            <header class="mb-14" in:fly={{ y: -20, duration: 600 }}>
                <div class="flex items-center gap-3 mb-4">
                    <span class="flex h-2.5 w-2.5 relative">
                        <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-[var(--accent)] opacity-75"></span>
                        <span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-[var(--accent)]"></span>
                    </span>
                    <span class="text-[10px] font-black tracking-[0.4em] text-[var(--accent)] uppercase">Live Stream Connected</span>
                </div>
                <h1 class="text-5xl lg:text-7xl font-black tracking-tighter uppercase leading-none italic drop-shadow-md">
                    Lain Radyo
                </h1>
                <p class="text-[var(--text-dim)] mt-4 max-w-lg font-medium text-sm leading-relaxed">
                    Sistem kütüphaneni analiz etti. Frekansa tıklayarak yayın akışını inceleyebilir ve müziğe yön verebilirsin.
                </p>
            </header>

            <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-5 lg:gap-6">
                {#each aktifIstasyonlar as ist, i}
                    <button 
                        type="button"
                        aria-label="{ist.isim} frekansını aç"
                        class="relative aspect-square rounded-[var(--radius)] overflow-hidden transition-all duration-500 border border-[var(--border)] p-0 text-left group shadow-lg
                        {ist.aktifMi ? 'hover:-translate-y-2 hover:border-[var(--accent)]/50 hover:shadow-2xl cursor-pointer' : 'opacity-20 grayscale cursor-not-allowed'}"
                        onclick={() => ist.aktifMi && istasyonAc(ist.isim, ist.ikon, ist.color, ist.keywords)}
                        disabled={!ist.aktifMi}
                        in:scale={{ duration: 400, start: 0.95, delay: i * 30 }}
                    >
                        <div class="absolute inset-0 bg-[var(--bg-card)] group-hover:bg-[var(--bg-card-hover)] transition-colors"></div>
                        <div class="absolute -bottom-10 -right-10 w-32 h-32 rounded-full blur-3xl opacity-0 group-hover:opacity-20 transition-opacity" style="background: {ist.color}"></div>

                        <div class="absolute inset-0 p-6 flex flex-col justify-between z-10">
                            <div class="text-5xl transition-all duration-500 group-hover:scale-110 group-hover:-rotate-6 group-hover:-translate-x-2 drop-shadow-lg">
                                {ist.ikon}
                            </div>
                            
                            <div class="max-w-[70%]">
                                <h3 class="text-lg font-black uppercase tracking-tight mb-1 group-hover:text-[var(--accent)] transition-colors">{ist.isim}</h3>
                                <div class="flex items-center gap-2">
                                    <div class="w-1.5 h-1.5 rounded-full {ist.aktifMi ? 'bg-[var(--accent)] animate-[pulse_1.5s_infinite]' : 'bg-zinc-600'}"></div>
                                    <p class="text-[var(--text-dim)] text-[9px] font-black uppercase tracking-widest">
                                        {ist.aktifMi ? 'SİNYAL OK' : 'YAYIN YOK'}
                                    </p>
                                </div>
                            </div>
                        </div>
                    </button>
                {/each}
            </div>

            {#if digerTarzlar.length > 0}
                <div class="mt-16 pt-8 border-t border-[var(--border)]" in:fade={{ delay: 300 }}>
                    <div class="flex items-center gap-4 mb-6">
                        <h2 class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-[0.4em]">Alternatif Frekanslar</h2>
                        <div class="h-px flex-1 bg-[var(--border)] opacity-50"></div>
                    </div>
                    <div class="flex flex-wrap gap-3">
                        {#each digerTarzlar as tarz}
                            <button 
                                type="button"
                                aria-label="{tarz} frekansını aç"
                                onclick={() => istasyonAc(tarz, "📻", "var(--accent)", [tarz], true)}
                                class="px-5 py-2.5 rounded-xl bg-[var(--bg-surface)] border border-[var(--border)] text-[var(--text-dim)] hover:text-white hover:border-[var(--accent)] hover:bg-[var(--accent)] transition-all text-[10px] font-black uppercase tracking-widest shadow-sm hover:shadow-[0_0_15px_var(--accent-glow)] active:scale-95"
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
                class="flex items-center gap-2 text-[var(--text-dim)] hover:text-[var(--accent)] transition-colors mb-8 group w-fit"
            >
                <svg class="w-5 h-5 group-hover:-translate-x-1 transition-transform" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path></svg>
                <span class="text-[10px] font-black uppercase tracking-[0.2em]">Frekanslara Dön</span>
            </button>

            <header class="flex items-end gap-6 mb-10 border-b border-[var(--border)] pb-8 relative">
                <div class="text-7xl lg:text-8xl drop-shadow-[0_0_20px_var(--accent-glow)]" style="color: {seciliIstasyon.color}">
                    {seciliIstasyon.ikon}
                </div>
                <div>
                    <h2 class="text-[10px] font-black tracking-[0.4em] uppercase opacity-70 mb-2" style="color: {seciliIstasyon.color}">Yayın Akışı Aktif</h2>
                    <h1 class="text-4xl lg:text-6xl font-black tracking-tighter uppercase italic">{seciliIstasyon.isim}</h1>
                    <p class="text-[var(--text-dim)] font-bold text-xs uppercase tracking-widest mt-2">{seciliIstasyon.sarkilar.length} Veri Dosyası</p>
                </div>
            </header>

            <div class="flex flex-col gap-2">
                {#each seciliIstasyon.sarkilar as sarki, index}
                    <div class="flex items-center gap-4 p-3 pr-6 rounded-xl border border-transparent hover:border-[var(--border)] bg-transparent hover:bg-[var(--bg-surface)] transition-all group">
                        
                        <div class="w-10 text-center font-mono text-[10px] font-bold text-[var(--text-dim)] group-hover:hidden">
                            {(index + 1).toString().padStart(2, '0')}
                        </div>
                        
                        <button 
                            type="button"
                            aria-label="Şarkıyı Oynat"
                            onclick={() => sarkiCal(sarki)}
                            class="w-10 h-10 hidden group-hover:flex items-center justify-center text-[var(--accent)] focus:outline-none"
                        >
                            <svg class="w-6 h-6 fill-current" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
                        </button>

                        <button 
                            type="button" 
                            class="flex-1 min-w-0 cursor-pointer text-left focus:outline-none" 
                            onclick={() => sarkiCal(sarki)}
                        >
                            <h4 class="text-sm font-black truncate {playerState.aktifSarki?.id === sarki.id ? 'text-[var(--accent)]' : 'text-[var(--text-main)]'}">{sarki.isim}</h4>
                            <p class="text-[10px] font-bold text-[var(--text-dim)] uppercase tracking-widest truncate mt-0.5">{sarki.sarkici}</p>
                        </button>

                        <div class="text-[10px] font-mono text-[var(--text-dim)] font-bold">
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