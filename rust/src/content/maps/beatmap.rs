use godot::register::GodotClass;

#[derive(Default, Clone, Debug, GodotClass)]
#[class(base=RefCounted, no_init)]
pub struct NoteData {
    pub x: f32,
    pub y: f32,
    pub time: f32,
}

#[derive(GodotClass, Clone, Debug)]
#[class(base=RefCounted, no_init)]
pub struct Beatmap {
    pub broken: bool,
    pub version: u8,
    pub path: String,
    pub name: String,
    pub notes: Vec<NoteData>,
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