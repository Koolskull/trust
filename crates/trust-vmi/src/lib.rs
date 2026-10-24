//                    ☦
//         ╔════════════════════╗
//         ║  The Word became   ║
//         ║  flesh.            ║
//         ╚════════════════════╝
//
//   John 1:14. Lord Jesus Christ, Son of God, have mercy on us.
//   Come, Holy Spirit.
//
//   SPDX-License-Identifier: MIT OR Apache-2.0

//! TRUST reads an exported scene folder. It does not read `.vmib`.
//! Picture and audio stems are six hex digits, `SSSXXX`.
//! `trust.json` says whether SSS is a visual state or a sample bank.
//! Names come from the folder and from JSON, not from a decorated filename.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;

pub const UI_REST: u16 = 0;
pub const UI_HOVER: u16 = 1;
pub const UI_PRESSED: u16 = 2;
pub const UI_AFTER: u16 = 3;

#[derive(Debug, Error)]
pub enum VmiError {
    #[error("io: {0}")]
    Io(String),
    #[error("json {0}: {1}")]
    Json(String, String),
    #[error("trust.json needs integer \"version\" and \"kind\" of \"ui\" or \"pack\"")]
    TrustMeta,
    #[error("stem {0} is not six hex digits")]
    Stem(String),
    #[error("{kind:?} rejects SSS {sss:03x} in {file}")]
    State { kind: Kind, sss: u16, file: String },
    #[error("missing {0}")]
    Missing(String),
    #[error("{0}")]
    Invalid(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Ui,
    Pack,
}

impl Kind {
    fn parse(text: &str) -> Result<Self, VmiError> {
        match text {
            "ui" => Ok(Kind::Ui),
            "pack" => Ok(Kind::Pack),
            _ => Err(VmiError::TrustMeta),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MediaFile {
    pub stem: String,
    pub ext: String,
    pub path: PathBuf,
    pub sss: u16,
    pub xxx: u16,
    pub label: Option<String>,
}

#[derive(Clone, Debug)]
pub struct LoadedObject {
    pub id: String,
    pub folder: String,
    pub name: String,
    pub files: Vec<MediaFile>,
    pub audio: Vec<MediaFile>,
}

#[derive(Clone, Debug)]
pub struct LoadedScene {
    pub version: u32,
    pub kind: Kind,
    pub scene: String,
    pub naming: String,
    pub objects: Vec<LoadedObject>,
}

#[derive(Deserialize)]
struct TrustFile {
    version: u32,
    kind: String,
}

#[derive(Deserialize)]
struct SceneFile {
    naming: Option<String>,
    scene: Option<String>,
    objects: Vec<SceneObject>,
}

#[derive(Deserialize)]
struct SceneObject {
    id: String,
    folder: String,
    slots: SlotsRaw,
}

#[derive(Deserialize)]
struct ObjectFile {
    name: Option<String>,
    slots: SlotsRaw,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum SlotsRaw {
    Map(BTreeMap<String, SlotDoc>),
    List(Vec<SlotDoc>),
}

impl SlotsRaw {
    fn iter(&self) -> Vec<&SlotDoc> {
        match self {
            SlotsRaw::Map(map) => map.values().collect(),
            SlotsRaw::List(list) => list.iter().collect(),
        }
    }
}

#[derive(Deserialize)]
struct SlotDoc {
    file: String,
    label: Option<String>,
    png: Option<bool>,
}

pub fn load_scene(dir: &Path) -> Result<LoadedScene, VmiError> {
    let trust: TrustFile = read_json(&dir.join("trust.json"))?;
    let kind = Kind::parse(&trust.kind)?;
    if trust.version != 1 {
        return Err(VmiError::TrustMeta);
    }
    let scene: SceneFile = read_json(&dir.join("scene.json"))?;
    let naming = scene.naming.unwrap_or_else(|| "SSSXXX".into());
    if naming != "SSSXXX" {
        return Err(VmiError::Invalid(format!("naming {naming} is not SSSXXX")));
    }
    let mut objects = Vec::new();
    for obj in &scene.objects {
        if obj.folder.contains("..") {
            return Err(VmiError::Invalid(format!(
                "folder {} escapes the scene",
                obj.folder
            )));
        }
        let folder = dir.join(&obj.folder);
        let doc: ObjectFile = read_json(&folder.join("object.json"))?;
        let mut files = scan_media(&folder, kind)?;
        apply_labels(&mut files, &obj.slots);
        apply_labels(&mut files, &doc.slots);
        check_slots(&folder, &obj.slots, kind)?;
        check_slots(&folder, &doc.slots, kind)?;
        let audio = choose_audio(&files);
        let name = doc
            .name
            .clone()
            .filter(|n| !n.is_empty())
            .unwrap_or_else(|| folder_name(&obj.folder));
        objects.push(LoadedObject {
            id: obj.id.clone(),
            folder: obj.folder.clone(),
            name,
            files,
            audio,
        });
    }
    Ok(LoadedScene {
        version: trust.version,
        kind,
        scene: scene.scene.unwrap_or_default(),
        naming,
        objects,
    })
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, VmiError> {
    let text =
        fs::read_to_string(path).map_err(|e| VmiError::Io(format!("{}: {e}", path.display())))?;
    let value: Value = serde_json::from_str(&text)
        .map_err(|e| VmiError::Json(path.display().to_string(), e.to_string()))?;
    reject_floats(&value).map_err(|e| VmiError::Json(path.display().to_string(), e))?;
    serde_json::from_value(value)
        .map_err(|e| VmiError::Json(path.display().to_string(), e.to_string()))
}

fn reject_floats(value: &Value) -> Result<(), String> {
    match value {
        Value::Number(n) if !n.is_i64() && !n.is_u64() => Err("numbers must be integers".into()),
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

fn scan_media(folder: &Path, kind: Kind) -> Result<Vec<MediaFile>, VmiError> {
    let rd =
        fs::read_dir(folder).map_err(|e| VmiError::Io(format!("{}: {e}", folder.display())))?;
    let mut files = Vec::new();
    for entry in rd {
        let entry = entry.map_err(|e| VmiError::Io(e.to_string()))?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(ext) = extension(&path) else {
            continue;
        };
        if !is_media(&ext) {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        let (sss, xxx) = parse_stem(&stem, kind, &path)?;
        files.push(MediaFile {
            stem: stem.to_ascii_lowercase(),
            ext,
            path,
            sss,
            xxx,
            label: None,
        });
    }
    files.sort_by(|a, b| a.stem.cmp(&b.stem).then(a.ext.cmp(&b.ext)));
    Ok(files)
}

fn check_slots(folder: &Path, slots: &SlotsRaw, kind: Kind) -> Result<(), VmiError> {
    for slot in slots.iter() {
        let (sss, _xxx) = parse_stem(&file_stem(&slot.file), kind, Path::new(&slot.file))?;
        let _ = sss;
        if resolve_ci(folder, &slot.file).is_none() {
            return Err(VmiError::Missing(format!(
                "{}/{}",
                folder.display(),
                slot.file
            )));
        }
        if slot.png == Some(true) {
            let ext = Path::new(&slot.file)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_ascii_lowercase();
            if ext != "png" {
                return Err(VmiError::Invalid(format!(
                    "{} claims png but the extension is {ext}",
                    slot.file
                )));
            }
        }
    }
    Ok(())
}

fn apply_labels(files: &mut [MediaFile], slots: &SlotsRaw) {
    for slot in slots.iter() {
        let stem = file_stem(&slot.file).to_ascii_lowercase();
        for file in files.iter_mut() {
            if file.stem == stem {
                if file.label.is_none() {
                    file.label = slot.label.clone();
                }
            }
        }
    }
}

pub fn parse_stem(stem: &str, kind: Kind, file: &Path) -> Result<(u16, u16), VmiError> {
    if stem.len() != 6 || !stem.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(VmiError::Stem(stem.to_string()));
    }
    let sss = u16::from_str_radix(&stem[..3], 16).map_err(|_| VmiError::Stem(stem.to_string()))?;
    let xxx = u16::from_str_radix(&stem[3..], 16).map_err(|_| VmiError::Stem(stem.to_string()))?;
    if kind == Kind::Ui && sss > UI_AFTER {
        return Err(VmiError::State {
            kind,
            sss,
            file: file.display().to_string(),
        });
    }
    Ok((sss, xxx))
}

fn choose_audio(files: &[MediaFile]) -> Vec<MediaFile> {
    let mut by_stem: BTreeMap<&str, Vec<&MediaFile>> = BTreeMap::new();
    for file in files {
        if is_audio(&file.ext) {
            by_stem.entry(&file.stem).or_default().push(file);
        }
    }
    by_stem
        .into_values()
        .filter_map(|group| {
            group
                .iter()
                .copied()
                .find(|f| f.ext == "wav")
                .or_else(|| group.iter().copied().find(|f| f.ext == "flac"))
                .or_else(|| group.iter().copied().find(|f| f.ext == "ogg"))
                .cloned()
        })
        .collect()
}

fn is_media(ext: &str) -> bool {
    ext == "png" || is_audio(ext)
}

fn is_audio(ext: &str) -> bool {
    matches!(ext, "wav" | "ogg" | "flac")
}

fn extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
}

fn file_stem(file: &str) -> String {
    Path::new(file)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string()
}

fn folder_name(folder: &str) -> String {
    Path::new(folder)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(folder)
        .to_string()
}

fn resolve_ci(folder: &Path, name: &str) -> Option<PathBuf> {
    let want = name.to_ascii_lowercase();
    let rd = fs::read_dir(folder).ok()?;
    for entry in rd.flatten() {
        if entry
            .file_name()
            .to_string_lossy()
            .eq_ignore_ascii_case(&want)
        {
            return Some(entry.path());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures")
            .join(name)
    }

    #[test]
    fn real_gamer_export_is_a_ui_scene() {
        let scene = load_scene(&fixture("vmi_scene")).unwrap();
        assert_eq!(scene.kind, Kind::Ui);
        assert_eq!(scene.scene, "gamer");
        assert_eq!(scene.naming, "SSSXXX");
        assert_eq!(scene.objects.len(), 13);
        let background = scene.objects.iter().find(|o| o.id == "Background").unwrap();
        assert_eq!(background.name, "Background");
        assert!(background
            .files
            .iter()
            .any(|f| f.ext == "png" && f.sss == UI_REST));
        assert!(background.files.iter().all(|f| f.path.exists()));
        assert!(scene.objects.iter().all(|o| !o.name.is_empty()));
    }

    #[test]
    fn pack_fixture_accepts_uppercase_wav_and_names_from_json() {
        let scene = load_scene(&fixture("vmi_pack")).unwrap();
        assert_eq!(scene.kind, Kind::Pack);
        assert_eq!(scene.objects.len(), 1);
        let kick = &scene.objects[0];
        assert_eq!(kick.name, "kick");
        assert_eq!(kick.audio.len(), 2);
        assert!(kick.audio.iter().all(|a| a.ext == "wav"));
        assert_eq!(kick.audio[0].label.as_deref(), Some("bank0-slice0"));
        assert_ne!(kick.audio[0].stem, kick.name);
    }

    #[test]
    fn kind_rules_and_bad_names() {
        let dir = std::env::temp_dir().join(format!("trust-vmi-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        write_min(&dir, "ui", "004000.wav");
        assert!(
            load_scene(&dir).is_err(),
            "ui SSS 004 is not a visual state"
        );

        let _ = fs::remove_dir_all(&dir);
        write_min(&dir, "pack", "004000.wav");
        let scene = load_scene(&dir).unwrap();
        assert_eq!(scene.objects[0].audio[0].sss, 0x004);

        let _ = fs::remove_dir_all(&dir);
        write_min(&dir, "pack", "kick_000000.wav");
        assert!(matches!(load_scene(&dir), Err(VmiError::Stem(_))));

        let _ = fs::remove_dir_all(&dir);
        write_min(&dir, "pack", "000000.wav");
        fs::write(dir.join("objects/kick/000000.ogg"), b"ogg").unwrap();
        let scene = load_scene(&dir).unwrap();
        assert_eq!(scene.objects[0].audio.len(), 1);
        assert_eq!(scene.objects[0].audio[0].ext, "wav");

        let _ = fs::remove_dir_all(&dir);
        write_min(&dir, "ui", "000000.PNG");
        let obj = dir.join("objects/kick/object.json");
        let mut text = fs::read_to_string(&obj).unwrap();
        text = text.replace("\"png\": false", "\"png\": true");
        fs::write(&obj, text).unwrap();
        fs::remove_file(dir.join("objects/kick/000000.PNG")).unwrap();
        assert!(load_scene(&dir).is_err(), "png true must exist");

        let _ = fs::remove_dir_all(&dir);
    }

    fn write_min(dir: &Path, kind: &str, media_name: &str) {
        fs::create_dir_all(dir.join("objects/kick")).unwrap();
        fs::write(
            dir.join("trust.json"),
            format!(r#"{{"version":1,"kind":"{kind}"}}"#),
        )
        .unwrap();
        fs::write(
            dir.join("scene.json"),
            r#"{"vmi":1,"naming":"SSSXXX","scene":"temp","canvas":{"w":1,"h":1},"source":{"file":"t","kind":"pack"},"objects":[{"id":"kick","folder":"objects/kick","type":"static","zIndex":0,"slots":{"000000":{"file":"PLACE","label":"from-json","layers":["kick"],"png":false}}}]}"#
                .replace("PLACE", media_name),
        )
        .unwrap();
        let stem = Path::new(media_name)
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let sss = if stem.len() >= 3 { &stem[..3] } else { "000" };
        let xxx = if stem.len() >= 6 { &stem[3..6] } else { "000" };
        fs::write(
            dir.join("objects/kick/object.json"),
            format!(
                r#"{{"name":"kick","type":"static","zIndex":0,"slots":[{{"file":"{media_name}","state":"{sss}","frame":"{xxx}","label":"from-json","layers":["kick"],"png":false}}]}}"#
            ),
        )
        .unwrap();
        fs::write(dir.join("objects/kick").join(media_name), b"x").unwrap();
    }
}
