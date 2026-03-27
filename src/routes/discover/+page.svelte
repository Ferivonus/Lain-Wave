<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { listen, type UnlistenFn, type Event as TauriEvent } from '@tauri-apps/api/event';
  import FavoriteButton from '$lib/FavoriteButton.svelte';
  import { 
      playerState, 
      sarkiCal, 
      initializePlayer, 
      muzikAra,
      youtubeIndir,
      handleSarkiSil,
      editModaliAc,
      handlePlaylistEkle
  } from '../../store.svelte';
  import { fade, fly, scale, slide } from 'svelte/transition';
  import { diller } from '../../constants/constants.svelte';

  onMount(async () => {
    if (playerState.sarkiListesi.length === 0) {
      await initializePlayer();
    }
  });

  let kategoriler = $derived.by(() => {
    const map = new Map();
    playerState.sarkiListesi.forEach(s => {
      if (!s.tarz) return;
      const t = s.tarz.trim();
      map.set(t, (map.get(t) || 0) + 1);
    });
    return Array.from(map.entries()).map(([isim, adet]) => ({ isim, adet }));
  });

  let enCokDinlenenler = $derived(
    [...playerState.sarkiListesi]
      .sort((a, b) => (b.dinlenme_sayisi || 0) - (a.dinlenme_sayisi || 0))
      .slice(0, 5)
  );

  let yeniEklenenler = $derived(
    [...playerState.sarkiListesi]
      .slice(-10) 
      .reverse()
  );

  const tarzlar = ["Pop", "Rock", "Lofi", "Electronic", "Jazz", "Hip-Hop", "Classical", "Podcast"];
  const tarzIkonlari: Record<string, string> = {
    "Pop": '<svg width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m12 3-1.9 5.8a2 2 0 0 1-1.3 1.3L3 12l5.8 1.9a2 2 0 0 1 1.3 1.3L12 21l1.9-5.8a2 2 0 0 1 1.3-1.3L21 12l-5.8-1.9a2 2 0 0 1-1.3-1.3Z"/></svg>',
    "Rock": '<svg width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z"/></svg>',
    "Lofi": '<svg width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M17 8h1a4 4 0 110 8h-1"/><path d="M3 8h14v9a4 4 0 01-4 4H7a4 4 0 01-4-4Z"/><line x1="6" y1="2" x2="6" y2="4"/><line x1="10" y1="2" x2="10" y2="4"/><line x1="14" y1="2" x2="14" y2="4"/></svg>',
    "Cyberpunk": '<svg width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="4" y="4" width="16" height="16" rx="2" ry="2"/><rect x="9" y="9" width="6" height="6"/><line x1="9" y1="1" x2="9" y2="4"/><line x1="15" y1="1" x2="15" y2="4"/><line x1="9" y1="20" x2="9" y2="23"/><line x1="15" y1="20" x2="15" y2="23"/><line x1="20" y1="9" x2="23" y2="9"/><line x1="20" y1="14" x2="23" y2="14"/><line x1="1" y1="9" x2="4" y2="9"/><line x1="1" y1="14" x2="4" y2="14"/></svg>',
    "Ghibli": '<svg width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11 20A7 7 0 0 1 9.8 6.1C15.5 5 17 4.48 19 2c1 2 2 4.18 2 8 0 5.5-4.78 10-10 10Z"/><path d="M2 21c0-3 1.85-5.36 5.08-6C9.5 14.52 12 13 13 12"/></svg>',
    "Electronic": '<svg width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/></svg>',
    "Jazz": '<svg width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><circle cx="12" cy="12" r="3"/></svg>',
    "Podcast": '<svg width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2a3 3 0 0 0-3 3v7a3 3 0 0 0 6 0V5a3 3 0 0 0-3-3Z"/><path d="M19 10v2a7 7 0 0 1-14 0v-2"/><line x1="12" y1="19" x2="12" y2="23"/><line x1="8" y1="23" x2="16" y2="23"/></svg>'
  };

  // İNDİRME AYARLARI
  let ayarlarAcik = $state(false);
  let secilenTarz = $state("Pop");
  let secilenDil = $state("auto");
  let ytCeviriKullan = $state(true);
  let aiKullan = $state(true);

  // İNDİRME DURUMU BİLGİSİ
  let downloadInfo = $state({ pct: 0, speed: "0KiB/s", eta: "00:00" });

  interface DownloadProgressPayload { percentage: number; speed: string; eta: string; }

  $effect(() => {
      let unlistenProgress: UnlistenFn;
      let unlistenWarning: UnlistenFn;
      
      listen<DownloadProgressPayload>("download-progress", (event: TauriEvent<DownloadProgressPayload>) => {
          downloadInfo.pct = event.payload.percentage;
          downloadInfo.speed = event.payload.speed;
          downloadInfo.eta = event.payload.eta;
      }).then((fn) => { unlistenProgress = fn; });

      listen<string>("download-warning", (event: TauriEvent<string>) => {
          playerState.indirmeMesaji = event.payload; 
      }).then((fn) => { unlistenWarning = fn; });

      return () => {
          if (unlistenProgress) unlistenProgress();
          if (unlistenWarning) unlistenWarning();
      };
  });

  async function ozellestirilmisIndir(url: string) {
      downloadInfo = { pct: 0, speed: "0KiB/s", eta: "00:00" };
      await youtubeIndir(url, secilenTarz, secilenDil, ytCeviriKullan, aiKullan);
  }
