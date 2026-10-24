//                    ☦
//         ╔════════════════════╗
//         ║  I am the vine;    ║
//         ║  you are the       ║
//         ║  branches.         ║
//         ╚════════════════════╝
//
//   John 15:5. Glory to the Father, and to the Son, and to the Holy Spirit.
//   Lord Jesus Christ, Son of God, have mercy on us.
//   Come, Holy Spirit.
//
//   SPDX-License-Identifier: MIT OR Apache-2.0

//! Versioned instrument graph. MIDI is a mapping layer and must not appear
//! in this JSON. Numbers are integers. Hex belongs in file stems, not here.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

pub const SCHEMA_VERSION: u32 = 1;
pub const PAD_COUNT: usize = 16;
pub const VOICES_PER_PAD: u32 = 3;
pub const DEFAULT_PER_OBJECT: u32 = 3;
pub const DEFAULT_GLOBAL: u32 = 32;
const MAX_VOICES: usize = 64;

#[derive(Debug, Error, PartialEq)]
pub enum GraphError {
    #[error("schema_version {0} is newer than {SCHEMA_VERSION}")]
    FutureVersion(u64),
    #[error("graph JSON must not contain a midi key")]
    MidiInGraph,
    #[error("schema_version must be an integer")]
    BadVersion,
    #[error("graph numbers must be integers")]
    FloatInJson,
    #[error("{0}")]
    Invalid(String),
    #[error("json: {0}")]
    Json(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Send {
    pub amount_milli: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pad {
    pub id: u32,
    /// Declared voices. Schema 1 requires 3. The runtime cap may be lower.
    pub voices: u32,
    pub send: Send,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cable {
    pub from: u32,
    pub to: u32,
    /// Schema 1 allows only `"gate"`.
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Caps {
    pub per_object: u32,
    pub global: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Graph {
    pub schema_version: u32,
    pub caps: Caps,
    pub pads: Vec<Pad>,
    pub cables: Vec<Cable>,
}

impl Graph {
    pub fn default_bank() -> Self {
        let pads = (0..PAD_COUNT as u32)
            .map(|id| Pad {
                id,
                voices: VOICES_PER_PAD,
                send: Send { amount_milli: 0 },
            })
            .collect();
        Self {
            schema_version: SCHEMA_VERSION,
            caps: Caps {
                per_object: DEFAULT_PER_OBJECT,
                global: DEFAULT_GLOBAL,
            },
            pads,
            cables: Vec::new(),
        }
    }
}

pub fn load_json(text: &str) -> Result<Graph, GraphError> {
    let value: Value = serde_json::from_str(text).map_err(|e| GraphError::Json(e.to_string()))?;
    if !value.is_object() {
        return Err(GraphError::Invalid("graph JSON must be an object".into()));
    }
    reject_midi(&value)?;
    let version = match value.get("schema_version") {
        None => 0,
        Some(Value::Number(n)) if n.is_u64() => n.as_u64().unwrap_or(0),
        Some(_) => return Err(GraphError::BadVersion),
    };
    if version > SCHEMA_VERSION as u64 {
        return Err(GraphError::FutureVersion(version));
    }
    reject_floats(&value)?;
    if version == 0 {
        return Ok(Graph::default_bank());
    }
    let graph: Graph =
        serde_json::from_value(value).map_err(|e| GraphError::Json(e.to_string()))?;
    validate(&graph)?;
    Ok(graph)
}

pub fn to_json(graph: &Graph) -> Result<String, GraphError> {
    validate(graph)?;
    serde_json::to_string(graph).map_err(|e| GraphError::Json(e.to_string()))
}

pub fn validate(graph: &Graph) -> Result<(), GraphError> {
    if graph.schema_version != SCHEMA_VERSION {
        return Err(GraphError::Invalid(format!(
            "schema_version {} is not {SCHEMA_VERSION}",
            graph.schema_version
        )));
    }
    if graph.pads.len() != PAD_COUNT {
        return Err(GraphError::Invalid(format!(
            "pad bank is {} pads, schema 1 requires {PAD_COUNT}",
            graph.pads.len()
        )));
    }
    if graph.caps.per_object > MAX_VOICES as u32 || graph.caps.global > MAX_VOICES as u32 {
        return Err(GraphError::Invalid(format!(
            "voice caps must be <= {MAX_VOICES}"
        )));
    }
    let mut seen = [false; PAD_COUNT];
    for pad in &graph.pads {
        if pad.voices != VOICES_PER_PAD {
            return Err(GraphError::Invalid(format!(
                "pad {} declares {} voices, schema 1 requires {VOICES_PER_PAD}",
                pad.id, pad.voices
            )));
        }
        if pad.id as usize >= PAD_COUNT || seen[pad.id as usize] {
            return Err(GraphError::Invalid(format!(
                "pad id {} is not unique in 0..{PAD_COUNT}",
                pad.id
            )));
        }
        seen[pad.id as usize] = true;
    }
    for cable in &graph.cables {
        if cable.kind != "gate" {
            return Err(GraphError::Invalid(format!(
                "cable kind {:?} is not gate",
                cable.kind
            )));
        }
        if !seen.get(cable.from as usize).copied().unwrap_or(false)
            || !seen.get(cable.to as usize).copied().unwrap_or(false)
        {
            return Err(GraphError::Invalid(
                "gate cable endpoint is not a pad".into(),
            ));
        }
    }
    Ok(())
}

fn reject_midi(value: &Value) -> Result<(), GraphError> {
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                if key.eq_ignore_ascii_case("midi") {
                    return Err(GraphError::MidiInGraph);
                }
                reject_midi(child)?;
            }
            Ok(())
        }
        Value::Array(items) => {
            for item in items {
                reject_midi(item)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn reject_floats(value: &Value) -> Result<(), GraphError> {
    match value {
        Value::Number(n) if !n.is_i64() && !n.is_u64() => Err(GraphError::FloatInJson),
        Value::Object(map) => {
            for child in map.values() {
                reject_floats(child)?;
            }
            Ok(())
        }
        Value::Array(items) => {
            for item in items {
                reject_floats(item)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VoiceRef {
    pub object: u32,
    pub slot: u32,
}

#[derive(Clone, Copy)]
struct Live {
    object: u32,
    slot: u32,
    born: u64,
}

/// Fixed voice list. `note_on` steals the oldest voice when a cap is hit.
/// Declared pad voices (16 × 3) may exceed `caps.global`.
#[derive(Clone)]
pub struct VoiceAlloc {
    voices: [Option<Live>; MAX_VOICES],
    clock: u64,
}

impl VoiceAlloc {
    pub const fn new() -> Self {
        Self {
            voices: [None; MAX_VOICES],
            clock: 0,
        }
    }

    pub fn alive(&self) -> usize {
        self.voices.iter().filter(|v| v.is_some()).count()
    }

    pub fn holds(&self, object: u32, slot: u32) -> bool {
        self.voices.iter().any(|v| {
            v.map(|live| live.object == object && live.slot == slot)
                .unwrap_or(false)
        })
    }

    pub fn note_off(&mut self, object: u32, slot: u32) -> bool {
        for voice in &mut self.voices {
            if voice
                .map(|live| live.object == object && live.slot == slot)
                .unwrap_or(false)
            {
                *voice = None;
                return true;
            }
        }
        false
    }

    pub fn note_on(&mut self, object: u32, caps: &Caps) -> Option<VoiceRef> {
        if caps.per_object == 0 || caps.global == 0 {
            return None;
        }
        let under_obj = self.count_object(object) < caps.per_object as usize;
        let under_global = self.alive() < caps.global as usize;
        if !under_obj {
            self.kill_oldest(Some(object));
        } else if !under_global || self.free_index().is_none() {
            self.kill_oldest(None);
        }
        let slot = self.free_slot(object, caps.per_object)?;
        let index = self.free_index()?;
        self.clock = self.clock.wrapping_add(1);
        self.voices[index] = Some(Live {
            object,
            slot,
            born: self.clock,
        });
        Some(VoiceRef { object, slot })
    }

    fn count_object(&self, object: u32) -> usize {
        self.voices
            .iter()
            .filter(|v| v.map(|live| live.object == object).unwrap_or(false))
            .count()
    }

    fn free_index(&self) -> Option<usize> {
        self.voices.iter().position(|v| v.is_none())
    }

    fn free_slot(&self, object: u32, per_object: u32) -> Option<u32> {
        (0..per_object).find(|slot| !self.holds(object, *slot))
    }

    fn kill_oldest(&mut self, only_object: Option<u32>) -> bool {
        let mut best: Option<(usize, u64)> = None;
        for (i, voice) in self.voices.iter().enumerate() {
            if let Some(live) = voice {
                if only_object.map(|o| live.object == o).unwrap_or(true)
                    && best.map(|(_, born)| live.born < born).unwrap_or(true)
                {
                    best = Some((i, live.born));
                }
            }
        }
        if let Some((i, _)) = best {
            self.voices[i] = None;
            true
        } else {
            false
        }
    }
}

impl Default for VoiceAlloc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_version_migrates_to_default_bank() {
        let graph = load_json("{}").unwrap();
        assert_eq!(graph, Graph::default_bank());
        assert_eq!(graph.pads.len(), 16);
        assert!(graph
            .pads
            .iter()
            .all(|p| p.voices == 3 && p.send.amount_milli == 0));
        assert_eq!(graph.caps.per_object, 3);
        assert_eq!(graph.caps.global, 32);
    }

    #[test]
    fn zero_version_migrates_and_discards_old_caps() {
        let graph =
            load_json(r#"{"schema_version":0,"caps":{"per_object":1,"global":1}}"#).unwrap();
        assert_eq!(graph.caps.global, 32);
        assert_eq!(graph.schema_version, 1);
    }

    #[test]
    fn future_version_float_and_midi_are_rejected() {
        assert!(matches!(
            load_json(r#"{"schema_version":2}"#),
            Err(GraphError::FutureVersion(2))
        ));
        assert!(matches!(
            load_json(r#"{"schema_version":1.5}"#),
            Err(GraphError::BadVersion)
        ));
        assert!(matches!(
            load_json(r#"{"midi":{"cc":1}}"#),
            Err(GraphError::MidiInGraph)
        ));
        assert!(matches!(
            load_json(r#"{"schema_version":1,"midi":{}}"#),
            Err(GraphError::MidiInGraph)
        ));
        assert!(matches!(
            load_json(r#"{"schema_version":1,"caps":{"per_object":3,"global":32.0}}"#),
            Err(GraphError::FloatInJson)
        ));
    }

    #[test]
    fn round_trip_keeps_a_gate_cable() {
        let mut graph = Graph::default_bank();
        graph.cables.push(Cable {
            from: 0,
            to: 4,
            kind: "gate".into(),
        });
        graph.pads[2].send.amount_milli = 250;
        let text = to_json(&graph).unwrap();
        assert!(!text.contains('.'), "json levels stay integers: {text}");
        let again = load_json(&text).unwrap();
        assert_eq!(graph, again);
    }

    #[test]
    fn audio_cable_and_wrong_voice_count_fail() {
        let mut graph = Graph::default_bank();
        graph.cables.push(Cable {
            from: 0,
            to: 1,
            kind: "audio".into(),
        });
        assert!(validate(&graph).is_err());
        graph.cables.clear();
        graph.pads[0].voices = 2;
        assert!(load_json(&serde_json::to_string(&graph).unwrap()).is_err());
    }

    #[test]
    fn sends_array_is_not_the_schema() {
        let graph = Graph::default_bank();
        let mut value = serde_json::to_value(&graph).unwrap();
        value["pads"][0].as_object_mut().unwrap().remove("send");
        value["pads"][0]["sends"] = serde_json::json!([{ "amount_milli": 0 }]);
        assert!(load_json(&value.to_string()).is_err());
    }

    #[test]
    fn oldest_voice_on_the_object_is_stolen() {
        let caps = Caps {
            per_object: 2,
            global: 4,
        };
        let mut alloc = VoiceAlloc::new();
        let first = alloc.note_on(1, &caps).unwrap();
        let second = alloc.note_on(1, &caps).unwrap();
        let third = alloc.note_on(1, &caps).unwrap();
        assert_eq!(third.slot, first.slot);
        assert!(!alloc.holds(1, first.slot) || third.slot == first.slot);
        assert!(alloc.holds(1, second.slot));
        assert!(alloc.holds(1, third.slot));
        assert_eq!(alloc.alive(), 2);
    }

    #[test]
    fn global_cap_steals_across_objects_and_stops_at_32() {
        let caps = Caps {
            per_object: 1,
            global: 2,
        };
        let mut alloc = VoiceAlloc::new();
        alloc.note_on(0, &caps).unwrap();
        let kept = alloc.note_on(1, &caps).unwrap();
        let stolen = alloc.note_on(0, &caps).unwrap();
        assert!(alloc.holds(stolen.object, stolen.slot));
        assert!(alloc.holds(kept.object, kept.slot));
        assert!(!alloc.holds(0, 0) || stolen.object == 0);
        assert_eq!(alloc.alive(), 2);

        let graph = Graph::default_bank();
        let mut bank = VoiceAlloc::new();
        for pad in 0..16u32 {
            for _ in 0..3 {
                bank.note_on(pad, &graph.caps).unwrap();
            }
        }
        assert_eq!(bank.alive(), graph.caps.global as usize);
        assert!(16 * 3 > graph.caps.global);
    }
}
