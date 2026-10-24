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

use crate::FULL_CLASS_AGE;

/// First age whose lesson is a click-through reading.
pub const READING_AGE: u8 = 5;

/// One big-letter page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Card {
    pub title: &'static str,
    pub lines: &'static [&'static str],
}

/// Pages for this age. Age 15 and older pass a class name, or none for the catalog.
pub fn lesson_cards(age: u8, class: Option<&str>) -> &'static [Card] {
    if age >= FULL_CLASS_AGE {
        return match class {
            Some("crush") => CRUSH,
            Some("hold") => HOLD,
            Some("level") => LEVEL,
            Some("pads") => PADS,
            Some("midi") => MIDI,
            Some("build") => BUILD,
            _ => CATALOG,
        };
    }
    match age {
        0 => AGE0,
        1 => AGE1,
        2 => AGE2,
        3 => AGE3,
        4 => AGE4,
        5 => AGE5,
        6 => AGE6,
        7 => AGE7,
        8 => AGE8,
        9 => AGE9,
        10 => AGE10,
        11 => AGE11,
        12 => AGE12,
        13 => AGE13,
        _ => AGE14,
    }
}

const AGE0: &[Card] = &[
    Card {
        title: "SURPRISE",
        lines: &["Every key.", "A different silly sound."],
    },
    Card {
        title: "TAP",
        lines: &["Tap PLAY.", "Then mash the keys."],
    },
];

const AGE1: &[Card] = &[
    Card {
        title: "SILLY KEYS",
        lines: &["Honk. Chirp.", "Robot. Bubble."],
    },
    Card {
        title: "FIND THEM",
        lines: &["Each key keeps its sound.", "Go find your favorite."],
    },
];

const AGE2: &[Card] = &[
    Card {
        title: "SOFT",
        lines: &["Some keys are quiet."],
    },
    Card {
        title: "BIGGER",
        lines: &["Some keys are bigger.", "They are still silly."],
    },
    Card {
        title: "PLAY",
        lines: &["Try every key.", "Hear which ones are soft."],
    },
];

const AGE3: &[Card] = &[
    Card {
        title: "CREATURES",
        lines: &["A honk. A chirp.", "A robot. A zap."],
    },
    Card {
        title: "ON PURPOSE",
        lines: &["The keys are silly.", "Each one is different."],
    },
    Card {
        title: "PLAY",
        lines: &["Tap PLAY.", "Try all the keys."],
    },
];

const AGE4: &[Card] = &[
    Card {
        title: "LOW",
        lines: &["Some keys sit low."],
    },
    Card {
        title: "HIGH",
        lines: &["Some keys jump high.", "They jump. They do not slide."],
    },
    Card {
        title: "PLAY",
        lines: &["Play the low ones.", "Play the high ones."],
    },
];

const AGE5: &[Card] = &[
    Card {
        title: "A SOUND",
        lines: &["A sound can start.", "You are the one who starts it."],
    },
    Card {
        title: "PRESS",
        lines: &[
            "Press a key.",
            "The sound begins.",
            "Your finger is holding it.",
        ],
    },
    Card {
        title: "LET GO",
        lines: &["Lift your finger.", "The sound stops.", "You stop it."],
    },
    Card {
        title: "TRY IT",
        lines: &[
            "Play any key.",
            "Let go to stop.",
            "Tap PLAY when you are ready.",
        ],
    },
];

const AGE6: &[Card] = &[
    Card {
        title: "STAIRS",
        lines: &["KRUSH cuts a sound", "into stairs."],
    },
    Card {
        title: "MORE",
        lines: &["More stairs", "sound smoother."],
    },
    Card {
        title: "FEWER",
        lines: &["Fewer stairs", "sound blocky."],
    },
    Card {
        title: "STEPS",
        lines: &["The stair count jumps.", "It does not glide."],
    },
    Card {
        title: "YOUR TURN",
        lines: &[
            "1 is smoother.",
            "2 is the middle.",
            "3 is blocky.",
            "Then play the letter keys.",
        ],
    },
];

const AGE7: &[Card] = &[
    Card {
        title: "BITS",
        lines: &["Bit depth is", "the stair count."],
    },
    Card {
        title: "1 TO 16",
        lines: &["The steps are integers.", "1, 2, 3, up to 16."],
    },
    Card {
        title: "16",
        lines: &["16 stairs stay close", "to the plain tone."],
    },
    Card {
        title: "2",
        lines: &["2 stairs sound", "like a square."],
    },
    Card {
        title: "YOU CHOOSE",
        lines: &["Tap 16, 8, 4, or 2.", "The keys play that choice."],
    },
];

const AGE8: &[Card] = &[
    Card {
        title: "HOLD",
        lines: &["The holder keeps", "one piece of the sound."],
    },
    Card {
        title: "FAST",
        lines: &["Fast grabs a new piece", "on every sample."],
    },
    Card {
        title: "SLOW",
        lines: &["Slow keeps that piece", "for longer."],
    },
    Card {
        title: "TRY",
        lines: &["1 is fast. 2 is slow.", "Choose, then play the keys."],
    },
];

