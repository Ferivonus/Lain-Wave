<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { fade, fly } from 'svelte/transition';

    let veriYolu = $state("Yükleniyor...");
    let surum = "0.1.0-alpha";
    let silmeOnayi = $state(false);

    onMount(async () => {
        veriYolu = await invoke('get_app_data_dir');
    });

    async function klasoruAc() {
        await invoke('open_data_folder');
    }

    async function verileriSifirla() {
        if (confirm("DİKKAT: Tüm kütüphane, playlistler ve favoriler silinecek! Bu işlem geri alınamaz. Emin misiniz?")) {
            // Burada basitlik adına json dosyalarını boşaltan bir Rust komutu çağrılabilir
            // Şimdilik sadece uyarı veriyoruz
            alert("Sıfırlama işlemi için lütfen veri klasöründeki .json dosyalarını manuel silin ve uygulamayı yeniden başlatın.");
        }
    }
</script>

<div class="p-10 w-full min-h-full pb-32 flex flex-col relative max-w-5xl mx-auto">
    
    <header class="mb-12" in:fly={{ y: -20, duration: 500 }}>
        <h1 class="text-5xl font-black text-white italic tracking-tighter uppercase drop-shadow-lg">
            Sistem Ayarları
        </h1>
        <div class="h-1 w-20 bg-pink-500 mt-2 rounded-full shadow-[0_0_15px_rgba(236,72,153,0.5)]"></div>
    </header>

    <div class="grid gap-8">
        
        <section class="bg-black/20 border border-white/5 rounded-3xl p-8 backdrop-blur-sm" in:fade={{ delay: 200 }}>
            <div class="flex items-center gap-4 mb-6">
                <div class="p-3 bg-blue-500/10 rounded-xl text-blue-400">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M3 7v10a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-6l-2-2H5a2 2 0 0 0-2 2z"></path></svg>
                </div>
                <h2 class="text-xl font-bold text-white uppercase tracking-wider">Kütüphane Yönetimi</h2>
            </div>

            <div class="space-y-6">
                <div>
                    <p class="text-white/40 text-xs font-bold uppercase mb-2 tracking-widest">Veri Saklama Konumu</p>
                    <div class="flex items-center gap-4 bg-black/40 p-4 rounded-xl border border-white/5 group">
                        <code class="text-pink-300 text-sm truncate flex-1 font-mono">{veriYolu}</code>
                        <button 
                            onclick={klasoruAc}
                            class="bg-white/5 hover:bg-white/10 text-white text-xs font-bold py-2 px-4 rounded-lg transition-all border border-white/10"
                        >
                            KLASÖRÜ AÇ
                        </button>
                    </div>
                </div>

                <div class="pt-4 border-t border-white/5">
                    <button 
                        onclick={verileriSifirla}
                        class="text-red-400 hover:text-red-300 text-sm font-bold flex items-center gap-2 transition-colors uppercase tracking-tighter"
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M19 7l-.867 12.142A2 2 0 0 1 16.138 21H7.862a2 2 0 0 1-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v3M4 7h16"></path></svg>
                        Tüm Verileri Temizle
                    </button>
                </div>
            </div>
        </section>

        <section class="bg-black/20 border border-white/5 rounded-3xl p-8 backdrop-blur-sm" in:fade={{ delay: 300 }}>
            <div class="flex items-center gap-4 mb-6">
                <div class="p-3 bg-purple-500/10 rounded-xl text-purple-400">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><circle cx="12" cy="12" r="3"></circle><path d="M12 2v2m0 16v2m10-10h-2M4 10H2m16.24-7.76l-1.42 1.42M6.42 16.58l-1.42 1.42m12.24 0l1.42-1.42M6.42 5.42L5 4"></path></svg>
                </div>
                <h2 class="text-xl font-bold text-white uppercase tracking-wider">Görünüm</h2>
            </div>

            <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
                <button class="aspect-video rounded-xl bg-gradient-to-br from-pink-600 to-purple-900 border-2 border-pink-400 relative overflow-hidden group">
                    <span class="absolute inset-0 flex items-center justify-center font-black text-xs text-white opacity-0 group-hover:opacity-100 transition-opacity">LAIN DEFAULT</span>
                </button>
                <button class="aspect-video rounded-xl bg-zinc-900 border border-white/10 grayscale hover:grayscale-0 transition-all opacity-40 cursor-not-available">
                    <span class="absolute inset-0 flex items-center justify-center font-bold text-[10px] text-white">COMING SOON</span>
                </button>
            </div>
        </section>

        <section class="flex flex-col items-center justify-center py-10 opacity-30 hover:opacity-100 transition-opacity duration-700" in:fade={{ delay: 400 }}>
            <div class="w-16 h-16 bg-pink-500 rounded-2xl flex items-center justify-center mb-4 rotate-12">
                <svg class="w-10 h-10 text-white" fill="currentColor" viewBox="0 0 24 24"><path d="M12 3v10.55c-.59-.34-1.27-.55-2-.55-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4V7h4V3h-6z"/></svg>
            </div>
            <p class="text-xl font-black italic tracking-widest text-white">LAIN WAVE AUDIO</p>
            <p class="text-xs font-mono text-pink-400">VERSION {surum}</p>
            <div class="mt-6 flex gap-6 text-[10px] font-bold uppercase tracking-[0.2em] text-white/40">
                <a href="https://tauri.app" target="_blank" class="hover:text-white transition-colors">Powered by Tauri</a>
                <span>•</span>
                <a href="https://svelte.dev" target="_blank" class="hover:text-white transition-colors">Built with Svelte</a>
            </div>
        </section>

    </div>
</div>

<style>
    section {
        box-shadow: 0 10px 30px -10px rgba(0,0,0,0.5);
    }
</style>