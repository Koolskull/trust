# Provenance

In the name of the Father, and of the Son, and of the Holy Spirit.
Lord Jesus Christ, Son of God, have mercy on us. Come, Holy Spirit.

## This build

TRUST is a clean-room instrument framework. KRUSH, the graph, the CLAP wrapper, the host, the VMI reader, and the WASM proof were written for this repository. clack and clap-sys keep their MIT OR Apache-2.0 terms. `assert_no_alloc` is BSD-1-Clause. The gamer scene under `tests/fixtures/vmi_scene/` is a copy of a VMI STUDIO export made on this machine (`Documents/work/CSP/BG/gamer`). `tests/fixtures/vmi_pack/` is synthetic silence, written so the pack rules have a fixture. It is not a studio export.

## Sliced loops

A sliced-loop file format is deferred. It is not implemented in this tree. No public name for it is settled. Any name has to clear a trademark check before it is used in a public crate, a file extension, or a document that ships as the product name.

The clean-room wall for that future format:

- Do not read ReCycle source, the REX SDK, or proprietary loop files as a specification.
- Do not copy REX chunk layouts, file extensions, or SDK types.
- When the format is eventually designed, slice tail behavior is an integer choice: cut at the next slice, ring out, or fade for a duration stored in integer milliseconds.
- Until that design exists, this repository has no slice container, no crate for one, and no sample files in that format.

VST is a Steinberg trademark. See `docs/VST3_LICENSE.md`. This build does not ship a VST3 binary.
