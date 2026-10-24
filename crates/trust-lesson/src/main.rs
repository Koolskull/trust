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
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU8, Ordering};
use std::sync::Arc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, SampleRate, StreamConfig};
use softbuffer::{Context, Surface};
use trust_lesson::{
    action_for, book_spots, digit_choice, knob_for, label_for, layout_for, lesson_cards,
    mode_from_id, paint, paint_book, parse_cli, played_sound, press_choice, render, render_block,
    selected_choices, silly_name, spot_at, Action, App, BoardMode, Cli, Knob, Sound, Voice,
    BACK_ID, EXIT_ID, FULL_CLASS_AGE, LESSON_ID, READING_AGE,
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
    if launch.dump {
        println!("{}", render(&app));
        println!("Big letters come first. Tap NEXT. PLAY opens the keys. ESC leaves.");
        return Ok(());
    }
    live(app.age)
}

fn live(age: u8) -> Result<(), String> {
    let controls = Arc::new(Controls::from_sound(&Sound::quiet()));
    let audio = start_audio(Arc::clone(&controls), age).ok();
    let event_loop = EventLoop::new().map_err(|err| err.to_string())?;
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut stage = Stage {
        age,
        mode: BoardMode::Lesson,
        window: None,
        context: None,
        surface: None,
        keyboard: Vec::new(),
        pointer: None,
        cursor: (0, 0),
        last: "lesson".to_string(),
        controls,
        _audio: audio,
        page: 0,
        on_board: false,
        picked: false,
        knob: knob_for(age, BoardMode::Lesson),
    };
    event_loop
        .run_app(&mut stage)
        .map_err(|err| err.to_string())
}

struct Stage {
    age: u8,
    mode: BoardMode,
    window: Option<Arc<Window>>,
    context: Option<Context<Arc<Window>>>,
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    keyboard: Vec<u16>,
    pointer: Option<u16>,
    cursor: (i32, i32),
    last: String,
    controls: Arc<Controls>,
    _audio: Option<cpal::Stream>,
    page: usize,
    on_board: bool,
    picked: bool,
    knob: Knob,
}

impl Stage {
    fn sync_sound(&self) {
        if !self.on_board {
            let mut quiet = Sound::quiet();
            quiet.playing = false;
            self.controls.store(&quiet);
            return;
        }
        let id = self
            .pointer
            .filter(|id| !self.choice_id(*id) && mode_from_id(*id).is_none() && *id != LESSON_ID)
            .or_else(|| self.keyboard.last().copied());
        let sound = match id {
            Some(id) => played_sound(self.age, id, self.mode, &self.knob),
            None => {
                let mut quiet = Sound::quiet();
                quiet.playing = false;
                quiet
            }
        };
        self.controls.store(&sound);
    }

    fn choice_id(&self, id: u16) -> bool {
        press_choice(self.age, self.mode, self.knob, id).is_some()
    }

