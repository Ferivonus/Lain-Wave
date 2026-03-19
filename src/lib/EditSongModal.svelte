<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { playerState, type Sarki } from '../store.svelte';
    import { fade, scale } from 'svelte/transition';
    import { cubicOut } from 'svelte/easing';

    let yukleniyor = $state(false);
    
    let formVerisi = $state({
        id: "", 
        isim: "", 
        sarkici: "", 
        album: "", 
        tarz: "", 
        yil: ""
    });

    $effect(() => {
        if (playerState.isEditModalOpen && playerState.duzenlenecekSarki) {
            const s = playerState.duzenlenecekSarki;
            formVerisi = {
                id: s.id,
                isim: s.isim,
                sarkici: s.sarkici,
                album: s.album || "",
                tarz: s.tarz || "",
                yil: s.yil ? s.yil.toString() : ""
            };
        }
    });

    function kapat() {
        if (yukleniyor) return;
        playerState.isEditModalOpen = false;
        setTimeout(() => playerState.duzenlenecekSarki = null, 300);
    }

    async function kaydet() {
        if (!formVerisi.isim.trim() || yukleniyor) return;
        yukleniyor = true;

        try {
            const guncelSarki = await invoke<Sarki>('sarki_guncelle', {
                id: formVerisi.id,
                isim: formVerisi.isim.trim(),
                sarkici: formVerisi.sarkici.trim(),
                album: formVerisi.album.trim(),
                tarz: formVerisi.tarz.trim() || null,
                yil: formVerisi.yil ? parseInt(formVerisi.yil) : null
            });

            const index = playerState.sarkiListesi.findIndex(s => s.id === formVerisi.id);
            if (index !== -1) {
                playerState.sarkiListesi[index] = guncelSarki;
            }
            
            if (playerState.aktifSarki?.id === formVerisi.id) {
                playerState.aktifSarki = guncelSarki;
            }

            kapat();
        } catch (error) {
            console.error("Güncelleme hatası:", error);
            alert("Veriler kaydedilirken bir sorun oluştu.");
        } finally {
            yukleniyor = false;
        }
    }

    function handleKeyDown(e: KeyboardEvent) {
        if (e.key === 'Escape') kapat();
        if (e.key === 'Enter' && !yukleniyor && e.ctrlKey) kaydet();
    }
</script>

<svelte:window onkeydown={handleKeyDown} />

{#if playerState.isEditModalOpen}
    <div 
        class="fixed inset-0 z-[110] flex items-center justify-center bg-black/80 backdrop-blur-sm p-4"
        transition:fade={{ duration: 200 }}
        onclick={kapat}
        onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && kapat()}
        role="button"
        tabindex="-1"
        aria-label="Modalı Kapat"
    >
        <div 
            class="bg-[var(--bg-surface)] text-[var(--text-main)] w-full max-w-md rounded-[var(--radius)] shadow-2xl overflow-hidden relative border border-[var(--border)]"
            transition:scale={{ start: 0.95, duration: 300, easing: cubicOut }}
            onclick={(e) => e.stopPropagation()}
            onkeydown={(e) => e.stopPropagation()}
            role="dialog"
            aria-modal="true"
            aria-labelledby="modal-title"
            tabindex="-1"
        >
            <div class="flex justify-between items-center px-6 py-5 bg-[var(--bg-card)] border-b border-[var(--border)]">
                <h2 id="modal-title" class="text-xs font-black uppercase tracking-[0.2em] text-[var(--accent)]">
                    Veri Bloğunu Güncelle
                </h2>
                <button 
                    type="button"
                    onclick={kapat} 
                    class="text-[var(--text-dim)] hover:text-white transition-colors p-1"
                    aria-label="Kapat"
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true">
                        <path d="M6 18L18 6M6 6l12 12"></path>
                    </svg>
                </button>
            </div>

            <div class="p-8 space-y-6">
                <div class="space-y-2">
                    <label for="edit-isim" class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-widest ml-1">Parça İsmi</label>
                    <input 
                        id="edit-isim" 
                        type="text" 
                        bind:value={formVerisi.isim} 
                        class="w-full bg-[var(--bg-card)] border border-[var(--border)] rounded-xl p-4 text-sm outline-none focus:border-[var(--accent)]/50 transition-all" 
                    />
                </div>
                
                <div class="grid grid-cols-2 gap-4">
                    <div class="space-y-2">
                        <label for="edit-sarkici" class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-widest ml-1">Sanatçı</label>
                        <input id="edit-sarkici" type="text" bind:value={formVerisi.sarkici} class="w-full bg-[var(--bg-card)] border border-[var(--border)] rounded-xl p-4 text-sm outline-none focus:border-[var(--accent)]/50 transition-all" />
                    </div>
                    <div class="space-y-2">
                        <label for="edit-album" class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-widest ml-1">Albüm</label>
                        <input id="edit-album" type="text" bind:value={formVerisi.album} class="w-full bg-[var(--bg-card)] border border-[var(--border)] rounded-xl p-4 text-sm outline-none focus:border-[var(--accent)]/50 transition-all" />
                    </div>
                </div>

                <div class="grid grid-cols-2 gap-4">
                    <div class="space-y-2">
                        <label for="edit-tarz" class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-widest ml-1">Tür / Frekans</label>
                        <input id="edit-tarz" type="text" bind:value={formVerisi.tarz} class="w-full bg-[var(--bg-card)] border border-[var(--border)] rounded-xl p-4 text-sm outline-none focus:border-[var(--accent)]/50 transition-all" />
                    </div>
                    <div class="space-y-2">
                        <label for="edit-yil" class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-widest ml-1">Yıl</label>
                        <input id="edit-yil" type="number" bind:value={formVerisi.yil} class="w-full bg-[var(--bg-card)] border border-[var(--border)] rounded-xl p-4 text-sm outline-none focus:border-[var(--accent)]/50 transition-all" />
                    </div>
                </div>

                <div class="flex gap-3 pt-6 border-t border-[var(--border)]">
                    <button 
                        type="button"
                        onclick={kapat} 
                        class="flex-1 py-4 rounded-xl font-bold text-[10px] uppercase tracking-[0.2em] text-[var(--text-dim)] bg-[var(--bg-card)] hover:bg-[var(--bg-card-hover)] hover:text-white transition-all active:scale-95"
                    >
                        İptal
                    </button>
                    <button 
                        type="button"
                        onclick={kaydet} 
                        disabled={yukleniyor || !formVerisi.isim.trim()} 
                        class="flex-1 py-4 rounded-xl font-black text-[10px] uppercase tracking-[0.2em] text-white bg-[var(--accent)] hover:shadow-[0_0_20px_var(--accent-glow)] transition-all active:scale-95 disabled:opacity-30"
                    >
                        {#if yukleniyor}
                            İşleniyor...
                        {:else}
                            Veriyi Güncelle
                        {/if}
                    </button>
                </div>
            </div>
        </div>
    </div>
{/if}

<style>
    /* Sayı girişi oklarını gizle ve appearance hatasını gider */
    input::-webkit-outer-spin-button,
    input::-webkit-inner-spin-button {
        -webkit-appearance: none;
        appearance: none;
        margin: 0;
    }
    input[type=number] {
        -moz-appearance: textfield;
        appearance: textfield;
    }
</style>