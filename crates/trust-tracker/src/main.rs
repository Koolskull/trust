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

use std::num::NonZeroU32;
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, SampleRate, StreamConfig};
use softbuffer::{Context, Surface};
use trust_tracker::{
    cell_at, dump, esc_at, layout, note_midi, paint, usage, Edit, Engine, Phrase, CHANNELS, EMPTY,
    ROWS, VERSION,
};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, MouseButton, TouchPhase, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, NamedKey, PhysicalKey};
use winit::window::{Fullscreen, Window, WindowId};

#[cfg(debug_assertions)]
#[global_allocator]
static TRUST_ALLOC: assert_no_alloc::AllocDisabler = assert_no_alloc::AllocDisabler;

const SCRATCH: usize = 8192;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match cli(&args) {
        Cli::Help => {
            print!("{}", usage());
            ExitCode::SUCCESS
        }
        Cli::Dump => {
            println!("{}", dump(&Phrase::new()));
            ExitCode::SUCCESS
        }
        Cli::Live => match live() {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("{err}");
                ExitCode::from(1)
            }
        },
        Cli::Unknown => {
            eprint!("{}", usage());
            ExitCode::from(2)
        }
    }
}

enum Cli {
    Help,
    Dump,
    Live,
    Unknown,
}

fn cli(args: &[String]) -> Cli {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        return Cli::Help;
    }
    if args.is_empty() {
        return Cli::Live;
    }
    if args.len() == 1 && args[0] == "--dump" {
        return Cli::Dump;
    }
    Cli::Unknown
}

fn live() -> Result<(), String> {
    let gate = Arc::new(Gate::new());
    let audio = match start_audio(Arc::clone(&gate)) {
        Ok(stream) => Some(stream),
        Err(err) => {
            eprintln!("audio: {err}");
            None
        }
    };
    let event_loop = EventLoop::new().map_err(|err| err.to_string())?;
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut stage = Stage {
        window: None,
        context: None,
        surface: None,
        phrase: Phrase::new(),
        gate,
        cursor: (0, 0),
        pen_down: false,
        _audio: audio,
    };
    event_loop
        .run_app(&mut stage)
        .map_err(|err| err.to_string())
}

struct Gate {
    notes: [AtomicU8; ROWS * CHANNELS],
    playing: AtomicBool,
    restart: AtomicBool,
    play_row: AtomicU8,
}

impl Gate {
    fn new() -> Self {
        Self {
            notes: std::array::from_fn(|_| AtomicU8::new(EMPTY)),
            playing: AtomicBool::new(false),
            restart: AtomicBool::new(false),
            play_row: AtomicU8::new(0),
        }
    }

    fn publish(&self, phrase: &Phrase, restart: bool) {
        let notes = phrase.notes();
        for row in 0..ROWS {
            for channel in 0..CHANNELS {
                self.notes[row * CHANNELS + channel].store(notes[row][channel], Ordering::Relaxed);
            }
        }
        if restart {
            self.restart.store(true, Ordering::Release);
        }
        self.playing.store(phrase.playing(), Ordering::Release);
    }
}

struct Stage {
    window: Option<Arc<Window>>,
    context: Option<Context<Arc<Window>>>,
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    phrase: Phrase,
    gate: Arc<Gate>,
    cursor: (i32, i32),
    pen_down: bool,
    _audio: Option<cpal::Stream>,
}

impl ApplicationHandler for Stage {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attrs = Window::default_attributes()
            .with_title(format!("THRASH {VERSION}"))
            .with_fullscreen(Some(Fullscreen::Borderless(None)));
        let window = match event_loop.create_window(attrs) {
            Ok(window) => Arc::new(window),
            Err(err) => {
                eprintln!("could not open the tracker window: {err}");
                event_loop.exit();
                return;
            }
        };
        let context = match Context::new(window.clone()) {
            Ok(context) => context,
            Err(err) => {
                eprintln!("could not open the tracker window: {err}");
                event_loop.exit();
                return;
            }
        };
        let surface = match Surface::new(&context, window.clone()) {
            Ok(surface) => surface,
            Err(err) => {
                eprintln!("could not open the tracker window: {err}");
                event_loop.exit();
                return;
            }
        };
        self.window = Some(window);
        self.context = Some(context);
        self.surface = Some(surface);
        self.redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => self.on_key(event_loop, event),
            WindowEvent::MouseInput { state, button, .. } => {
                if button != MouseButton::Left && button != MouseButton::Middle {
                    return;
                }
                if state == ElementState::Pressed {
                    self.pen_down = true;
                    if self.point(self.cursor.0, self.cursor.1) {
                        event_loop.exit();
                    }
                } else {
                    self.pen_down = false;
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor = (position.x as i32, position.y as i32);
                if self.pen_down && self.point(self.cursor.0, self.cursor.1) {
                    event_loop.exit();
                }
            }
            WindowEvent::Touch(touch) => {
                let x = touch.location.x as i32;
                let y = touch.location.y as i32;
                self.cursor = (x, y);
                match touch.phase {
                    TouchPhase::Started => {
                        self.pen_down = true;
                        if self.point(x, y) {
                            event_loop.exit();
                        }
                    }
                    TouchPhase::Moved => {
                        if self.pen_down && self.point(x, y) {
                            event_loop.exit();
                        }
                    }
                    TouchPhase::Ended | TouchPhase::Cancelled => self.pen_down = false,
                }
            }
            WindowEvent::RedrawRequested => self.paint(),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.phrase.playing() {
            self.redraw();
            event_loop.set_control_flow(ControlFlow::WaitUntil(
                Instant::now() + Duration::from_millis(33),
            ));
        } else {
            event_loop.set_control_flow(ControlFlow::Wait);
        }
    }
}

