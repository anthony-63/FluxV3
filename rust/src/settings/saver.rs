use super::Settings;

use godot::engine::Os;

impl Settings {
    pub fn save(&self, filename: String) {
        let os = Os::singleton();
        let user_dir = os.get_user_data_dir().to_string();

        let setting_path = format!("{}/{}", user_dir, filename);
        let f = std::fs::File::create(setting_path).unwrap();
        let _ = bincode::serialize_into(f, &self);
    }
}