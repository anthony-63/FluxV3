use std::io::Read;

use super::Settings;

use godot::{engine::Os, log::godot_warn};

impl Settings {
    pub fn load(&mut self, filename: String) {
        let os = Os::singleton();
        let user_dir = os.get_user_data_dir().to_string();

        let setting_path = format!("{}/{}", user_dir, filename);
        let mut f = match std::fs::File::open(setting_path) {
            Ok(f) => f,
            Err(err) => {
                godot_warn!("failed to open settings file: {}", err);
                *self = Settings::new();
                return;
            }
        };

        let mut buffer: Vec<u8> = vec![];
        let _ = f.read_to_end(&mut buffer);

        let settings: Settings = match bincode::deserialize(&buffer) {
            Ok(settings) => settings,
            Err(error) => {
                godot_warn!("failed to load settings: {}", error);
                Settings::new()
            }
        };

        *self = settings;
    }
}