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

struct BlockTexture {
    top: Texture2D,
    bottom: Texture2D,
    front: Texture2D,
    back: Texture2D,
    left: Texture2D,
    right: Texture2D,
}

enum BlockType {
    Stone,
    Grass,
}

struct Block {
    block_type: BlockType,
    texture: BlockTexture,
    position: Vec3,
}

fn vert(pos: Vec3, uv: Vec2) -> Vertex {
    Vertex {
        position: pos,
        uv,
        color: WHITE,
    }
}

impl Block {
    fn new(block_type: BlockType, position: Vec3, texture: Texture2D) -> Self {
        let texture = match block_type {
            BlockType::Stone => BlockTexture {
                top: texture.clone(),
                bottom: texture.clone(),
                front: texture.clone(),
                back: texture.clone(),
                left: texture.clone(),
                right: texture.clone(),
            },
            BlockType::Grass => BlockTexture {
                top: texture.clone(),
                bottom: texture.clone(),
                front: texture.clone(),
                back: texture.clone(),
                left: texture.clone(),
                right: texture.clone(),
            },
        };

        Self {
            block_type,
            texture,
            position,
        }
    }

    fn make_mesh(&self) {
        let mesh = Mesh {
            vertices: vec![
                // bottom
                vert(self.position + vec3(0.0, 0.0, 0.0), vec2(1.0, 1.0)),
                vert(self.position + vec3(1.0, 0.0, 0.0), vec2(0.0, 1.0)),
                vert(self.position + vec3(1.0, 0.0, 1.0), vec2(0.0, 0.0)),
                vert(self.position + vec3(0.0, 0.0, 0.0), vec2(1.0, 1.0)),
                vert(self.position + vec3(0.0, 0.0, 1.0), vec2(1.0, 0.0)),
                vert(self.position + vec3(1.0, 0.0, 1.0), vec2(0.0, 0.0)),
                // top
                vert(self.position + vec3(0.0, 1.0, 0.0), vec2(0.0, 0.0)),
                vert(self.position + vec3(1.0, 1.0, 0.0), vec2(0.0, 1.0)),
                vert(self.position + vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)),
                vert(self.position + vec3(0.0, 1.0, 1.0), vec2(1.0, 0.0)),
                // front
                vert(self.position + vec3(1.0, 1.0, 0.0), vec2(1.0, 1.0)),
                vert(self.position + vec3(0.0, 1.0, 0.0), vec2(1.0, 0.0)),
            ],
            indices: vec![
                0, 1, 2, 3, 4,
                5, // bottom
                  // 4, 5, 6, 4, 7, 6, // top
                  // 0, 1, 8, 0, 9, 8, // front
                  // 3, 2, 6, 3, 7, 6, // back
                  // 0, 3, 7, 0, 7, 4, // left
                  // 1, 2, 6, 1, 6, 5, // right
            ],
            texture: Some(self.texture.top.clone()),
        };
        draw_mesh(&mesh);
    }
}

async fn load_tex() -> Vec<Texture2D> {
    let mut textures: Vec<Texture2D> = vec![];

    let stone = load_texture("assets/textures/stone.png").await.unwrap();
    stone.set_filter(FilterMode::Nearest);

    let grass = load_texture("assets/textures/grass.png").await.unwrap();
    grass.set_filter(FilterMode::Nearest);

    textures.push(stone);
    textures.push(grass);

    textures
}

#[macroquad::main(conf)]
async fn main() {
    let textures = load_tex().await;

    let blocks = (0..8)
        .map(|x| {
            (0..8).map({
                let value = textures.clone();
                move |z| {
                    Block::new(
                        BlockType::Stone,
                        vec3(x as f32, 0.0, z as f32),
                        value[0].clone(),
                    )
                }
            })
        })
        .flatten()
        .collect::<Vec<_>>();

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

        for b in &blocks {
            b.make_mesh();
        }
        // for b in &blocks {
        //     // b.draw();
        // }

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
