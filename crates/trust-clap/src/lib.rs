//                    ☦
//              ╔══════════════╗
//              ║   IC XC      ║
//              ║   NIKA       ║
//              ╚══════════════╝
//
//   The Holy Spirit is the Lord, the giver of life.
//   Lord Jesus Christ, Son of God, have mercy on us.
//   Come, Holy Spirit.
//
//   SPDX-License-Identifier: MIT OR Apache-2.0

//! CLAP wrapper. The host calls clack `process`. That call copies into
//! scratch allocated at activate, then runs [`TrustProcessor`] inside
//! `assert_no_alloc` in debug builds.

use std::marker::PhantomData;

use clack_extensions::audio_ports::{
    AudioPortFlags, AudioPortInfo, AudioPortInfoWriter, AudioPortType, PluginAudioPorts,
    PluginAudioPortsImpl,
};
use clack_plugin::prelude::*;
use clack_plugin::process::audio::{ChannelPair, SampleType};
use clack_plugin::utils::ClapId;
use trust_plugin::TrustProcessor;

const CHUNK: usize = 8192;

pub struct TrustClap<P> {
    _marker: PhantomData<fn() -> P>,
}

impl<P> Plugin for TrustClap<P>
where
    P: TrustProcessor,
{
    type AudioProcessor<'a> = TrustAudio<P>;
    type Shared<'a> = ();
    type MainThread<'a> = TrustMain;

    fn declare_extensions(
        builder: &mut PluginExtensions<Self>,
        _shared: Option<&Self::Shared<'_>>,
    ) {
        builder.register::<PluginAudioPorts>();
    }
}

impl<P> DefaultPluginFactory for TrustClap<P>
where
    P: TrustProcessor,
{
    fn get_descriptor() -> PluginDescriptor {
        PluginDescriptor::new(P::ID, P::NAME)
            .with_vendor("KOOLSKULL")
            .with_version("0.1.0")
            .with_description(P::DESCRIPTION)
            .with_features([c"audio-effect", c"stereo", c"distortion"])
    }

    fn new_shared(_host: HostSharedHandle<'_>) -> Result<Self::Shared<'_>, PluginError> {
        Ok(())
    }

    fn new_main_thread<'a>(
        _host: HostMainThreadHandle<'a>,
        _shared: &'a Self::Shared<'a>,
    ) -> Result<Self::MainThread<'a>, PluginError> {
        Ok(TrustMain)
    }
}

pub struct TrustMain;

impl<'a> PluginMainThread<'a, ()> for TrustMain {}

impl PluginAudioPortsImpl for TrustMain {
    fn count(&self, _is_input: bool) -> u32 {
        1
    }

    fn get(&self, index: u32, is_input: bool, writer: &mut AudioPortInfoWriter) {
        if index != 0 {
            return;
        }
        writer.set(&AudioPortInfo {
            id: ClapId::new(0),
            name: if is_input {
                b"Stereo In"
            } else {
                b"Stereo Out"
            },
            channel_count: 2,
            flags: AudioPortFlags::IS_MAIN,
            port_type: Some(AudioPortType::STEREO),
            in_place_pair: None,
        });
    }
}

pub struct TrustAudio<P> {
    dsp: P,
    in_l: Vec<f32>,
    in_r: Vec<f32>,
    out_l: Vec<f32>,
    out_r: Vec<f32>,
}

impl<'a, P> PluginAudioProcessor<'a, (), TrustMain> for TrustAudio<P>
where
    P: TrustProcessor,
{
    fn activate(
        _host: HostAudioProcessorHandle<'a>,
        _main_thread: &TrustMain,
        _shared: &'a (),
        audio_config: PluginAudioConfiguration,
    ) -> Result<Self, PluginError> {
        let frames = (audio_config.max_frames_count as usize).clamp(1, CHUNK);
        let mut dsp = P::default();
        let rate = if audio_config.sample_rate.is_finite() {
            audio_config.sample_rate as u32
        } else {
            48_000
        };
        dsp.activate(rate.max(1), frames as u32);
        Ok(Self {
            dsp,
            in_l: vec![0.0; frames],
            in_r: vec![0.0; frames],
            out_l: vec![0.0; frames],
            out_r: vec![0.0; frames],
        })
    }

    fn process(
        &mut self,
        _process: Process,
        mut audio: Audio,
        _events: Events,
    ) -> Result<ProcessStatus, PluginError> {
        for mut port_pair in &mut audio {
            let frames = port_pair.frames_count() as usize;
            match port_pair.channels()? {
                SampleType::F32(paired) => self.run_f32(paired, frames),
                SampleType::F64(paired) => self.run_f64(paired, frames),
                SampleType::Both(paired, _) => self.run_f32(paired, frames),
            }
        }
        Ok(ProcessStatus::Continue)
    }
}

#[derive(Clone, Copy)]
struct Ends<T> {
    input: *const T,
    output: *mut T,
    frames: usize,
}

