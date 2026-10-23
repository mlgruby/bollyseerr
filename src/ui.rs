pub const INDEX_HTML: &str = r##"<!DOCTYPE html>
<html lang="en" class="dark">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>BollySeerr — Hindi & Bollywood Discovery</title>
  <script src="https://cdn.tailwindcss.com"></script>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&display=swap" rel="stylesheet">
  <script>
    tailwind.config = {
      darkMode: 'class',
      theme: {
        extend: {
          fontFamily: { sans: ['Inter', 'sans-serif'] },
          colors: {
            seerr: {
              bg: '#111827',
              card: '#1f2937',
              border: '#374151',
              accent: '#6366f1',
              accentHover: '#4f46e5'
            }
          }
        }
      }
    };
  </script>
  <style>
    body { background-color: #0b0f19; color: #f3f4f6; }
    .poster-card { transition: transform 0.2s cubic-bezier(0.16, 1, 0.3, 1), box-shadow 0.2s ease; }
    .poster-card:hover { transform: translateY(-4px); box-shadow: 0 14px 30px -5px rgba(0, 0, 0, 0.7), 0 0 15px rgba(99, 102, 241, 0.25); }
    ::-webkit-scrollbar { width: 8px; height: 8px; }
    ::-webkit-scrollbar-track { background: #0b0f19; }
    ::-webkit-scrollbar-thumb { background: #374151; border-radius: 4px; }
    ::-webkit-scrollbar-thumb:hover { background: #4b5563; }
  </style>
</head>
<body class="min-h-screen flex flex-col selection:bg-indigo-500 selection:text-white">
  <!-- Sticky Top Navigation -->
  <header class="sticky top-0 z-40 backdrop-blur-xl bg-gray-950/85 border-b border-gray-800/80">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-3.5 flex flex-wrap items-center justify-between gap-4">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 flex items-center justify-center shadow-lg shadow-indigo-500/20 font-extrabold text-xl tracking-tight text-white">
          B
        </div>
        <div>
          <div class="flex items-center gap-2">
            <h1 class="text-lg sm:text-xl font-extrabold tracking-tight bg-gradient-to-r from-white via-indigo-200 to-purple-300 bg-clip-text text-transparent">
              BollySeerr
            </h1>
            <span class="text-[11px] font-semibold uppercase tracking-wider px-2 py-0.5 rounded-full bg-indigo-500/15 text-indigo-300 border border-indigo-500/30">
              5-Month Rolling Agent
            </span>
          </div>
          <p class="text-xs text-gray-400 hidden sm:block">
            Hindi & Pan-India Movies and OTT Web Series • Auto-synced with Seerr, Radarr & Sonarr
          </p>
        </div>
      </div>

      <!-- Search & Sync Controls -->
      <div class="flex items-center gap-2.5 flex-1 sm:flex-initial justify-end">
        <div class="relative w-full sm:w-64">
          <input
            id="searchInput"
            type="search"
            placeholder="Search movies, series, OTT..."
            class="w-full bg-gray-900/90 border border-gray-700/80 focus:border-indigo-500 rounded-xl pl-9 pr-3 py-1.5 text-sm text-gray-100 placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500/30"
          />
          <svg class="w-4 h-4 text-gray-500 absolute left-3 top-2.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
          </svg>
        </div>

        <button
          id="syncBtn"
          onclick="triggerSync()"
          class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-semibold bg-gray-800 hover:bg-gray-700 text-gray-200 border border-gray-700 transition cursor-pointer shrink-0"
          title="Trigger TMDB Discovery & Homelab Status Sync"
        >
          <svg id="syncIcon" class="w-3.5 h-3.5 text-indigo-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
          </svg>
          <span id="syncBtnText">Sync Now</span>
        </button>
      </div>
    </div>

    <!-- Filter Bar -->
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pb-3 flex flex-wrap items-center justify-between gap-3 border-t border-gray-900/80 pt-2.5">
      <!-- Window Tabs -->
      <div class="flex flex-wrap items-center gap-1.5 bg-gray-900/90 p-1 rounded-xl border border-gray-800">
        <button onclick="setWindowFilter('released')" id="tab-released" class="px-3 py-1.5 rounded-lg text-xs font-semibold transition">
          Released (Last 5M) <span id="count-released" class="ml-1 px-1.5 py-0.2 rounded-full text-[10px] bg-gray-800 text-gray-300">0</span>
        </button>
        <button onclick="setWindowFilter('in_theaters')" id="tab-in_theaters" class="px-3 py-1.5 rounded-lg text-xs font-semibold transition">
          🎬 In Theaters <span id="count-in_theaters" class="ml-1 px-1.5 py-0.2 rounded-full text-[10px] bg-gray-800 text-gray-300">0</span>
        </button>
        <button onclick="setWindowFilter('on_ott')" id="tab-on_ott" class="px-3 py-1.5 rounded-lg text-xs font-semibold transition">
          📺 Now on OTT <span id="count-on_ott" class="ml-1 px-1.5 py-0.2 rounded-full text-[10px] bg-gray-800 text-gray-300">0</span>
        </button>
        <button onclick="setWindowFilter('upcoming')" id="tab-upcoming" class="px-3 py-1.5 rounded-lg text-xs font-semibold transition">
          Upcoming <span id="count-upcoming" class="ml-1 px-1.5 py-0.2 rounded-full text-[10px] bg-gray-800 text-gray-300">0</span>
        </button>
        <button onclick="setWindowFilter('library')" id="tab-library" class="px-3 py-1.5 rounded-lg text-xs font-semibold transition">
          Requested / Library <span id="count-library" class="ml-1 px-1.5 py-0.2 rounded-full text-[10px] bg-gray-800 text-gray-300">0</span>
        </button>
        <button onclick="setWindowFilter('all')" id="tab-all" class="px-3 py-1.5 rounded-lg text-xs font-semibold transition">
          All
        </button>
      </div>

      <!-- Media Type & Language Filters -->
      <div class="flex flex-wrap items-center gap-2">
        <div class="flex items-center gap-1 bg-gray-900/90 p-1 rounded-xl border border-gray-800">
          <button onclick="setTypeFilter('all')" id="type-all" class="px-2.5 py-1 rounded-lg text-xs font-medium transition">All Types</button>
          <button onclick="setTypeFilter('movie')" id="type-movie" class="px-2.5 py-1 rounded-lg text-xs font-medium transition">Movies</button>
          <button onclick="setTypeFilter('tv')" id="type-tv" class="px-2.5 py-1 rounded-lg text-xs font-medium transition">Web Series</button>
        </div>

        <select id="langFilter" onchange="renderGrid()" class="bg-gray-900 border border-gray-800 rounded-xl px-2.5 py-1.5 text-xs text-gray-300 focus:outline-none focus:border-indigo-500">
          <option value="all">Hindi + Pan-India Dub</option>
          <option value="hi">Original Hindi Only</option>
          <option value="dub">Pan-India (Hindi Dub)</option>
        </select>

        <select id="ottFilter" onchange="renderGrid()" class="bg-gray-900 border border-gray-800 rounded-xl px-2.5 py-1.5 text-xs text-gray-300 focus:outline-none focus:border-indigo-500">
          <option value="all">All Platforms / Theatrical</option>
          <option value="Netflix">Netflix</option>
          <option value="Prime Video">Prime Video</option>
          <option value="JioHotstar">JioHotstar</option>
          <option value="SonyLIV">SonyLIV</option>
          <option value="ZEE5">ZEE5</option>
        </select>
      </div>
    </div>
  </header>

  <!-- Main Content -->
  <main class="flex-1 max-w-7xl w-full mx-auto px-4 sm:px-6 lg:px-8 py-6">
    <!-- Status Banner -->
    <div id="statusBar" class="mb-5 flex flex-wrap items-center justify-between gap-2 text-xs text-gray-400 bg-gray-900/60 border border-gray-800/80 rounded-xl px-4 py-2.5">
      <div class="flex items-center gap-2">
        <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
        <span id="statusSummary">Loading Bollywood & Hindi catalog...</span>
      </div>
      <div id="syncTimestamps" class="text-[11px] text-gray-500 font-mono"></div>
    </div>

    <!-- Grid -->
    <div id="mediaGrid" class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4 sm:gap-5"></div>

    <!-- Empty State -->
    <div id="emptyState" class="hidden py-20 text-center">
      <p class="text-gray-400 text-base font-medium">No matching Hindi movies or web series found.</p>
      <p class="text-gray-600 text-xs mt-1">Try adjusting your filters or click "Sync Now" to refresh from TMDB.</p>
    </div>
  </main>

  <!-- Details & Trailer Modal -->
  <div id="detailModal" class="fixed inset-0 z-50 hidden bg-black/80 backdrop-blur-sm flex items-center justify-center p-4" onclick="if(event.target === this) closeModal()">
    <div class="bg-gray-900 border border-gray-800 rounded-2xl max-w-2xl w-full overflow-hidden shadow-2xl">
      <div id="modalContent"></div>
    </div>
  </div>

  <!-- Toast Notification -->
  <div id="toast" class="fixed bottom-5 right-5 z-50 hidden px-4 py-3 rounded-xl shadow-xl border text-sm font-medium transition-all"></div>

  <script>
    let allItems = [];
    let syncMeta = {};
    let currentWindow = 'released';
    let currentType = 'all';
    const todayStr = new Date().toISOString().slice(0, 10);

    document.getElementById('searchInput').addEventListener('input', () => renderGrid());

    function showToast(msg, isError = false) {
      const t = document.getElementById('toast');
      t.textContent = msg;
      t.className = `fixed bottom-5 right-5 z-50 px-4 py-3 rounded-xl shadow-xl border text-sm font-medium ${
        isError
          ? 'bg-red-950/95 border-red-700 text-red-200'
          : 'bg-indigo-950/95 border-indigo-600 text-indigo-100'
      }`;
      t.classList.remove('hidden');
      setTimeout(() => t.classList.add('hidden'), 4000);
    }

    function isItemOnOtt(item) {
      return (item.ott_providers && item.ott_providers.length > 0) ||
             (item.release_type && item.release_type.includes('OTT')) ||
             item.media_type === 'tv';
    }

    function setWindowFilter(w) {
      currentWindow = w;
      ['released', 'in_theaters', 'on_ott', 'upcoming', 'library', 'all'].forEach(k => {
        const el = document.getElementById(`tab-${k}`);
        if (!el) return;
        if (k === w) {
          el.className = 'px-3 py-1.5 rounded-lg text-xs font-semibold bg-indigo-600 text-white shadow transition';
        } else {
          el.className = 'px-3 py-1.5 rounded-lg text-xs font-semibold text-gray-400 hover:text-gray-200 transition';
        }
      });
      renderGrid();
    }

    function setTypeFilter(t) {
      currentType = t;
      ['all', 'movie', 'tv'].forEach(k => {
        const el = document.getElementById(`type-${k}`);
        if (k === t) {
          el.className = 'px-2.5 py-1 rounded-lg text-xs font-semibold bg-gray-800 text-white border border-gray-700 transition';
        } else {
          el.className = 'px-2.5 py-1 rounded-lg text-xs font-medium text-gray-400 hover:text-gray-200 transition';
        }
      });
      renderGrid();
    }

    async function fetchCatalog() {
      try {
        const [itemsResp, statusResp] = await Promise.all([
          fetch('/api/items'),
          fetch('/api/status')
        ]);
        allItems = await itemsResp.json();
        syncMeta = await statusResp.json();
        updateHeaderCounts();
        renderGrid();
      } catch (e) {
        console.error('Failed to load catalog:', e);
      }
    }

    function updateHeaderCounts() {
      const releasedItems = allItems.filter(i => i.release_date <= todayStr);
      const inTheatersCount = releasedItems.filter(i => i.media_type === 'movie' && !isItemOnOtt(i)).length;
      const onOttCount = releasedItems.filter(i => isItemOnOtt(i)).length;

      document.getElementById('count-released').textContent = syncMeta.released_count ?? releasedItems.length;
      document.getElementById('count-in_theaters').textContent = inTheatersCount;
      document.getElementById('count-on_ott').textContent = onOttCount;
      document.getElementById('count-upcoming').textContent = syncMeta.upcoming_count ?? 0;
      document.getElementById('count-library').textContent = syncMeta.library_or_requested_count ?? 0;

      const summary = `${syncMeta.total_movies || 0} Movies • ${syncMeta.total_series || 0} Web Series (Rolling 5-Month / 150-Day Window + Upcoming • Auto-pruned: ${syncMeta.pruned_last_run || 0})`;
      document.getElementById('statusSummary').textContent = summary;

      const lastDisc = syncMeta.last_discovery_sync ? new Date(syncMeta.last_discovery_sync).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'}) : 'running...';
      const lastStat = syncMeta.last_status_sync ? new Date(syncMeta.last_status_sync).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'}) : 'running...';
      document.getElementById('syncTimestamps').textContent = `TMDB Sync: ${lastDisc} | Homelab Sync: ${lastStat}`;

      const btnText = document.getElementById('syncBtnText');
      const syncIcon = document.getElementById('syncIcon');
      if (syncMeta.is_syncing) {
        btnText.textContent = 'Syncing...';
        syncIcon.classList.add('animate-spin');
      } else {
        btnText.textContent = 'Sync Now';
        syncIcon.classList.remove('animate-spin');
      }
    }

    function daysDifference(dateStr) {
      const target = new Date(dateStr + 'T00:00:00Z');
      const now = new Date(todayStr + 'T00:00:00Z');
      return Math.round((target - now) / (1000 * 60 * 60 * 24));
    }

    function statusBadgeHtml(item) {
      const st = item.homelab_status;
      if (st === 'available') {
        return `<span class="px-2 py-0.5 rounded-md text-[10px] font-bold uppercase tracking-wide bg-emerald-500/90 text-gray-950 shadow">Available</span>`;
      }
      if (st === 'partially_available') {
        return `<span class="px-2 py-0.5 rounded-md text-[10px] font-bold uppercase tracking-wide bg-teal-500/90 text-gray-950 shadow">Partial</span>`;
      }
      if (st === 'monitored') {
        return `<span class="px-2 py-0.5 rounded-md text-[10px] font-bold uppercase tracking-wide bg-amber-500/90 text-gray-950 shadow">Monitored</span>`;
      }
      if (st === 'requested') {
        return `<span class="px-2 py-0.5 rounded-md text-[10px] font-bold uppercase tracking-wide bg-sky-500/90 text-gray-950 shadow">Requested</span>`;
      }
      return '';
    }

    function actionButtonHtml(item) {
      const st = item.homelab_status;
      if (st === 'available') {
        return `<button disabled class="w-full py-1.5 px-3 rounded-lg text-xs font-semibold bg-emerald-500/20 text-emerald-300 border border-emerald-500/30 cursor-default">✓ In Library</button>`;
      }
      if (st === 'partially_available') {
        return `<button onclick="event.stopPropagation(); requestItem(${item.tmdb_id}, '${item.media_type}')" class="w-full py-1.5 px-3 rounded-lg text-xs font-semibold bg-teal-500/20 hover:bg-teal-500/30 text-teal-200 border border-teal-500/30 transition cursor-pointer">Request Remaining</button>`;
      }
      if (st === 'monitored' || st === 'requested') {
        return `<button disabled class="w-full py-1.5 px-3 rounded-lg text-xs font-semibold bg-amber-500/20 text-amber-300 border border-amber-500/30 cursor-default">⏳ Requested</button>`;
      }
      return `<button id="req-btn-${item.media_type}-${item.tmdb_id}" onclick="event.stopPropagation(); requestItem(${item.tmdb_id}, '${item.media_type}')" class="w-full py-1.5 px-3 rounded-lg text-xs font-semibold bg-indigo-600 hover:bg-indigo-500 text-white shadow-sm shadow-indigo-500/30 transition cursor-pointer flex items-center justify-center gap-1">
        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M12 4v16m8-8H4"/></svg>
        Request
      </button>`;
    }

    function levenshteinDistance(s1, s2) {
      const m = s1.length, n = s2.length;
      if (Math.abs(m - n) > 2) return 99;
      const dp = Array.from({ length: m + 1 }, () => new Array(n + 1).fill(0));
      for (let i = 0; i <= m; i++) dp[i][0] = i;
      for (let j = 0; j <= n; j++) dp[0][j] = j;
      for (let i = 1; i <= m; i++) {
        for (let j = 1; j <= n; j++) {
          const cost = s1[i - 1] === s2[j - 1] ? 0 : 1;
          dp[i][j] = Math.min(
            dp[i - 1][j] + 1,
            dp[i][j - 1] + 1,
            dp[i - 1][j - 1] + cost
          );
        }
      }
      return dp[m][n];
    }

    function fuzzyMatch(haystack, needle) {
      if (!needle) return true;
      if (haystack.includes(needle)) return true;
      const words = haystack.split(/[\s:,\-_]+/);
      for (const w of words) {
        if (w.length >= 4 && Math.abs(w.length - needle.length) <= 2) {
          if (levenshteinDistance(w, needle) <= 2) return true;
        }
      }
      return false;
    }

    function getFallbackArtHtml(title, mediaType) {
      const isTv = mediaType === 'tv';
      const cleanTitle = (title || 'Untitled').replace(/"/g, '&quot;');
      return `<div class="w-full h-full flex flex-col items-center justify-center p-4 bg-gradient-to-br from-indigo-950/90 via-slate-900 to-gray-950 text-center relative overflow-hidden group-hover:scale-105 transition duration-300">
        <div class="absolute inset-0 opacity-15 bg-[radial-gradient(#818cf8_1px,transparent_1px)] [background-size:12px_12px]"></div>
        <div class="w-12 h-12 rounded-2xl bg-indigo-500/10 border border-indigo-400/20 flex items-center justify-center text-indigo-400 mb-3 shadow-lg shadow-indigo-950/50">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.8" d="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18M3 16h4m10 0h4M4 20h16a1 1 0 001-1V5a1 1 0 00-1-1H4a1 1 0 00-1 1v14a1 1 0 001 1z"/></svg>
        </div>
        <span class="text-xs font-bold tracking-wider text-indigo-100 uppercase line-clamp-3 leading-snug drop-shadow px-1">${cleanTitle}</span>
        <span class="text-[9px] font-semibold text-indigo-400/80 uppercase tracking-widest mt-2 px-2 py-0.5 rounded-full bg-indigo-950/80 border border-indigo-800/40">${isTv ? 'WEB SERIES' : 'CINEMA'}</span>
      </div>`;
    }

    function renderGrid() {
      const q = document.getElementById('searchInput').value.trim().toLowerCase();
      const langMode = document.getElementById('langFilter').value;
      const ottMode = document.getElementById('ottFilter').value;

      let filtered = allItems.filter(item => {
        // When searching with a query, search globally across all tabs
        if (!q) {
          if (currentWindow === 'released' && item.release_date > todayStr) return false;
          if (currentWindow === 'in_theaters' && (item.release_date > todayStr || item.media_type !== 'movie' || isItemOnOtt(item))) return false;
          if (currentWindow === 'on_ott' && (item.release_date > todayStr || !isItemOnOtt(item))) return false;
          if (currentWindow === 'upcoming' && item.release_date <= todayStr) return false;
          if (currentWindow === 'library' && item.homelab_status === 'unrequested') return false;
        }

        if (currentType !== 'all' && item.media_type !== currentType) return false;
        if (langMode === 'hi' && item.is_hindi_dub) return false;
        if (langMode === 'dub' && !item.is_hindi_dub) return false;
        if (ottMode !== 'all' && !(item.ott_providers || []).includes(ottMode)) return false;

        if (q) {
          const hay = `${item.title} ${item.original_title} ${(item.genres || []).join(' ')} ${(item.ott_providers || []).join(' ')}`.toLowerCase();
          if (!fuzzyMatch(hay, q)) return false;
        }
        return true;
      });

      if (q) {
        // Sort search results: exact title matches first, then by popularity
        filtered.sort((a, b) => {
          const aExact = a.title.toLowerCase().includes(q) ? 1 : 0;
          const bExact = b.title.toLowerCase().includes(q) ? 1 : 0;
          if (aExact !== bExact) return bExact - aExact;
          return b.popularity - a.popularity;
        });
      } else if (currentWindow === 'upcoming') {
        filtered.sort((a, b) => a.release_date.localeCompare(b.release_date));
      } else {
        filtered.sort((a, b) => b.release_date.localeCompare(a.release_date));
      }

      const grid = document.getElementById('mediaGrid');
      const empty = document.getElementById('emptyState');

      if (filtered.length === 0) {
        grid.innerHTML = '';
        empty.classList.remove('hidden');
        return;
      }
      empty.classList.add('hidden');

      grid.innerHTML = filtered.map(item => {
        const posterPath = item.poster_path || item.backdrop_path;
        const posterArtHtml = posterPath
          ? `<img src="https://image.tmdb.org/t/p/w500${posterPath}" alt="${item.title.replace(/"/g, '&quot;')}" loading="lazy" class="w-full h-full object-cover group-hover:scale-105 transition duration-300" onerror="this.outerHTML = getFallbackArtHtml('${item.title.replace(/'/g, "\\'")}', '${item.media_type}')" />`
          : getFallbackArtHtml(item.title, item.media_type);

        const diff = daysDifference(item.release_date);
        const isUpcoming = item.release_date > todayStr;
        let windowBadge = '';
        if (diff > 0) {
          windowBadge = `⏳ In ${diff}d`;
        } else {
          const daysLeft = Math.max(0, 150 + diff);
          windowBadge = `${Math.abs(diff)}d ago • ${daysLeft}d left`;
        }

        const langTag = item.is_hindi_dub
          ? `<span class="px-1.5 py-0.5 rounded text-[10px] font-semibold bg-purple-950/90 text-purple-200 border border-purple-700/50">Pan-India (${item.original_language.toUpperCase()})</span>`
          : `<span class="px-1.5 py-0.5 rounded text-[10px] font-semibold bg-gray-900/90 text-gray-200 border border-gray-700/60">Hindi</span>`;

        const typeBadge = item.media_type === 'tv'
          ? `<span class="px-1.5 py-0.5 rounded text-[10px] font-bold bg-pink-600/90 text-white">SERIES</span>`
          : `<span class="px-1.5 py-0.5 rounded text-[10px] font-bold bg-indigo-600/90 text-white">MOVIE</span>`;

        const ottPills = (item.ott_providers || []).slice(0, 2).map(p =>
          `<span class="px-1.5 py-0.5 rounded text-[10px] font-medium bg-indigo-950/90 text-indigo-300 border border-indigo-700/60">📺 ${p}</span>`
        ).join(' ');

        let releaseModeTag = '';
        if (isUpcoming) {
          releaseModeTag = isItemOnOtt(item)
            ? `<span class="px-1.5 py-0.5 rounded text-[10px] font-semibold bg-emerald-950/90 text-emerald-300 border border-emerald-700/50">📺 Upcoming OTT</span>`
            : `<span class="px-1.5 py-0.5 rounded text-[10px] font-semibold bg-amber-950/90 text-amber-300 border border-amber-700/50">🎬 Theatrical Release</span>`;
        } else {
          releaseModeTag = isItemOnOtt(item)
            ? `<span class="px-1.5 py-0.5 rounded text-[10px] font-semibold bg-emerald-950/90 text-emerald-300 border border-emerald-700/50">${item.release_type}</span>`
            : `<span class="px-1.5 py-0.5 rounded text-[10px] font-semibold bg-amber-950/90 text-amber-300 border border-amber-700/50">🎬 ${item.release_type}</span>`;
        }

        return `
          <div onclick="openModal(${item.tmdb_id}, '${item.media_type}')" class="poster-card group bg-gray-900/90 border border-gray-800/90 rounded-2xl overflow-hidden flex flex-col cursor-pointer">
            <div class="relative aspect-[2/3] bg-gray-950 overflow-hidden">
              ${posterArtHtml}
              <div class="absolute inset-0 bg-gradient-to-t from-gray-950 via-transparent to-black/40"></div>
              <div class="absolute top-2 left-2 flex flex-wrap gap-1">
                ${typeBadge}
                ${langTag}
              </div>
              <div class="absolute top-2 right-2">
                ${statusBadgeHtml(item)}
              </div>
              <div class="absolute bottom-2 left-2 right-2 flex items-center justify-between text-[11px] text-gray-200 font-medium">
                <span class="bg-black/65 backdrop-blur-sm px-2 py-0.5 rounded-md border border-white/10">${item.release_date}</span>
                ${item.vote_average > 0 ? `<span class="bg-black/65 backdrop-blur-sm px-1.5 py-0.5 rounded-md text-amber-400 font-semibold">★ ${item.vote_average.toFixed(1)}</span>` : ''}
              </div>
            </div>

            <div class="p-3 flex-1 flex flex-col justify-between gap-2">
              <div>
                <h3 class="font-bold text-sm text-gray-100 line-clamp-1 group-hover:text-indigo-300 transition" title="${item.title.replace(/"/g, '&quot;')}">
                  ${item.title}
                </h3>
                <p class="text-[11px] text-gray-400 mt-0.5">${windowBadge}</p>
                <div class="flex flex-wrap gap-1 mt-1.5">
                  ${releaseModeTag}
                  ${ottPills}
                </div>
              </div>

              <div class="pt-1">
                ${actionButtonHtml(item)}
              </div>
            </div>
          </div>
        `;
      }).join('');
    }

    async function requestItem(tmdbId, mediaType) {
      const btn = document.getElementById(`req-btn-${mediaType}-${tmdbId}`);
      if (btn) {
        btn.disabled = true;
        btn.textContent = 'Requesting...';
      }

      try {
        const resp = await fetch('/api/request', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ tmdb_id: tmdbId, media_type: mediaType, seasons: 'all' })
        });
        const data = await resp.json();
        if (!resp.ok || !data.success) {
          throw new Error(data.message || 'Request failed');
        }
        showToast(`✓ ${data.message}`);
        await fetchCatalog();
      } catch (err) {
        showToast(`✗ ${err.message}`, true);
        if (btn) {
          btn.disabled = false;
          btn.textContent = 'Request';
        }
      }
    }

    async function triggerSync() {
      try {
        showToast('Triggered TMDB discovery & Homelab status sync...');
        await fetch('/api/sync', { method: 'POST' });
        setTimeout(fetchCatalog, 1500);
      } catch (e) {
        showToast('Failed to trigger sync', true);
      }
    }

    function openModal(tmdbId, mediaType) {
      const item = allItems.find(i => i.tmdb_id === tmdbId && i.media_type === mediaType);
      if (!item) return;

      const modal = document.getElementById('detailModal');
      const content = document.getElementById('modalContent');
      const backdrop = item.backdrop_path
        ? `https://image.tmdb.org/t/p/w780${item.backdrop_path}`
        : (item.poster_path ? `https://image.tmdb.org/t/p/w500${item.poster_path}` : '');

      const trailerBtn = item.trailer_key
        ? `<a href="https://www.youtube.com/watch?v=${item.trailer_key}" target="_blank" rel="noopener" class="px-3.5 py-2 rounded-xl text-xs font-semibold bg-red-600/90 hover:bg-red-500 text-white transition inline-flex items-center gap-1.5">▶ Watch Trailer</a>`
        : '';

      const tmdbUrl = `https://www.themoviedb.org/${item.media_type}/${item.tmdb_id}`;

      content.innerHTML = `
        <div class="relative h-48 sm:h-64 bg-gray-800">
          ${backdrop ? `<img src="${backdrop}" class="w-full h-full object-cover opacity-60" />` : ''}
          <div class="absolute inset-0 bg-gradient-to-t from-gray-900 via-gray-900/50 to-transparent"></div>
          <button onclick="closeModal()" class="absolute top-3 right-3 w-8 h-8 rounded-full bg-black/60 text-gray-200 hover:bg-black flex items-center justify-center">✕</button>
          <div class="absolute bottom-4 left-5 right-5">
            <div class="flex items-center gap-2 mb-1">
              <span class="px-2 py-0.5 rounded text-[10px] font-bold uppercase bg-indigo-600 text-white">${item.release_type}</span>
              ${statusBadgeHtml(item)}
            </div>
            <h2 class="text-xl sm:text-2xl font-extrabold text-white">${item.title}</h2>
            <p class="text-xs text-gray-300 mt-0.5">Release Date: ${item.release_date} • Original Lang: ${item.original_language.toUpperCase()}</p>
          </div>
        </div>
        <div class="p-5 space-y-4">
          <div class="flex flex-wrap gap-1.5">
            ${(item.genres || []).map(g => `<span class="px-2.5 py-0.5 rounded-full text-xs bg-gray-800 text-gray-300 border border-gray-700">${g}</span>`).join('')}
            ${(item.ott_providers || []).map(p => `<span class="px-2.5 py-0.5 rounded-full text-xs bg-indigo-950 text-indigo-300 border border-indigo-700/50">${p}</span>`).join('')}
          </div>
          <p class="text-sm text-gray-300 leading-relaxed">${item.overview || 'No synopsis available yet on TMDB.'}</p>
          <div class="flex flex-wrap items-center justify-between gap-3 pt-2 border-t border-gray-800">
            <div class="flex items-center gap-2">
              ${trailerBtn}
              <a href="${tmdbUrl}" target="_blank" rel="noopener" class="px-3 py-2 rounded-xl text-xs font-semibold bg-gray-800 hover:bg-gray-700 text-gray-300 border border-gray-700">TMDB ↗</a>
            </div>
            <div class="w-44" onclick="closeModal()">
              ${actionButtonHtml(item)}
            </div>
          </div>
        </div>
      `;
      modal.classList.remove('hidden');
    }

    function closeModal() {
      document.getElementById('detailModal').classList.add('hidden');
    }

    setWindowFilter('released');
    setTypeFilter('all');
    fetchCatalog();
    setInterval(fetchCatalog, 15000);
  </script>
</body>
</html>
"##;