</script>

<div class="p-8 lg:p-12 w-full min-h-full pb-32 flex flex-col relative min-w-0 bg-transparent text-(--text-main) transition-colors duration-500 overflow-y-auto custom-scrollbar">
  
  <section class="relative w-full h-72 rounded-(--radius) overflow-hidden mb-12 shadow-2xl border border-(--border) group shrink-0" in:fade>
    <div class="absolute inset-0 bg-linear-to-r from-(--accent) via-(--accent-sec) to-(--bg-main) opacity-60 z-10"></div>
    <div class="absolute inset-0 bg-[url('https://images.unsplash.com/photo-1614613535308-eb5fbd3d2c17?q=80&w=2070')] bg-cover bg-center mix-blend-overlay group-hover:scale-105 transition-transform duration-1000"></div>
    <div class="absolute inset-0 p-10 flex flex-col justify-center z-20">
      <div class="flex items-center gap-3 mb-4">
        <span class="w-10 h-0.5 bg-white/50"></span>
        <span class="text-[10px] font-black tracking-[0.4em] text-white/90 uppercase">Lain Wave Intelligence</span>
      </div>
      <h1 class="text-5xl lg:text-7xl font-black text-white mb-4 tracking-tighter italic leading-none drop-shadow-2xl">KEŞFET</h1>
      <p class="text-white/80 max-w-lg font-medium text-sm leading-relaxed">Sistem kütüphaneni analiz etti. Yeni veri akışları yakalayabilir veya mevcut arşivi inceleyebilirsin.</p>
    </div>
  </section>

  <section class="mb-16 bg-(--bg-card) border border-(--border) rounded-(--radius) p-8 shadow-xl relative overflow-visible group shrink-0">
      <div class="absolute top-0 right-0 w-64 h-64 bg-(--accent)/5 blur-[80px] rounded-full pointer-events-none"></div>
      
      <div class="flex items-start justify-between mb-6 relative z-10">
          <div class="flex items-center gap-4">
              <div class="w-12 h-12 rounded-2xl bg-(--accent)/10 text-(--accent) flex items-center justify-center border border-(--accent)/20 shrink-0">
                  <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M15.5 14h-.79l-.28-.27A6.471 6.471 0 0 0 16 9.5 6.5 6.5 0 1 0 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 14 11.99 14 9.5 14z"/></svg>
              </div>
              <div>
                  <h2 class="text-xl font-black uppercase tracking-tight italic">Ağ Tarayıcısı</h2>
                  <p class="text-[10px] text-(--text-dim) font-bold tracking-widest uppercase mt-1">YouTube üzerinden müzik arayın veya URL yapıştırın</p>
              </div>
          </div>
          
          <button 
              type="button" 
              onclick={() => ayarlarAcik = !ayarlarAcik}
              aria-label="İndirme Ayarları"
              title="İndirme Ayarları"
              class="w-10 h-10 rounded-xl border flex items-center justify-center transition-all {ayarlarAcik ? 'bg-(--accent) text-white border-(--accent) shadow-[0_0_15px_var(--accent-glow)]' : 'bg-(--bg-surface) text-(--text-dim) border-(--border) hover:border-(--accent)/50 hover:text-(--accent)'}"
          >
              <svg class="w-5 h-5 transition-transform {ayarlarAcik ? 'rotate-90' : ''}" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path><circle cx="12" cy="12" r="3"></circle></svg>
          </button>
      </div>

      {#if ayarlarAcik}
          <div class="mb-6 p-4 bg-black/20 border border-(--border) rounded-2xl relative z-10" in:slide>
              <div class="flex flex-wrap items-end gap-4">
                  <div class="flex-1 min-w-30 space-y-2">
                      <label for="kat-secici" class="text-[9px] font-black text-(--text-dim) uppercase tracking-widest block">Kategori</label>
                      <select id="kat-secici" bind:value={secilenTarz} class="w-full bg-(--bg-surface) border border-(--border) text-(--text-main) p-2.5 rounded-xl text-xs font-bold outline-none focus:border-(--accent) appearance-none cursor-pointer uppercase tracking-widest">
                          {#each tarzlar as tarz}
                              <option value={tarz} class="bg-[#1e1e1e] text-white">{tarz}</option>
                          {/each}
                      </select>
                  </div>

                  <div class="flex-1 min-w-30 space-y-2">
                      <label for="dil-secici" class="text-[9px] font-black text-(--text-dim) uppercase tracking-widest block">Hedef Dil</label>
                      <select id="dil-secici" bind:value={secilenDil} class="w-full bg-(--bg-surface) border border-(--border) text-(--text-main) p-2.5 rounded-xl text-xs font-bold outline-none focus:border-(--accent) appearance-none cursor-pointer uppercase tracking-widest">
                          {#each diller as dil}
                              <option value={dil.kod} class="bg-[#1e1e1e] text-white">{dil.ad}</option>
                          {/each}
                      </select>
                  </div>

                  <div class="flex flex-col gap-3 min-w-40 pb-1">
                      <label class="flex items-center gap-3 cursor-pointer group">
                          <input type="checkbox" bind:checked={ytCeviriKullan} class="accent-(--accent) w-4 h-4 cursor-pointer"/>
                          <span class="text-[10px] font-bold text-(--text-dim) group-hover:text-white uppercase tracking-widest transition-colors">YouTube Çevirisi</span>
                      </label>
                      <label class="flex items-center gap-3 cursor-pointer group" title="YouTube'da altyazı yoksa yapay zeka sesi analiz edip sözleri çıkarır.">
                          <input type="checkbox" bind:checked={aiKullan} class="accent-(--accent) w-4 h-4 cursor-pointer"/>
                          <span class="text-[10px] font-bold text-(--text-dim) group-hover:text-white uppercase tracking-widest transition-colors">Whisper AI Kullan</span>
                      </label>
                  </div>
              </div>
          </div>
      {/if}

      <div class="relative z-10 mb-4">
          <div class="flex flex-col md:flex-row gap-4">
              <input 
                  type="text" 
                  bind:value={playerState.aramaSorgusu}
                  onkeydown={(e) => e.key === 'Enter' && !playerState.aramaYapiliyor && muzikAra()}
                  placeholder="Şarkı adı, link veya playlist adresi yazın..." 
                  class="flex-1 bg-(--bg-surface) border border-(--border) rounded-xl px-6 py-4 outline-none text-sm text-(--text-main) focus:border-(--accent)/50 transition-colors placeholder:text-(--text-dim)/50 font-mono"
                  disabled={playerState.aramaYapiliyor || playerState.aktifIndirmeler.size > 0}
              />
              <button 
                  type="button"
                  onclick={muzikAra}
                  disabled={playerState.aramaYapiliyor || !playerState.aramaSorgusu.trim() || playerState.aktifIndirmeler.size > 0}
                  aria-label="Aramayı Başlat"
                  class="bg-(--accent) hover:bg-(--accent-sec) text-white font-black uppercase tracking-[0.2em] text-[10px] px-10 py-4 rounded-xl transition-all shadow-lg active:scale-95 disabled:opacity-50 shrink-0 min-w-40 flex items-center justify-center"
              >
                  {#if playerState.aramaYapiliyor}
                      <svg class="w-5 h-5 animate-spin" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
                  {:else} Ağı Tara {/if}
              </button>
          </div>

          {#if playerState.aktifIndirmeler.size > 0}
              <div class="mt-4 bg-(--bg-surface) border border-(--border) p-4 rounded-xl shadow-inner" in:slide>
                  <div class="flex justify-between text-[10px] mb-2 font-black uppercase tracking-widest text-(--text-dim)">
                      <span class="text-(--accent) animate-pulse">Veri Akışı Sağlanıyor</span>
                      <span>{Math.round(downloadInfo.pct)}%</span>
                  </div>
                  <div class="w-full h-2 bg-black/50 rounded-full overflow-hidden border border-(--border)">
                      <div class="h-full bg-(--accent) transition-all duration-300" style="width: {downloadInfo.pct}%"></div>
                  </div>
                  <div class="flex justify-between items-center mt-2">
                      <p class="text-[9px] text-(--text-dim) font-mono">{downloadInfo.speed}</p>
                      <p class="text-[9px] text-(--text-dim) font-mono">Kalan: {downloadInfo.eta}</p>
                  </div>
              </div>
          {/if}
      </div>
      
      {#if playerState.indirmeMesaji}
          <div class="mb-2 text-[10px] font-mono font-bold uppercase tracking-widest {playerState.indirmeMesaji.includes('başarı') || playerState.indirmeMesaji.includes('tamamlandı') ? 'text-emerald-400' : 'text-(--accent)'}" in:slide>> {playerState.indirmeMesaji}</div>
      {/if}

      {#if playerState.aramaSonuclari.length > 0}
          <div class="flex flex-col gap-2 mt-6" in:fade>
              <div class="flex justify-between items-end border-b border-(--border) pb-2 mb-2">
                  <h3 class="text-[10px] font-black text-(--text-dim) uppercase tracking-[0.3em]">
                      Bulunan Sinyaller ({playerState.aramaSonuclari.length})
                  </h3>
                  {#if playerState.aramaSonuclari.length > 1}
                      <button
                          type="button"
                          onclick={async () => {
                              if (playerState.topluIndirmeAktif) return;
                              playerState.topluIndirmeAktif = true;
                              playerState.gosterilenSayi = playerState.aramaSonuclari.length;
                              playerState.indirmeMesaji = "Toplu veri akışı başlatıldı...";
                              for (const sonuc of playerState.aramaSonuclari) {
                                  if (!playerState.aktifIndirmeler.has(sonuc.webpage_url)) {
                                      await ozellestirilmisIndir(sonuc.webpage_url);
                                  }
                              }
                              playerState.topluIndirmeAktif = false;
                              playerState.indirmeMesaji = "Toplu aktarım tamamlandı.";
                          }}
                          disabled={playerState.topluIndirmeAktif}
                          class="text-[9px] font-black uppercase tracking-widest px-3 py-1.5 rounded-lg border transition-all {playerState.topluIndirmeAktif ? 'bg-(--accent)/10 text-(--accent) border-(--accent)/30 cursor-not-allowed' : 'text-(--text-main) border-(--border) hover:border-(--accent) hover:text-(--accent)'}"
                      >
                          {#if playerState.topluIndirmeAktif} Veri Çekiliyor... {:else} Tümünü İndir {/if}
                      </button>
                  {/if}
              </div>
              
              {#each playerState.aramaSonuclari.slice(0, playerState.gosterilenSayi) as sonuc}
                  {@const isDownloading = playerState.aktifIndirmeler.has(sonuc.webpage_url)}
                  <div class="flex items-center gap-4 p-3 bg-(--bg-surface) border border-(--border) hover:border-(--accent)/30 rounded-xl group transition-all w-full min-w-0">
                      <div class="w-16 h-10 bg-black rounded-lg overflow-hidden shrink-0 relative">
                          <img src={sonuc.thumbnail} alt="" class="w-full h-full object-cover opacity-70 group-hover:opacity-100 transition-opacity" />
                      </div>
                      <div class="flex-1 min-w-0">
                          <p class="text-xs font-bold text-(--text-main) truncate group-hover:text-(--accent) transition-colors">{sonuc.title}</p>
                          <div class="flex items-center gap-2 mt-1">
                              <span class="text-[9px] font-black text-(--text-dim) uppercase truncate max-w-30">{sonuc.channel}</span>
                              <span class="w-1 h-1 bg-(--border) rounded-full"></span>
                              <span class="text-[9px] font-mono text-(--text-dim)">{sonuc.duration_string}</span>
                          </div>
                      </div>
                      
                      <button 
                          type="button"
                          onclick={() => ozellestirilmisIndir(sonuc.webpage_url)}
                          disabled={isDownloading || playerState.topluIndirmeAktif}
                          aria-label="{sonuc.title} indir"
                          class="p-3 text-(--text-dim) hover:text-white hover:bg-(--accent) rounded-lg transition-all disabled:opacity-50 shrink-0 {isDownloading ? 'text-(--accent) bg-(--accent)/10' : ''}"
                      >
                          {#if isDownloading}
                              <svg class="w-5 h-5 animate-spin" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
                          {:else}
                              <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true"><path stroke-linecap="round" stroke-linejoin="round" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path></svg>
                          {/if}
                      </button>
                  </div>
              {/each}

              {#if playerState.gosterilenSayi < playerState.aramaSonuclari.length}
                  <button 
                      type="button" 
                      onclick={() => playerState.gosterilenSayi += 5}
                      class="mt-4 py-3 w-full rounded-xl border border-dashed border-(--border) text-[10px] font-black uppercase tracking-widest text-(--text-dim) hover:text-(--accent) hover:border-(--accent)/50 hover:bg-(--accent)/5 transition-all"
                  >
                      Daha Fazla Göster ({playerState.aramaSonuclari.length - playerState.gosterilenSayi} Kaldı)
                  </button>
              {/if}
          </div>
      {/if}
  </section>

  <section class="mb-16 shrink-0">
    <h2 class="text-xs font-black text-(--text-dim) mb-6 uppercase tracking-[0.4em] flex items-center gap-4">FREKANS GRUPLARI <div class="h-px flex-1 bg-(--border) opacity-50"></div></h2>
    <div class="flex gap-4 overflow-x-auto pb-4 no-scrollbar">
      {#each kategoriler as kat, i}
        <a href="/search?q={kat.isim}" class="shrink-0 w-36 h-44 bg-(--bg-card) border border-(--border) rounded-(--radius) p-5 flex flex-col justify-between hover:bg-(--bg-card-hover) hover:border-(--accent)/50 transition-all group shadow-lg" in:scale={{ duration: 400, delay: i * 50 }}>
          <span class="text-4xl group-hover:scale-110 transition-transform text-(--text-dim) group-hover:text-(--text-main)">
             {@html tarzIkonlari[kat.isim] || '<svg width="1em" height="1em" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 18V5l12-2v13"/><circle cx="6" cy="18" r="3"/><circle cx="18" cy="16" r="3"/></svg>'}
          </span>
          <div class="min-w-0">
            <p class="font-black text-sm uppercase tracking-tight group-hover:text-(--accent) transition-colors truncate">{kat.isim}</p>
            <p class="text-[9px] font-bold text-(--text-dim) uppercase">{kat.adet} Parça</p>
          </div>
        </a>
      {/each}
    </div>
  </section>

  <div class="grid grid-cols-1 lg:grid-cols-12 gap-12">
    <div class="lg:col-span-5 flex flex-col min-w-0">
      <h2 class="text-xl font-black text-(--text-main) mb-8 flex items-center gap-4 uppercase italic tracking-tight"><span class="text-(--accent) text-3xl font-serif">#</span> Zirvedekiler</h2>
      <div class="flex flex-col gap-3">
        {#each enCokDinlenenler as sarki, index}
          <div 
            role="button" tabindex="0"
            onclick={() => sarkiCal(sarki)}
            onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(sarki)}
            aria-label="{sarki.isim} çal"
            class="flex items-center gap-4 p-4 rounded-2xl transition-all group cursor-pointer border {playerState.aktifSarki?.id === sarki.id ? 'bg-(--accent)/10 border-(--accent)/30' : 'bg-(--bg-card) border-(--border) hover:bg-(--bg-card-hover) hover:border-(--accent)/30'}"
            in:fly={{ x: -20, duration: 400, delay: index * 50 }}
          >
            <div class="w-6 text-center shrink-0">
               {#if playerState.aktifSarki?.id === sarki.id && playerState.suAnOynuyorMu}
                  <div class="flex items-end justify-center gap-0.5 h-3.5 mb-1">
                     <div class="w-1 bg-(--accent) animate-bounce"></div>
                     <div class="w-1 bg-(--accent) animate-[bounce_1.2s_infinite]"></div>
                     <div class="w-1 bg-(--accent) animate-[bounce_0.8s_infinite]"></div>
                  </div>
               {:else}
                  <span class="text-lg font-black text-(--text-dim)/30 group-hover:hidden italic">{index + 1}</span>
                  <svg class="w-4 h-4 mx-auto hidden group-hover:block text-(--accent)" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M8 5v14l11-7z"/></svg>
               {/if}
            </div>
            <div class="w-12 h-12 rounded-xl overflow-hidden shrink-0 border border-(--border) bg-(--bg-surface)">
              {#if sarki.kapak_yolu}
                <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover group-hover:scale-110 transition-transform" />
              {:else}
                <div class="w-full h-full flex items-center justify-center text-(--text-dim)/30 italic font-black text-xs">LW</div>
              {/if}
            </div>
            <div class="flex-1 min-w-0">
              <span class="font-bold text-(--text-main) truncate block text-sm group-hover:text-(--accent) transition-colors">{sarki.isim}</span>
              <button onclick={(e) => { e.stopPropagation(); }} class="text-[10px] text-(--text-dim) font-bold uppercase tracking-widest truncate block opacity-80 hover:text-(--accent) text-left w-full">
                {sarki.sarkici}
              </button>
            </div>
            
            <div class="shrink-0 flex items-center gap-1" onclick={(e) => e.stopPropagation()} role="presentation">
              <select aria-label="Listeye Ekle" onchange={(e) => handlePlaylistEkle(sarki.id, e)} class="bg-(--bg-surface) text-[9px] text-(--text-dim) rounded-lg px-1 py-1 outline-none border border-(--border) w-16 focus:border-(--accent) opacity-70 hover:opacity-100 hidden sm:block transition-all cursor-pointer font-bold appearance-none">
                <option value="">➕ LİSTE</option>
                {#each playerState.playlistler as pl}
                  {#if !pl.sarkilar.includes(sarki.id)}<option value={pl.id}>{pl.isim.toUpperCase()}</option>{/if}
                {/each}
              </select>
              
              <button type="button" aria-label="Düzenle" title="Bilgileri Düzenle" onclick={(e) => editModaliAc(sarki, e)} class="text-(--text-dim)/50 hover:text-(--accent) hover:bg-(--accent)/10 p-1.5 rounded-lg transition-all hidden sm:block">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true"><path d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg>
              </button>
              
              <FavoriteButton sarkiId={sarki.id} />
              
              <button type="button" aria-label="Sil" title="Sil" onclick={(e) => handleSarkiSil(sarki, e)} class="text-(--text-dim)/50 hover:text-red-500 hover:bg-red-500/10 p-1.5 rounded-lg transition-all">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true"><path d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg>
              </button>
            </div>
          </div>
        {/each}
      </div>
    </div>

    <div class="lg:col-span-7 flex flex-col min-w-0">
      <h2 class="text-xl font-black text-(--text-main) mb-8 flex items-center gap-4 uppercase italic tracking-tight"><span class="text-(--accent-sec) text-3xl font-serif">/</span> Son Eklenenler</h2>
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        {#each yeniEklenenler as sarki, i}
          <div 
            role="button" tabindex="0"
            onclick={() => sarkiCal(sarki)}
            onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && sarkiCal(sarki)}
            aria-label="{sarki.isim} çal"
            class="flex items-center gap-3 p-3 rounded-2xl transition-all cursor-pointer group border {playerState.aktifSarki?.id === sarki.id ? 'bg-(--accent)/10 border-(--accent)/30' : 'bg-(--bg-card) border-(--border) hover:bg-(--bg-card-hover) hover:border-(--accent-sec)/30'}"
            in:fly={{ y: 10, duration: 400, delay: i * 30 }}
          >
            <div class="w-10 h-10 rounded-xl overflow-hidden shrink-0 relative border border-(--border)">
               {#if sarki.kapak_yolu}
                  <img src={convertFileSrc(sarki.kapak_yolu)} alt="" class="w-full h-full object-cover group-hover:scale-110 transition-transform" />
               {:else}
                  <div class="w-full h-full bg-(--bg-surface) flex items-center justify-center text-(--text-dim)/20 text-xs">🎵</div>
               {/if}
               <div class="absolute inset-0 bg-(--accent)/10 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center"><svg class="w-4 h-4 text-white" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true"><path d="M8 5v14l11-7z"/></svg></div>
            </div>
            <div class="flex-1 min-w-0">
              <span class="text-xs font-bold text-(--text-main) truncate block leading-tight group-hover:text-(--accent) transition-colors">{sarki.isim}</span>
              <span class="text-[9px] text-(--text-dim) font-bold truncate uppercase tracking-widest opacity-80 block">{sarki.sarkici}</span>
            </div>
            
            <div onclick={(e) => e.stopPropagation()} role="presentation" class="shrink-0 flex items-center gap-1">
               <button type="button" aria-label="Düzenle" title="Düzenle" onclick={(e) => editModaliAc(sarki, e)} class="text-(--text-dim)/50 hover:text-(--accent) hover:bg-(--accent)/10 p-1.5 rounded-lg transition-all">
                  <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true"><path d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path></svg>
               </button>
               
               <FavoriteButton sarkiId={sarki.id} />
               
               <button type="button" aria-label="Sil" title="Sil" onclick={(e) => handleSarkiSil(sarki, e)} class="text-(--text-dim)/50 hover:text-red-500 hover:bg-red-500/10 p-1.5 rounded-lg transition-all">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true"><path d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg>
               </button>
            </div>
          </div>
        {/each}
      </div>

      <div class="mt-8 p-6 rounded-(--radius) bg-linear-to-br from-(--bg-card) to-(--bg-surface) border border-(--border) flex items-center justify-between shadow-xl shrink-0" in:fade={{ delay: 500 }}>
        <div class="flex flex-col">
          <span class="text-[10px] font-black text-(--text-dim) uppercase tracking-[0.3em] mb-2">Toplam Veri Akışı</span>
          <div class="flex items-end gap-2 leading-none">
             <span class="text-4xl font-black text-(--text-main) italic tracking-tighter">{playerState.sarkiListesi.length}</span>
             <span class="text-[10px] font-bold text-(--accent) uppercase tracking-widest mb-1">Dizin</span>
          </div>
        </div>
        <div class="w-12 h-12 rounded-full border border-(--border) flex items-center justify-center text-(--accent) bg-(--bg-surface) shadow-inner">
          <svg class="w-5 h-5 animate-pulse" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" aria-hidden="true"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M17 8l-5-5-5 5M12 3v12"></path></svg>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  @keyframes bounce { 0%, 100% { height: 4px; } 50% { height: 14px; } }
  .no-scrollbar::-webkit-scrollbar { display: none; }
  .no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
  .custom-scrollbar::-webkit-scrollbar { width: 4px; }
  .custom-scrollbar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: var(--accent); }
  
  select {
      background-image: none; /* Select ok işaretini gizle */
  }
</style>