    fn cards(&self) -> &'static [trust_lesson::Card] {
        let class = if self.age >= FULL_CLASS_AGE && self.picked {
            Some(self.mode.name())
        } else {
            None
        };
        lesson_cards(self.age, class)
    }

    fn chooser(&self) -> bool {
        let cards = self.cards();
        self.age >= FULL_CLASS_AGE && !self.picked && self.page + 1 >= cards.len().max(1)
    }

    fn go_next(&mut self) {
        if self.chooser() {
            self.redraw();
            return;
        }
        let len = self.cards().len();
        if self.page + 1 < len {
            self.page += 1;
        } else {
            self.on_board = true;
            self.keyboard.clear();
            self.pointer = None;
        }
        self.sync_sound();
        self.redraw();
    }

    fn go_back(&mut self) {
        if self.on_board {
            self.on_board = false;
            self.keyboard.clear();
            self.pointer = None;
            self.sync_sound();
            self.redraw();
            return;
        }
        if self.page > 0 {
            self.page -= 1;
        } else if self.picked {
            self.picked = false;
            self.mode = BoardMode::Lesson;
            self.page = self.cards().len().saturating_sub(1);
        }
        self.sync_sound();
        self.redraw();
    }

    fn open_class(&mut self, mode: BoardMode) {
        self.mode = mode;
        self.picked = true;
        self.page = 0;
        self.on_board = false;
        self.knob = knob_for(self.age, mode);
        self.last = mode.name().to_string();
        self.pointer = None;
        self.keyboard.clear();
        self.sync_sound();
        self.redraw();
    }

    fn down(&mut self, id: u16) {
        if id == EXIT_ID || id == LESSON_ID || id == BACK_ID {
            return;
        }
        if let Some(knob) = press_choice(self.age, self.mode, self.knob, id) {
            self.knob = knob;
            self.last = label_for(id).to_string();
            self.sync_sound();
            return;
        }
        if let Some(choice) = digit_choice(self.age, self.mode, id) {
            if let Some(knob) = press_choice(self.age, self.mode, self.knob, choice) {
                self.knob = knob;
            }
        }
        if !self.keyboard.contains(&id) {
            self.keyboard.push(id);
        }
        self.last = if self.age < READING_AGE {
            silly_name(id).to_string()
        } else {
            label_for(id).to_string()
        };
        self.sync_sound();
    }

    fn up(&mut self, id: u16) {
        self.keyboard.retain(|held| *held != id);
        self.sync_sound();
    }

    fn redraw(&self) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

impl ApplicationHandler for Stage {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attrs = Window::default_attributes()
            .with_title("KOS TRUST LESSON")
            .with_fullscreen(Some(Fullscreen::Borderless(None)));
        let window = match event_loop.create_window(attrs) {
            Ok(window) => Arc::new(window),
            Err(err) => {
                eprintln!("could not open the lesson window: {err}");
                event_loop.exit();
                return;
            }
        };
        let context = match Context::new(window.clone()) {
            Ok(context) => context,
            Err(err) => {
                eprintln!("could not open the lesson window: {err}");
                event_loop.exit();
                return;
            }
        };
        let surface = match Surface::new(&context, window.clone()) {
            Ok(surface) => surface,
            Err(err) => {
                eprintln!("could not open the lesson window: {err}");
                event_loop.exit();
                return;
            }
        };
        self.window = Some(window);
        self.context = Some(context);
        self.surface = Some(surface);
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
                    if self.point(self.cursor.0, self.cursor.1) {
                        event_loop.exit();
                    }
                } else {
                    self.pointer = None;
                    self.sync_sound();
                    self.redraw();
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor = (position.x as i32, position.y as i32);
                if self.on_board
                    && self.pointer.is_some()
                    && self.point(self.cursor.0, self.cursor.1)
                {
                    event_loop.exit();
                }
            }
            WindowEvent::Touch(touch) => {
                let x = touch.location.x as i32;
                let y = touch.location.y as i32;
                self.cursor = (x, y);
                match touch.phase {
                    TouchPhase::Started => {
                        if self.point(x, y) {
                            event_loop.exit();
                        }
                    }
                    TouchPhase::Moved => {
                        if self.on_board && self.point(x, y) {
                            event_loop.exit();
                        }
                    }
                    TouchPhase::Ended | TouchPhase::Cancelled => {
                        self.pointer = None;
                        self.sync_sound();
                        self.redraw();
                    }
                }
            }
            WindowEvent::RedrawRequested => self.paint(),
            _ => {}
        }
    }
}

impl Stage {
    fn on_key(&mut self, event_loop: &ActiveEventLoop, event: KeyEvent) {
        if event.repeat {
            return;
        }
        let token = token_from_key(&event);
        if !self.on_board {
            if event.state != ElementState::Pressed {
                return;
            }
            match action_for(&token) {
                Action::Exit => event_loop.exit(),
                Action::Play(id) if is_back_key(id) => self.go_back(),
                Action::Play(_) | Action::Mode(_) => self.go_next(),
            }
            return;
        }
        match action_for(&token) {
            Action::Exit if event.state == ElementState::Pressed => event_loop.exit(),
            Action::Exit => {}
            Action::Mode(mode) if event.state == ElementState::Pressed => {
                self.mode = mode;
                self.knob = knob_for(self.age, mode);
                self.last = mode.name().to_string();
                self.sync_sound();
                self.redraw();
            }
            Action::Mode(_) => {}
            Action::Play(id) => {
                if event.state == ElementState::Pressed {
                    self.down(id);
                } else {
                    self.up(id);
                }
                self.redraw();
            }
        }
    }

