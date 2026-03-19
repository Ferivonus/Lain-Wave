<script lang="ts">
    import { onMount } from 'svelte';
    import { fade, fly, scale } from 'svelte/transition';
    import { playerState, sarkiCal, initializePlayer } from '../../store.svelte';

    // YENİ: Sayfa doğrudan açıldığında kütüphaneyi garantiye alalım
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

    function istasyonuBaslat(tarz: string) {
        const uygunSarkilar = playerState.sarkiListesi.filter(s => 
            s.tarz?.toLowerCase().includes(tarz.toLowerCase())
        );
        if (uygunSarkilar.length > 0) {
            const rastgeleSarki = uygunSarkilar[Math.floor(Math.random() * uygunSarkilar.length)];
            sarkiCal(rastgeleSarki);
        }
    }

    const istasyonlar = [
        { id: "synth", isim: "Synthwave", renk: "from-pink-500 to-purple-600", ikon: "⚡" },
        { id: "lofi", isim: "Lo-Fi", renk: "from-orange-400 to-yellow-600", ikon: "☕" },
        { id: "cyber", isim: "Cyberpunk", renk: "from-cyan-500 to-blue-700", ikon: "🤖" },
        { id: "chill", isim: "Relax", renk: "from-emerald-400 to-teal-600", ikon: "🌊" },
    ];
</script>

<div class="p-10 w-full min-h-full pb-32 flex flex-col relative overflow-hidden">
    
    <div class="absolute top-[-10%] right-[-10%] w-[500px] h-[500px] bg-pink-500/10 blur-[120px] rounded-full -z-10 animate-pulse"></div>
    <div class="absolute bottom-[-10%] left-[-10%] w-[400px] h-[400px] bg-blue-500/10 blur-[100px] rounded-full -z-10"></div>

    <header class="mb-12" in:fly={{ y: -20, duration: 600 }}>
        <div class="flex items-center gap-4 mb-2">
            <span class="flex h-3 w-3 relative">
                <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-red-400 opacity-75"></span>
                <span class="relative inline-flex rounded-full h-3 w-3 bg-red-500"></span>
            </span>
            <span class="text-xs font-black tracking-[0.4em] text-red-500 uppercase">Live Frequency</span>
        </div>
        <h1 class="text-6xl font-black text-white italic tracking-tighter uppercase leading-none">
            LAIN RADYO
        </h1>
        <p class="text-white/40 mt-4 max-w-md font-medium">Kütüphanendeki veriler analiz ediliyor... Ruh haline en uygun frekansı seç ve akışa bırak.</p>
    </header>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        {#each istasyonlar as istasyon}
            {@const aktifMi = mevcutTarzlar.some(t => t.toLowerCase().includes(istasyon.isim.toLowerCase()))}
            
            <button 
                type="button"
                class="relative aspect-square rounded-3xl overflow-hidden cursor-pointer group shadow-2xl transition-all duration-500 border-none p-0 text-left
                {aktifMi ? 'hover:-translate-y-2' : 'opacity-40 grayscale cursor-not-allowed'}"
                onclick={() => aktifMi && istasyonuBaslat(istasyon.isim)}
                disabled={!aktifMi}
                in:scale={{ duration: 400, start: 0.9, delay: 200 }}
                aria-label="{istasyon.isim} istasyonunu başlat"
            >
                <div class="absolute inset-0 bg-gradient-to-br {istasyon.renk} opacity-80 group-hover:opacity-100 transition-opacity"></div>
                <div class="absolute inset-0 bg-white/5 backdrop-blur-[2px] group-hover:backdrop-blur-none transition-all"></div>

                <div class="absolute inset-0 p-8 flex flex-col justify-between z-10">
                    <span class="text-5xl drop-shadow-2xl group-hover:scale-125 transition-transform duration-500">{istasyon.ikon}</span>
                    <div>
                        <h3 class="text-2xl font-black text-white uppercase italic tracking-tighter">{istasyon.isim}</h3>
                        <p class="text-white/70 text-[10px] font-bold tracking-widest uppercase">
                            {aktifMi ? 'İstasyon Aktif' : 'Şarkı Bulunamadı'}
                        </p>
                    </div>
                </div>

                <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-20 h-20 bg-white/20 rounded-full flex items-center justify-center opacity-0 group-hover:opacity-100 transition-all duration-300 backdrop-blur-md border border-white/20">
                    <svg class="w-10 h-10 text-white fill-current" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
                </div>
            </button>
        {/each}
    </div>

    {#if mevcutTarzlar.length > 0}
        <div class="mt-16" in:fade={{ delay: 500 }}>
            <h2 class="text-sm font-black text-white/30 uppercase tracking-[0.3em] mb-6">Tespit Edilen Frekanslar</h2>
            <div class="flex flex-wrap gap-3">
                {#each mevcutTarzlar as tarz}
                    <button 
                        type="button"
                        onclick={() => istasyonuBaslat(tarz)}
                        class="px-6 py-3 rounded-full bg-white/5 border border-white/10 text-white/60 hover:text-pink-400 hover:border-pink-500/50 hover:bg-pink-500/5 transition-all text-xs font-bold uppercase tracking-widest cursor-pointer"
                    >
                        {tarz}
                    </button>
                {/each}
            </div>
        </div>
    {/if}
</div>

<style>
    .animate-pulse {
        animation: pulse 4s cubic-bezier(0.4, 0, 0.6, 1) infinite;
    }

    @keyframes pulse {
        0%, 100% { opacity: 0.1; transform: scale(1); }
        50% { opacity: 0.2; transform: scale(1.1); }
    }
</style>