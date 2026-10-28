pub const INDEX_HTML: &str = concat!(
    r##"<!DOCTYPE html>
<html lang="en" class="dark">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>BollySeerr — Hindi & Bollywood Discovery</title>
  <link rel="icon" type="image/svg+xml" href="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 512 512'%3E%3Crect width='512' height='512' rx='128' fill='%230f172a'/%3E%3Cpath d='M116 130 L136 130 L106 176 L86 176 Z' fill='%23FF9933'/%3E%3Cpath d='M166 130 L186 130 L156 176 L136 176 Z' fill='%23FFFFFF'/%3E%3Cpath d='M216 130 L236 130 L206 176 L186 176 Z' fill='%2310B981'/%3E%3Cpolygon points='256,220 268,255 305,255 275,277 286,312 256,290 226,312 237,277 207,255 244,255' fill='%23F59E0B' stroke='%23FEF08A' stroke-width='4'/%3E%3C/svg%3E">
  <style>"##,
    include_str!("../assets/tailwind.css"),
    r##"</style>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800;900&display=swap" rel="stylesheet">
  <style>
    body { background-color: #080c14; color: #f3f4f6; }
    .poster-card { transition: transform 0.25s cubic-bezier(0.16, 1, 0.3, 1), box-shadow 0.25s ease, border-color 0.25s ease; }
    .poster-card:hover { transform: translateY(-6px); box-shadow: 0 16px 36px -6px rgba(0, 0, 0, 0.8), 0 0 20px rgba(99, 102, 241, 0.25); border-color: rgba(99, 102, 241, 0.5); }
    ::-webkit-scrollbar { width: 8px; height: 8px; }
    ::-webkit-scrollbar-track { background: #080c14; }
    ::-webkit-scrollbar-thumb { background: #1f2937; border-radius: 4px; }
    ::-webkit-scrollbar-thumb:hover { background: #374151; }
  </style>
</head>
<body class="min-h-screen flex flex-col selection:bg-indigo-500 selection:text-white">
  <!-- Sticky Top Navigation -->
  <header class="sticky top-0 z-40 backdrop-blur-xl bg-gray-950/85 border-b border-gray-800/80">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-3.5 flex flex-wrap items-center justify-between gap-4">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-amber-500/20 via-orange-500/10 to-indigo-950/80 border border-amber-500/30 flex items-center justify-center shadow-lg shadow-amber-500/10 p-1.5 overflow-hidden">
          <svg width="24" height="24" viewBox="0 0 512 512" class="w-full h-full drop-shadow">
            <g transform="translate(256, 260) rotate(-6)">
              <rect x="-140" y="-75" width="280" height="190" rx="18" fill="#18181b" stroke="#3f3f46" stroke-width="4"/>
              <g transform="translate(-140, -130) rotate(-14, 0, 45)">
                <rect x="0" y="0" width="280" height="46" rx="8" fill="#09090b" stroke="#F59E0B" stroke-width="3"/>
                <path d="M35 0 L55 0 L25 46 L5 46 Z" fill="#FF9933"/>
                <path d="M85 0 L105 0 L75 46 L55 46 Z" fill="#FFFFFF"/>
                <path d="M135 0 L155 0 L125 46 L105 46 Z" fill="#10B981"/>
                <path d="M185 0 L205 0 L175 46 L155 46 Z" fill="#FF9933"/>
                <path d="M235 0 L255 0 L225 46 L205 46 Z" fill="#FFFFFF"/>
              </g>
              <rect x="-140" y="-75" width="280" height="38" rx="6" fill="#09090b" stroke="#52525b" stroke-width="2"/>
              <path d="M-105 -75 L-85 -75 L-115 -37 L-135 -37 Z" fill="#FF9933"/>
              <path d="M-55 -75 L-35 -75 L-65 -37 L-85 -37 Z" fill="#FFFFFF"/>
              <path d="M-5 -75 L15 -75 L-15 -37 L-35 -37 Z" fill="#10B981"/>
              <path d="M45 -75 L65 -75 L35 -37 L15 -37 Z" fill="#FF9933"/>
              <path d="M95 -75 L115 -75 L85 -37 L65 -37 Z" fill="#FFFFFF"/>
              <circle cx="0" cy="36" r="46" fill="#F59E0B" fill-opacity="0.3"/>
              <path d="M0 0 L9 26 L38 26 L15 44 L24 70 L0 54 L-24 70 L-15 44 L-38 26 L-9 26 Z" fill="#F59E0B" stroke="#FEF08A" stroke-width="3"/>
            </g>
          </svg>
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
            placeholder="Search movies, series, cast, OTT..."
            class="w-full bg-gray-900/90 border border-gray-700/80 focus:border-indigo-500 rounded-xl pl-9 pr-8 py-1.5 text-sm text-gray-100 placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500/30"
          />
          <svg width="24" height="24" class="w-4 h-4 text-gray-500 absolute left-3 top-2.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
          </svg>
          <span class="absolute right-2.5 top-2 text-[10px] text-gray-500 font-mono border border-gray-700/60 rounded px-1.5 py-0.5 pointer-events-none hidden sm:inline">/</span>
        </div>

        <button
          id="syncBtn"
          onclick="triggerSync()"
          class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-semibold bg-gray-800 hover:bg-gray-700 text-gray-200 border border-gray-700 transition cursor-pointer shrink-0"
          title="Trigger TMDB Discovery & Homelab Status Sync"
        >
          <svg width="24" height="24" id="syncIcon" class="w-3.5 h-3.5 text-indigo-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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
        <button onclick="setWindowFilter('released')" id="tab-released" class="px-3 py-1.5 rounded-lg text-xs font-semibold transition cursor-pointer">
          Released (Last 5M) <span id="count-released" class="ml-1 px-1.5 py-0.2 rounded-full text-[10px] bg-gray-800 text-gray-300">0</span>
        </button>
        <button onclick="setWindowFilter('in_theaters')" id="tab-in_theaters" class="px-3 py-1.5 rounded-lg text-xs font-semibold transition cursor-pointer">
          🎬 In Theaters <span id="count-in_theaters" class="ml-1 px-1.5 py-0.2 rounded-full text-[10px] bg-gray-800 text-gray-300">0</span>
        </button>
        <button onclick="setWindowFilter('on_ott')" id="tab-on_ott" class="px-3 py-1.5 rounded-lg text-xs font-semibold transition cursor-pointer">
          📺 Now on OTT <span id="count-on_ott" class="ml-1 px-1.5 py-0.2 rounded-full text-[10px] bg-gray-800 text-gray-300">0</span>
        </button>
        <button onclick="setWindowFilter('upcoming')" id="tab-upcoming" class="px-3 py-1.5 rounded-lg text-xs font-semibold transition cursor-pointer">
          Upcoming <span id="count-upcoming" class="ml-1 px-1.5 py-0.2 rounded-full text-[10px] bg-gray-800 text-gray-300">0</span>
        </button>
        <button onclick="setWindowFilter('upcoming_theaters')" id="tab-upcoming_theaters" class="px-3 py-1.5 rounded-lg text-xs font-semibold transition cursor-pointer">
          🎬 Upcoming Theaters <span id="count-upcoming_theaters" class="ml-1 px-1.5 py-0.2 rounded-full text-[10px] bg-gray-800 text-gray-300">0</span>
        </button>
        <button onclick="setWindowFilter('upcoming_ott')" id="tab-upcoming_ott" class="px-3 py-1.5 rounded-lg text-xs font-semibold transition cursor-pointer">
          📺 Upcoming OTT <span id="count-upcoming_ott" class="ml-1 px-1.5 py-0.2 rounded-full text-[10px] bg-gray-800 text-gray-300">0</span>
        </button>
        <button onclick="setWindowFilter('library')" id="tab-library" class="px-3 py-1.5 rounded-lg text-xs font-semibold transition cursor-pointer">
          Requested / Library <span id="count-library" class="ml-1 px-1.5 py-0.2 rounded-full text-[10px] bg-gray-800 text-gray-300">0</span>
        </button>
        <button onclick="setWindowFilter('all')" id="tab-all" class="px-3 py-1.5 rounded-lg text-xs font-semibold transition cursor-pointer">
          All
        </button>
      </div>

      <!-- Media Type, Language, OTT Platform & Sort Filters -->
      <div class="flex flex-wrap items-center gap-2">
        <div class="flex items-center gap-1 bg-gray-900/90 p-1 rounded-xl border border-gray-800">
          <button onclick="setTypeFilter('all')" id="type-all" class="px-2.5 py-1 rounded-lg text-xs font-medium transition cursor-pointer">All Types</button>
          <button onclick="setTypeFilter('movie')" id="type-movie" class="px-2.5 py-1 rounded-lg text-xs font-medium transition cursor-pointer">Movies</button>
          <button onclick="setTypeFilter('tv')" id="type-tv" class="px-2.5 py-1 rounded-lg text-xs font-medium transition cursor-pointer">Web Series</button>
        </div>

        <select id="langFilter" onchange="renderGrid()" class="bg-gray-900 border border-gray-800 rounded-xl px-2.5 py-1.5 text-xs text-gray-300 focus:outline-none focus:border-indigo-500 cursor-pointer">
          <option value="all">Hindi + Pan-India Dub</option>
          <option value="hi">Original Hindi Only</option>
          <option value="dub">Pan-India (Hindi Dub)</option>
        </select>

        <select id="ottFilter" onchange="renderGrid()" class="bg-gray-900 border border-gray-800 rounded-xl px-2.5 py-1.5 text-xs text-gray-300 focus:outline-none focus:border-indigo-500 cursor-pointer">
          <option value="all">All Platforms / Theatrical</option>
          <option value="theatrical">🎬 Theatrical Only</option>
          <option value="ott">📺 All OTT Platforms</option>
          <option value="Netflix">Netflix</option>
          <option value="Prime Video">Prime Video</option>
          <option value="JioHotstar">JioHotstar</option>
          <option value="SonyLIV">SonyLIV</option>
          <option value="ZEE5">ZEE5</option>
          <option value="JioCinema">JioCinema</option>
        </select>

        <select id="sortFilter" onchange="renderGrid()" class="bg-gray-900 border border-gray-800 rounded-xl px-2.5 py-1.5 text-xs text-gray-300 focus:outline-none focus:border-indigo-500">
          <option value="newest">Latest Releases</option>
          <option value="rating">Top Rated (★)</option>
          <option value="popular">Most Popular</option>
          <option value="expiring">Expiring Soonest</option>
          <option value="title">Title (A-Z)</option>
        </select>
      </div>
    </div>
  </header>

  <!-- Main Content -->
  <main class="flex-1 max-w-7xl w-full mx-auto px-4 sm:px-6 lg:px-8 py-6">
    <!-- Hero Spotlight Section -->
    <div id="heroSpotlight" class="hidden mb-6"></div>

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

  <!-- Details & In-Modal Trailer Player Modal -->
  <div id="detailModal" class="fixed inset-0 z-50 hidden bg-black/80 backdrop-blur-sm flex items-center justify-center p-3 sm:p-4 overflow-y-auto" onclick="if(event.target === this) closeModal()">
    <div class="bg-gray-900 border border-gray-800 rounded-2xl max-w-2xl w-full max-h-[90vh] overflow-y-auto shadow-2xl">
      <div id="modalContent"></div>
    </div>
  </div>

  <!-- Delete Confirmation Modal -->
  <div id="deleteModal" class="fixed inset-0 z-50 hidden bg-black/85 backdrop-blur-md flex items-center justify-center p-4" onclick="if(event.target === this) closeDeleteModal()">
    <div class="bg-gray-900 border border-rose-900/60 rounded-2xl max-w-md w-full p-6 shadow-2xl relative text-left">
      <div class="w-12 h-12 rounded-xl bg-rose-500/10 border border-rose-500/30 flex items-center justify-center text-rose-400 mb-4">
        <svg width="24" height="24" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/></svg>
      </div>
      <h3 class="text-lg font-bold text-white mb-1.5">Remove from Homelab?</h3>
      <p id="deleteConfirmText" class="text-sm text-gray-300 mb-4 leading-relaxed">
        Are you sure you want to delete this item?
      </p>
      <div class="p-3 rounded-xl bg-rose-950/40 border border-rose-900/50 text-xs text-rose-200/90 mb-5 space-y-1">
        <p class="font-semibold text-rose-200 flex items-center gap-1.5">
          <span>⚠️</span> This action will:
        </p>
        <ul class="list-disc list-inside space-y-0.5 text-[11px] text-rose-300/80">
          <li>Delete media files from Radarr/Sonarr storage</li>
          <li>Remove media files so Jellyfin cleans up library</li>
          <li>Cancel or delete the request from Jellyseerr</li>
          <li>Reset status in BollySeerr to Unrequested</li>
        </ul>
      </div>
      <div class="flex items-center justify-end gap-2.5">
        <button onclick="closeDeleteModal()" class="px-4 py-2 rounded-xl text-xs font-semibold bg-gray-800 hover:bg-gray-700 text-gray-300 border border-gray-700 transition cursor-pointer">
          Cancel
        </button>
        <button id="confirmDeleteBtn" onclick="executeDelete()" class="px-4 py-2 rounded-xl text-xs font-semibold bg-rose-600 hover:bg-rose-500 text-white shadow-lg shadow-rose-900/40 transition cursor-pointer flex items-center gap-1.5">
          <svg width="24" height="24" class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/></svg>
          Delete Media
        </button>
      </div>
    </div>
  </div>

  <!-- Toast Notification -->
  <div id="toast" class="fixed bottom-5 right-5 z-50 hidden px-4 py-3 rounded-xl shadow-xl border text-sm font-medium transition-all"></div>

  <script>
    let allItems = [];
    let syncMeta = {};
    let currentWindow = 'released';
    let currentType = 'all';
    let heroIndex = 0;
    let heroTimer = null;
    let heroItems = [];
    let heroRenderedIds = '';
    const todayStr = new Date().toISOString().slice(0, 10);

    document.getElementById('searchInput').addEventListener('input', () => {
      renderHeroSpotlight();
      renderGrid();
    });

    // Keyboard Shortcuts: / to focus search, Esc to close modals
    document.addEventListener('keydown', (e) => {
      if (e.key === '/' && document.activeElement !== document.getElementById('searchInput')) {
        e.preventDefault();
        const s = document.getElementById('searchInput');
        if (s) {
          s.focus();
          s.select();
        }
      }
      if (e.key === 'Escape') {
        closeModal();
        closeDeleteModal();
      }
    });

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

    function getOttBrandBadge(provider) {
      const p = (provider || '').trim();
      if (!p) return '';
      if (/netflix/i.test(p)) {
        return `<span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[10px] font-bold bg-[#E50914]/20 text-[#ff4d58] border border-[#E50914]/40 shadow-sm shadow-[#E50914]/20"><span class="w-1.5 h-1.5 rounded-full bg-[#E50914]"></span>Netflix</span>`;
      }
      if (/prime|amazon/i.test(p)) {
        return `<span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[10px] font-bold bg-[#00A8E1]/20 text-[#38c9ff] border border-[#00A8E1]/40 shadow-sm shadow-[#00A8E1]/20"><span class="w-1.5 h-1.5 rounded-full bg-[#00A8E1]"></span>Prime Video</span>`;
      }
      if (/hotstar|jiohotstar|disney/i.test(p)) {
        return `<span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[10px] font-bold bg-[#002D62]/40 text-[#40c4ff] border border-[#0052cc]/50 shadow-sm shadow-[#002D62]/30"><span class="w-1.5 h-1.5 rounded-full bg-[#00E5FF]"></span>Hotstar</span>`;
      }
      if (/sonyliv|sony/i.test(p)) {
        return `<span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[10px] font-bold bg-[#FF6600]/20 text-[#ff944d] border border-[#FF6600]/40 shadow-sm shadow-[#FF6600]/20"><span class="w-1.5 h-1.5 rounded-full bg-[#FF6600]"></span>SonyLIV</span>`;
      }
      if (/zee5|zee/i.test(p)) {
        return `<span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[10px] font-bold bg-[#8230C6]/20 text-[#c084fc] border border-[#8230C6]/40 shadow-sm shadow-[#8230C6]/20"><span class="w-1.5 h-1.5 rounded-full bg-[#8230C6]"></span>ZEE5</span>`;
      }
      if (/jiocinema|jio/i.test(p)) {
        return `<span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[10px] font-bold bg-[#E1193C]/20 text-[#ff4d6d] border border-[#E1193C]/40 shadow-sm shadow-[#E1193C]/20"><span class="w-1.5 h-1.5 rounded-full bg-[#E1193C]"></span>JioCinema</span>`;
      }
      if (/apple/i.test(p)) {
        return `<span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[10px] font-bold bg-zinc-800 text-zinc-200 border border-zinc-600 shadow-sm"> Apple TV+</span>`;
      }
      return `<span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[10px] font-semibold bg-gray-800/90 text-gray-300 border border-gray-700/60">📺 ${p}</span>`;
    }

    function setWindowFilter(w) {
      currentWindow = w;
      ['released', 'in_theaters', 'on_ott', 'upcoming', 'upcoming_theaters', 'upcoming_ott', 'library', 'all'].forEach(k => {
        const el = document.getElementById(`tab-${k}`);
        if (!el) return;
        if (k === w) {
          el.className = 'px-3 py-1.5 rounded-lg text-xs font-semibold bg-indigo-600 text-white shadow transition cursor-pointer';
        } else {
          el.className = 'px-3 py-1.5 rounded-lg text-xs font-semibold text-gray-400 hover:text-gray-200 transition cursor-pointer';
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
      renderHeroSpotlight();
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
        renderHeroSpotlight();
        renderGrid();
      } catch (e) {
        console.error('Failed to load catalog:', e);
      }
    }

    function updateHeaderCounts() {
      const releasedItems = allItems.filter(i => i.release_date <= todayStr);
      const inTheatersCount = releasedItems.filter(i => i.media_type === 'movie' && !isItemOnOtt(i)).length;
      const onOttCount = releasedItems.filter(i => isItemOnOtt(i)).length;

      const upcomingItems = allItems.filter(i => i.release_date > todayStr);
      const upcomingTheatersCount = upcomingItems.filter(i => i.media_type === 'movie' && !isItemOnOtt(i)).length;
      const upcomingOttCount = upcomingItems.filter(i => isItemOnOtt(i)).length;

      document.getElementById('count-released').textContent = syncMeta.released_count ?? releasedItems.length;
      document.getElementById('count-in_theaters').textContent = inTheatersCount;
      document.getElementById('count-on_ott').textContent = onOttCount;
      document.getElementById('count-upcoming').textContent = syncMeta.upcoming_count ?? upcomingItems.length;
      const elUpTheaters = document.getElementById('count-upcoming_theaters');
      if (elUpTheaters) elUpTheaters.textContent = upcomingTheatersCount;
      const elUpOtt = document.getElementById('count-upcoming_ott');
      if (elUpOtt) elUpOtt.textContent = upcomingOttCount;
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
      const safeTitle = (item.title || '').replace(/'/g, "\\'").replace(/"/g, '&quot;');
      const trashBtn = `<button onclick="event.stopPropagation(); promptDeleteItem(${item.tmdb_id}, '${item.media_type}', '${safeTitle}')" title="Delete from Radarr/Sonarr & Jellyfin" class="px-2 py-1.5 rounded-lg bg-rose-500/15 hover:bg-rose-600/30 text-rose-300 hover:text-rose-100 border border-rose-500/30 transition cursor-pointer flex items-center justify-center shrink-0">
        <svg width="24" height="24" class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/></svg>
      </button>`;

      if (st === 'available') {
        return `<div class="flex items-center gap-1.5">
          <button disabled class="flex-1 py-1.5 px-2.5 rounded-lg text-xs font-semibold bg-emerald-500/20 text-emerald-300 border border-emerald-500/30 cursor-default truncate">✓ In Library</button>
          ${trashBtn}
        </div>`;
      }
      if (st === 'partially_available') {
        return `<div class="flex items-center gap-1.5">
          <button onclick="event.stopPropagation(); requestItem(${item.tmdb_id}, '${item.media_type}')" class="flex-1 py-1.5 px-2.5 rounded-lg text-xs font-semibold bg-teal-500/20 hover:bg-teal-500/30 text-teal-200 border border-teal-500/30 transition cursor-pointer truncate">Request Rest</button>
          ${trashBtn}
        </div>`;
      }
      if (st === 'monitored' || st === 'requested') {
        return `<div class="flex items-center gap-1.5">
          <button disabled class="flex-1 py-1.5 px-2.5 rounded-lg text-xs font-semibold bg-amber-500/20 text-amber-300 border border-amber-500/30 cursor-default truncate">⏳ Requested</button>
          ${trashBtn}
        </div>`;
      }
      return `<button id="req-btn-${item.media_type}-${item.tmdb_id}" onclick="event.stopPropagation(); requestItem(${item.tmdb_id}, '${item.media_type}')" class="w-full py-1.5 px-3 rounded-lg text-xs font-semibold bg-indigo-600 hover:bg-indigo-500 text-white shadow-sm shadow-indigo-500/30 transition cursor-pointer flex items-center justify-center gap-1">
        <svg width="24" height="24" class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M12 4v16m8-8H4"/></svg>
        Request
      </button>`;
    }

    function fuzzyMatch(haystack, needle) {
      if (!needle) return true;
      if (haystack.includes(needle)) return true;
      const words = haystack.split(/[\s:,\-_]+/);
      return words.some(w => w.startsWith(needle) || needle.startsWith(w));
    }

    // Hero Spotlight Carousel Logic
    function startHeroTimer() {
      if (heroTimer) clearInterval(heroTimer);
      heroTimer = setInterval(() => {
        if (heroItems.length > 1) {
          nextHeroSlide();
        }
      }, 7000);
    }

    function pauseHeroTimer() {
      if (heroTimer) clearInterval(heroTimer);
    }

    function prevHeroSlide() {
      if (!heroItems.length) return;
      setHeroIndex((heroIndex - 1 + heroItems.length) % heroItems.length);
    }

    function nextHeroSlide() {
      if (!heroItems.length) return;
      setHeroIndex((heroIndex + 1) % heroItems.length);
    }

    function setHeroIndex(idx) {
      if (!heroItems.length) return;
      const nextIdx = (idx + heroItems.length) % heroItems.length;
      if (nextIdx === heroIndex && document.getElementById(`hero-slide-${nextIdx}`)) return;

      const prevIdx = heroIndex;
      heroIndex = nextIdx;

      // Cross-fade slides smoothly
      const prevSlide = document.getElementById(`hero-slide-${prevIdx}`);
      if (prevSlide) {
        prevSlide.classList.remove('opacity-100', 'z-10', 'pointer-events-auto');
        prevSlide.classList.add('opacity-0', 'z-0', 'pointer-events-none');
      }
      const prevGlow = document.getElementById(`hero-glow-${prevIdx}`);
      if (prevGlow) {
        prevGlow.classList.remove('opacity-25');
        prevGlow.classList.add('opacity-0');
      }

      const nextSlide = document.getElementById(`hero-slide-${heroIndex}`);
      if (nextSlide) {
        nextSlide.classList.remove('opacity-0', 'z-0', 'pointer-events-none');
        nextSlide.classList.add('opacity-100', 'z-10', 'pointer-events-auto');
      }
      const nextGlow = document.getElementById(`hero-glow-${heroIndex}`);
      if (nextGlow) {
        nextGlow.classList.remove('opacity-0');
        nextGlow.classList.add('opacity-25');
      }

      // Smoothly update dot indicator sizes and styling
      heroItems.forEach((_, i) => {
        const dot = document.getElementById(`hero-dot-${i}`);
        if (dot) {
          if (i === heroIndex) {
            dot.className = 'hero-dot h-2 rounded-full transition-all duration-500 cursor-pointer w-8 bg-indigo-500 shadow-md shadow-indigo-500/50';
          } else {
            dot.className = 'hero-dot h-2 rounded-full transition-all duration-500 cursor-pointer w-2 bg-gray-600/70 hover:bg-gray-400';
          }
        }
      });

      startHeroTimer();
    }

    function renderHeroSpotlight() {
      const heroContainer = document.getElementById('heroSpotlight');
      const q = (document.getElementById('searchInput')?.value || '').trim();
      if (!heroContainer) return;

      if (q || allItems.length === 0) {
        heroContainer.classList.add('hidden');
        pauseHeroTimer();
        return;
      }

      // Pick top 5 popular released blockbusters with backdrop images
      const candidateItems = allItems
        .filter(i => i.backdrop_path && i.vote_average > 0)
        .sort((a, b) => (b.popularity * 0.7 + b.vote_average * 10) - (a.popularity * 0.7 + a.vote_average * 10))
        .slice(0, 5);

      if (candidateItems.length === 0) {
        heroContainer.classList.add('hidden');
        pauseHeroTimer();
        return;
      }

      const candidateIds = candidateItems.map(i => i.tmdb_id).join(',');
      if (candidateIds === heroRenderedIds && document.getElementById('heroCarousel')) {
        heroContainer.classList.remove('hidden');
        return;
      }

      heroItems = candidateItems;
      heroRenderedIds = candidateIds;
      if (heroIndex >= heroItems.length) heroIndex = 0;
      heroContainer.classList.remove('hidden');

      // Pre-render all slides into the DOM so images are preloaded and cross-fading is instantaneous & smooth
      const glowsHtml = heroItems.map((item, idx) => `
        <div id="hero-glow-${idx}" class="absolute inset-0 bg-cover bg-center filter blur-3xl scale-110 pointer-events-none transition-opacity duration-1000 ease-in-out ${idx === heroIndex ? 'opacity-25' : 'opacity-0'}" style="background-image: url('https://image.tmdb.org/t/p/w1280${item.backdrop_path}')"></div>
      `).join('');

      const slidesHtml = heroItems.map((item, idx) => {
        const backdropUrl = `https://image.tmdb.org/t/p/w1280${item.backdrop_path}`;
        const safeTitle = (item.title || '').replace(/'/g, "\\'").replace(/"/g, '&quot;');
        const year = (item.release_date || '').slice(0, 4);

        const typeBadge = item.media_type === 'tv'
          ? `<span class="px-2.5 py-0.5 rounded text-[10px] font-bold bg-pink-600/90 text-white shadow-sm">SERIES</span>`
          : `<span class="px-2.5 py-0.5 rounded text-[10px] font-bold bg-indigo-600/90 text-white shadow-sm">MOVIE</span>`;

        const langTag = item.is_hindi_dub
          ? `<span class="px-2 py-0.5 rounded text-[10px] font-semibold bg-purple-950/90 text-purple-200 border border-purple-700/50">Pan-India (${item.original_language.toUpperCase()})</span>`
          : `<span class="px-2 py-0.5 rounded text-[10px] font-semibold bg-gray-900/90 text-gray-200 border border-gray-700/60">Hindi</span>`;

        const ottBadges = (item.ott_providers || []).map(p => getOttBrandBadge(p)).join(' ');

        const trailerBtn = item.trailer_key
          ? `<button onclick="openModalWithTrailer(${item.tmdb_id}, '${item.media_type}')" class="px-4 py-2.5 rounded-xl text-xs font-bold bg-red-600/90 hover:bg-red-500 text-white shadow-lg shadow-red-600/30 transition flex items-center gap-2 cursor-pointer hover:scale-105 active:scale-95">
               <svg class="w-4 h-4 fill-current" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
               Watch Trailer
             </button>`
          : '';

        const isActive = idx === heroIndex;
        return `
          <div id="hero-slide-${idx}" class="hero-slide absolute inset-0 transition-opacity duration-1000 ease-in-out ${isActive ? 'opacity-100 z-10 pointer-events-auto' : 'opacity-0 z-0 pointer-events-none'}">
            <!-- Crisp hero banner image with ambient scale -->
            <img src="${backdropUrl}" alt="${safeTitle}" class="w-full h-full object-cover object-center transform group-hover:scale-102 transition duration-1000" />
            <div class="absolute inset-0 bg-gradient-to-t from-gray-950 via-gray-950/60 to-transparent"></div>
            <div class="absolute inset-0 bg-gradient-to-r from-gray-950/90 via-gray-950/40 to-transparent"></div>

            <!-- Spotlight Content -->
            <div class="absolute bottom-6 sm:bottom-8 left-5 sm:left-8 right-5 sm:right-8 flex flex-col justify-end">
              <div class="flex flex-wrap items-center gap-2 mb-2">
                <span class="px-2.5 py-0.5 rounded-full text-[11px] font-extrabold uppercase tracking-wider bg-gradient-to-r from-amber-500 to-orange-500 text-gray-950 shadow">
                  ★ Spotlight
                </span>
                ${typeBadge}
                ${langTag}
                ${item.vote_average > 0 ? `<span class="px-2 py-0.5 rounded-md text-xs font-bold bg-black/70 backdrop-blur-md text-amber-400 border border-amber-500/30">★ ${item.vote_average.toFixed(1)}</span>` : ''}
                <span class="text-xs text-gray-400 font-medium">${year}</span>
                ${ottBadges}
              </div>

              <h2 class="text-2xl sm:text-4xl lg:text-5xl font-black text-white tracking-tight drop-shadow-md line-clamp-2 max-w-3xl">
                ${item.title}
              </h2>

              <p class="text-xs sm:text-sm text-gray-300 line-clamp-2 sm:line-clamp-3 max-w-2xl mt-2 drop-shadow leading-relaxed">
                ${item.overview || 'Hindi cinema discovery on your private homelab.'}
              </p>

              <!-- Actions -->
              <div class="flex items-center gap-3 flex-wrap mt-4 pt-2">
                ${trailerBtn}
                <button onclick="openModal(${item.tmdb_id}, '${item.media_type}')" class="px-4 py-2.5 rounded-xl text-xs font-bold bg-gray-800/90 hover:bg-gray-700 text-gray-100 border border-gray-700 transition flex items-center gap-1.5 cursor-pointer backdrop-blur-md">
                  More Info
                </button>
                <div class="w-36">
                  ${actionButtonHtml(item)}
                </div>
              </div>
            </div>
          </div>
        `;
      }).join('');

      const dotsHtml = heroItems.map((_, idx) => `
        <button id="hero-dot-${idx}" onclick="setHeroIndex(${idx})" class="hero-dot h-2 rounded-full transition-all duration-500 cursor-pointer ${idx === heroIndex ? 'w-8 bg-indigo-500 shadow-md shadow-indigo-500/50' : 'w-2 bg-gray-600/70 hover:bg-gray-400'}" title="Go to spotlight ${idx + 1}"></button>
      `).join('');

      heroContainer.innerHTML = `
        <div id="heroCarousel" class="relative w-full rounded-3xl overflow-hidden border border-gray-800/90 bg-gray-950 shadow-2xl mb-8 group" onmouseenter="pauseHeroTimer()" onmouseleave="startHeroTimer()">
          ${glowsHtml}

          <div class="relative h-72 sm:h-96 md:h-[28rem] w-full overflow-hidden">
            ${slidesHtml}

            <!-- Prev / Next Chevrons on Hover -->
            <button onclick="prevHeroSlide()" class="absolute left-3 top-1/2 -translate-y-1/2 z-20 w-10 h-10 rounded-full bg-black/50 hover:bg-black/80 text-white/70 hover:text-white flex items-center justify-center transition-all opacity-0 group-hover:opacity-100 backdrop-blur-md border border-white/10 cursor-pointer shadow-lg hover:scale-110 active:scale-95" title="Previous Spotlight">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M15 19l-7-7 7-7"/></svg>
            </button>
            <button onclick="nextHeroSlide()" class="absolute right-3 top-1/2 -translate-y-1/2 z-20 w-10 h-10 rounded-full bg-black/50 hover:bg-black/80 text-white/70 hover:text-white flex items-center justify-center transition-all opacity-0 group-hover:opacity-100 backdrop-blur-md border border-white/10 cursor-pointer shadow-lg hover:scale-110 active:scale-95" title="Next Spotlight">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M9 5l7 7-7 7"/></svg>
            </button>

            <!-- Dots positioned bottom-right over active slide -->
            <div class="absolute bottom-6 sm:bottom-8 right-5 sm:right-8 z-20 flex items-center gap-1.5 bg-black/40 backdrop-blur-md px-3 py-1.5 rounded-full border border-white/10">
              ${dotsHtml}
            </div>
          </div>
        </div>
      `;

      startHeroTimer();
    }

    function getFallbackArtHtml(title, mediaType) {
      const icon = mediaType === 'tv'
        ? `<svg width="24" height="24" class="w-12 h-12 text-indigo-400/80 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/></svg>`
        : `<svg width="24" height="24" class="w-12 h-12 text-purple-400/80 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18M3 16h4m10 0h4M4 20h16a1 1 0 001-1V5a1 1 0 00-1-1H4a1 1 0 00-1 1v14a1 1 0 001 1z"/></svg>`;
      return `
        <div class="w-full h-full flex flex-col items-center justify-center p-4 text-center bg-gradient-to-br from-gray-900 via-indigo-950/40 to-gray-950 border border-gray-800/60 select-none">
          ${icon}
          <span class="text-xs font-bold text-gray-200 line-clamp-2 px-1">${title}</span>
          <span class="text-[10px] text-gray-500 font-semibold uppercase tracking-wider mt-1">${mediaType === 'tv' ? 'Web Series' : 'Movie'}</span>
        </div>
      `;
    }

    function getActorFallbackAvatar(name) {
      const initial = (name || 'A').trim().charAt(0).toUpperCase();
      return `
        <div class="w-full h-full flex items-center justify-center bg-indigo-950 text-indigo-300 font-bold text-base select-none">
          ${initial}
        </div>
      `;
    }

    function filterByActor(actorName) {
      closeModal();
      const searchInput = document.getElementById('searchInput');
      if (searchInput) {
        searchInput.value = actorName;
        renderHeroSpotlight();
        renderGrid();
        searchInput.focus();
        showToast(`🔍 Showing titles starring "${actorName}"`);
      }
    }

    function renderGrid() {
      const q = (document.getElementById('searchInput')?.value || '').trim().toLowerCase();
      const langMode = document.getElementById('langFilter')?.value || 'all';
      const ottMode = document.getElementById('ottFilter')?.value || 'all';
      const sortMode = document.getElementById('sortFilter')?.value || 'newest';

      const filtered = allItems.filter(item => {
        if (!q) {
          if (currentWindow === 'released' && item.release_date > todayStr) return false;
          if (currentWindow === 'in_theaters' && (item.release_date > todayStr || item.media_type !== 'movie' || isItemOnOtt(item))) return false;
          if (currentWindow === 'on_ott' && (item.release_date > todayStr || !isItemOnOtt(item))) return false;
          if (currentWindow === 'upcoming' && item.release_date <= todayStr) return false;
          if (currentWindow === 'upcoming_theaters' && (item.release_date <= todayStr || item.media_type !== 'movie' || isItemOnOtt(item))) return false;
          if (currentWindow === 'upcoming_ott' && (item.release_date <= todayStr || !isItemOnOtt(item))) return false;
          if (currentWindow === 'library' && item.homelab_status === 'unrequested') return false;
        }

        if (currentType !== 'all' && item.media_type !== currentType) return false;
        if (langMode === 'hi' && item.is_hindi_dub) return false;
        if (langMode === 'dub' && !item.is_hindi_dub) return false;
        if (ottMode === 'theatrical' && (item.media_type !== 'movie' || isItemOnOtt(item))) return false;
        if (ottMode === 'ott' && !isItemOnOtt(item)) return false;
        if (ottMode !== 'all' && ottMode !== 'theatrical' && ottMode !== 'ott' && !(item.ott_providers || []).includes(ottMode)) return false;

        if (q) {
          const hay = `${item.title} ${item.original_title} ${(item.genres || []).join(' ')} ${(item.ott_providers || []).join(' ')} ${item.overview || ''}`.toLowerCase();
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
      } else {
        if (sortMode === 'rating') {
          filtered.sort((a, b) => b.vote_average - a.vote_average || b.popularity - a.popularity);
        } else if (sortMode === 'popular') {
          filtered.sort((a, b) => b.popularity - a.popularity);
        } else if (sortMode === 'expiring') {
          filtered.sort((a, b) => a.release_date.localeCompare(b.release_date));
        } else if (sortMode === 'title') {
          filtered.sort((a, b) => a.title.localeCompare(b.title));
        } else {
          // newest
          if (currentWindow.startsWith('upcoming')) {
            filtered.sort((a, b) => a.release_date.localeCompare(b.release_date));
          } else {
            filtered.sort((a, b) => b.release_date.localeCompare(a.release_date));
          }
        }
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

        const ottPills = (item.ott_providers || []).slice(0, 2).map(p => getOttBrandBadge(p)).join(' ');

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

        const ratingBadge = item.vote_average > 0
          ? `<span class="bg-black/75 backdrop-blur-md px-1.5 py-0.5 rounded-md text-[11px] font-bold ${
              item.vote_average >= 7.5 ? 'text-amber-400 border border-amber-500/30' : (item.vote_average >= 6.0 ? 'text-emerald-400 border border-emerald-500/30' : 'text-gray-300 border border-white/10')
            }">★ ${item.vote_average.toFixed(1)}</span>`
          : '';

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
                ${ratingBadge}
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

    let pendingDelete = null;

    function promptDeleteItem(tmdbId, mediaType, title) {
      pendingDelete = { tmdbId, mediaType, title };
      const textEl = document.getElementById('deleteConfirmText');
      if (textEl) {
        textEl.innerHTML = `Are you sure you want to delete <strong class="text-white">${title}</strong>?`;
      }
      document.getElementById('deleteModal').classList.remove('hidden');
    }

    function closeDeleteModal() {
      document.getElementById('deleteModal').classList.add('hidden');
      pendingDelete = null;
    }

    async function executeDelete() {
      if (!pendingDelete) return;
      const { tmdbId, mediaType, title } = pendingDelete;
      const btn = document.getElementById('confirmDeleteBtn');
      if (btn) {
        btn.disabled = true;
        btn.textContent = 'Deleting...';
      }

      try {
        const resp = await fetch('/api/delete', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ tmdb_id: tmdbId, media_type: mediaType })
        });
        const data = await resp.json();
        if (!resp.ok || !data.success) {
          throw new Error(data.message || 'Deletion failed');
        }
        closeDeleteModal();
        closeModal();
        showToast(`✓ Deleted "${title}" from Radarr/Sonarr & Jellyfin`);
        await fetchCatalog();
      } catch (err) {
        showToast(`✗ ${err.message}`, true);
      } finally {
        if (btn) {
          btn.disabled = false;
          btn.innerHTML = `<svg width="24" height="24" class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/></svg> Delete Media`;
        }
      }
    }

    function playTrailerInModal(key) {
      const container = document.getElementById('trailerIframeContainer');
      const iframe = document.getElementById('trailerIframe');
      const overlay = document.getElementById('playTrailerOverlay');
      if (container && iframe) {
        iframe.src = `https://www.youtube-nocookie.com/embed/${key}?autoplay=1&rel=0&modestbranding=1`;
        container.classList.remove('hidden');
        if (overlay) overlay.classList.add('hidden');
      }
    }

    function stopTrailerInModal() {
      const container = document.getElementById('trailerIframeContainer');
      const iframe = document.getElementById('trailerIframe');
      const overlay = document.getElementById('playTrailerOverlay');
      if (container && iframe) {
        iframe.src = '';
        container.classList.add('hidden');
        if (overlay) overlay.classList.remove('hidden');
      }
    }

    function openModalWithTrailer(tmdbId, mediaType) {
      openModal(tmdbId, mediaType, true);
    }

    function openModal(tmdbId, mediaType, autoPlayTrailer = false) {
      const item = allItems.find(i => i.tmdb_id === tmdbId && i.media_type === mediaType);
      if (!item) return;

      const modal = document.getElementById('detailModal');
      const content = document.getElementById('modalContent');
      const backdrop = item.backdrop_path
        ? `https://image.tmdb.org/t/p/w1280${item.backdrop_path}`
        : (item.poster_path ? `https://image.tmdb.org/t/p/w500${item.poster_path}` : '');

      const tmdbUrl = `https://www.themoviedb.org/${item.media_type}/${item.tmdb_id}`;

      content.innerHTML = `
        <div id="modalHeaderMedia" class="relative h-56 sm:h-72 bg-gray-950 overflow-hidden rounded-t-2xl">
          ${backdrop ? `<img id="modalBackdropImg" src="${backdrop}" class="w-full h-full object-cover opacity-60 transition duration-300" />` : ''}
          <div class="absolute inset-0 bg-gradient-to-t from-gray-900 via-gray-900/50 to-transparent"></div>
          <button onclick="closeModal()" class="absolute top-3 right-3 z-30 w-8 h-8 rounded-full bg-black/70 hover:bg-black text-gray-200 flex items-center justify-center transition cursor-pointer">✕</button>

          ${item.trailer_key ? `
            <div id="playTrailerOverlay" class="absolute inset-0 z-20 flex items-center justify-center">
              <button onclick="playTrailerInModal('${item.trailer_key}')" class="px-4 py-2.5 rounded-xl bg-red-600/90 hover:bg-red-500 text-white font-bold text-xs flex items-center gap-2 shadow-2xl shadow-red-600/50 backdrop-blur-sm hover:scale-105 active:scale-95 transition cursor-pointer">
                <svg width="24" height="24" class="w-4 h-4 fill-current" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg>
                Play Trailer
              </button>
            </div>
            <div id="trailerIframeContainer" class="hidden absolute inset-0 z-20 w-full h-full bg-black">
              <iframe id="trailerIframe" src="" class="w-full h-full border-0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
              <button onclick="stopTrailerInModal()" class="absolute top-3 left-3 z-30 px-2.5 py-1 rounded-lg bg-black/80 hover:bg-black text-gray-200 text-xs font-semibold backdrop-blur-md flex items-center gap-1.5 transition cursor-pointer">✕ Close Video</button>
            </div>
          ` : ''}

          <div class="absolute bottom-4 left-5 right-5 z-10 pointer-events-none">
            <div class="flex items-center gap-2 mb-1 flex-wrap">
              <span class="px-2 py-0.5 rounded text-[10px] font-bold uppercase bg-indigo-600 text-white">${item.release_type}</span>
              ${statusBadgeHtml(item)}
              ${item.vote_average > 0 ? `<span class="px-2 py-0.5 rounded-md text-[11px] font-bold bg-black/70 backdrop-blur-md text-amber-400 border border-amber-500/30">★ ${item.vote_average.toFixed(1)}</span>` : ''}
            </div>
            <h2 class="text-xl sm:text-2xl font-extrabold text-white tracking-tight">${item.title}</h2>
            <p class="text-xs text-gray-300 mt-0.5">Release Date: ${item.release_date} • Original Lang: ${item.original_language.toUpperCase()}${item.season_count ? ` • ${item.season_count} Season${item.season_count > 1 ? 's' : ''}` : ''}</p>
          </div>
        </div>

        <div class="p-5 space-y-4">
          <div class="flex flex-wrap gap-1.5">
            ${(item.genres || []).map(g => `<span class="px-2.5 py-0.5 rounded-full text-xs bg-gray-800 text-gray-300 border border-gray-700">${g}</span>`).join('')}
            ${(item.ott_providers || []).map(p => getOttBrandBadge(p)).join(' ')}
          </div>
          <p class="text-sm text-gray-300 leading-relaxed">${item.overview || 'No synopsis available yet on TMDB.'}</p>

          <!-- Crew Section -->
          <div id="modalCrewSection" class="space-y-1.5 pt-1 border-t border-gray-800/80">
            <span class="text-[11px] font-bold text-gray-400 uppercase tracking-wider">Key Crew</span>
            <div id="modalCrewList" class="flex flex-wrap gap-2 text-xs">
              <span class="px-2.5 py-1 rounded-lg bg-gray-800/50 text-gray-500 animate-pulse">Loading crew...</span>
            </div>
          </div>

          <!-- Cast Section -->
          <div id="modalCastSection" class="space-y-2 pt-1 border-t border-gray-800/80">
            <div class="flex items-center justify-between">
              <span class="text-[11px] font-bold text-gray-400 uppercase tracking-wider">Top Cast</span>
              <span class="text-[10px] text-gray-500">Click any actor to find their titles</span>
            </div>
            <div id="modalCastList" class="flex gap-3 overflow-x-auto pb-2 -mx-1 px-1">
              <div class="flex-shrink-0 w-24 flex flex-col items-center text-center animate-pulse">
                <div class="w-14 h-14 rounded-full bg-gray-800/70 border border-gray-700/50 mb-1.5"></div>
                <div class="w-16 h-2.5 bg-gray-800 rounded mb-1"></div>
                <div class="w-12 h-2 bg-gray-800/60 rounded"></div>
              </div>
              <div class="flex-shrink-0 w-24 flex flex-col items-center text-center animate-pulse">
                <div class="w-14 h-14 rounded-full bg-gray-800/70 border border-gray-700/50 mb-1.5"></div>
                <div class="w-16 h-2.5 bg-gray-800 rounded mb-1"></div>
                <div class="w-12 h-2 bg-gray-800/60 rounded"></div>
              </div>
              <div class="flex-shrink-0 w-24 flex flex-col items-center text-center animate-pulse">
                <div class="w-14 h-14 rounded-full bg-gray-800/70 border border-gray-700/50 mb-1.5"></div>
                <div class="w-16 h-2.5 bg-gray-800 rounded mb-1"></div>
                <div class="w-12 h-2 bg-gray-800/60 rounded"></div>
              </div>
              <div class="flex-shrink-0 w-24 flex flex-col items-center text-center animate-pulse">
                <div class="w-14 h-14 rounded-full bg-gray-800/70 border border-gray-700/50 mb-1.5"></div>
                <div class="w-16 h-2.5 bg-gray-800 rounded mb-1"></div>
                <div class="w-12 h-2 bg-gray-800/60 rounded"></div>
              </div>
            </div>
          </div>

          <!-- Tags Section -->
          <div id="modalTagsSection" class="hidden space-y-1.5 pt-1 border-t border-gray-800/80">
            <span class="text-[11px] font-bold text-gray-400 uppercase tracking-wider">Tags & Keywords</span>
            <div id="modalTagList" class="flex flex-wrap gap-1.5 text-xs"></div>
          </div>

          <div class="flex flex-wrap items-center justify-between gap-3 pt-3 border-t border-gray-800">
            <div class="flex items-center gap-2 flex-wrap">
              ${item.trailer_key ? `
                <button onclick="playTrailerInModal('${item.trailer_key}')" class="px-3.5 py-2 rounded-xl text-xs font-semibold bg-red-600/90 hover:bg-red-500 text-white transition inline-flex items-center gap-1.5 cursor-pointer">
                  <svg width="24" height="24" class="w-3.5 h-3.5 fill-current" viewBox="0 0 24 24"><path d="M8 5v14l11-7z"/></svg> Play Trailer
                </button>
                <a href="https://www.youtube.com/watch?v=${item.trailer_key}" target="_blank" rel="noopener" class="px-2.5 py-2 rounded-xl text-xs font-semibold bg-gray-800 hover:bg-gray-700 text-gray-400 hover:text-gray-200 border border-gray-700 transition" title="Open on YouTube">YouTube ↗</a>
              ` : ''}
              <a href="${tmdbUrl}" target="_blank" rel="noopener" class="px-3 py-2 rounded-xl text-xs font-semibold bg-gray-800 hover:bg-gray-700 text-gray-300 border border-gray-700">TMDB ↗</a>
              ${item.homelab_status !== 'unrequested' ? `
                <button onclick="promptDeleteItem(${item.tmdb_id}, '${item.media_type}', '${(item.title || '').replace(/'/g, "\\'").replace(/"/g, '&quot;')}')" class="px-3 py-2 rounded-xl text-xs font-semibold bg-rose-600/15 hover:bg-rose-600 text-rose-300 hover:text-white border border-rose-500/30 transition inline-flex items-center gap-1.5 cursor-pointer">
                  <svg width="24" height="24" class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/></svg>
                  Delete from Homelab
                </button>
              ` : ''}
            </div>
            <div class="w-48">
              ${actionButtonHtml(item)}
            </div>
          </div>
        </div>
      `;
      modal.classList.remove('hidden');

      if (autoPlayTrailer && item.trailer_key) {
        setTimeout(() => playTrailerInModal(item.trailer_key), 100);
      }

      // Fetch Cast, Crew and Tags asynchronously
      fetch(`/api/details/${item.media_type}/${item.tmdb_id}`)
        .then(r => {
          if (!r.ok) throw new Error(`HTTP ${r.status}`);
          return r.json();
        })
        .then(details => {
          if (!details) return;

          // Crew
          const crewList = document.getElementById('modalCrewList');
          const crewSec = document.getElementById('modalCrewSection');
          if (details.crew && details.crew.length > 0) {
            if (crewList && crewSec) {
              crewList.innerHTML = details.crew.map(c =>
                `<span class="px-2.5 py-1 rounded-lg bg-gray-800/90 border border-gray-700/60 text-gray-200">
                  <strong class="text-indigo-400 font-semibold">${c.job}:</strong> ${c.name}
                </span>`
              ).join('');
              crewSec.classList.remove('hidden');
            }
          } else if (crewSec) {
            crewSec.classList.add('hidden');
          }

          // Cast
          const castList = document.getElementById('modalCastList');
          const castSec = document.getElementById('modalCastSection');
          if (details.cast && details.cast.length > 0) {
            if (castList && castSec) {
              castList.innerHTML = details.cast.map(c => {
                const safeName = (c.name || 'Actor').replace(/'/g, "\\'").replace(/"/g, '&quot;');
                const photo = c.profile_path
                  ? `<img src="https://image.tmdb.org/t/p/w185${c.profile_path}" alt="${safeName}" class="w-full h-full object-cover" onerror="this.outerHTML = getActorFallbackAvatar('${safeName}')" />`
                  : getActorFallbackAvatar(c.name);
                return `
                  <div onclick="filterByActor('${safeName}')" class="flex-shrink-0 w-24 flex flex-col items-center text-center cursor-pointer group/actor transition hover:scale-105" title="Filter titles starring ${safeName}">
                    <div class="w-14 h-14 rounded-full overflow-hidden bg-gray-800 border border-gray-700/80 mb-1.5 shadow-sm group-hover/actor:border-indigo-500 transition">
                      ${photo}
                    </div>
                    <span class="text-[11px] font-semibold text-gray-200 line-clamp-2 leading-tight w-full group-hover/actor:text-indigo-300 transition">${c.name}</span>
                    <span class="text-[10px] text-gray-400 line-clamp-2 leading-tight w-full mt-0.5">${c.character}</span>
                  </div>
                `;
              }).join('');
              castSec.classList.remove('hidden');
            }
          } else if (castSec) {
            castSec.classList.add('hidden');
          }

          // Tags
          if (details.keywords && details.keywords.length > 0) {
            const tagList = document.getElementById('modalTagList');
            const tagSec = document.getElementById('modalTagsSection');
            if (tagList && tagSec) {
              tagList.innerHTML = details.keywords.map(kw =>
                `<span onclick="closeModal(); document.getElementById('searchInput').value='${kw}'; renderGrid();" class="px-2 py-0.5 rounded-md bg-gray-800/80 hover:bg-indigo-950/80 hover:text-indigo-300 hover:border-indigo-600/50 text-gray-300 text-[11px] border border-gray-700/50 transition cursor-pointer">#${kw}</span>`
              ).join('');
              tagSec.classList.remove('hidden');
            }
          }
        })
        .catch(e => {
          console.warn('Details fetch error:', e);
          const castList = document.getElementById('modalCastList');
          if (castList) {
            castList.innerHTML = `<span class="text-xs text-gray-400 py-1">Cast details currently unavailable from TMDB</span>`;
          }
          const crewSec = document.getElementById('modalCrewSection');
          if (crewSec) {
            crewSec.classList.add('hidden');
          }
        });
    }

    function closeModal() {
      stopTrailerInModal();
      document.getElementById('detailModal').classList.add('hidden');
    }

    setWindowFilter('released');
    setTypeFilter('all');
    fetchCatalog();
    startHeroTimer();
    setInterval(fetchCatalog, 15000);
  </script>
</body>
</html>
"##
);

#[cfg(test)]
mod tests {
    use super::INDEX_HTML;

    #[test]
    fn layout_styles_are_embedded_without_the_runtime_cdn() {
        assert!(!INDEX_HTML.contains("cdn.tailwindcss.com"));
        assert!(!INDEX_HTML.contains("tailwind.config"));
        assert!(INDEX_HTML.contains(".hidden{display:none}"));
        assert!(INDEX_HTML.contains(".w-4{width:1rem}"));
        assert!(INDEX_HTML.contains(".h-4{height:1rem}"));
        assert!(INDEX_HTML.contains("sm\\:grid-cols-3"));
        assert!(INDEX_HTML.contains("<svg width=\"24\" height=\"24\""));
    }
}
