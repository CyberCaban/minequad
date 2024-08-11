use macroquad::{
    camera::{set_camera, Camera3D},
    input::{
        is_key_down, is_key_pressed, is_key_released, mouse_position, set_cursor_grab, show_mouse,
        KeyCode,
    },
    math::{vec3, Vec2, Vec3},
    time::get_frame_time,
    window::{screen_height, screen_width},
};

const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.09;
const FLY_SPEED: f32 = 0.06;

pub struct Player {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub grabbed: bool,
    last_mouse_position: Vec2,
}

impl Player {
    pub fn new() -> Self {
        set_cursor_grab(false);
        show_mouse(true);
        Self {
            position: Vec3::new(0.0, 0.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            grabbed: false,
            last_mouse_position: mouse_position().into(),
        }
    }

    fn update_pos(&mut self) {
        let world_up = vec3(0.0, 1.0, 0.0);
        let front_velocity = vec3(self.yaw.cos(), 0.0, self.yaw.sin());
        let left_velocity = vec3(self.yaw.sin(), 0.0, -self.yaw.cos());
        let mut velocity = vec3(0.0, 0.0, 0.0);

        if is_key_down(KeyCode::W) {
            velocity += front_velocity;
        }
        if is_key_down(KeyCode::S) {
            velocity -= front_velocity;
        }
        if is_key_down(KeyCode::A) {
            velocity += left_velocity;
        }
        if is_key_down(KeyCode::D) {
            velocity -= left_velocity;
        }
        if is_key_down(KeyCode::Space) {
            velocity += world_up;
        }
        if is_key_down(KeyCode::LeftShift) {
            velocity -= world_up;
        }
        if velocity.x + velocity.y + velocity.z == 0.0 {
            velocity = vec3(0.0, 0.0, 0.0);
        } else {
            velocity = velocity.normalize();
        }
        self.position += velocity * MOVE_SPEED * get_frame_time() * 100.0;
    }
    fn update_look(&mut self) {
        if is_key_down(KeyCode::LeftAlt) {
            show_mouse(true);
        }
        if is_key_released(KeyCode::LeftAlt) {
            show_mouse(false);
        }
        if is_key_pressed(KeyCode::Tab) {
            self.grabbed = !self.grabbed;
            set_cursor_grab(self.grabbed);
            show_mouse(!self.grabbed);
        }
        let delta = get_frame_time();
        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - self.last_mouse_position;
        self.last_mouse_position = mouse_position;
        if !self.grabbed {
            return;
        }

        self.yaw += mouse_delta.x * delta * LOOK_SPEED;
        self.pitch += mouse_delta.y * delta * -LOOK_SPEED;
        self.pitch = if self.pitch > 1.5 { 1.5 } else { self.pitch };
        self.pitch = if self.pitch < -1.5 { -1.5 } else { self.pitch };
    }
    fn update_mouse_grab(&mut self) {
    }
    fn get_camera_orientation(&self) -> (Vec3, Vec3) {
        let world_up = vec3(0.0, 1.0, 0.0);
        let front = vec3(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize();
        let right = front.cross(world_up).normalize();
        let up = right.cross(front).normalize();
        let target = self.position + front;
        (up, target)
    }
    pub fn update(&mut self) {
        self.update_mouse_grab();
        self.update_pos();
        self.update_look();
        let (up, target) = self.get_camera_orientation();
        set_camera(&Camera3D {
            position: self.position,
            up,
            target,
            fovy: 45.0, // min: 38.0 max: 47.0
            aspect: Some(screen_width() / screen_height()),
            ..Default::default()
        });
    }
}
