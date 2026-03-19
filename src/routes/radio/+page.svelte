<script lang="ts">
    import { onMount } from 'svelte';
    import { fade, fly, scale } from 'svelte/transition';
    import { playerState, sarkiCal, initializePlayer } from '../../store.svelte';

    onMount(async () => {
        if (playerState.sarkiListesi.length === 0) {
            await initializePlayer();
        }
    });

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

    function istasyonuBaslat(istId: string) {
        const tanim = istasyonTanimlari.find(i => i.id === istId);
        if (!tanim) return;

        const uygunSarkilar = playerState.sarkiListesi.filter(s => 
            s.tarz && tanim.keywords.some(k => s.tarz!.toLowerCase().includes(k))
        );
        
        if (uygunSarkilar.length > 0) {
            const rastgeleSarki = uygunSarkilar[Math.floor(Math.random() * uygunSarkilar.length)];
            sarkiCal(rastgeleSarki);
        }
    }

    let digerTarzlar = $derived(
        mevcutTarzlar.filter(t => 
            !istasyonTanimlari.some(ist => 
                ist.keywords.some(k => t.toLowerCase().includes(k))
            )
        )
    );
</script>

<div class="p-8 lg:p-12 w-full min-h-full pb-32 flex flex-col relative overflow-hidden bg-transparent text-[var(--text-main)] transition-colors duration-500 overflow-y-auto custom-scrollbar">
    
    <div class="absolute top-[-5%] right-[-5%] w-[400px] h-[400px] bg-[var(--accent)] opacity-5 blur-[120px] rounded-full -z-10 animate-pulse-slow"></div>
    <div class="absolute bottom-[-5%] left-[-5%] w-[300px] h-[300px] bg-[var(--accent-sec)] opacity-5 blur-[100px] rounded-full -z-10"></div>

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
            Sistem kütüphaneni analiz etti. Tespit edilen frekanslar üzerinden otomatik yayın akışı başlatabilirsin.
        </p>
    </header>

    <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-5 lg:gap-6">
        {#each aktifIstasyonlar as ist, i}
            <button 
                type="button"
                aria-label="{ist.isim} istasyonunu başlat"
                class="relative aspect-square rounded-[var(--radius)] overflow-hidden transition-all duration-500 border border-[var(--border)] p-0 text-left group shadow-lg
                {ist.aktifMi ? 'hover:-translate-y-2 hover:border-[var(--accent)]/50 hover:shadow-2xl cursor-pointer' : 'opacity-20 grayscale cursor-not-allowed'}"
                onclick={() => ist.aktifMi && istasyonuBaslat(ist.id)}
                disabled={!ist.aktifMi}
                in:scale={{ duration: 400, start: 0.95, delay: i * 30 }}
            >
                <div class="absolute inset-0 bg-[var(--bg-card)] group-hover:bg-[var(--bg-card-hover)] transition-colors"></div>
                
                <div class="absolute -bottom-10 -right-10 w-32 h-32 rounded-full blur-3xl opacity-0 group-hover:opacity-20 transition-opacity" 
                     style="background: {ist.color}"></div>

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

                {#if ist.aktifMi}
                    <div class="absolute inset-0 bg-gradient-to-t from-black/40 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                    
                    <div class="absolute bottom-4 right-4 z-20 transform translate-y-4 opacity-0 group-hover:translate-y-0 group-hover:opacity-100 transition-all duration-300 ease-out">
                        <div class="w-12 h-12 bg-[var(--accent)] text-white rounded-full flex items-center justify-center shadow-[0_8px_20px_rgba(0,0,0,0.4)] hover:scale-110 active:scale-95 transition-transform">
                            <svg class="w-6 h-6 fill-current ml-0.5" viewBox="0 0 24 24">
                                <path d="M8 5v14l11-7z"/>
                            </svg>
                        </div>
                    </div>
                {/if}
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
                        aria-label="{tarz} frekansını başlat"
                        onclick={() => {
                            const uygunSarkilar = playerState.sarkiListesi.filter(s => s.tarz === tarz);
                            if (uygunSarkilar.length > 0) sarkiCal(uygunSarkilar[Math.floor(Math.random() * uygunSarkilar.length)]);
                        }}
                        class="px-5 py-2.5 rounded-xl bg-[var(--bg-surface)] border border-[var(--border)] text-[var(--text-dim)] hover:text-white hover:border-[var(--accent)] hover:bg-[var(--accent)] transition-all text-[10px] font-black uppercase tracking-widest shadow-sm hover:shadow-[0_0_15px_var(--accent-glow)] active:scale-95"
                    >
                        {tarz}
                    </button>
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