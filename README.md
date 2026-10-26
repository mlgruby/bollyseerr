# BollySeerr

Bollywood & Hindi Movie and Web Series Discovery Agent and Request UI for Jellyseerr (Seerr), Radarr, and Sonarr.

## Overview

BollySeerr continuously discovers Hindi-language and Pan-India Hindi-dubbed movies and OTT web series (excluding daily TV soaps), keeps a rolling 3-month window of released titles alongside upcoming releases, automatically prunes titles older than 90 days after release, and syncs live status with your homelab Seerr, Radarr, and Sonarr instances for 1-click requesting.

### UI stylesheet

The Tailwind stylesheet is compiled and embedded in the Rust binary. Layout and
icon sizing work without a browser-side Tailwind CDN or JavaScript compiler.
To update utility classes, run `npm ci && npm run build:css` and commit
`assets/tailwind.css`. CI regenerates the file to prevent stale styles. Rust and
Docker builds consume the checked-in stylesheet; Node is only needed to update it.