    fn point(&mut self, x: i32, y: i32) -> bool {
        let Some(window) = &self.window else {
            return false;
        };
        let size = window.inner_size();
        let width = size.width as i32;
        let height = size.height as i32;
        if !self.on_board {
            let cards = self.cards();
            let page = self.page.min(cards.len().saturating_sub(1));
            let spots = book_spots(
                width,
                height,
                page,
                cards.len(),
                self.page > 0 || self.picked,
                self.chooser(),
            );
            let Some(id) = spot_at(&spots, x, y) else {
                self.go_next();
                return false;
            };
            if id == EXIT_ID {
                return true;
            }
            if id == BACK_ID {
                self.go_back();
                return false;
            }
            if let Some(mode) = mode_from_id(id) {
                self.open_class(mode);
                return false;
            }
            self.go_next();
            return false;
        }
        let spots = layout_for(width, height, self.age, self.mode);
        let Some(id) = spot_at(&spots, x, y) else {
            return false;
        };
        if id == EXIT_ID {
            return true;
        }
        if id == LESSON_ID {
            self.go_back();
            return false;
        }
        if let Some(mode) = mode_from_id(id) {
            self.mode = mode;
            self.knob = knob_for(self.age, mode);
            self.last = mode.name().to_string();
            self.pointer = Some(id);
            self.sync_sound();
            self.redraw();
            return false;
        }
        if let Some(knob) = press_choice(self.age, self.mode, self.knob, id) {
            self.knob = knob;
            self.last = label_for(id).to_string();
            self.pointer = Some(id);
            self.sync_sound();
            self.redraw();
            return false;
        }
        self.pointer = Some(id);
        self.last = if self.age < READING_AGE {
            silly_name(id).to_string()
        } else {
            label_for(id).to_string()
        };
        self.sync_sound();
        self.redraw();
        false
    }

    fn paint(&mut self) {
        let Some(window) = self.window.clone() else {
            return;
        };
        let size = window.inner_size();
        let width_px = size.width as i32;
        let height_px = size.height as i32;
        let book = if self.on_board {
            None
        } else {
            let cards = self.cards();
            if cards.is_empty() {
                return;
            }
            let page = self.page.min(cards.len() - 1);
            let spots = book_spots(
                width_px,
                height_px,
                page,
                cards.len(),
                self.page > 0 || self.picked,
                self.chooser(),
            );
            Some((cards, page, spots))
        };
        let board = if self.on_board {
            let spots = layout_for(width_px, height_px, self.age, self.mode);
            let mut held = self.keyboard.clone();
            if let Some(id) = self.pointer {
                if !held.contains(&id) {
                    held.push(id);
                }
            }
            for id in selected_choices(self.age, self.mode, &self.knob) {
                if !held.contains(&id) {
                    held.push(id);
                }
            }
            Some((spots, held))
        } else {
            None
        };
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
        if let Some((cards, page, spots)) = book {
            paint_book(
                &mut buffer,
                width_px,
                height_px,
                &cards[page],
                &spots,
                page,
                cards.len(),
                self.age,
            );
        } else if let Some((spots, held)) = board {
            paint(
                &mut buffer,
                width_px,
                height_px,
                &spots,
                &held,
                self.age,
                &self.last,
                self.mode,
            );
        }
        let _ = buffer.present();
    }
}

fn is_back_key(id: u16) -> bool {
    matches!(label_for(id), "left" | "back")
}

