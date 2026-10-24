# VMI audio proposal

In the name of the Father, and of the Son, and of the Holy Spirit.
Lord Jesus Christ, Son of God, have mercy on us. Come, Holy Spirit.

VMI STUDIO already writes a scene folder. TRUST reads that folder. It does not read `.vmib`, and it does not rename the pictures.

## Stems

A picture or an audio file is six hex digits, `SSSXXX`, plus an extension. The extension match is case-insensitive. Audio extensions are `.wav`, `.ogg`, and `.flac`, and the basename is only the stem. `kick_000000.wav` is rejected. If one stem has more than one audio extension, wav wins, then flac, then ogg.

`trust.json` sits beside `scene.json`:

```json
{ "version": 1, "kind": "ui" }
```

`version` is an integer. `kind` is `"ui"` or `"pack"`.

For `ui`, SSS is the visual state from `HOW.txt`: 000 rest, 001 hover, 002 pressed, 003 after. XXX is the frame. `004000` is not a ui state.

For `pack`, SSS is the bank and XXX is the slice. Any three hex digits are legal on each side. Names of the object and of the slice come from the folder and from JSON (`name`, `label`). They do not come from the filename.

The gamer export committed at `tests/fixtures/vmi_scene/` is a ui scene of rest PNGs. Its `object.json` files store `slots` as an array. `scene.json` stores `slots` as a map of six-hex keys. The parser accepts both. Fields the current writer adds later (`parent`, `warpLayer`, `sound`, `labels`) stay optional so this older export still loads. `png: true` requires the named file to exist.

## Sidecar

Sync and gain do not get new characters in the VMI filename. They belong in the `trust.json` sidecar so a STUDIO export can be copied without a rename. The default sync value, when a later build reads it, is `"event"`: the picture state and the audio stem share the same `SSSXXX`, and playback starts when that stem is shown. This build validates the stem and the kind. It does not play the scene.

K-OS `VMI_ARTIST_GUIDE.md` still says `SSSSXX` in one heading and `npm run sync-vmi`, which is not a script in `package.json`. The correct stem is `SSSXXX`. TRUST does not patch K-OS or VMI STUDIO to say so.