impl Stage {
    fn on_key(&mut self, event_loop: &ActiveEventLoop, event: KeyEvent) {
        if event.repeat || event.state != ElementState::Pressed {
            return;
        }
        let PhysicalKey::Code(code) = event.physical_key else {
            if event.logical_key == winit::keyboard::Key::Named(NamedKey::Escape) {
                event_loop.exit();
            }
            return;
        };
        if code == KeyCode::Escape {
            event_loop.exit();
            return;
        }
        let edit = match code {
            KeyCode::Space => Edit::Play,
            KeyCode::ArrowLeft => Edit::Left,
            KeyCode::ArrowRight => Edit::Right,
            KeyCode::ArrowUp => Edit::Up,
            KeyCode::ArrowDown => Edit::Down,
            KeyCode::Backspace | KeyCode::Delete => Edit::Clear,
            KeyCode::BracketLeft => Edit::OctaveDown,
            KeyCode::BracketRight => Edit::OctaveUp,
            other => match note_midi(key_token(other), self.phrase.octave()) {
                Some(note) => Edit::Note(note),
                None => return,
            },
        };
        let restart = self.phrase.edit(edit);
        self.gate.publish(&self.phrase, restart);
        self.redraw();
    }

    /// Returns true when the pen hit the on-screen ESC key.
    fn point(&mut self, x: i32, y: i32) -> bool {
        let Some(window) = &self.window else {
            return false;
        };
        let size = window.inner_size();
        let board = layout(size.width as i32, size.height as i32);
        if esc_at(&board, x, y) {
            return true;
        }
        if let Some((channel, row)) = cell_at(&board, x, y) {
            self.phrase.select(channel, row);
            self.gate.publish(&self.phrase, false);
            self.redraw();
        }
        false
    }

    fn redraw(&self) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn paint(&mut self) {
        let Some(window) = self.window.clone() else {
            return;
        };
        let size = window.inner_size();
        let width_px = size.width as i32;
        let height_px = size.height as i32;
        let play_row = if self.phrase.playing() {
            Some(usize::from(self.gate.play_row.load(Ordering::Relaxed)))
        } else {
            None
        };
        let phrase = self.phrase.clone();
        let Some(surface) = self.surface.as_mut() else {
            return;
        };
        let (Some(width), Some(height)) =
            (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
        else {
            return;
        };
        if surface.resize(width, height).is_err() {
            return;
        }
        let mut buffer = match surface.buffer_mut() {
            Ok(buffer) => buffer,
            Err(_) => return,
        };
        paint(&mut buffer, width_px, height_px, &phrase, play_row);
        let _ = buffer.present();
    }
}

fn key_token(code: KeyCode) -> &'static str {
    match code {
        KeyCode::KeyZ => "z",
        KeyCode::KeyS => "s",
        KeyCode::KeyX => "x",
        KeyCode::KeyD => "d",
        KeyCode::KeyC => "c",
        KeyCode::KeyV => "v",
        KeyCode::KeyG => "g",
        KeyCode::KeyB => "b",
        KeyCode::KeyH => "h",
        KeyCode::KeyN => "n",
        KeyCode::KeyJ => "j",
        KeyCode::KeyM => "m",
        KeyCode::KeyQ => "q",
        KeyCode::Digit2 => "2",
        KeyCode::KeyW => "w",
        KeyCode::Digit3 => "3",
        KeyCode::KeyE => "e",
        KeyCode::KeyR => "r",
        KeyCode::Digit5 => "5",
        KeyCode::KeyT => "t",
        KeyCode::Digit6 => "6",
        KeyCode::KeyY => "y",
        KeyCode::Digit7 => "7",
        KeyCode::KeyU => "u",
        _ => "",
    }
}

fn start_audio(gate: Arc<Gate>) -> Result<cpal::Stream, String> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or("no default output device")?;
    let range = device
        .supported_output_configs()
        .map_err(|err| err.to_string())?
        .find(|config| config.sample_format() == SampleFormat::F32)
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
    let mut engine = Engine::new(rate);
    let mut out_l = vec![0.0f32; SCRATCH];
    let mut out_r = vec![0.0f32; SCRATCH];
    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _| {
                let mut notes = [[EMPTY; CHANNELS]; ROWS];
                for row in 0..ROWS {
                    for channel in 0..CHANNELS {
                        notes[row][channel] =
                            gate.notes[row * CHANNELS + channel].load(Ordering::Relaxed);
                    }
                }
                let playing = gate.playing.load(Ordering::Acquire);
                let restart = gate.restart.swap(false, Ordering::AcqRel);
                if restart {
                    assert_no_alloc::assert_no_alloc(|| engine.reset());
                }
                let frames = data.len() / channels;
                let mut offset = 0;
                while offset < frames {
                    let n = (frames - offset).min(out_l.len());
                    assert_no_alloc::assert_no_alloc(|| {
                        engine.render(playing, &notes, &mut out_l[..n], &mut out_r[..n]);
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
                gate.play_row.store(engine.row() as u8, Ordering::Relaxed);
            },
            |err| eprintln!("audio: {err}"),
            None,
        )
        .map_err(|err| err.to_string())?;
    stream.play().map_err(|err| err.to_string())?;
    Ok(stream)
}