fn token_from_key(event: &KeyEvent) -> String {
    if let PhysicalKey::Code(code) = event.physical_key {
        let name = match code {
            KeyCode::Escape => "escape",
            KeyCode::F1 => "f1",
            KeyCode::F2 => "f2",
            KeyCode::F3 => "f3",
            KeyCode::F4 => "f4",
            KeyCode::F5 => "f5",
            KeyCode::F6 => "f6",
            KeyCode::F7 => "f7",
            KeyCode::F8 => "f8",
            KeyCode::F9 => "f9",
            KeyCode::F10 => "f10",
            KeyCode::F11 => "f11",
            KeyCode::F12 => "f12",
            KeyCode::Backquote => "`",
            KeyCode::Digit1 | KeyCode::Numpad1 => "1",
            KeyCode::Digit2 | KeyCode::Numpad2 => "2",
            KeyCode::Digit3 | KeyCode::Numpad3 => "3",
            KeyCode::Digit4 | KeyCode::Numpad4 => "4",
            KeyCode::Digit5 | KeyCode::Numpad5 => "5",
            KeyCode::Digit6 | KeyCode::Numpad6 => "6",
            KeyCode::Digit7 | KeyCode::Numpad7 => "7",
            KeyCode::Digit8 | KeyCode::Numpad8 => "8",
            KeyCode::Digit9 | KeyCode::Numpad9 => "9",
            KeyCode::Digit0 | KeyCode::Numpad0 => "0",
            KeyCode::Minus | KeyCode::NumpadSubtract => "-",
            KeyCode::Equal | KeyCode::NumpadEqual => "=",
            KeyCode::Backspace => "back",
            KeyCode::Tab => "tab",
            KeyCode::BracketLeft => "[",
            KeyCode::BracketRight => "]",
            KeyCode::Backslash => "\\",
            KeyCode::Semicolon => ";",
            KeyCode::Quote => "'",
            KeyCode::Enter | KeyCode::NumpadEnter => "enter",
            KeyCode::CapsLock => "caps",
            KeyCode::ShiftLeft | KeyCode::ShiftRight => "shift",
            KeyCode::Comma => ",",
            KeyCode::Period | KeyCode::NumpadDecimal => ".",
            KeyCode::Slash | KeyCode::NumpadDivide => "/",
            KeyCode::Space => "space",
            KeyCode::ControlLeft | KeyCode::ControlRight => "ctrl",
            KeyCode::AltLeft | KeyCode::AltRight => "alt",
            KeyCode::SuperLeft | KeyCode::SuperRight => "super",
            KeyCode::ArrowLeft => "left",
            KeyCode::ArrowDown => "down",
            KeyCode::ArrowUp => "up",
            KeyCode::ArrowRight => "right",
            KeyCode::KeyA => "a",
            KeyCode::KeyB => "b",
            KeyCode::KeyC => "c",
            KeyCode::KeyD => "d",
            KeyCode::KeyE => "e",
            KeyCode::KeyF => "f",
            KeyCode::KeyG => "g",
            KeyCode::KeyH => "h",
            KeyCode::KeyI => "i",
            KeyCode::KeyJ => "j",
            KeyCode::KeyK => "k",
            KeyCode::KeyL => "l",
            KeyCode::KeyM => "m",
            KeyCode::KeyN => "n",
            KeyCode::KeyO => "o",
            KeyCode::KeyP => "p",
            KeyCode::KeyQ => "q",
            KeyCode::KeyR => "r",
            KeyCode::KeyS => "s",
            KeyCode::KeyT => "t",
            KeyCode::KeyU => "u",
            KeyCode::KeyV => "v",
            KeyCode::KeyW => "w",
            KeyCode::KeyX => "x",
            KeyCode::KeyY => "y",
            KeyCode::KeyZ => "z",
            other => return format!("{other:?}"),
        };
        return name.to_string();
    }
    match &event.logical_key {
        winit::keyboard::Key::Named(NamedKey::Escape) => "escape".to_string(),
        winit::keyboard::Key::Character(text) => text.to_string(),
        other => format!("{other:?}"),
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
