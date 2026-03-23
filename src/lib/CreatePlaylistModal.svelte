<script lang="ts">
    import { open } from '@tauri-apps/plugin-dialog';
    import { readTextFile } from '@tauri-apps/plugin-fs';
    import { 
        playerState, 
        playlistOlusturAPI, 
        youtubeIndirAPI, 
        sarkiPlaylisteEkle, 
        youtubeAramaAPI,
        type YouTubeSonuc 
    } from '../store.svelte';
    import { fade, scale, slide } from 'svelte/transition';
    import { cubicOut } from 'svelte/easing';

    let mod = $state<'yeni' | 'import'>('yeni');
    let yukleniyor = $state(false);
    let isim = $state("");

    type ImportDurum = 'bekliyor' | 'indiriliyor' | 'tamamlandi' | 'hata' | 'link_yok';
    
    interface ImportSarki {
        isim: string;
        sarkici: string;
        youtube_linki: string | null;
        tarz?: string;
        durum: ImportDurum;
    }

    let importListIsmi = $state("");
    let importSarkilar = $state<ImportSarki[]>([]);
    let isImporting = $state(false);
    let yaratilanListeId = $state<string | null>(null);

    let tamamlananSayi = $derived(importSarkilar.filter(s => s.durum === 'tamamlandi').length);
    let bekleyenSayi = $derived(importSarkilar.filter(s => s.durum === 'bekliyor').length);

    let aramaModu = $state(false);
    let aktifAramaSarki = $state<ImportSarki | null>(null);
    let aramaSorgusu = $state("");
    let aramaYapiliyor = $state(false);
    let aramaSonuclari = $state<YouTubeSonuc[]>([]);
    let gosterilenAramaSayisi = $state(5);
    let secilenSonuc = $state<YouTubeSonuc | null>(null);

    $effect(() => {
        if (playerState.isCreatePlaylistModalOpen) {
            isim = "";
            mod = 'yeni';
            importListIsmi = "";
            importSarkilar = [];
            isImporting = false;
            yaratilanListeId = null;
            aramaModu = false;
        }
    });

    function kapat() {
        if (yukleniyor || isImporting) return;
        playerState.isCreatePlaylistModalOpen = false;
    }

    async function kaydet() {
        if (!isim.trim() || yukleniyor) return;
        yukleniyor = true;

        try {
            await playlistOlusturAPI(isim.trim());
            kapat();
        } catch (hata) {
            alert("Liste oluşturulurken bir sorun oluştu.");
        } finally {
            yukleniyor = false;
        }
    }

    async function dosyaSec() {
        try {
            const secilen = await open({
                multiple: false,
                filters: [{ name: 'JSON', extensions: ['json'] }]
            });

            if (secilen) {
                const icerik = await readTextFile(secilen);
                const data = JSON.parse(icerik);

                if (data.playlist_adi && data.sarkilar) {
                    importListIsmi = data.playlist_adi;
                    importSarkilar = data.sarkilar.map((s: any) => ({
                        isim: s.isim,
                        sarkici: s.sarkici,
                        youtube_linki: s.youtube_linki,
                        tarz: s.tarz,
                        durum: s.youtube_linki ? 'bekliyor' : 'link_yok'
                    }));
                } else {
                    alert("Geçersiz playlist formatı.");
                }
            }
        } catch (e) {
            alert("Dosya okunamadı veya parse edilemedi.");
        }
    }

    async function importBaslat() {
        if (importSarkilar.length === 0 || isImporting) return;
        isImporting = true;

        try {
            if (!yaratilanListeId) {
                const yeniListe = await playlistOlusturAPI(importListIsmi);
                yaratilanListeId = yeniListe.id;
            }

            for (let i = 0; i < importSarkilar.length; i++) {
                if (importSarkilar[i].durum === 'bekliyor' && importSarkilar[i].youtube_linki) {
                    importSarkilar[i].durum = 'indiriliyor';

                    try {
                        const yeniSarki = await youtubeIndirAPI(
                            importSarkilar[i].youtube_linki as string, 
                            importSarkilar[i].tarz || "Bilinmiyor"
                        );

                        await sarkiPlaylisteEkle(yeniSarki.id, yaratilanListeId);

                        importSarkilar[i].durum = 'tamamlandi';
                    } catch (err) {
                        console.error("Şarkı indirme/ekleme hatası:", err);
                        importSarkilar[i].durum = 'hata';
                    }
                }
            }
        } catch (e) {
            console.error("Import hatası:", e);
            alert("İşlem sırasında genel bir hata oluştu.");
        } finally {
            isImporting = false;
        }
    }

    async function manuelAra(sarki: ImportSarki) {
        aktifAramaSarki = sarki;
        aramaSorgusu = `${sarki.isim} ${sarki.sarkici}`.trim();
        aramaModu = true;
        secilenSonuc = null;
        aramaSonuclari = [];
        gosterilenAramaSayisi = 5;
        await baslatManuelArama();
    }

    async function baslatManuelArama() {
        if (!aramaSorgusu.trim()) return;
        aramaYapiliyor = true;
        aramaSonuclari = [];
        gosterilenAramaSayisi = 5;
        secilenSonuc = null;

        try {
            aramaSonuclari = await youtubeAramaAPI(aramaSorgusu);
        } catch (e) {
            alert("Arama başarısız: " + e);
        } finally {
            aramaYapiliyor = false;
        }
    }

    function eslestirmeyiOnayla() {
        if (aktifAramaSarki && secilenSonuc) {
            const index = importSarkilar.findIndex(s => s === aktifAramaSarki);
            if (index !== -1) {
                importSarkilar[index].youtube_linki = secilenSonuc.webpage_url;
                importSarkilar[index].durum = 'bekliyor';
            }
            aramaModu = false;
            aktifAramaSarki = null;
        }
    }

    function handleWindowKeydown(e: KeyboardEvent) {
        if (e.key === 'Escape') kapat();
        if (e.key === 'Enter' && mod === 'yeni' && !yukleniyor && playerState.isCreatePlaylistModalOpen && !aramaModu) {
            kaydet();
        }
    }

    function focusAcilista(node: HTMLInputElement) {
        node.focus();
    }
