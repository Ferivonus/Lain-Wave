<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { fade, fly, scale } from 'svelte/transition';
    import { playerState } from '../../store.svelte';

    let veriYolu = $state("Yükleniyor...");
    let surum = "2.1"; 

    const temalar = [
        { id: 'theme-modern', ad: 'Modern Dark', renkler: ['#6366f1', '#18181b'], desc: 'Profesyonel ve dengeli' },
        { id: 'theme-cyberpunk', ad: 'Cyberpunk', renkler: ['#ff0055', '#00ff41'], desc: 'Neon ve yüksek enerji' },
        { id: 'theme-lofi', ad: 'Lo-Fi Night', renkler: ['#ff9a9e', '#1e1b29'], desc: 'Sakin çalışma modu' },
        { id: 'theme-ghibli', ad: 'Studio Ghibli', renkler: ['#8ba8a9', '#dce4e2'], desc: 'The Wind Rises estetiği', fav: true },
        { id: 'theme-retro', ad: 'Retro 80s', renkler: ['#f97316', '#2b1055'], desc: 'Nostaljik arcade' },
        { id: 'theme-ocean', ad: 'Deep Ocean', renkler: ['#00d2ff', '#010b13'], desc: 'Derin ve huzurlu', fav: true },
        { id: 'theme-sakura', ad: 'Sakura Zen', renkler: ['#f472b6', '#120f10'], desc: 'Zarif dokunuşlar' },
        { id: 'theme-oled', ad: 'OLED Eclipse', renkler: ['#ffffff', '#000000'], desc: 'Maksimum kontrast' }
    ];

    onMount(async () => {
        try {
            veriYolu = await invoke('get_app_data_dir');
        } catch (e) {
            veriYolu = "Dizin bulunamadı.";
        }
    });

    async function klasoruAc() {
        await invoke('open_data_folder');
    }

    function temaSec(temaId: string) {
        playerState.currentTheme = temaId;
        localStorage.setItem('lainwave_theme', temaId);
    }

    async function verileriSifirla() {
        const onay = confirm("DİKKAT: Tüm kütüphane ve ayarlar silinecek! Emin misiniz?");
        if (onay && confirm("Veritabanı kalıcı olarak boşaltılacak. Onaylıyor musunuz?")) {
            await klasoruAc();
            alert("Lütfen açılan klasördeki .json dosyalarını manuel olarak silip uygulamayı yeniden başlatın.");
        }
    }
</script>

