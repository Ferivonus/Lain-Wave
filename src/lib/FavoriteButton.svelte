<script lang="ts">
    import { playerState, toggleFavori } from '../store.svelte';
    import { scale } from 'svelte/transition';

    let { sarkiId } = $props<{ sarkiId: string }>();

    let isFavorite = $derived(playerState.favoriler.includes(sarkiId));

    function handleToggle(e: Event) {
        e.stopPropagation();
        toggleFavori(sarkiId);
    }
</script>

<button 
    type="button" 
    aria-label="Favori" 
    aria-pressed={isFavorite}
    title={isFavorite ? "Favorilerden Çıkar" : "Favorilere Ekle"} 
    onclick={handleToggle} 
    class="relative flex items-center justify-center transition-all duration-300 active:scale-75 hover:scale-110 shrink-0 focus:outline-none rounded-full p-2
    {isFavorite ? 'text-(--accent)' : 'text-(--text-dim) hover:text-(--text-main)'}">
    
    {#if isFavorite}
        <div 
            in:scale={{ duration: 400, start: 0 }}
            class="absolute inset-0 rounded-full blur-md opacity-60"
            style="background-color: var(--accent-glow);">
        </div>
    {/if}

    <div class="relative z-10 flex items-center justify-center">
        {#if isFavorite}
            <div in:scale={{ duration: 250, start: 0.5 }}>
                <svg 
                    class="w-5 h-5 transition-all duration-300" 
                    fill="currentColor" 
                    viewBox="0 0 24 24"
                    style="filter: drop-shadow(0 0 4px var(--accent-glow));">
                    <path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/>
                </svg>
            </div>
        {:else}
            <div in:scale={{ duration: 200, start: 0.8 }}>
                <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" viewBox="0 0 24 24">
                    <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"></path>
                </svg>
            </div>
        {/if}
    </div>
</button>