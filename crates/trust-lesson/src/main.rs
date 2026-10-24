//                    ☦
//         ╔════════════════════╗
//         ║  Let the children  ║
//         ║  come to me.       ║
//         ╚════════════════════╝
//
//   Mark 10:14. Lord Jesus Christ, Son of God, have mercy on us.
//   Come, Holy Spirit.
//
//   SPDX-License-Identifier: MIT OR Apache-2.0

use std::io::{self, IsTerminal, Write};
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU8, Ordering};
use std::sync::Arc;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, SampleRate, StreamConfig};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{execute, queue};
use trust_lesson::{parse_cli, render, render_block, App, Cli, Key, Sound, Speaker, Voice};

#[cfg(debug_assertions)]
#[global_allocator]
static TRUST_ALLOC: assert_no_alloc::AllocDisabler = assert_no_alloc::AllocDisabler;

const SCRATCH: usize = 8192;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match parse_cli(&args) {
        Ok(Cli::Help) => {
            print!("{}", trust_lesson::usage_text());
            ExitCode::SUCCESS
        }
        Ok(Cli::Run(launch)) => match run(launch) {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("{err}");
                ExitCode::from(1)
            }
        },
        Err(err) => {
            eprintln!("{}", err.message);
            ExitCode::from(2)
        }
    }
}

fn run(launch: trust_lesson::Launch) -> Result<(), String> {
    let mut app = App::open(launch.age);
    if !launch.keys.is_empty() {
        for key in &launch.keys {
            app.press(*key);
            println!("{}", render(&app));
            if app.quit {
                break;
            }
        }
        return Ok(());
    }
    if launch.dump || !io::stdin().is_terminal() {
        println!("{}", render(&app));
        if !launch.dump && !io::stdin().is_terminal() {
            eprintln!("Open a terminal to press the keys.");
        }
        return Ok(());
    }
    live(&mut app)
}

fn live(app: &mut App) -> Result<(), String> {
    let controls = Arc::new(Controls::from_sound(&app.sound));
    let speaker = match start_audio(Arc::clone(&controls), app.age) {
        Ok(stream) => {
            app.speaker = Speaker::On;
            Some(stream)
        }
        Err(_) => {
            app.speaker = Speaker::Off;
            None
        }
    };
    let _guard = TermGuard::enter()?;
    loop {
        draw(app)?;
        if !event::poll(Duration::from_millis(200)).map_err(|e| e.to_string())? {
            continue;
        }
        let event = event::read().map_err(|e| e.to_string())?;
        let Event::Key(key) = event else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }
        let parsed = match key.code {
            KeyCode::Char(c) => Key::Char(c),
            KeyCode::Esc => Key::Quit,
            _ => continue,
        };
        app.press(parsed);
        controls.store(&app.sound);
        if app.quit {
            break;
        }
    }
    drop(speaker);
    Ok(())
}

fn draw(app: &App) -> Result<(), String> {
    let mut stdout = io::stdout();
    queue!(stdout, MoveTo(0, 0)).map_err(|e| e.to_string())?;
    let frame = render(app);
    for line in frame.lines() {
        queue!(
            stdout,
            crossterm::style::Print(line),
            crossterm::style::Print("\r\n")
        )
        .map_err(|e| e.to_string())?;
    }
    queue!(
        stdout,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::FromCursorDown)
    )
    .map_err(|e| e.to_string())?;
    stdout.flush().map_err(|e| e.to_string())
}

struct TermGuard;

impl TermGuard {
    fn enter() -> Result<Self, String> {
        enable_raw_mode().map_err(|e| e.to_string())?;
        execute!(io::stdout(), EnterAlternateScreen, Hide).map_err(|e| e.to_string())?;
        Ok(Self)
    }
}

impl Drop for TermGuard {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), LeaveAlternateScreen, Show);
        let _ = disable_raw_mode();
    }
}

struct Controls {
    playing: AtomicBool,
    bits: AtomicU8,
    rate_hz: AtomicU32,
    gain_milli: AtomicI32,
    mix_milli: AtomicI32,
    ceiling_db_milli: AtomicI32,
    freq_hz: AtomicU32,
}

impl Controls {
    fn from_sound(sound: &Sound) -> Self {
        Self {
            playing: AtomicBool::new(sound.playing),
            bits: AtomicU8::new(sound.bits),
            rate_hz: AtomicU32::new(sound.rate_hz),
            gain_milli: AtomicI32::new(sound.gain_milli),
            mix_milli: AtomicI32::new(sound.mix_milli),
            ceiling_db_milli: AtomicI32::new(sound.ceiling_db_milli),
            freq_hz: AtomicU32::new(sound.freq_hz),
        }
    }

    fn store(&self, sound: &Sound) {
        self.playing.store(sound.playing, Ordering::Relaxed);
        self.bits.store(sound.bits, Ordering::Relaxed);
        self.rate_hz.store(sound.rate_hz, Ordering::Relaxed);
        self.gain_milli.store(sound.gain_milli, Ordering::Relaxed);
        self.mix_milli.store(sound.mix_milli, Ordering::Relaxed);
        self.ceiling_db_milli
            .store(sound.ceiling_db_milli, Ordering::Relaxed);
        self.freq_hz.store(sound.freq_hz, Ordering::Relaxed);
    }

    fn load(&self) -> Sound {
        Sound {
            playing: self.playing.load(Ordering::Relaxed),
            bits: self.bits.load(Ordering::Relaxed),
            rate_hz: self.rate_hz.load(Ordering::Relaxed),
            gain_milli: self.gain_milli.load(Ordering::Relaxed),
            mix_milli: self.mix_milli.load(Ordering::Relaxed),
            ceiling_db_milli: self.ceiling_db_milli.load(Ordering::Relaxed),
            freq_hz: self.freq_hz.load(Ordering::Relaxed),
        }
    }
}

fn start_audio(controls: Arc<Controls>, age: u8) -> Result<cpal::Stream, String> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or("no default output device")?;
    let range = device
        .supported_output_configs()
        .map_err(|e| e.to_string())?
        .find(|r| r.sample_format() == SampleFormat::F32)
        .ok_or("output device has no f32 config")?;
    let supported = if range.min_sample_rate().0 <= 48_000 && range.max_sample_rate().0 >= 48_000 {
        range.with_sample_rate(SampleRate(48_000))
    } else {
        range.with_max_sample_rate()
    };
    let rate = supported.sample_rate().0.max(1);
    let channels = supported.channels() as usize;
    if channels == 0 {
        return Err("output device has no channels".into());
    }
    let config: StreamConfig = supported.into();
    let mut voice = Voice::new(rate);
    let mut out_l = vec![0.0f32; SCRATCH];
    let mut out_r = vec![0.0f32; SCRATCH];
    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _| {
                let sound = controls.load();
                let frames = data.len() / channels;
                let mut offset = 0;
                while offset < frames {
                    let n = (frames - offset).min(out_l.len());
                    assert_no_alloc::assert_no_alloc(|| {
                        render_block(&mut voice, &sound, age, &mut out_l[..n], &mut out_r[..n]);
                    });
                    for i in 0..n {
                        let base = (offset + i) * channels;
                        data[base] = out_l[i];
                        if channels > 1 {
                            data[base + 1] = out_r[i];
                        }
                    }
                    offset += n;
                }
            },
            |err| eprintln!("audio: {err}"),
            None,
        )
        .map_err(|e| e.to_string())?;
    stream.play().map_err(|e| e.to_string())?;
    Ok(stream)
}