</script>

<svelte:window onkeydown={handleWindowKeydown} />

{#if playerState.isCreatePlaylistModalOpen}
    <div 
        class="fixed inset-0 z-120 flex items-center justify-center bg-black/80 backdrop-blur-sm p-4"
        transition:fade={{ duration: 200 }}
        onclick={kapat}
        onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && kapat()}
        role="button"
        tabindex="0"
        aria-label="Modalı Kapat"
    >
        <div 
            class="bg-(--bg-surface) text-(--text-main) w-full {mod === 'import' && importSarkilar.length > 0 ? 'max-w-2xl' : 'max-w-sm'} rounded-(--radius) shadow-2xl overflow-hidden relative border border-(--border) transition-all duration-500 flex flex-col max-h-[90vh]"
            transition:scale={{ start: 0.95, duration: 300, easing: cubicOut }}
            onclick={(e) => e.stopPropagation()}
            onkeydown={(e) => e.stopPropagation()}
            role="dialog"
            aria-modal="true"
            tabindex="-1"
        >
            <div class="flex border-b border-(--border) bg-(--bg-card) shrink-0">
                <button type="button" class="flex-1 py-4 text-[10px] font-black uppercase tracking-[0.2em] transition-all {mod === 'yeni' ? 'text-(--accent) border-b-2 border-(--accent) bg-(--bg-surface)' : 'text-(--text-dim) hover:text-white hover:bg-(--bg-card-hover)'}" onclick={() => mod = 'yeni'}>
                    Sıfırdan Oluştur
                </button>
                <button type="button" class="flex-1 py-4 text-[10px] font-black uppercase tracking-[0.2em] transition-all {mod === 'import' ? 'text-(--accent) border-b-2 border-(--accent) bg-(--bg-surface)' : 'text-(--text-dim) hover:text-white hover:bg-(--bg-card-hover)'}" onclick={() => mod = 'import'}>
                    JSON İçe Aktar
                </button>
            </div>

            <div class="overflow-y-auto custom-scrollbar flex-1 min-h-0">
                {#if mod === 'yeni'}
                    <div class="p-8 space-y-6" in:fade>
                        <div class="space-y-2">
                            <label for="pl-isim" class="text-[10px] font-black text-(--text-dim) uppercase tracking-widest ml-1">Liste Adı</label>
                            <input id="pl-isim" type="text" bind:value={isim} use:focusAcilista onkeydown={(e) => e.key === 'Enter' && !yukleniyor && isim.trim() && kaydet()} placeholder="Örn: Gece Sürüşü, Retrowave..." class="w-full bg-(--bg-card) border border-(--border) rounded-xl p-4 text-sm outline-none focus:border-(--accent)/50 transition-all font-bold placeholder:text-(--text-dim)/30" />
                        </div>
                        <div class="flex gap-3 pt-4 border-t border-(--border)">
                            <button type="button" onclick={kapat} class="flex-1 py-3.5 rounded-xl font-bold text-[10px] uppercase tracking-[0.2em] text-(--text-dim) bg-(--bg-card) hover:bg-(--bg-card-hover) hover:text-white transition-all active:scale-95">İptal</button>
                            <button type="button" onclick={kaydet} disabled={yukleniyor || !isim.trim()} class="flex-1 py-3.5 rounded-xl font-black text-[10px] uppercase tracking-[0.2em] text-white bg-(--accent) hover:shadow-[0_0_20px_var(--accent-glow)] transition-all active:scale-95 disabled:opacity-30">
                                {#if yukleniyor} Oluşturuluyor... {:else} Oluştur {/if}
                            </button>
                        </div>
                    </div>
                {/if}

                {#if mod === 'import'}
                    <div class="p-6 md:p-8 flex flex-col h-full" in:fade>
                        {#if aramaModu}
                            <div class="space-y-5" in:fade>
                                <div class="flex items-center justify-between border-b border-(--border) pb-4">
                                    <div>
                                        <h3 class="text-sm font-black text-(--accent) uppercase tracking-widest">Manuel Frekans Arama</h3>
                                        <p class="text-[10px] text-(--text-dim) font-bold mt-1.5 uppercase">Hedef: <span class="text-white">{aktifAramaSarki?.isim} - {aktifAramaSarki?.sarkici}</span></p>
                                    </div>
                                    <button type="button" onclick={() => aramaModu = false} class="px-4 py-2 border border-(--border) bg-(--bg-card) rounded-lg text-[10px] font-black uppercase tracking-widest text-(--text-dim) hover:text-white hover:bg-(--bg-card-hover) transition-all active:scale-95">İptal</button>
                                </div>

                                <div class="flex gap-3">
                                    <input type="text" bind:value={aramaSorgusu} onkeydown={(e) => e.key === 'Enter' && !aramaYapiliyor && baslatManuelArama()} class="flex-1 bg-(--bg-surface) border border-(--border) rounded-xl px-4 py-3 text-xs outline-none focus:border-(--accent)/50 transition-all font-mono placeholder:text-(--text-dim)/50" placeholder="Şarkı veya sanatçı adı..." />
                                    <button type="button" onclick={baslatManuelArama} disabled={aramaYapiliyor} class="bg-(--accent) text-white px-6 rounded-xl font-black text-[10px] uppercase tracking-[0.2em] hover:shadow-[0_0_15px_var(--accent-glow)] transition-all active:scale-95 disabled:opacity-50 min-w-[100px] flex items-center justify-center">
                                        {#if aramaYapiliyor} <svg class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg> {:else} Ara {/if}
                                    </button>
                                </div>

                                {#if secilenSonuc}
                                    <div class="flex flex-col items-center justify-center p-8 border-2 border-(--accent) rounded-2xl bg-(--accent)/10 shadow-[0_0_30px_var(--accent-glow)]/30 transition-all" in:scale>
                                        <img src={secilenSonuc.thumbnail} alt="" class="w-40 h-24 object-cover rounded-xl shadow-2xl mb-5" />
                                        <h4 class="text-sm font-black text-center text-white mb-2 leading-tight px-4">{secilenSonuc.title}</h4>
                                        <p class="text-[10px] text-(--accent) font-bold uppercase tracking-widest mb-8 text-center">{secilenSonuc.channel} <span class="mx-2 opacity-50">•</span> {secilenSonuc.duration_string}</p>
                                        
                                        <div class="flex gap-4 w-full max-w-sm">
                                            <button type="button" onclick={() => secilenSonuc = null} class="flex-1 py-3.5 rounded-xl border border-(--border) text-[10px] font-black uppercase tracking-widest text-(--text-dim) hover:text-white hover:bg-(--bg-card) transition-all active:scale-95">Vazgeç</button>
                                            <button type="button" onclick={eslestirmeyiOnayla} class="flex-1 py-3.5 rounded-xl bg-(--accent) text-white text-[10px] font-black uppercase tracking-widest shadow-lg hover:shadow-[0_0_20px_var(--accent-glow)] hover:scale-105 transition-all">Eşleştir</button>
                                        </div>
                                    </div>
                                {:else if aramaSonuclari.length > 0}
                                    <div class="max-h-[40vh] overflow-y-auto custom-scrollbar space-y-2 pr-2" in:fade>
                                        {#each aramaSonuclari.slice(0, gosterilenAramaSayisi) as sonuc}
                                            <button type="button" onclick={() => secilenSonuc = sonuc} class="w-full flex items-center gap-4 p-3 bg-(--bg-surface) border border-(--border) hover:border-(--accent)/50 rounded-xl group transition-all text-left">
                                                <img src={sonuc.thumbnail} alt="" class="w-20 h-12 object-cover rounded-lg opacity-60 group-hover:opacity-100 transition-opacity" />
                                                <div class="flex-1 min-w-0 pr-4">
                                                    <p class="text-xs font-bold text-(--text-main) truncate group-hover:text-(--accent) transition-colors">{sonuc.title}</p>
                                                    <p class="text-[9px] font-bold uppercase tracking-widest text-(--text-dim) truncate mt-1.5 opacity-80">{sonuc.channel} <span class="mx-1">•</span> {sonuc.duration_string}</p>
                                                </div>
                                                <svg class="w-5 h-5 text-(--text-dim) group-hover:text-(--accent) opacity-0 group-hover:opacity-100 transition-all -translate-x-2 group-hover:translate-x-0" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"></path></svg>
                                            </button>
                                        {/each}
                                        
                                        {#if gosterilenAramaSayisi < aramaSonuclari.length}
                                            <button type="button" onclick={() => gosterilenAramaSayisi += 5} class="w-full mt-2 py-4 rounded-xl border-2 border-dashed border-(--border) text-[10px] font-black uppercase tracking-widest text-(--text-dim) hover:text-(--accent) hover:border-(--accent)/50 hover:bg-(--accent)/5 transition-all active:scale-95">
                                                Daha Fazla Göster ({aramaSonuclari.length - gosterilenAramaSayisi} Kaldı)
                                            </button>
                                        {/if}
                                    </div>
                                {/if}
                            </div>

                        {:else if importSarkilar.length === 0}
                            <button type="button" onclick={dosyaSec} class="w-full border-2 border-dashed border-(--border) rounded-2xl p-12 flex flex-col items-center justify-center text-center hover:border-(--accent)/50 hover:bg-(--accent)/5 transition-all group my-auto">
                                <svg class="w-12 h-12 text-(--text-dim) group-hover:text-(--accent) mb-4 transition-colors" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"></path></svg>
                                <span class="text-sm font-black text-(--text-main) group-hover:text-(--accent) transition-colors">JSON Dosyası Seç</span>
                                <span class="text-[10px] text-(--text-dim) mt-3 uppercase tracking-[0.2em] font-bold">Dışa aktarılmış playlist dosyasını yükleyin</span>
                            </button>
                        {:else}
                            <div class="flex flex-col sm:flex-row sm:items-end justify-between gap-4 border-b border-(--border) pb-4 shrink-0" in:slide>
                                <div class="min-w-0">
                                    <h3 class="text-xl font-black text-(--accent) truncate italic">{importListIsmi}</h3>
                                    <p class="text-[10px] font-bold text-(--text-dim) uppercase tracking-widest mt-1.5 flex gap-4">
                                        <span class="text-white">TOPLAM: {importSarkilar.length}</span>
                                        <span class="text-emerald-400">TAMAM: {tamamlananSayi}</span>
                                        <span class="text-orange-400">BEKLEYEN: {bekleyenSayi}</span>
                                    </p>
                                </div>
                                <button type="button" onclick={() => { importSarkilar = []; importListIsmi = ''; isImporting = false; yaratilanListeId = null; }} disabled={isImporting} class="text-[10px] text-(--text-dim) hover:text-red-400 uppercase font-black transition-colors disabled:opacity-30 shrink-0 bg-(--bg-card) px-3 py-1.5 rounded-lg border border-(--border)">
                                    Dosyayı İptal Et
                                </button>
                            </div>

                            <div class="flex-1 overflow-y-auto custom-scrollbar rounded-xl border border-(--border) bg-(--bg-card) p-2 space-y-1.5 my-4">
                                {#each importSarkilar as sarki, i}
                                    <div class="flex items-center p-3 rounded-lg bg-(--bg-surface) border border-(--border) text-xs transition-colors {sarki.durum === 'indiriliyor' ? 'border-(--accent)/50 shadow-[0_0_10px_var(--accent-glow)]/10' : ''}">
                                        <div class="w-6 text-[10px] font-mono text-(--text-dim)/50 font-bold shrink-0">{i + 1}</div>
                                        <div class="flex-1 min-w-0 pr-4">
                                            <p class="font-black truncate text-(--text-main) {sarki.durum === 'indiriliyor' ? 'text-(--accent)' : ''}">{sarki.isim}</p>
                                            <p class="text-[9px] uppercase tracking-[0.2em] text-(--text-dim) font-bold truncate mt-0.5">{sarki.sarkici}</p>
                                        </div>
                                        <div class="shrink-0 flex items-center justify-end w-28">
                                            {#if sarki.durum === 'bekliyor'}
                                                <span class="text-[10px] font-black uppercase tracking-widest text-(--text-dim)">Sırada</span>
                                            {:else if sarki.durum === 'indiriliyor'}
                                                <span class="text-[10px] font-black uppercase tracking-widest text-(--accent) animate-pulse flex items-center gap-1.5"><svg class="w-3 h-3 animate-spin" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg> İniyor</span>
                                            {:else if sarki.durum === 'tamamlandi'}
                                                <span class="text-[10px] font-black uppercase tracking-widest text-emerald-500 flex items-center gap-1"><svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" stroke-width="3" viewBox="0 0 24 24"><polyline points="20 6 9 17 4 12"></polyline></svg> Tamam</span>
                                            {:else if sarki.durum === 'hata' || sarki.durum === 'link_yok'}
                                                <button type="button" onclick={() => manuelAra(sarki)} class="bg-(--bg-card) hover:bg-(--accent) text-(--accent) hover:text-white border border-(--accent) px-4 py-1.5 rounded-lg text-[9px] font-black uppercase tracking-widest transition-colors flex items-center gap-1.5 active:scale-95 shadow-sm hover:shadow-[0_0_10px_var(--accent-glow)]/50">
                                                    <svg class="w-3 h-3" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>
                                                    ARA
                                                </button>
                                            {/if}
                                        </div>
                                    </div>
                                {/each}
                            </div>

                            <div class="flex gap-3 pt-2 shrink-0">
                                {#if tamamlananSayi + importSarkilar.filter(s => s.durum === 'link_yok' || s.durum === 'hata').length === importSarkilar.length}
                                    <button type="button" onclick={kapat} class="flex-1 py-4 rounded-xl font-black text-[10px] uppercase tracking-[0.2em] text-white bg-emerald-500 hover:bg-emerald-600 hover:shadow-[0_0_20px_rgba(16,185,129,0.4)] transition-all active:scale-95">
                                        İşlem Tamamlandı - Kapat
                                    </button>
                                {:else}
                                    <button type="button" onclick={kapat} disabled={isImporting} class="w-1/3 py-4 rounded-xl font-bold text-[10px] uppercase tracking-[0.2em] text-(--text-dim) bg-(--bg-card) hover:bg-(--bg-card-hover) hover:text-white transition-all active:scale-95 disabled:opacity-30">
                                        Gizle
                                    </button>
                                    <button type="button" onclick={importBaslat} disabled={isImporting || bekleyenSayi === 0} class="flex-1 py-4 rounded-xl font-black text-[10px] uppercase tracking-[0.2em] text-white bg-(--accent) hover:shadow-[0_0_20px_var(--accent-glow)] transition-all active:scale-95 disabled:opacity-30 flex items-center justify-center gap-2">
                                        {#if isImporting}
                                            <svg class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
                                            Frekanslar İndiriliyor...
                                        {:else}
                                            Frekansları İndir
                                        {/if}
                                    </button>
                                {/if}
                            </div>
                        {/if}
                    </div>
                {/if}
            </div>
        </div>
    </div>
{/if}

<style>
    div[role="dialog"]:focus { outline: none; }
    .custom-scrollbar::-webkit-scrollbar { width: 4px; }
    .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--accent); }
</style>