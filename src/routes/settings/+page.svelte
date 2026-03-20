<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { fade, fly } from 'svelte/transition';
    import { playerState } from '../../store.svelte';

    let veriYolu = $state("Yükleniyor...");
    let surum = "2.1"; 

    let kullaniciAdi = $state("");
    let discordAktif = $state(true);
    let medyaTuslariAktif = $state(true);
    let kayitMesaji = $state("");

    const temalar = [
        { id: 'theme-modern', ad: 'Modern Dark', renkler: ['#6366f1', '#18181b'], desc: 'Profesyonel ve dengeli', reklam: 'VARSAYILAN' },
        { id: 'theme-orkun-favori', ad: 'Orkun Favori', renkler: ['#ff2d81', '#050505'], desc: 'Neon ve yüksek enerji', reklam: 'ORKUN FAVORİ' },
        { id: 'theme-lofi', ad: 'Lo-Fi Night', renkler: ['#ff9a9e', '#1e1b29'], desc: 'Sakin çalışma modu' },
        { id: 'theme-ghibli', ad: 'Studio Ghibli', renkler: ['#a3c9a8', '#232b2b'], desc: 'The Wind Rises estetiği', reklam: 'DEV CHOICE' },
        { id: 'theme-retro', ad: 'Retro 80s', renkler: ['#f97316', '#2b1055'], desc: 'Nostaljik arcade' },
        { id: 'theme-ocean', ad: 'Deep Ocean', renkler: ['#00d2ff', '#010b13'], desc: 'Derin ve huzurlu', reklam: 'EN POPÜLER' },
        { id: 'theme-sakura', ad: 'Sakura Zen', renkler: ['#f472b6', '#120f10'], desc: 'Zarif dokunuşlar' },
        { id: 'theme-oled', ad: 'OLED Eclipse', renkler: ['#ffffff', '#000000'], desc: 'Maksimum kontrast' }
    ];

    onMount(async () => {
        try {
            veriYolu = await invoke('get_app_data_dir');
            
            const ayarlar: any = await invoke('ayarlari_getir');
            kullaniciAdi = ayarlar.kullanici_adi || "";
            discordAktif = ayarlar.discord_aktif ?? true;
            medyaTuslariAktif = ayarlar.medya_tuslari_aktif ?? true;
            
            if (ayarlar.tema) {
                playerState.currentTheme = ayarlar.tema;
            }
        } catch (e) {
            veriYolu = "Dizin bulunamadı.";
        }
    });

    async function ayarlariKaydet() {
        kayitMesaji = "Sisteme işleniyor...";
        try {
            await invoke('ayarlari_kaydet', {
                ayarlar: {
                    kullanici_adi: kullaniciAdi.trim(),
                    discord_aktif: discordAktif,
                    medya_tuslari_aktif: medyaTuslariAktif,
                    tema: playerState.currentTheme
                }
            });
            kayitMesaji = "Ayarlar başarıyla kaydedildi.";
            setTimeout(() => kayitMesaji = "", 3000);
        } catch (e) {
            kayitMesaji = "Kayıt başarısız!";
            setTimeout(() => kayitMesaji = "", 3000);
        }
    }

    async function klasoruAc() {
        await invoke('open_data_folder');
    }

    async function temaSec(temaId: string) {
        playerState.currentTheme = temaId;
        await ayarlariKaydet();
    }

    async function toggleDiscord() {
        discordAktif = !discordAktif;
        await ayarlariKaydet();
        
        if (!discordAktif) {
            try {
                await invoke('clear_discord_status');
            } catch (e) {}
        }
    }

    async function toggleMedyaTuslari() {
        medyaTuslariAktif = !medyaTuslariAktif;
        await ayarlariKaydet();
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
    
    <button 
        type="button"
        onclick={() => history.back()} 
        class="flex items-center gap-2 text-[var(--text-dim)] hover:text-[var(--accent)] transition-colors mb-8 group w-fit shrink-0"
        in:fly={{ x: -20, duration: 500 }}
    >
        <svg class="w-5 h-5 group-hover:-translate-x-1 transition-transform" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path></svg>
        <span class="text-[10px] font-black uppercase tracking-[0.2em]">Ana Sisteme Dön</span>
    </button>

    <header class="mb-12 shrink-0" in:fly={{ y: -20, duration: 600, delay: 100 }}>
        <div class="flex items-center gap-3 mb-4">
            <span class="flex h-2.5 w-2.5 relative">
                <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-[var(--accent)] opacity-75"></span>
                <span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-[var(--accent)]"></span>
            </span>
            <span class="text-[10px] font-black uppercase tracking-[0.4em] text-[var(--accent)]">System Configuration</span>
        </div>
        <h1 class="text-5xl lg:text-7xl font-black tracking-tighter uppercase italic leading-none drop-shadow-lg truncate">Ayarlar</h1>
        <p class="text-[var(--text-dim)] mt-4 font-bold text-sm uppercase tracking-widest opacity-60 truncate">Lain Wave Terminal v{surum}</p>
    </header>

    <div class="grid gap-10">
        
        <section class="bg-[var(--bg-card)] border border-[var(--border)] rounded-[var(--radius)] p-8 lg:p-10 shadow-2xl relative overflow-hidden shrink-0" in:fade={{ delay: 150 }}>
            <div class="flex items-center gap-5 mb-8">
                <div class="p-4 bg-[var(--accent)]/10 rounded-2xl text-[var(--accent)] shadow-inner">
                    <svg class="w-7 h-7" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path><circle cx="12" cy="7" r="4"></circle></svg>
                </div>
                <div>
                    <h2 class="text-2xl font-black uppercase italic tracking-tight">Kullanıcı Profili</h2>
                    <p class="text-[var(--text-dim)] text-[10px] uppercase tracking-widest font-bold mt-1 opacity-80 truncate">Sistem hitabı ve kişiselleştirme</p>
                </div>
            </div>

            <div class="flex flex-col md:flex-row gap-4 items-end">
                <div class="w-full space-y-2 flex-1">
                    <label for="kullanici-adi" class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-widest ml-1">Sistem Hitap Adı</label>
                    <input 
                        id="kullanici-adi" 
                        type="text" 
                        bind:value={kullaniciAdi}
                        onkeydown={(e) => e.key === 'Enter' && ayarlariKaydet()}
                        placeholder="Örn: Lain, Orkun..." 
                        class="w-full bg-[var(--bg-surface)] border border-[var(--border)] rounded-xl p-4 text-sm outline-none focus:border-[var(--accent)]/50 transition-all font-bold placeholder:text-[var(--text-dim)]/30" 
                    />
                </div>
                <button 
                    type="button" 
                    onclick={ayarlariKaydet} 
                    class="w-full md:w-auto bg-[var(--accent)] text-white font-black py-4 px-10 rounded-xl hover:shadow-[0_0_15px_var(--accent-glow)] transition-all active:scale-95 uppercase tracking-[0.2em] text-[10px] shrink-0"
                >
                    Profili Kaydet
                </button>
            </div>
            {#if kayitMesaji}
                <p class="text-[10px] font-bold text-[var(--accent)] mt-4 uppercase tracking-widest" in:fade>{kayitMesaji}</p>
            {/if}
        </section>

        <section class="space-y-6 shrink-0" in:fade={{ delay: 200 }}>
            <div class="flex items-end justify-between border-b border-[var(--border)] pb-4">
                <div class="min-w-0 pr-4">
                    <h2 class="text-2xl font-black uppercase italic tracking-tight">Görünüm</h2>
                    <p class="text-[var(--text-dim)] text-xs font-bold uppercase tracking-widest mt-1 truncate">Sistem arayüzü ve renk paletleri</p>
                </div>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
                {#each temalar as tema}
                    <button 
                        type="button"
                        onclick={() => temaSec(tema.id)}
                        class="flex flex-col gap-4 group text-left relative transition-all duration-300 hover:-translate-y-1 active:scale-95 w-full min-w-0"
                    >
                        <div 
                            class="aspect-[16/10] w-full rounded-[var(--radius)] border-2 transition-all duration-500 relative overflow-hidden p-5 flex flex-col justify-between shrink-0
                            {playerState.currentTheme === tema.id 
                                ? 'border-[var(--accent)] shadow-[0_15px_40px_rgba(0,0,0,0.4),0_0_20px_var(--accent-glow)]' 
                                : 'border-[var(--border)] opacity-60 hover:opacity-100 bg-[var(--bg-card)] hover:shadow-xl'}"
                        >
                            {#if tema.reklam}
                                <div class="absolute top-2 left-3 z-10 opacity-80 mix-blend-overlay">
                                    <span class="text-[9px] font-black uppercase tracking-[0.2em]" style="color: {tema.renkler[0]}">
                                        {tema.reklam}
                                    </span>
                                </div>
                            {/if}

                            <div class="relative z-20 space-y-2 mt-4">
                                <div class="flex gap-1.5">
                                    <div class="w-2.5 h-2.5 rounded-full shadow-lg shrink-0" style="background: {tema.renkler[0]}"></div>
                                    <div class="w-8 h-2.5 rounded-full opacity-30 shrink-0" style="background: {tema.renkler[0]}"></div>
                                </div>
                            </div>

                            <div class="absolute -bottom-4 -right-4 text-6xl font-black opacity-5 italic select-none group-hover:opacity-10 transition-opacity tracking-tighter" style="color: {tema.renkler[0]}">
                                {tema.ad.split(' ')[0]}
                            </div>
                            
                            {#if playerState.currentTheme === tema.id}
                                <div class="absolute inset-0 bg-[var(--accent)]/10 flex items-center justify-center backdrop-blur-[2px] z-30" in:fade>
                                    <div class="w-10 h-10 bg-white text-black rounded-full flex items-center justify-center shadow-2xl scale-100 shrink-0">
                                        <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24"><path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/></svg>
                                    </div>
                                </div>
                            {/if}
                        </div>

                        <div class="px-1 min-w-0">
                            <span class="text-sm font-black uppercase tracking-widest block group-hover:text-[var(--accent)] transition-colors truncate">{tema.ad}</span>
                            <span class="text-[10px] text-[var(--text-dim)] font-bold uppercase tracking-tighter opacity-70 truncate block">{tema.desc}</span>
                        </div>
                    </button>
                {/each}
            </div>
        </section>

        <section class="bg-[var(--bg-card)] border border-[var(--border)] rounded-[var(--radius)] p-8 lg:p-10 shadow-2xl relative overflow-hidden shrink-0" in:fade={{ delay: 300 }}>
            <div class="flex items-center gap-5 mb-10">
                <div class="p-4 bg-[var(--accent)]/10 rounded-2xl text-[var(--accent)] shadow-inner">
                    <svg class="w-7 h-7" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"></path></svg>
                </div>
                <div class="min-w-0 pr-4">
                    <h2 class="text-2xl font-black uppercase italic tracking-tight">Sistem Davranışı</h2>
                    <p class="text-[var(--text-dim)] text-[10px] uppercase tracking-widest font-bold mt-1 opacity-80 truncate">Uygulama protokolleri ve entegrasyonlar</p>
                </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6 lg:gap-8">
                <button type="button" onclick={toggleDiscord} class="flex items-center justify-between p-6 bg-[var(--bg-surface)] rounded-2xl border border-[var(--border)] hover:border-[var(--accent)]/50 transition-all group w-full min-w-0">
                    <div class="text-left min-w-0 pr-4">
                        <p class="text-xs font-black uppercase tracking-widest truncate">Discord Rich Presence</p>
                        <p class="text-[10px] text-[var(--text-dim)] mt-1 font-bold uppercase truncate">Durumunuzu ağda paylaşın</p>
                    </div>
                    <div class="w-12 h-6 rounded-full relative transition-colors shrink-0 {discordAktif ? 'bg-[var(--accent)]' : 'bg-gray-700'}">
                        <div class="absolute top-1 w-4 h-4 bg-white rounded-full transition-all {discordAktif ? 'left-7' : 'left-1'}"></div>
                    </div>
                </button>

                <button type="button" onclick={toggleMedyaTuslari} class="flex items-center justify-between p-6 bg-[var(--bg-surface)] rounded-2xl border border-[var(--border)] hover:border-[var(--accent)]/50 transition-all group w-full min-w-0">
                    <div class="text-left min-w-0 pr-4">
                        <p class="text-xs font-black uppercase tracking-widest truncate">Global Medya Tuşları</p>
                        <p class="text-[10px] text-[var(--text-dim)] mt-1 font-bold uppercase truncate">Arka planda kontrol izni</p>
                    </div>
                    <div class="w-12 h-6 rounded-full relative transition-colors shrink-0 {medyaTuslariAktif ? 'bg-[var(--accent)]' : 'bg-gray-700'}">
                        <div class="absolute top-1 w-4 h-4 bg-white rounded-full transition-all {medyaTuslariAktif ? 'left-7' : 'left-1'}"></div>
                    </div>
                </button>
            </div>
        </section>
        
        <section class="bg-[var(--bg-card)] border border-[var(--border)] rounded-[var(--radius)] p-8 lg:p-10 shadow-2xl relative overflow-hidden shrink-0" in:fade={{ delay: 400 }}>
            <div class="absolute top-0 right-0 w-96 h-96 bg-[var(--accent)]/5 blur-[100px] -z-10 rounded-full pointer-events-none"></div>

            <div class="flex items-center gap-5 mb-10">
                <div class="p-4 bg-[var(--accent-sec)]/10 rounded-2xl text-[var(--accent-sec)] shadow-inner">
                    <svg class="w-7 h-7" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><path d="M20 7H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2z"></path><path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16"></path></svg>
                </div>
                <div class="min-w-0 pr-4">
                    <h2 class="text-2xl font-black uppercase italic tracking-tight">Veri Yönetimi</h2>
                    <p class="text-[var(--text-dim)] text-[10px] uppercase tracking-widest font-bold mt-1 opacity-80 truncate">Sistem dosyaları ve dizin yapılandırması</p>
                </div>
            </div>

            <div class="grid gap-10">
                <div class="space-y-4 w-full min-w-0">
                    <p class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-[0.3em]">Application Data Path</p>
                    <div class="flex flex-col md:flex-row items-center gap-4 bg-[var(--bg-surface)] p-2.5 rounded-2xl border border-[var(--border)] group hover:border-[var(--accent)]/30 transition-colors w-full min-w-0">
                        <code class="text-[var(--accent)] text-xs truncate flex-1 font-mono px-4 py-3 select-all bg-black/20 rounded-xl block w-full">{veriYolu}</code>
                        <button 
                            type="button"
                            onclick={klasoruAc}
                            class="w-full md:w-auto bg-[var(--text-main)] text-[var(--bg-main)] hover:bg-[var(--accent)] hover:text-white text-[10px] font-black py-4 px-10 rounded-xl transition-all active:scale-95 uppercase tracking-[0.2em] shadow-lg shrink-0"
                        >
                            Klasörü Göster
                        </button>
                    </div>
                </div>

                <div class="pt-8 border-t border-[var(--border)] flex flex-col md:flex-row items-center justify-between gap-6 w-full min-w-0">
                    <div class="text-center md:text-left min-w-0 pr-4">
                        <p class="text-sm font-black uppercase tracking-tight text-red-500/80 truncate">Tehlikeli Bölge</p>
                        <p class="text-[10px] text-[var(--text-dim)] font-bold uppercase tracking-widest mt-1 truncate">Sıfırlama işlemi kütüphaneyi tamamen temizler</p>
                    </div>
                    <button 
                        type="button"
                        onclick={verileriSifirla}
                        class="w-full md:w-auto bg-red-500/10 hover:bg-red-500 text-red-500 hover:text-white text-[10px] font-black py-3.5 px-8 rounded-xl transition-all uppercase tracking-widest border border-red-500/20 active:scale-95 shrink-0"
                    >
                        Sistemi Sıfırla
                    </button>
                </div>
            </div>
        </section>

        <footer class="flex flex-col items-center justify-center py-20 group shrink-0" in:fade={{ delay: 500 }}>
            <div class="w-16 h-16 bg-[var(--accent)] rounded-[var(--radius)] flex items-center justify-center mb-6 rotate-3 group-hover:rotate-0 transition-transform duration-700 shadow-[0_10px_30px_var(--accent-glow)]">
                <svg class="w-8 h-8 text-white" fill="currentColor" viewBox="0 0 24 24"><path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/></svg>
            </div>
            <h3 class="text-3xl font-black tracking-[0.4em] uppercase italic drop-shadow-md text-center w-full truncate">Lain Wave</h3>
            
            <div class="mt-8 flex flex-col items-center gap-2 w-full min-w-0">
                <p class="text-[11px] font-black text-[var(--accent)] uppercase tracking-[0.5em] animate-[pulse_3s_infinite] text-center w-full truncate px-4">
                    Fahrettin Baştürk tarafından yapıldı
                </p>
                <div class="flex items-center justify-center gap-3 mt-1 w-full">
                    <span class="h-px w-8 bg-[var(--text-dim)] opacity-20 shrink-0"></span>
                    <p class="text-[9px] font-mono text-[var(--text-dim)] font-bold uppercase tracking-widest opacity-60 shrink-0">Version {surum} Build</p>
                    <span class="h-px w-8 bg-[var(--text-dim)] opacity-20 shrink-0"></span>
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