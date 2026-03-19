<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { playerState, type Sarki } from '../store.svelte';
    import { fade, scale } from 'svelte/transition';
    import { cubicOut } from 'svelte/easing';

    let yukleniyor = $state(false);
    
    let formVerisi = $state({
        id: "", isim: "", sarkici: "", album: "", tarz: "", yil: ""
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
        playerState.isEditModalOpen = false;
        setTimeout(() => playerState.duzenlenecekSarki = null, 300);
    }

    async function kaydet() {
        if (!formVerisi.isim.trim()) return;
        yukleniyor = true;

        try {
            const guncelSarki = await invoke<Sarki>('sarki_guncelle', {
                id: formVerisi.id,
                isim: formVerisi.isim,
                sarkici: formVerisi.sarkici,
                album: formVerisi.album,
                tarz: formVerisi.tarz || null,
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
            alert("Güncelleme başarısız: " + error);
        } finally {
            yukleniyor = false;
        }
    }
</script>

{#if playerState.isEditModalOpen}
    <div 
        class="fixed inset-0 z-[110] flex items-center justify-center bg-black/70 backdrop-blur-md p-4"
        transition:fade={{ duration: 200 }}
        onclick={kapat}
        role="presentation"
    >
        <div 
            class="bg-[var(--bg-surface)] text-[var(--text-main)] w-full max-w-md rounded-[var(--radius)] shadow-2xl overflow-hidden relative border border-[var(--border)]"
            transition:scale={{ start: 0.95, duration: 300, easing: cubicOut }}
            onclick={(e) => e.stopPropagation()}
            role="presentation"
        >
            <div class="flex justify-between items-center px-6 py-4 bg-[var(--bg-card)] border-b border-[var(--border)]">
                <h2 class="text-sm font-black uppercase tracking-widest text-[var(--text-main)]/70">Veri Bloğunu Düzenle</h2>
            </div>

            <div class="p-6 space-y-4">
                <div class="space-y-2">
                    <label for="edit-isim" class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-widest">Şarkı Adı</label>
                    <input id="edit-isim" type="text" bind:value={formVerisi.isim} class="w-full bg-[var(--bg-card)] border border-[var(--border)] rounded-xl p-3.5 text-sm outline-none focus:border-[var(--accent)]/50 transition-colors" />
                </div>
                
                <div class="grid grid-cols-2 gap-4">
                    <div class="space-y-2">
                        <label for="edit-sarkici" class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-widest">Sanatçı</label>
                        <input id="edit-sarkici" type="text" bind:value={formVerisi.sarkici} class="w-full bg-[var(--bg-card)] border border-[var(--border)] rounded-xl p-3.5 text-sm outline-none focus:border-[var(--accent)]/50 transition-colors" />
                    </div>
                    <div class="space-y-2">
                        <label for="edit-album" class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-widest">Albüm</label>
                        <input id="edit-album" type="text" bind:value={formVerisi.album} class="w-full bg-[var(--bg-card)] border border-[var(--border)] rounded-xl p-3.5 text-sm outline-none focus:border-[var(--accent)]/50 transition-colors" />
                    </div>
                </div>

                <div class="grid grid-cols-2 gap-4">
                    <div class="space-y-2">
                        <label for="edit-tarz" class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-widest">Tür / Tarz</label>
                        <input id="edit-tarz" type="text" bind:value={formVerisi.tarz} placeholder="Örn: Synthwave" class="w-full bg-[var(--bg-card)] border border-[var(--border)] rounded-xl p-3.5 text-sm outline-none focus:border-[var(--accent)]/50 transition-colors" />
                    </div>
                    <div class="space-y-2">
                        <label for="edit-yil" class="text-[10px] font-black text-[var(--text-dim)] uppercase tracking-widest">Yıl</label>
                        <input id="edit-yil" type="number" bind:value={formVerisi.yil} placeholder="2024" class="w-full bg-[var(--bg-card)] border border-[var(--border)] rounded-xl p-3.5 text-sm outline-none focus:border-[var(--accent)]/50 transition-colors" />
                    </div>
                </div>

                <div class="flex gap-3 pt-4 border-t border-[var(--border)] mt-6">
                    <button onclick={kapat} class="flex-1 py-3.5 rounded-xl font-bold text-[10px] uppercase tracking-widest text-[var(--text-dim)] bg-[var(--bg-card)] hover:bg-[var(--bg-card-hover)] transition-colors">İptal</button>
                    <button onclick={kaydet} disabled={yukleniyor || !formVerisi.isim.trim()} class="flex-1 py-3.5 rounded-xl font-black text-[10px] uppercase tracking-widest text-white bg-[var(--accent)] hover:opacity-90 transition-opacity disabled:opacity-50">
                        {yukleniyor ? 'Kaydediliyor...' : 'Güncelle'}
                    </button>
                </div>
            </div>
        </div>
    </div>
{/if}