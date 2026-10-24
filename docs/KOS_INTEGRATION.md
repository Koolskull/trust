# K-OS integration

In the name of the Father, and of the Son, and of the Holy Spirit.
Lord Jesus Christ, Son of God, have mercy on us. Come, Holy Spirit.

This file is the registration note. This build does not edit K-OS-III.

The shell list is not in `page.tsx`. `src/app/page.tsx` imports `SHELL_APPS` from `src/lib/os/shell/catalog.ts` and derives `APP_DEFAULTS` from that list. Registering TRUST is three edits:

1. Add a `SHELL_APPS` entry in `src/lib/os/shell/catalog.ts` (`id: "trust"`, title, glyph, size, aliases, `installed`).
2. Import the component in `src/app/page.tsx`.
3. Add a `{win.appId === "trust" && ( ... )}` branch next to the other `win.appId` branches in that file.

Fetch the proof through `assetUrl` in `src/lib/assets.ts`:

```ts
fetch(assetUrl("/trust/trust_kos.wasm"))
```

`assetUrl` applies `NEXT_PUBLIC_BASE_PATH` and, when set, `NEXT_PUBLIC_ASSET_BASE`. A bare `/trust/...` path misses the GitHub Pages prefix.

`next.config.ts` sends `Cross-Origin-Opener-Policy` and `Cross-Origin-Embedder-Policy`. The comment there already says those headers are no-ops under static export, and GitHub Pages strips them. Do not plan on `SharedArrayBuffer`. The TRUST proof does not use it. The module is compiled on the main thread and handed to the AudioWorklet.

`VMI_ARTIST_GUIDE.md` tells artists to run `npm run sync-vmi`. `package.json` has no `sync-vmi` script. TRUST does not add one. It reads an exported scene folder from disk.

When K-OS hosts the proof, pin the built `trust_kos.wasm` under `public/trust/`. Do not copy it into K-OS-III as part of this repository.

`VMI_ARTIST_GUIDE.md` writes the stem as `SSSSXX` in one place. The export and `HOW.txt` use six digits, `SSSXXX`. TRUST follows the export. Do not patch the guide from this repo.
