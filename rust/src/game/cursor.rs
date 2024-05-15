use godot::{engine::{camera_3d::KeepAspect, input::MouseMode, InputEvent, InputEventMouseMotion, Sprite3D}, prelude::*};

use crate::FLUX;

static CLAMP: f32 = (6. - 0.525) / 2.;

#[derive(GodotClass)]
#[class(base=Sprite3D)]
pub struct Cursor {
    base: Base<Sprite3D>,
    position: Vector2,
    pub clamped_position: Vector2,
    camera: Option<Gd<Camera3D>>,
    sensitivity: f32,
}

#[godot_api]
impl INode3D for Cursor {
    fn init(base: Base<Sprite3D>) -> Self {
        Self {
            base,
            position: Vector2::ZERO,
            clamped_position: Vector2::ZERO,
            camera: None,
            sensitivity: 0.5,
        }
    }

    fn enter_tree(&mut self) {
        let mut camera = self.base_mut().get_node_as::<Camera3D>("../Camera");
        
        camera.set_keep_aspect_mode(KeepAspect::HEIGHT);
        
        self.camera = Some(camera);

        unsafe {
            self.sensitivity = FLUX.settings.as_ref().unwrap().cursor.sensitivity;
        }

        Input::singleton().set_mouse_mode(MouseMode::CAPTURED);

    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let Ok(event_motion) = event.try_cast::<InputEventMouseMotion>() else {
            return;
        };

        let relative = event_motion.get_relative() * (self.sensitivity / 4.);

        self.position += Vector2::new(relative.x, -relative.y) * 0.1675;
        self.clamped_position = Vector2::new(
            self.position.x.clamp(-CLAMP, CLAMP),
            self.position.y.clamp(-CLAMP, CLAMP),
        );

        self.position = self.clamped_position;

        let mut transform = self.base().get_transform();
        transform.origin.x = self.clamped_position.x;
        transform.origin.y = self.clamped_position.y;
        self.base_mut().set_transform(transform);
        
        let clamped = self.clamped_position;

        let camera = self.camera.as_mut().unwrap();

        let mut camera_transform = camera.get_transform();

        camera_transform.origin.x = clamped.x * 0.1;
        camera_transform.origin.y = clamped.y * 0.1;

        camera.set_transform(camera_transform);
    }
}

#[godot_api]
impl Cursor {

}