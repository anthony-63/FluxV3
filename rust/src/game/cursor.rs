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
    abs_camera: Option<Gd<Camera3D>>,

    sensitivity: f32,
    absolute_scale: f32,
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
            abs_camera: None,
            sensitivity: 0.5,
            absolute_scale: 1.,
            spin: false,
            pitch: 0.,
            yaw: 0.,
        }
    } 

    fn enter_tree(&mut self) {
        unsafe {
            self.sensitivity = FLUX.settings.as_ref().unwrap().cursor.sensitivity;
            self.absolute_scale = FLUX.settings.as_ref().unwrap().cursor.absolute_scale;
            self.spin = FLUX.settings.as_ref().unwrap().camera.spin;
        }

        let mut camera = self.base_mut().get_node_as::<Camera3D>("../Camera");
        let mut abs_camera = self.base_mut().get_node_as::<Camera3D>("../AbsCamera");
        
        camera.set_keep_aspect_mode(KeepAspect::HEIGHT);
        camera.set_fov(unsafe { FLUX.settings.as_ref().unwrap().camera.fov });
        abs_camera.set_keep_aspect_mode(KeepAspect::HEIGHT);
        
        if !self.spin {
            let mut cam_transform = camera.get_transform();
            cam_transform.origin.z = 0.5;
            camera.set_transform(cam_transform);

            let mut abs_cam_transform = abs_camera.get_transform();
            abs_cam_transform.origin.z = 0.5;
            abs_camera.set_transform(abs_cam_transform);
        }

        if unsafe { FLUX.settings.as_ref().unwrap().cursor.absolute } {
            Input::singleton().set_mouse_mode(MouseMode::CONFINED);
        } else {
            Input::singleton().set_mouse_mode(MouseMode::CAPTURED);
        }

        self.camera = Some(camera);
        self.abs_camera = Some(abs_camera);

        Input::singleton().set_use_accumulated_input(false);
        Input::singleton().set_custom_mouse_cursor(load("res://assets/images/blank.png"));
    }

    fn exit_tree(&mut self) {
        Input::singleton().call("set_custom_mouse_cursor".into(), &[Variant::nil()]);
        Input::singleton().set_mouse_mode(MouseMode::VISIBLE);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let Ok(event_motion) = event.try_cast::<InputEventMouseMotion>() else {
            return;
        };

        let relative = event_motion.get_relative() * (self.sensitivity / 4.);

        if unsafe { FLUX.settings.as_ref().unwrap().cursor.absolute } {
            if self.spin {
                let pos = self.abs_camera.as_ref().unwrap().project_position(event_motion.get_position(), 3.5) * (self.absolute_scale * 2.);
                self.position = Vector2::new(pos.x, pos.y);
                self.camera.as_mut().unwrap().look_at(Vector3::new(pos.x, pos.y, 0.));
            } else {
                let pos = self.abs_camera.as_ref().unwrap().project_position(event_motion.get_position(), 3.75) * (self.absolute_scale * 2.);
                self.position = Vector2::new(pos.x, pos.y);
            }
        } else {
            if self.spin {
                let camera = self.camera.as_mut().unwrap();
                camera.set_rotation_degrees(Vector3::new(self.pitch, self.yaw, 0.));

                let camera_trans = camera.get_global_transform();
                let basis = camera.get_global_basis();

                let look = Vector3::new(basis.rows[0].z, basis.rows[1].z, basis.rows[2].z);

                let pos_vec = Vector2::new(camera_trans.origin.x, camera_trans.origin.y);
                self.position = pos_vec - Vector2::new(look.x, look.y) * (camera_trans.origin.z / look.z);
            } else {
                self.pitch = 0.;
                self.yaw = 0.;
                self.position += Vector2::new(relative.x, -relative.y) * 0.1675;
            }
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
        
        let camera = self.camera.as_mut().unwrap();
        
        let parallax = unsafe { FLUX.settings.as_ref().unwrap().camera.parallax };

        let mut camera_transform = camera.get_transform();
        camera_transform.origin.x = (self.clamped_position.x * parallax) / 10.;
        camera_transform.origin.y = (self.clamped_position.y * parallax) / 10.;
        camera_transform.origin /= Vector3::ONE * 4. + camera_transform.basis.rows[2] / 2.;
        camera.set_transform(camera_transform);

        if self.spin {
            self.pitch -= relative.y;
            self.yaw -= relative.x;

            self.pitch = self.pitch.clamp(-90., 90.);
            self.yaw = self.yaw.clamp(-180., 180.);
        }
    }
}

#[godot_api]
impl Cursor {

}