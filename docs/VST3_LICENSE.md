# VST3 license

In the name of the Father, and of the Son, and of the Holy Spirit.
Lord Jesus Christ, Son of God, have mercy on us. Come, Holy Spirit.

TRUST does not link VST3 in this build.

`vst3-sys` (the RustAudio bindings) is GPL-3.0-or-later. Depending on it would pull that license over the plugin. This workspace does not depend on it.

The Steinberg VST 3 SDK, from 3.8 (2025), distributes the interface headers (`pluginterfaces` and the SDK helpers) under the MIT license. The GPLv3 offer and the older proprietary Steinberg VST3 license are no longer the terms for that SDK. VSTGUI and the mda examples stay on their own BSD-style licenses. VST is a Steinberg trademark. Using the interface does not grant the trademark.

The newer `vst3` crate (MIT OR Apache-2.0 bindings, aimed at the MIT SDK) is the permissive Rust option if a later build wraps VST3 directly. This build does not add it.

Audio Unit comes later, through clap-wrapper, which takes the CLAP plugin and produces the other formats. That wrapper is not vendored here. KRUSH ships as CLAP, via clack (`MIT OR Apache-2.0`), and as the standalone cpal host.
