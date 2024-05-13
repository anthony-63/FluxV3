use super::beatmapset::BeatmapSet;

pub struct NoteData {
    x: f32,
    y: f32,
    time: f32,
}


pub struct Beatmap {
    broken: bool,
    version: u8,
    path: String,
    name: String,
    notes: Vec<NoteData>,
}

impl Beatmap {
    pub fn from_file(path: String) -> Self {
        let data_json = std::fs::read_to_string(path.clone()).expect("data json not found somehow?");
        let data = json::parse(&data_json).unwrap();

        let version = data["_version"].as_u8().expect("expected number for version");
        let name = data["_name"].to_string();

        let mut notes: Vec<NoteData> = vec![];
        for note in data["_notes"].members() {
            notes.push(NoteData {
                x: note["_x"].as_f32().unwrap(),
                y: note["_y"].as_f32().unwrap(),
                time: note["_time"].as_f32().unwrap(),
            });
        }

        Self {
            broken: false,
            version,
            path,
            name,
            notes,
        }
    }
}