<div class="p-8 lg:p-12 w-full min-h-full pb-32 flex flex-col relative max-w-6xl mx-auto bg-transparent text-[var(--text-main)] transition-colors duration-500 overflow-y-auto custom-scrollbar">
    
    <header class="mb-12" in:fly={{ y: -20, duration: 600 }}>
        <div class="flex items-center gap-3 mb-4">
            <span class="flex h-2.5 w-2.5 relative">
                <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-[var(--accent)] opacity-75"></span>
                <span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-[var(--accent)]"></span>
            </span>
            <span class="text-[10px] font-black uppercase tracking-[0.4em] text-[var(--accent)]">System Configuration</span>
        </div>
        <h1 class="text-5xl lg:text-7xl font-black tracking-tighter uppercase italic leading-none drop-shadow-lg">Ayarlar</h1>
        <p class="text-[var(--text-dim)] mt-4 font-bold text-sm uppercase tracking-widest opacity-60">Lain Wave Terminal v{surum}</p>
    </header>

    <div class="grid gap-12">
        
        <section class="space-y-8" in:fade={{ delay: 200 }}>
            <div class="flex items-end justify-between border-b border-[var(--border)] pb-4">
                <div>
                    <h2 class="text-2xl font-black uppercase italic tracking-tight">Görünüm</h2>
                    <p class="text-[var(--text-dim)] text-xs font-bold uppercase tracking-widest mt-1">Sistem arayüzü ve renk paletleri</p>
                </div>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
                {#each temalar as tema}
                    <button 
                        type="button"
                        onclick={() => temaSec(tema.id)}
                        class="flex flex-col gap-4 group text-left relative transition-all duration-300 hover:-translate-y-1 active:scale-95"
                    >
                        <div 
                            class="aspect-[16/10] rounded-[var(--radius)] border-2 transition-all duration-500 relative overflow-hidden p-5 flex flex-col justify-between
                            {playerState.currentTheme === tema.id 
                                ? 'border-[var(--accent)] shadow-[0_15px_40px_rgba(0,0,0,0.4),0_0_20px_var(--accent-glow)]' 
                                : 'border-[var(--border)] opacity-60 hover:opacity-100 bg-[var(--bg-card)] hover:shadow-xl'}"
                        >
                            {#if tema.fav}
                                <div class="absolute top-0 right-0 bg-[var(--accent)] text-white text-[8px] font-black px-3 py-1 rounded-bl-xl uppercase tracking-tighter z-20 shadow-lg">
                                    DEV CHOICE
                                </div>
                            {/if}

                            <div class="relative z-10 space-y-2">
                                <div class="flex gap-1.5">
                                    <div class="w-2.5 h-2.5 rounded-full" style="background: {tema.renkler[0]}"></div>
                                    <div class="w-8 h-2.5 rounded-full opacity-30" style="background: {tema.renkler[0]}"></div>
                                </div>
                            </div>

                            <div class="absolute -bottom-4 -right-4 text-5xl font-black opacity-5 italic select-none group-hover:opacity-10 transition-opacity">
                                {tema.ad.split(' ')[0]}
                            </div>
                            
                            {#if playerState.currentTheme === tema.id}
                                <div class="absolute inset-0 bg-[var(--accent)]/10 flex items-center justify-center backdrop-blur-[2px]" in:fade>
                                    <div class="w-10 h-10 bg-white text-black rounded-full flex items-center justify-center shadow-2xl scale-100" in:scale>
                                        <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24"><path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/></svg>
                                    </div>
                                </div>
                            {/if}
                        </div>

                        <div class="px-1">
                            <span class="text-sm font-black uppercase tracking-widest block group-hover:text-[var(--accent)] transition-colors">{tema.ad}</span>
                            <span class="text-[10px] text-[var(--text-dim)] font-bold uppercase tracking-tighter opacity-70">{tema.desc}</span>
                        </div>
                    </button>
                {/each}
            </div>
        </section>
        
        <section class="bg-[var(--bg-card)] border border-[var(--border)] rounded-[var(--radius)] p-10 shadow-2xl relative overflow-hidden" in:fade={{ delay: 300 }}>
            <div class="absolute top-0 right-0 w-96 h-96 bg-[var(--accent)]/5 blur-[100px] -z-10 rounded-full pointer-events-none"></div>

            <div class="flex items-center gap-5 mb-10">
                <div class="p-4 bg-[var(--accent-sec)]/10 rounded-2xl text-[var(--accent-sec)] shadow-inner">
                    <svg class="w-7 h-7" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><path d="M20 7H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2z"></path><path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16"></path></svg>
                </div>
                <div>
                    <h2 class="text-2xl font-black uppercase italic tracking-tight">Veri Yönetimi</h2>
                    <p class="text-[var(--text-dim)] text-[10px] uppercase tracking-widest font-bold mt-1 opacity-80">Sistem dosyaları ve yedekleme dizini</p>
                </div>
            </div>

            <div class="grid gap-10">
                <div class="space-y-4">
                    <p class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-[0.3em]">Application Data Path</p>
                    <div class="flex flex-col md:flex-row items-center gap-4 bg-[var(--bg-surface)] p-2.5 rounded-2xl border border-[var(--border)] group hover:border-[var(--accent)]/30 transition-colors">
                        <code class="text-[var(--accent)] text-xs truncate flex-1 font-mono px-4 py-3 select-all bg-black/20 rounded-xl">{veriYolu}</code>
                        <button 
                            type="button"
                            onclick={klasoruAc}
                            class="w-full md:w-auto bg-[var(--text-main)] text-[var(--bg-main)] hover:bg-[var(--accent)] hover:text-white text-[10px] font-black py-4 px-10 rounded-xl transition-all active:scale-95 uppercase tracking-[0.2em] shadow-lg"
                        >
                            Klasörü Göster
                        </button>
                    </div>
                </div>

                <div class="pt-8 border-t border-[var(--border)] flex flex-col md:flex-row items-center justify-between gap-6">
                    <div class="text-center md:text-left">
                        <p class="text-sm font-black uppercase tracking-tight text-red-500/80">Tehlikeli Bölge</p>
                        <p class="text-[10px] text-[var(--text-dim)] font-bold uppercase tracking-widest mt-1">Sıfırlama işlemi kütüphaneyi tamamen temizler</p>
                    </div>
                    <button 
                        type="button"
                        onclick={verileriSifirla}
                        class="bg-red-500/10 hover:bg-red-500 text-red-500 hover:text-white text-[10px] font-black py-3.5 px-8 rounded-xl transition-all uppercase tracking-widest border border-red-500/20 active:scale-95"
                    >
                        Sistemi Sıfırla
                    </button>
                </div>
            </div>
        </section>

        <footer class="flex flex-col items-center justify-center py-20 group" in:fade={{ delay: 400 }}>
            <div class="w-16 h-16 bg-[var(--accent)] rounded-[var(--radius)] flex items-center justify-center mb-6 rotate-3 group-hover:rotate-0 transition-transform duration-700 shadow-[0_10px_30px_var(--accent-glow)]">
                <svg class="w-8 h-8 text-white" fill="currentColor" viewBox="0 0 24 24"><path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/></svg>
            </div>
            <h3 class="text-3xl font-black tracking-[0.4em] uppercase italic drop-shadow-md">Lain Wave</h3>
            
            <div class="mt-8 flex flex-col items-center gap-2">
                <p class="text-[11px] font-black text-[var(--accent)] uppercase tracking-[0.5em] animate-[pulse_3s_infinite]">
                    Fahrettin Baştürk tarafından yapıldı
                </p>
                <div class="flex items-center gap-3 mt-1">
                    <span class="h-px w-8 bg-[var(--text-dim)] opacity-20"></span>
                    <p class="text-[9px] font-mono text-[var(--text-dim)] font-bold uppercase tracking-widest opacity-60">Version {surum} Build</p>
                    <span class="h-px w-8 bg-[var(--text-dim)] opacity-20"></span>
                </div>
            </div>
        </footer>

    </div>
</div>

<style>
    .custom-scrollbar::-webkit-scrollbar { width: 4px; }
    .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--accent); }

    button {
        cursor: pointer;
        outline: none;
    }
</style>