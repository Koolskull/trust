//                    ☦
//   Lord Jesus Christ, Son of God, have mercy on us.
//   Come, Holy Spirit.
//
//   SPDX-License-Identifier: MIT OR Apache-2.0

use trust_dsp::Krush;

use crate::FULL_CLASS_AGE;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Quit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Speaker {
    Untried,
    On,
    Off,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassId {
    Crush,
    Hold,
    Level,
    Pads,
    Midi,
    Build,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Screen {
    Guided(u8),
    Catalog,
    Class(ClassId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sound {
    pub playing: bool,
    pub bits: u8,
    pub rate_hz: u32,
    pub gain_milli: i32,
    pub mix_milli: i32,
    pub ceiling_db_milli: i32,
    pub freq_hz: u32,
}

impl Sound {
    pub const fn quiet() -> Self {
        Self {
            playing: false,
            bits: 8,
            rate_hz: 48_000,
            gain_milli: 200,
            mix_milli: 1000,
            ceiling_db_milli: -12_000,
            freq_hz: 220,
        }
    }

    /// Pull gain and the ceiling down to the lesson cap for `age`.
    pub fn capped(self, age: u8) -> Self {
        let cap = level_cap(age);
        let mut sound = self;
        sound.gain_milli = sound.gain_milli.clamp(0, cap.gain_milli);
        if sound.ceiling_db_milli > cap.ceiling_db_milli {
            sound.ceiling_db_milli = cap.ceiling_db_milli;
        }
        sound
    }
}

/// How hot this lesson is allowed to get. A tutorial limit, not a hearing-safety claim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LevelCap {
    pub gain_milli: i32,
    pub ceiling_db_milli: i32,
}

pub fn level_cap(age: u8) -> LevelCap {
    if age <= 5 {
        LevelCap {
            gain_milli: 350,
            ceiling_db_milli: -12_000,
        }
    } else if age <= 10 {
        LevelCap {
            gain_milli: 500,
            ceiling_db_milli: -9_000,
        }
    } else if age < FULL_CLASS_AGE {
        LevelCap {
            gain_milli: 700,
            ceiling_db_milli: -6_000,
        }
    } else {
        LevelCap {
            gain_milli: 800,
            ceiling_db_milli: -3_000,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct App {
    pub age: u8,
    pub screen: Screen,
    pub sound: Sound,
    pub send_milli: i32,
    pub note: String,
    pub speaker: Speaker,
    pub quit: bool,
    /// Current guided step. Full classes leave this at 0.
    pub step: u8,
    got: String,
}

impl App {
    pub fn open(age: u8) -> Self {
        let screen = if age >= FULL_CLASS_AGE {
            Screen::Catalog
        } else {
            Screen::Guided(age.max(1).min(14))
        };
        let mut app = Self {
            age,
            screen,
            sound: Sound::quiet(),
            send_milli: 0,
            note: String::new(),
            speaker: Speaker::Untried,
            quit: false,
            step: 0,
            got: String::new(),
        };
        app.prepare_screen();
        app
    }

    pub fn press(&mut self, key: Key) {
        let key = match key {
            Key::Quit => {
                self.quit = true;
                return;
            }
            Key::Char(c) => c.to_ascii_lowercase(),
        };
        if key == 'q' {
            self.quit = true;
            return;
        }
        match self.screen {
            Screen::Catalog => self.catalog_key(key),
            Screen::Class(class) => self.class_key(class, key),
            Screen::Guided(year) => self.guided_press(year, key),
        }
    }

    fn prepare_screen(&mut self) {
        self.sound = Sound::quiet().capped(self.age);
        self.send_milli = 0;
        self.step = 0;
        self.got.clear();
        self.note = match self.screen {
            Screen::Catalog => "pick a subject".to_string(),
            Screen::Class(_) => "ready".to_string(),
            Screen::Guided(year) => steps_for(year)[0].prompt.to_string(),
        };
    }

    fn guided_press(&mut self, year: u8, key: char) {
        if !active_keys(year, self.step).contains(&key) {
            self.note = self.prompt().to_string();
            return;
        }
        self.guided_key(year, key);
        self.commit();
        self.mark_step(year, key);
    }

    fn prompt(&self) -> &'static str {
        let Screen::Guided(year) = self.screen else {
            return "";
        };
        let steps = steps_for(year);
        let step = (self.step as usize).min(steps.len().saturating_sub(1));
        steps[step].prompt
    }

    fn mark_step(&mut self, year: u8, key: char) {
        let steps = steps_for(year);
        let step = self.step as usize;
        if step >= steps.len() {
            return;
        }
        let need = steps[step].need;
        if !need.contains(&key) {
            return;
        }
        if !self.got.contains(key) {
            self.got.push(key);
        }
        let done = need.iter().all(|needed| self.got.contains(*needed));
        if done && step + 1 < steps.len() {
            self.step += 1;
            self.got.clear();
            self.note = steps[self.step as usize].prompt.to_string();
        }
    }

    fn commit(&mut self) {
        self.sound = self.sound.capped(self.age);
    }

    fn catalog_key(&mut self, key: char) {
        let class = match key {
            '1' => Some(ClassId::Crush),
            '2' => Some(ClassId::Hold),
            '3' => Some(ClassId::Level),
            '4' => Some(ClassId::Pads),
            '5' => Some(ClassId::Midi),
            '6' => Some(ClassId::Build),
            _ => None,
        };
        if let Some(class) = class {
            self.screen = Screen::Class(class);
            self.prepare_screen();
            self.note = "class open".to_string();
        }
    }

    fn class_key(&mut self, class: ClassId, key: char) {
        if key == 'b' {
            self.screen = Screen::Catalog;
            self.prepare_screen();
            return;
        }
        match class {
            ClassId::Crush => match key {
                '1' => self.hear_bits(4, "4 stairs"),
                '2' => self.hear_bits(8, "8 stairs"),
                '3' => self.hear_bits(12, "12 stairs"),
                '4' => self.hear_bits(16, "16 stairs, almost the first sound"),
                ' ' => self.toggle("crush"),
                _ => {}
            },
            ClassId::Hold => match key {
                '1' => self.hear_rate(48_000, "no hold"),
                '2' => self.hear_rate(12_000, "hold at 12000 Hz"),
                '3' => self.hear_rate(3_000, "hold at 3000 Hz"),
                ' ' => self.toggle("hold"),
                _ => {}
            },
            ClassId::Level => match key {
                '1' => self.hear_gain(250, "gain 250 milli"),
                '2' => self.hear_gain(1000, "gain 1000 milli, unity"),
                '3' => {
                    self.sound.mix_milli = 0;
                    self.sound.playing = true;
                    self.note = "mix dry, 0 milli".to_string();
                }
                '4' => {
                    self.sound.mix_milli = 1000;
                    self.sound.playing = true;
                    self.note = "mix wet, 1000 milli".to_string();
                }
                '5' => {
                    self.sound.ceiling_db_milli = 0;
                    self.sound.gain_milli = 1000;
                    self.sound.playing = true;
                    self.note = "ceiling open, 0 dB milli".to_string();
                }
                '6' => {
                    self.sound.ceiling_db_milli = -6_000;
                    self.sound.gain_milli = 1000;
                    self.sound.playing = true;
                    self.note =
                        "ceiling lid, -6000 dB milli. A level limit, not hearing protection."
                            .to_string();
                }
                ' ' => self.toggle("level"),
                _ => {}
            },
            ClassId::Pads => match key {
                '1' => {
                    self.sound.mix_milli = 1000;
                    self.sound.playing = true;
                    self.note = "gate open. One pad, voice 1 of 3.".to_string();
                }
                '2' => {
                    self.sound.mix_milli = 0;
                    self.sound.playing = false;
                    self.note = "gate closed".to_string();
                }
                '3' => {
                    self.send_milli = (self.send_milli + 250).min(1000);
                    self.sound.rate_hz = send_rate(self.send_milli);
                    self.note = format!(
                        "send {} milli. The bank still has one send.",
                        self.send_milli
                    );
                }
                ' ' => self.toggle("pad"),
                _ => {}
            },
            ClassId::Midi => match key {
                '1' => self.hear_bits(4, "CC1 would step bits. MIDI is not graph data."),
                '2' => self.hear_bits(12, "CC1 again, more stairs"),
                '3' => {
                    self.sound.mix_milli = if self.sound.mix_milli == 0 { 1000 } else { 0 };
                    self.sound.playing = self.sound.mix_milli > 0;
                    self.note =
                        "note 60 toggles the gate. The graph never stores the note.".to_string();
                }
                ' ' => self.toggle("midi"),
                _ => {}
            },
            ClassId::Build => {
                if key == ' ' {
                    self.hear_bits(
                        8,
                        "KRUSH is examples/krush. cargo xtask hear writes the WAV.",
                    );
                }
            }
        }
        self.commit();
    }

    fn guided_key(&mut self, year: u8, key: char) {
        match year {
            1 => match key {
                ' ' => self.toggle("the sound"),
                _ => {}
            },
            2 | 3 => match key {
                ' ' => self.toggle("the big button"),
                '1' => self.hear_gain(120, "soft"),
                '2' => self.hear_gain(700, "loud"),
                _ => {}
            },
            4 => match key {
                '1' => self.hear_freq(220, "low"),
                '2' => self.hear_freq(660, "high"),
                ' ' => self.toggle("the note"),
                _ => {}
            },
            5 => match key {
                ' ' | '1' => self.hear_gain(500, "go"),
                's' => {
                    self.sound.playing = false;
                    self.note = "stop".to_string();
                }
                _ => {}
            },
            6 => match key {
                '1' => self.hear_bits(12, "stair 1, smoother"),
                '2' => self.hear_bits(6, "stair 2"),
                '3' => self.hear_bits(3, "stair 3, blocky"),
                _ => {}
            },
            7 => match key {
                '1' => self.hear_bits(16, "16 stairs"),
                '2' => self.hear_bits(8, "8 stairs"),
                '3' => self.hear_bits(4, "4 stairs"),
                '4' => self.hear_bits(2, "2 stairs"),
                _ => {}
            },
            8 => match key {
                '1' => self.hear_rate(48_000, "fast"),
                '2' => self.hear_rate(3_000, "slow"),
                _ => {}
            },
            9 => match key {
                '1' => self.hear_rate(24_000, "short hold"),
                '2' => self.hear_rate(8_000, "medium hold"),
                '3' => self.hear_rate(2_000, "long hold"),
                _ => {}
            },
            10 => match key {
                '1' => {
                    self.sound.mix_milli = 0;
                    self.sound.playing = true;
                    self.note = "a little of the crush, almost none".to_string();
                }
                '2' => {
                    self.sound.mix_milli = 500;
                    self.sound.playing = true;
                    self.note = "half".to_string();
                }
                '3' => {
                    self.sound.mix_milli = 1000;
                    self.sound.playing = true;
                    self.note = "all of the crush".to_string();
                }
                _ => {}
            },
            11 => match key {
                '1' => {
                    self.sound.mix_milli = 250;
                    self.sound.bits = 8;
                    self.sound.playing = true;
                    self.note = "a little crush".to_string();
                }
                '2' => {
                    self.sound.mix_milli = 1000;
                    self.sound.bits = 4;
                    self.sound.playing = true;
                    self.note = "a lot of crush".to_string();
                }
                _ => {}
            },
            12 => match key {
                '1' => {
                    self.sound.gain_milli = 1000;
                    self.sound.ceiling_db_milli = 0;
                    self.sound.playing = true;
                    self.note = "hot, lid off".to_string();
                }
                '2' => {
                    self.sound.gain_milli = 1000;
                    self.sound.ceiling_db_milli = -18_000;
                    self.sound.playing = true;
                    self.note =
                        "lid on. This is a level limit, not hearing protection.".to_string();
                }
                _ => {}
            },
            13 | 14 => match key {
                '1' => {
                    self.sound.mix_milli = 1000;
                    self.sound.playing = true;
                    self.note = if year == 14 {
                        "gate open. mix 1000 milli.".to_string()
                    } else {
                        "gate open".to_string()
                    };
                }
                '2' => {
                    self.sound.mix_milli = 0;
                    self.sound.playing = false;
                    self.note = "gate closed".to_string();
                }
                '3' => {
                    self.send_milli = (self.send_milli + 250).min(1000);
                    self.sound.rate_hz = send_rate(self.send_milli);
                    self.note = format!("send {}", self.send_milli);
                }
                '4' if year == 14 => self.hear_bits(6, "bits 6, an integer step"),
                _ => {}
            },
            _ => {}
        }
    }

    fn toggle(&mut self, name: &str) {
        self.sound.playing = !self.sound.playing;
        self.note = if self.sound.playing {
            format!("{name} on")
        } else {
            format!("{name} off")
        };
    }

    fn hear_bits(&mut self, bits: u8, name: &str) {
        self.sound.bits = bits;
        self.sound.playing = true;
        self.note = name.to_string();
    }

    fn hear_rate(&mut self, rate_hz: u32, name: &str) {
        self.sound.rate_hz = rate_hz;
        self.sound.playing = true;
        self.note = name.to_string();
    }

    fn hear_gain(&mut self, gain_milli: i32, name: &str) {
        self.sound.gain_milli = gain_milli;
        self.sound.playing = true;
        self.note = name.to_string();
    }

    fn hear_freq(&mut self, freq_hz: u32, name: &str) {
        self.sound.freq_hz = freq_hz;
        self.sound.playing = true;
        self.note = name.to_string();
    }
}

struct Step {
    prompt: &'static str,
    need: &'static [char],
}

fn steps_for(year: u8) -> &'static [Step] {
    match year {
        1 => &[
            Step {
                prompt: "Press SPACE.",
                need: &[' '],
            },
            Step {
                prompt: "Press SPACE again to stop.",
                need: &[' '],
            },
            Step {
                prompt: "Press Q when you are done.",
                need: &[],
            },
        ],
        2 | 3 => &[
            Step {
                prompt: "Press SPACE.",
                need: &[' '],
            },
            Step {
                prompt: "Press 1 for soft. Press 2 for loud.",
                need: &['1', '2'],
            },
            Step {
                prompt: "Press Q when you are done.",
                need: &[],
            },
        ],
        4 => &[
            Step {
                prompt: "Press 1 for the low note.",
                need: &['1'],
            },
            Step {
                prompt: "Press 2 for the high note.",
                need: &['2'],
            },
            Step {
                prompt: "Press Q when you are done.",
                need: &[],
            },
        ],
        5 => &[
            Step {
                prompt: "Press SPACE to start.",
                need: &[' '],
            },
            Step {
                prompt: "Press S to stop.",
                need: &['s'],
            },
            Step {
                prompt: "Press Q when you are done.",
                need: &[],
            },
        ],
        6 => &[
            Step {
                prompt: "Press 1. More stairs, smoother.",
                need: &['1'],
            },
            Step {
                prompt: "Press 2.",
                need: &['2'],
            },
            Step {
                prompt: "Press 3. Fewer stairs, blockier.",
                need: &['3'],
            },
            Step {
                prompt: "Press Q when you are done.",
                need: &[],
            },
        ],
        7 => &[
            Step {
                prompt: "Press 1 for 16 stairs.",
                need: &['1'],
            },
            Step {
                prompt: "Press 2 for 8 stairs.",
                need: &['2'],
            },
            Step {
                prompt: "Press 3 for 4 stairs.",
                need: &['3'],
            },
            Step {
                prompt: "Press 4 for 2 stairs.",
                need: &['4'],
            },
            Step {
                prompt: "Press Q when you are done.",
                need: &[],
            },
        ],
        8 => &[
            Step {
                prompt: "Press 1 for fast.",
                need: &['1'],
            },
            Step {
                prompt: "Press 2 for slow.",
                need: &['2'],
            },
            Step {
                prompt: "Press Q when you are done.",
                need: &[],
            },
        ],
        9 => &[
            Step {
                prompt: "Press 1 for a short hold.",
                need: &['1'],
            },
            Step {
                prompt: "Press 2 for a medium hold.",
                need: &['2'],
            },
            Step {
                prompt: "Press 3 for a long hold.",
                need: &['3'],
            },
            Step {
                prompt: "Press Q when you are done.",
                need: &[],
            },
        ],
        10 => &[
            Step {
                prompt: "Press 1 for almost none of the crush.",
                need: &['1'],
            },
            Step {
                prompt: "Press 2 for half.",
                need: &['2'],
            },
            Step {
                prompt: "Press 3 for all of the crush.",
                need: &['3'],
            },
            Step {
                prompt: "Press Q when you are done.",
                need: &[],
            },
        ],
        11 => &[
            Step {
                prompt: "Press 1 for a little crush.",
                need: &['1'],
            },
            Step {
                prompt: "Press 2 for a lot of crush.",
                need: &['2'],
            },
            Step {
                prompt: "Press Q when you are done.",
                need: &[],
            },
        ],
        12 => &[
            Step {
                prompt: "Press 1. The lid is off.",
                need: &['1'],
            },
            Step {
                prompt: "Press 2. The lid is a level limit, not hearing protection.",
                need: &['2'],
            },
            Step {
                prompt: "Press Q when you are done.",
                need: &[],
            },
        ],
        13 => &[
            Step {
                prompt: "Press 1 to open the gate.",
                need: &['1'],
            },
            Step {
                prompt: "Press 2 to close the gate.",
                need: &['2'],
            },
            Step {
                prompt: "Press 3 to send more.",
                need: &['3'],
            },
            Step {
                prompt: "Press Q when you are done.",
                need: &[],
            },
        ],
        _ => &[
            Step {
                prompt: "Press 1 to open the gate.",
                need: &['1'],
            },
            Step {
                prompt: "Press 2 to close the gate.",
                need: &['2'],
            },
            Step {
                prompt: "Press 3 to raise the send.",
                need: &['3'],
            },
            Step {
                prompt: "Press 4. Bits are an integer step.",
                need: &['4'],
            },
            Step {
                prompt: "Press Q when you are done.",
                need: &[],
            },
        ],
    }
}

fn active_keys(year: u8, step: u8) -> Vec<char> {
    let mut keys = Vec::new();
    for page in steps_for(year).iter().take(step as usize + 1) {
        for key in page.need {
            if !keys.contains(key) {
                keys.push(*key);
            }
        }
    }
    keys
}

fn key_label(year: u8, key: char) -> &'static str {
    match (year, key) {
        (1, ' ') => "SPACE   the sound",
        (2 | 3, ' ') => "SPACE   press the button",
        (2 | 3, '1') => "1       soft",
        (2 | 3, '2') => "2       loud",
        (4, '1') => "1       low",
        (4, '2') => "2       high",
        (5, ' ') => "SPACE   go",
        (5, 's') => "S       stop",
        (6, '1') => "1       smoother",
        (6, '2') => "2       middle",
        (6, '3') => "3       blocky",
        (7, '1') => "1       16 stairs",
        (7, '2') => "2       8 stairs",
        (7, '3') => "3       4 stairs",
        (7, '4') => "4       2 stairs",
        (8, '1') => "1       fast",
        (8, '2') => "2       slow",
        (9, '1') => "1       short hold",
        (9, '2') => "2       medium hold",
        (9, '3') => "3       long hold",
        (10, '1') => "1       almost dry",
        (10, '2') => "2       half",
        (10, '3') => "3       all crush",
        (11, '1') => "1       a little crush",
        (11, '2') => "2       a lot of crush",
        (12, '1') => "1       lid off",
        (12, '2') => "2       lid on",
        (13 | 14, '1') => "1       open the gate",
        (13 | 14, '2') => "2       close the gate",
        (13 | 14, '3') => "3       more send",
        (14, '4') => "4       bits 6",
        _ => "key",
    }
}

fn send_rate(send_milli: i32) -> u32 {
    if send_milli <= 0 {
        48_000
    } else {
        (12_000 - (send_milli as u32 / 250) * 2_000).max(2_000)
    }
}

pub struct Voice {
    krush: Krush,
    phase: u32,
    sample_rate: u32,
}

impl Voice {
    pub fn new(sample_rate: u32) -> Self {
        let sample_rate = sample_rate.max(1);
        Self {
            krush: Krush::new(sample_rate),
            phase: 0,
            sample_rate,
        }
    }

    /// Fill `out_l` and `out_r`. No allocation. Call this on the audio thread.
    /// `age` applies the lesson cap again, so a hot value cannot skip it.
    pub fn render(&mut self, sound: &Sound, age: u8, out_l: &mut [f32], out_r: &mut [f32]) {
        let sound = sound.capped(age);
        let n = out_l.len().min(out_r.len());
        if !sound.playing {
            out_l[..n].fill(0.0);
            out_r[..n].fill(0.0);
            return;
        }
        self.krush.set_sample_rate(self.sample_rate);
        self.krush.set_bits(i32::from(sound.bits));
        self.krush.set_rate_hz(sound.rate_hz as i32);
        self.krush.snap_gain(sound.gain_milli);
        self.krush.snap_mix(sound.mix_milli);
        self.krush.set_ceiling_db_milli(sound.ceiling_db_milli);
        let mut input_l = [0.0f32; 128];
        let mut input_r = [0.0f32; 128];
        let mut o_l = [0.0f32; 128];
        let mut o_r = [0.0f32; 128];
        let mut done = 0;
        let rate = u64::from(self.sample_rate);
        while done < n {
            let chunk = (n - done).min(128);
            let step = ((u64::from(sound.freq_hz) << 32) / rate) as u32;
            for i in 0..chunk {
                let sample =
                    libm::sinf(self.phase as f32 * (core::f32::consts::TAU / 4_294_967_296.0));
                input_l[i] = sample;
                input_r[i] = sample;
                self.phase = self.phase.wrapping_add(step);
            }
            self.krush.process(
                &input_l[..chunk],
                &input_r[..chunk],
                &mut o_l[..chunk],
                &mut o_r[..chunk],
                &[],
            );
            out_l[done..done + chunk].copy_from_slice(&o_l[..chunk]);
            out_r[done..done + chunk].copy_from_slice(&o_r[..chunk]);
            done += chunk;
        }
    }
}

pub fn render_block(
    voice: &mut Voice,
    sound: &Sound,
    age: u8,
    out_l: &mut [f32],
    out_r: &mut [f32],
) {
    voice.render(sound, age, out_l, out_r);
}

pub fn render(app: &App) -> String {
    match app.screen {
        Screen::Catalog => render_catalog(app),
        Screen::Class(class) => render_class(app, class),
        Screen::Guided(year) => render_guided(app, year),
    }
}

fn render_guided(app: &App, year: u8) -> String {
    let (title, body, _) = guided_copy(year, app.age);
    let steps = steps_for(year);
    let step = (app.step as usize).min(steps.len().saturating_sub(1));
    let mut out = String::new();
    header(&mut out, app.age, title);
    out.push_str("Turn the computer volume down first.\n");
    if year <= 5 {
        out.push_str("A grown-up can read this aloud.\n");
    }
    out.push('\n');
    out.push_str(body);
    out.push('\n');
    out.push_str(&format!("step {} of {}\n", step + 1, steps.len()));
    out.push_str(steps[step].prompt);
    out.push_str("\n\n");
    if year <= 3 {
        let label = if app.sound.playing { "ON" } else { "HEAR IT" };
        out.push_str("  +------------------+\n");
        out.push_str(&box_line(""));
        out.push_str(&box_line("BIG BUTTON"));
        out.push_str(&box_line(""));
        out.push_str(&box_line(label));
        out.push_str("  +------------------+\n\n");
    }
    for key in active_keys(year, app.step) {
        out.push_str(key_label(year, key));
        out.push('\n');
    }
    out.push_str("Q       all done\n");
    footer(&mut out, app);
    out
}

fn guided_copy(year: u8, age: u8) -> (&'static str, &'static str, &'static [&'static str]) {
    let _ = age;
    match year {
        1 => (
            "ONE SOUND",
            "One button. One sound.\nPress SPACE. The sound starts.\nPress SPACE again. It stops.\n",
            &["SPACE   the sound", "Q       all done"],
        ),
        2 => (
            "SOFT AND LOUD",
            "The same sound can be soft or loud.\n",
            &[
                "SPACE   hear it",
                "1       soft",
                "2       loud",
                "Q       all done",
            ],
        ),
        3 => (
            "BIG BUTTON",
            "This is the button. Press it.\n",
            &[
                "SPACE   press the button",
                "1       soft",
                "2       loud",
                "Q       all done",
            ],
        ),
        4 => (
            "LOW AND HIGH",
            "Two notes live on this button.\n",
            &["1       low", "2       high", "SPACE   hear or stop", "Q       all done"],
        ),
        5 => (
            "GO AND STOP",
            "You start the sound. You stop the sound.\n",
            &["SPACE   go", "S       stop", "Q       all done"],
        ),
        6 => (
            "THREE STAIRS",
            "KRUSH cuts a sound into stairs. Fewer stairs sounds blockier.\nThe stair count is an integer. It does not glide.\n",
            &["1       smoother", "2       middle", "3       blocky", "Q       done"],
        ),
        7 => (
            "MORE STAIRS",
            "Bit depth is the stair count, from 1 to 16.\n16 is close to the first sound. 2 is a square.\n",
            &["1       16", "2       8", "3       4", "4       2", "Q       done"],
        ),
        8 => (
            "FAST AND SLOW",
            "The holder keeps one piece of the sound.\nFast means it grabs a new piece every sample.\n",
            &["1       fast", "2       slow", "Q       done"],
        ),
        9 => (
            "HOLD THE SOUND",
            "The hold is a rate in Hz. A lower rate holds longer.\n",
            &[
                "1       short hold",
                "2       medium hold",
                "3       long hold",
                "Q       done",
            ],
        ),
        10 => (
            "A LITTLE OR A LOT",
            "Mix is how much of the crushed sound you keep.\nThe dry sound is the sine. The wet sound is KRUSH.\n",
            &["1       almost dry", "2       half", "3       all crush", "Q       done"],
        ),
        11 => (
            "MIX",
            "Mix is milli from 0 to 1000. 1000 is all wet.\nBits stay an integer step while the mix moves.\n",
            &["1       mix 250, 8 bits", "2       mix 1000, 4 bits", "Q       done"],
        ),
        12 => (
            "THE LID",
            "The ceiling is a digital level limit on the block.\nIt is not hearing protection and it is not a loudness meter.\n",
            &["1       lid off", "2       lid on", "Q       done"],
        ),
        13 => (
            "ONE PAD",
            "A pad is one object. This pad has 3 voice slots and one send.\nThe gate opens the sound. TRUST's bank has 16 pads.\n",
            &[
                "1       open the gate",
                "2       close the gate",
                "3       more send",
                "Q       done",
            ],
        ),
        _ => (
            "THE PAD, NAMED",
            "Same pad. The names are the ones in the instrument.\nBits step. Rate is Hz. Mix is milli. One send per pad.\n",
            &[
                "1       gate open, mix 1000",
                "2       gate closed",
                "3       send up",
                "4       bits 6",
                "Q       done",
            ],
        ),
    }
}

fn render_catalog(app: &App) -> String {
    let mut out = String::new();
    header(&mut out, app.age, "FULL CLASS");
    out.push_str("Turn the computer volume down first.\n");
    out.push_str("The lesson holds the level under a cap.\n\n");
    out.push_str("You choose the lesson from what you want to learn.\n\n");
    out.push_str("  1  crush     bit depth, an integer step 1 to 16\n");
    out.push_str("  2  hold      sample-rate reducer, in Hz\n");
    out.push_str("  3  level     gain, mix, and the ceiling\n");
    out.push_str("  4  pads      16 pads, 3 voices, one send, gate cables\n");
    out.push_str("  5  midi      a mapping layer, never graph data\n");
    out.push_str("  6  build     the crates and the hear-pass\n\n");
    out.push_str("Q   leave\n");
    footer(&mut out, app);
    out
}

fn render_class(app: &App, class: ClassId) -> String {
    let mut out = String::new();
    let (title, body, keys) = match class {
        ClassId::Crush => (
            "CRUSH",
            "Bit depth never smooths. The value changes on the sample that reads it.\n",
            &["1  4 bits", "2  8 bits", "3  12 bits", "4  16 bits", "SPACE  hear or stop", "B  back", "Q  leave"][..],
        ),
        ClassId::Hold => (
            "HOLD",
            "Target rate is Hz. 0, or a rate at the sample rate, holds one frame.\n",
            &["1  48000 Hz", "2  12000 Hz", "3  3000 Hz", "SPACE  hear or stop", "B  back", "Q  leave"][..],
        ),
        ClassId::Level => (
            "LEVEL",
            "Gain and mix may ramp in the plugin. This class snaps them so you can hear the step.\nThe ceiling is a 4× linear digital limit. It is not hearing protection.\n",
            &["1  quiet gain", "2  unity gain", "3  dry mix", "4  wet mix", "5  ceiling open", "6  ceiling lid", "B  back", "Q  leave"][..],
        ),
        ClassId::Pads => (
            "PADS",
            "schema_version 1 is 16 pads. Each pad declares 3 voices and one send.\nCables are gate cables. MIDI is not in the graph.\n",
            &["1  gate open", "2  gate closed", "3  more send", "SPACE  hear or stop", "B  back", "Q  leave"][..],
        ),
        ClassId::Midi => (
            "MIDI",
            "CC1 steps bits. Note 60 gates the mix. The map lives in trust-plugin.\nA midi key inside graph JSON is an error.\n",
            &["1  CC1, 4 bits", "2  CC1, 12 bits", "3  note 60 gate", "B  back", "Q  leave"][..],
        ),
        ClassId::Build => (
            "BUILD",
            "trust-core and trust-dsp are no_std. trust-clap wraps clack.\nexamples/krush is the plugin. cargo xtask hear writes target/hear/krush-hear.wav.\n",
            &["SPACE  play KRUSH", "B  back", "Q  leave"][..],
        ),
    };
    header(&mut out, app.age, title);
    out.push_str("The lesson holds the level under a cap.\n");
    out.push_str(body);
    out.push('\n');
    for line in keys {
        out.push_str(line);
        out.push('\n');
    }
    if class == ClassId::Pads {
        out.push_str(&format!(
            "\nvoices  * * *\nsend    {} milli\n",
            app.send_milli
        ));
    }
    footer(&mut out, app);
    out
}

fn box_line(text: &str) -> String {
    let width = 18usize;
    let left = width.saturating_sub(text.len()) / 2;
    let right = width.saturating_sub(text.len() + left);
    format!("  |{}{text}{}|\n", " ".repeat(left), " ".repeat(right))
}

fn header(out: &mut String, age: u8, title: &str) {
    out.push_str("☦  KOS TRUST LESSON\n");
    out.push_str(&format!("age {age}\n{title}\n\n"));
}

fn footer(out: &mut String, app: &App) {
    let speaker = match app.speaker {
        Speaker::Untried => "speaker starts with the live lesson",
        Speaker::On => "speaker on",
        Speaker::Off => "no speaker — the keys still count",
    };
    out.push_str(&format!("\nnow: {}\n{speaker}\n", app.note));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FULL_CLASS_AGE;

    #[test]
    fn three_yo_is_the_big_button_and_not_the_catalog() {
        let app = App::open(3);
        assert_eq!(app.screen, Screen::Guided(3));
        let frame = render(&app);
        assert!(frame.contains("BIG BUTTON"));
        assert!(frame.contains("press the button"));
        assert!(frame.contains("Press SPACE."));
        assert!(frame.contains("Turn the computer volume down first."));
        assert!(!frame.contains("1       soft"));
        let border = frame.lines().find(|line| line.contains('+')).unwrap();
        for line in frame
            .lines()
            .filter(|line| line.contains('|') || line.contains('+'))
        {
            assert_eq!(line.len(), border.len(), "{line}");
        }
        assert!(!frame.contains("FULL CLASS"));
        assert!(!frame.contains("Pick a subject") && !frame.contains("You choose"));
    }

    #[test]
    fn each_year_before_15_has_its_own_lesson() {
        let mut titles = Vec::new();
        for age in 1..FULL_CLASS_AGE {
            let app = App::open(age);
            assert!(matches!(app.screen, Screen::Guided(_)), "{age}");
            let frame = render(&app);
            let title = frame.lines().nth(2).unwrap().to_string();
            assert!(!titles.contains(&title), "{title} repeated");
            titles.push(title);
            assert!(!frame.contains("FULL CLASS"));
        }
        assert_eq!(titles.len(), 14);
    }

    #[test]
    fn age_zero_uses_the_youngest_lesson() {
        let app = App::open(0);
        assert_eq!(app.screen, Screen::Guided(1));
        assert!(render(&app).contains("ONE SOUND"));
    }

    #[test]
    fn fourteen_stays_guided_and_fifteen_opens_the_catalog() {
        let young = App::open(14);
        assert_eq!(young.screen, Screen::Guided(14));
        assert!(render(&young).contains("THE PAD, NAMED"));
        let older = App::open(15);
        assert_eq!(older.screen, Screen::Catalog);
        let frame = render(&older);
        assert!(frame.contains("crush"));
        assert!(frame.contains("pads"));
        assert!(frame.contains("You choose"));
        let adult = App::open(42);
        assert_eq!(adult.screen, Screen::Catalog);
    }

    #[test]
    fn three_yo_soft_and_loud_change_the_sound() {
        let mut app = App::open(3);
        let cap = level_cap(3);
        app.press(Key::Char('2'));
        assert!(!app.sound.playing);
        assert_eq!(app.step, 0);
        app.press(Key::Char(' '));
        assert!(app.sound.playing);
        assert_eq!(app.step, 1);
        assert!(render(&app).contains("ON"));
        assert!(render(&app).contains("1       soft"));
        app.press(Key::Char('2'));
        assert_eq!(app.sound.gain_milli, cap.gain_milli);
        assert!(app.sound.gain_milli < 700);
        assert_eq!(app.note, "loud");
        assert_eq!(app.step, 1);
        app.press(Key::Char('1'));
        assert_eq!(app.sound.gain_milli, 120);
        assert_eq!(app.step, 2);
        assert!(render(&app).contains("Press Q when you are done."));
        app.press(Key::Char('q'));
        assert!(app.quit);
    }

    #[test]
    fn fifteen_picks_a_class_and_can_go_back() {
        let mut app = App::open(15);
        app.press(Key::Char('1'));
        assert_eq!(app.screen, Screen::Class(ClassId::Crush));
        assert!(render(&app).contains("CRUSH"));
        app.press(Key::Char('4'));
        assert_eq!(app.sound.bits, 16);
        app.press(Key::Char('b'));
        assert_eq!(app.screen, Screen::Catalog);
        app.press(Key::Char('4'));
        assert_eq!(app.screen, Screen::Class(ClassId::Pads));
        app.press(Key::Char('3'));
        assert_eq!(app.send_milli, 250);
    }

    #[test]
    fn loud_is_hotter_than_soft_through_krush() {
        let mut voice = Voice::new(48_000);
        let mut soft = Sound::quiet();
        soft.playing = true;
        soft.bits = 16;
        soft.gain_milli = 250;
        soft.ceiling_db_milli = 0;
        let mut loud = soft;
        loud.gain_milli = 700;
        let mut soft_l = [0.0; 128];
        let mut soft_r = [0.0; 128];
        let mut loud_l = [0.0; 128];
        let mut loud_r = [0.0; 128];
        voice.render(&soft, 15, &mut soft_l, &mut soft_r);
        voice.phase = 0;
        voice.render(&loud, 15, &mut loud_l, &mut loud_r);
        let soft_peak = peak(&soft_l);
        let loud_peak = peak(&loud_l);
        assert!(loud_peak > soft_peak, "{loud_peak} vs {soft_peak}");
        assert!(soft_peak > 0.0);
    }

    #[test]
    fn a_quiet_block_is_silent() {
        let mut voice = Voice::new(48_000);
        let mut l = [1.0; 32];
        let mut r = [1.0; 32];
        voice.render(&Sound::quiet(), 3, &mut l, &mut r);
        assert!(l.iter().all(|s| *s == 0.0));
        assert!(r.iter().all(|s| *s == 0.0));
    }

    #[test]
    fn a_hot_request_stays_under_the_age_cap() {
        let mut voice = Voice::new(48_000);
        let mut hot = Sound::quiet();
        hot.playing = true;
        hot.bits = 16;
        hot.gain_milli = 1000;
        hot.ceiling_db_milli = 0;
        let mut young_l = [0.0; 256];
        let mut young_r = [0.0; 256];
        let mut older_l = [0.0; 256];
        let mut older_r = [0.0; 256];
        voice.render(&hot, 3, &mut young_l, &mut young_r);
        voice.phase = 0;
        voice.render(&hot, 15, &mut older_l, &mut older_r);
        let young = peak(&young_l);
        let older = peak(&older_l);
        assert!(young > 0.0);
        assert!(
            young < older,
            "age 3 {young} should stay under age 15 {older}"
        );
        assert!(young < 0.3, "age 3 peak {young}");
        assert!(older < 0.8, "age 15 peak {older}");
    }

    fn peak(buf: &[f32]) -> f32 {
        buf.iter().fold(0.0f32, |p, s| p.max(s.abs()))
    }
}
