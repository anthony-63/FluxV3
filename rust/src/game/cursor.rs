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
    spin: bool,

    pub yaw: f32,
    pub pitch: f32,
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
            spin: false,
            pitch: 0.,
            yaw: 0.,
        }
    }

    fn enter_tree(&mut self) {
        unsafe {
            self.sensitivity = FLUX.settings.as_ref().unwrap().cursor.sensitivity;
            self.spin = FLUX.settings.as_ref().unwrap().camera.spin;
        }

        let mut camera = self.base_mut().get_node_as::<Camera3D>("../Camera");
        
        camera.set_keep_aspect_mode(KeepAspect::HEIGHT);
        
        if !self.spin {
            let mut cam_transform = camera.get_transform();
            cam_transform.origin.z = 0.5;
            camera.set_transform(cam_transform);
        }

        self.camera = Some(camera);

        Input::singleton().set_mouse_mode(MouseMode::CAPTURED);
        Input::singleton().set_use_accumulated_input(false);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let Ok(event_motion) = event.try_cast::<InputEventMouseMotion>() else {
            return;
        };

        let relative = event_motion.get_relative() * (self.sensitivity / 4.);

        if self.spin {
            let camera = self.camera.as_ref().unwrap();
            let camera_trans = camera.get_transform();
            let camera_rot = camera.get_rotation();

            let rot_vec = Vector2::new(-camera_rot.y, camera_rot.x);
            let pos_vec = Vector2::new(camera_trans.origin.x, camera_trans.origin.y);
            self.position = pos_vec + rot_vec * -(camera_trans.origin.z + self.base().get_transform().origin.z);
        } else {
            self.pitch = 0.;
            self.yaw = 0.;
            self.position += Vector2::new(relative.x, -relative.y) * 0.1675;
        }

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

        
        if !self.spin {
            let mut camera_transform = camera.get_transform();
            camera_transform.origin.x = clamped.x * 0.1;
            camera_transform.origin.y = clamped.y * 0.1;
            camera.set_transform(camera_transform);
        } else {
            let prev_rot = camera.get_rotation_degrees();
            camera.set_rotation_degrees(prev_rot - Vector3::new(relative.y, relative.x, 0.));
        }

    }
}

#[godot_api]
impl Cursor {

}