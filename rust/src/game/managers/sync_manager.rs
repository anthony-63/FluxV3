use godot::{engine::{AudioServer, AudioStream, Time}, prelude::*};

use crate::game::Game;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct SyncManager {
    base: Base<Node>,
    game: Option<Gd<Game>>,
    audio_player: Option<Gd<AudioStreamPlayer>>,
    pub speed: f32,
    
    last_time: f64,
    time_delay: f64,
    playing: bool,
    
    pub start_timer: f32,
    pub start_delay: f32,
    
    pub real_time: f64,
}

#[godot_api]
impl INode for SyncManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            game: None,
            audio_player: None,
            last_time: 0.,
            real_time: 0.,
            time_delay: 0.,
            playing: false,
            start_timer: 0.,
            start_delay: 1.,
            speed: 1.,
        }
    }

    fn enter_tree(&mut self) {
        let game = self.base_mut().get_node_as::<Game>("../GameManager");
        let audio_player = self.base_mut().get_node_as::<AudioStreamPlayer>("Music");

        self.audio_player = Some(audio_player);
        self.game = Some(game);
    }

    fn process(&mut self, dt: f64) {
        if self.start_timer < self.start_delay {
            self.start_timer += dt as f32;
        }

        if !self.playing {
            return
        }

        let now = Time::singleton().get_ticks_usec() as f64;
        let time = self.speed as f64 * (now - self.last_time) * 0.000001;
        self.last_time = now;
        self.real_time += time;
    }
}

#[godot_api]
impl SyncManager {
    #[func]
    pub fn set_stream(&mut self, stream: Gd<AudioStream>) {
        let mut audio_player = self.base_mut().get_node_as::<AudioStreamPlayer>("Music");

        audio_player.stop();
        audio_player.set_stream(stream);
        
        self.audio_player = Some(audio_player);
    }

    #[func]
    pub fn set_offset(&mut self) {
        let audio_player = self.audio_player.as_mut().unwrap();

        self.time_delay = AudioServer::singleton().get_time_to_next_mix() + AudioServer::singleton().get_output_latency();
        audio_player.seek((self.real_time + self.time_delay) as f32);
    }

    #[func]
    pub fn start(&mut self, from: f64) {
        let audio_player = self.audio_player.as_mut().unwrap();

        self.last_time = Time::singleton().get_ticks_usec() as f64;
        self.real_time = from.min(from * self.speed as f64);
        audio_player.seek(self.real_time as f32);
        audio_player.play();
        audio_player.set_pitch_scale(self.speed);
        self.set_offset();
        self.playing = true;
    }
}