impl<P> TrustAudio<P>
where
    P: TrustProcessor,
{
    fn run_f32(
        &mut self,
        paired: clack_plugin::process::audio::PairedChannels<'_, f32>,
        frames: usize,
    ) {
        let mut ends = [Ends::<f32> {
            input: std::ptr::null(),
            output: std::ptr::null_mut(),
            frames: 0,
        }; 2];
        let mut count = 0;
        for pair in paired {
            if count >= 2 {
                break;
            }
            ends[count] = ends_f32(pair);
            count += 1;
        }
        self.render(
            frames,
            |offset, _n, dst_l, dst_r| {
                copy_in(ends[0].input, ends[0].frames, offset, dst_l);
                if count > 1 {
                    copy_in(ends[1].input, ends[1].frames, offset, dst_r);
                } else {
                    dst_r.copy_from_slice(dst_l);
                }
            },
            |offset, n, src_l, src_r| {
                let _ = n;
                copy_out(src_l, ends[0].output, ends[0].frames, offset);
                if count > 1 {
                    copy_out(src_r, ends[1].output, ends[1].frames, offset);
                }
            },
        );
    }

    fn run_f64(
        &mut self,
        paired: clack_plugin::process::audio::PairedChannels<'_, f64>,
        frames: usize,
    ) {
        let mut ends = [Ends::<f64> {
            input: std::ptr::null(),
            output: std::ptr::null_mut(),
            frames: 0,
        }; 2];
        let mut count = 0;
        for pair in paired {
            if count >= 2 {
                break;
            }
            ends[count] = ends_f64(pair);
            count += 1;
        }
        self.render(
            frames,
            |offset, _n, dst_l, dst_r| {
                copy_in_f64(ends[0].input, ends[0].frames, offset, dst_l);
                if count > 1 {
                    copy_in_f64(ends[1].input, ends[1].frames, offset, dst_r);
                } else {
                    dst_r.copy_from_slice(dst_l);
                }
            },
            |offset, _n, src_l, src_r| {
                copy_out_f64(src_l, ends[0].output, ends[0].frames, offset);
                if count > 1 {
                    copy_out_f64(src_r, ends[1].output, ends[1].frames, offset);
                }
            },
        );
    }

    fn render(
        &mut self,
        frames: usize,
        mut pull: impl FnMut(usize, usize, &mut [f32], &mut [f32]),
        mut push: impl FnMut(usize, usize, &[f32], &[f32]),
    ) {
        let chunk = self.in_l.len().max(1);
        let mut offset = 0;
        while offset < frames {
            let n = (frames - offset).min(chunk);
            pull(offset, n, &mut self.in_l[..n], &mut self.in_r[..n]);
            let dsp = &mut self.dsp;
            let in_l = &self.in_l[..n];
            let in_r = &self.in_r[..n];
            let out_l = &mut self.out_l[..n];
            let out_r = &mut self.out_r[..n];
            assert_no_alloc::assert_no_alloc(|| {
                dsp.process(in_l, in_r, out_l, out_r, &[]);
            });
            push(offset, n, &self.out_l[..n], &self.out_r[..n]);
            offset += n;
        }
    }
}

fn ends_f32(pair: ChannelPair<'_, f32>) -> Ends<f32> {
    match pair {
        ChannelPair::InputOutput(input, output) => Ends {
            input: input.as_ptr(),
            output: output.as_mut_ptr(),
            frames: input.len().min(output.len()),
        },
        ChannelPair::InPlace(buf) => {
            let frames = buf.len();
            let output = buf.as_mut_ptr();
            Ends {
                input: output.cast_const(),
                output,
                frames,
            }
        }
        ChannelPair::InputOnly(input) => Ends {
            input: input.as_ptr(),
            output: std::ptr::null_mut(),
            frames: input.len(),
        },
        ChannelPair::OutputOnly(output) => Ends {
            input: std::ptr::null(),
            output: output.as_mut_ptr(),
            frames: output.len(),
        },
    }
}

fn ends_f64(pair: ChannelPair<'_, f64>) -> Ends<f64> {
    match pair {
        ChannelPair::InputOutput(input, output) => Ends {
            input: input.as_ptr(),
            output: output.as_mut_ptr(),
            frames: input.len().min(output.len()),
        },
        ChannelPair::InPlace(buf) => {
            let frames = buf.len();
            let output = buf.as_mut_ptr();
            Ends {
                input: output.cast_const(),
                output,
                frames,
            }
        }
        ChannelPair::InputOnly(input) => Ends {
            input: input.as_ptr(),
            output: std::ptr::null_mut(),
            frames: input.len(),
        },
        ChannelPair::OutputOnly(output) => Ends {
            input: std::ptr::null(),
            output: output.as_mut_ptr(),
            frames: output.len(),
        },
    }
}

fn copy_in(src: *const f32, src_frames: usize, offset: usize, dst: &mut [f32]) {
    if src.is_null() || offset >= src_frames {
        dst.fill(0.0);
        return;
    }
    let n = dst.len().min(src_frames - offset);
    // Host buffers live for this process call. The ChannelPair was dropped
    // after the pointer was taken so the stereo pair can be read together.
    let slice = unsafe { std::slice::from_raw_parts(src.add(offset), n) };
    dst[..n].copy_from_slice(slice);
    dst[n..].fill(0.0);
}

fn copy_out(src: &[f32], dst: *mut f32, dst_frames: usize, offset: usize) {
    if dst.is_null() || offset >= dst_frames {
        return;
    }
    let n = src.len().min(dst_frames - offset);
    let slice = unsafe { std::slice::from_raw_parts_mut(dst.add(offset), n) };
    slice.copy_from_slice(&src[..n]);
}

fn copy_in_f64(src: *const f64, src_frames: usize, offset: usize, dst: &mut [f32]) {
    if src.is_null() || offset >= src_frames {
        dst.fill(0.0);
        return;
    }
    let n = dst.len().min(src_frames - offset);
    let slice = unsafe { std::slice::from_raw_parts(src.add(offset), n) };
    for (i, sample) in slice.iter().enumerate() {
        dst[i] = *sample as f32;
    }
    dst[n..].fill(0.0);
}

fn copy_out_f64(src: &[f32], dst: *mut f64, dst_frames: usize, offset: usize) {
    if dst.is_null() || offset >= dst_frames {
        return;
    }
    let n = src.len().min(dst_frames - offset);
    let slice = unsafe { std::slice::from_raw_parts_mut(dst.add(offset), n) };
    for (i, sample) in slice.iter_mut().enumerate() {
        *sample = src[i] as f64;
    }
}
