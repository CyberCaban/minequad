use macroquad::{models::Vertex, prelude::*};
// use glam::vec3;

const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.1;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Minequad"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut x = 0.0;
    let mut switch = false;
    let bounds = 8.0;

    let world_up = vec3(0.0, 1.0, 0.0);
    let mut yaw: f32 = 1.18;
    let mut pitch: f32 = 0.0;

    let mut front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();
    let mut right = front.cross(world_up).normalize();
    let mut up = right.cross(front).normalize();

    let mut position = vec3(0.0, 1.0, 0.0);
    let mut last_mouse_position: Vec2 = mouse_position().into();

    let mut grabbed = false;
    set_cursor_grab(grabbed);
    show_mouse(true);

    loop {
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }
        let mut velocity = vec3(0.0, 0.0, 0.0);
        if is_key_down(KeyCode::W) {
            velocity += vec3(yaw.cos(), 0.0, yaw.sin());
        }
        if is_key_down(KeyCode::S) {
            velocity -= vec3(yaw.cos(), 0.0, yaw.sin());
        }
        if is_key_down(KeyCode::A) {
            velocity -= right;
        }
        if is_key_down(KeyCode::D) {
            velocity += right;
        }
        position += velocity * MOVE_SPEED;
        if is_key_down(KeyCode::Space) {
            position += world_up * 0.03;
        }
        if is_key_down(KeyCode::LeftShift) {
            position -= world_up * 0.03;
        }

        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - last_mouse_position;
        last_mouse_position = mouse_position;

        if grabbed {
            yaw += mouse_delta.x * delta * LOOK_SPEED;
            pitch += mouse_delta.y * delta * -LOOK_SPEED;

            pitch = if pitch > 1.5 { 1.5 } else { pitch };
            pitch = if pitch < -1.5 { -1.5 } else { pitch };

            front = vec3(
                yaw.cos() * pitch.cos(),
                pitch.sin(),
                yaw.sin() * pitch.cos(),
            )
            .normalize();

            right = front.cross(world_up).normalize();
            up = right.cross(front).normalize();

            x += if switch { 0.04 } else { -0.04 };
            if x >= bounds || x <= -bounds {
                switch = !switch;
            }
        }

        clear_background(LIGHTGRAY);

        // Going 3d!

        set_camera(&Camera3D {
            position: position,
            up: up,
            target: position + front,
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);

        draw_line_3d(
            vec3(x, 0.0, x),
            vec3(5.0, 5.0, 5.0),
            Color::new(1.0, 1.0, 0.0, 1.0),
        );

        draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), GREEN);
        draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), BLUE);
        draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), RED);

        // Back to screen space, render some text

        set_default_camera();
        draw_text("First Person Camera", 10.0, 20.0, 30.0, BLACK);

        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            10.0,
            44.0,
            30.0,
            BLACK,
        );
        draw_text(
            format!(
                "X: {:.2} Y: {:.2} Z: {:.2}",
                position.x, position.y, position.z
            )
            .as_str(),
            10.0,
            48.0 + 18.0,
            30.0,
            BLACK,
        );
        draw_text(
            format!("Press <TAB> to toggle mouse grab: {}", grabbed).as_str(),
            10.0,
            48.0 + 42.0,
            30.0,
            BLACK,
        );
        draw_text(
            format!(
                "Yaw: {:.2} Pitch: {:.2}",
                yaw.to_degrees(),
                pitch.to_degrees(),
            )
            .as_str(),
            10.0,
            48.0 + 58.0,
            30.0,
            BLACK,
        );
        next_frame().await
    }
}