const AGE9: &[Card] = &[
    Card {
        title: "A RATE",
        lines: &["The hold is a rate.", "The unit is Hz."],
    },
    Card {
        title: "HIGHER",
        lines: &["A higher rate", "holds a shorter piece."],
    },
    Card {
        title: "LOWER",
        lines: &["A lower rate", "holds longer."],
    },
    Card {
        title: "TRY",
        lines: &["Pick short, medium,", "or long. Then play."],
    },
];

const AGE10: &[Card] = &[
    Card {
        title: "TWO SOUNDS",
        lines: &["Dry is the plain tone.", "Wet is KRUSH."],
    },
    Card {
        title: "A LITTLE",
        lines: &["A little mix keeps", "mostly the plain tone."],
    },
    Card {
        title: "A LOT",
        lines: &["All of the mix", "is the crushed sound."],
    },
    Card {
        title: "TRY",
        lines: &["1 is almost dry.", "2 is half.", "3 is all crush."],
    },
];

const AGE11: &[Card] = &[
    Card {
        title: "MIX",
        lines: &["Mix is a number", "from 0 to 1000."],
    },
    Card {
        title: "MILLI",
        lines: &["We call that number milli.", "1000 milli is unity."],
    },
    Card {
        title: "DRY AND WET",
        lines: &["0 is dry.", "1000 is all wet."],
    },
    Card {
        title: "BITS STAY STEPPED",
        lines: &["Bits still jump.", "Mix is how much you keep."],
    },
    Card {
        title: "TRY",
        lines: &["1 is a little crush.", "2 is a lot.", "Then play the keys."],
    },
];

const AGE12: &[Card] = &[
    Card {
        title: "THE LID",
        lines: &["The ceiling is a", "digital level limit."],
    },
    Card {
        title: "NOT EARS",
        lines: &[
            "It is not hearing protection.",
            "Turn the computer volume down.",
        ],
    },
    Card {
        title: "NOT A METER",
        lines: &["It is not a loudness meter.", "It limits the block."],
    },
    Card {
        title: "TRY",
        lines: &[
            "1 leaves the lid off.",
            "2 puts the lid on.",
            "On is quieter.",
        ],
    },
];

const AGE13: &[Card] = &[
    Card {
        title: "A PAD",
        lines: &["A pad is one object."],
    },
    Card {
        title: "3 VOICES",
        lines: &["This pad has", "3 voice slots."],
    },
    Card {
        title: "ONE SEND",
        lines: &["The pad has one send."],
    },
    Card {
        title: "THE GATE",
        lines: &["Open the gate to play.", "Close the gate to stop."],
    },
    Card {
        title: "THE BANK",
        lines: &["TRUST has 16 pads.", "Cables are gates."],
    },
    Card {
        title: "TRY",
        lines: &["1 opens. 2 closes.", "3 sends more.", "Then play a key."],
    },
];

const AGE14: &[Card] = &[
    Card {
        title: "NAMES",
        lines: &["Same pad.", "Now the names match", "the instrument."],
    },
    Card {
        title: "BITS",
        lines: &["Bits are an integer.", "The steps run from 1 to 16."],
    },
    Card {
        title: "RATE",
        lines: &["Rate is Hz.", "Lower Hz holds longer."],
    },
    Card {
        title: "MIX",
        lines: &["Mix is milli.", "1000 is all wet."],
    },
    Card {
        title: "ONE SEND",
        lines: &["One send on the pad.", "The gate still opens it."],
    },
    Card {
        title: "TRY",
        lines: &[
            "Open, close, send,",
            "then bits 6.",
            "Play after each choice.",
        ],
    },
];

const CATALOG: &[Card] = &[
    Card {
        title: "FULL CLASS",
        lines: &[
            "You choose the lesson.",
            "Choose what you want",
            "to learn.",
        ],
    },
    Card {
        title: "CRUSH",
        lines: &[
            "Bit depth.",
            "An integer from 1 to 16.",
            "It does not glide.",
        ],
    },
    Card {
        title: "HOLD",
        lines: &["A sample-rate reducer.", "The rate is in Hz."],
    },
    Card {
        title: "LEVEL",
        lines: &[
            "Gain, mix, and the ceiling.",
            "The ceiling is a digital limit.",
        ],
    },
    Card {
        title: "PADS",
        lines: &["16 pads.", "3 voices. One send.", "Gate cables."],
    },
    Card {
        title: "MIDI",
        lines: &["A mapping layer.", "Never graph data."],
    },
    Card {
        title: "BUILD",
        lines: &["The crates.", "And the hear-pass."],
    },
    Card {
        title: "PICK ONE",
        lines: &["Tap a name.", "Read that lesson.", "Then play it."],
    },
];

const CRUSH: &[Card] = &[
    Card {
        title: "BITS",
        lines: &["Bit depth is", "the stair count."],
    },
    Card {
        title: "INTEGERS",
        lines: &["The value is an integer", "from 1 to 16."],
    },
    Card {
        title: "NO GLIDE",
        lines: &[
            "It changes on the sample",
            "that reads it.",
            "It does not smooth.",
        ],
    },
    Card {
        title: "HEAR IT",
        lines: &["16 stays near the tone.", "2 sounds like a square."],
    },
    Card {
        title: "YOUR KNOB",
        lines: &["Tap 16, 8, 4, or 2.", "Then play the keys."],
    },
];

const HOLD: &[Card] = &[
    Card {
        title: "HOLD",
        lines: &["The reducer keeps", "one piece of the sound."],
    },
    Card {
        title: "HZ",
        lines: &["The target rate is in Hz."],
    },
    Card {
        title: "ONE FRAME",
        lines: &["As fast as the audio,", "the hold is one frame."],
    },
    Card {
        title: "LOWER",
        lines: &["A lower rate", "holds that piece longer."],
    },
    Card {
        title: "YOUR KNOB",
        lines: &["Tap a rate.", "Then play the keys."],
    },
];

const LEVEL: &[Card] = &[
    Card {
        title: "GAIN",
        lines: &["Gain is how strong", "the sound is."],
    },
    Card {
        title: "MIX",
        lines: &[
            "Mix is the plain tone",
            "against KRUSH.",
            "1000 milli is all wet.",
        ],
    },
    Card {
        title: "A SNAP",
        lines: &["This class snaps the step", "so you can hear it."],
    },
    Card {
        title: "THE LID",
        lines: &[
            "The ceiling is a",
            "digital level limit.",
            "It is not hearing protection.",
        ],
    },
    Card {
        title: "YOUR KNOB",
        lines: &["Quiet or full.", "Dry or wet.", "Lid off or lid on."],
    },
];

const PADS: &[Card] = &[
    Card {
        title: "16 PADS",
        lines: &["The bank has 16 pads."],
    },
    Card {
        title: "3 VOICES",
        lines: &["Each pad declares", "3 voices."],
    },
    Card {
        title: "ONE SEND",
        lines: &["Each pad has one send."],
    },
    Card {
        title: "GATES",
        lines: &["Cables are gates.", "MIDI is not in the graph."],
    },
    Card {
        title: "YOUR KNOB",
        lines: &["Open, close,", "or send more.", "Then play."],
    },
];

const MIDI: &[Card] = &[
    Card {
        title: "A MAP",
        lines: &["MIDI is a mapping layer."],
    },
    Card {
        title: "NOT THE GRAPH",
        lines: &["A midi key in the graph", "is an error."],
    },
    Card {
        title: "CC1",
        lines: &["CC1 steps the bits."],
    },
    Card {
        title: "NOTE 60",
        lines: &["Note 60 opens the mix."],
    },
    Card {
        title: "THIS SCREEN",
        lines: &[
            "These buttons take",
            "the same steps.",
            "This page reads no MIDI cable.",
        ],
    },
];

const BUILD: &[Card] = &[
    Card {
        title: "CRATES",
        lines: &[
            "trust-core and trust-dsp",
            "do not allocate",
            "while the sound runs.",
        ],
    },
    Card {
        title: "CLAP",
        lines: &["trust-clap wraps clack."],
    },
    Card {
        title: "KRUSH",
        lines: &["examples/krush", "is the plugin."],
    },
    Card {
        title: "HEAR",
        lines: &["cargo xtask hear writes", "the hear-pass wav."],
    },
    Card {
        title: "PLAY",
        lines: &["The keys play", "that same KRUSH path."],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    fn joined(cards: &[Card]) -> String {
        let mut out = String::new();
        for card in cards {
            out.push_str(card.title);
            out.push(' ');
            for line in card.lines {
                out.push_str(line);
                out.push(' ');
            }
        }
        out
    }

    #[test]
    fn reading_starts_around_five() {
        assert!(lesson_cards(4, None).len() <= 3);
        assert!(lesson_cards(5, None).len() >= 4);
        let text = joined(lesson_cards(5, None)).to_ascii_lowercase();
        assert!(text.contains("stop"));
        assert!(text.contains("let go") || text.contains("lift"));
    }

    #[test]
    fn each_year_introduces_its_idea() {
        let stairs = joined(lesson_cards(6, None)).to_ascii_lowercase();
        assert!(stairs.contains("stairs"));
        assert!(stairs.contains("does not glide"));
        let lid = joined(lesson_cards(12, None));
        assert!(lid.contains("not hearing protection"));
        let named = joined(lesson_cards(14, None)).to_ascii_lowercase();
        assert!(named.contains("milli"));
        assert!(named.contains("hz"));
    }

    #[test]
    fn classes_teach_before_the_board() {
        let catalog = joined(lesson_cards(15, None)).to_ascii_lowercase();
        assert!(catalog.contains("crush"));
        assert!(catalog.contains("midi"));
        let crush = joined(lesson_cards(15, Some("crush"))).to_ascii_lowercase();
        assert!(crush.contains("does not smooth"));
        let level = joined(lesson_cards(20, Some("level")));
        assert!(level.contains("not hearing protection"));
        let midi = joined(lesson_cards(42, Some("midi"))).to_ascii_lowercase();
        assert!(midi.contains("error"));
    }
}
