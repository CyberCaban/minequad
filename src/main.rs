mod systems;

use std::vec;

use macroquad::{models::Vertex, prelude::*};

use crate::systems::controls::*;
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
                vert(self.position + vec3(0.0, 1.0, 0.0), vec2(0.0, 0.0)),
                vert(self.position + vec3(0.0, 1.0, 1.0), vec2(1.0, 0.0)),
                vert(self.position + vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)),
                // front
                vert(self.position + vec3(0.0, 0.0, 0.0), vec2(0.0, 0.0)),
                vert(self.position + vec3(1.0, 0.0, 0.0), vec2(0.0, 1.0)),
                vert(self.position + vec3(1.0, 1.0, 0.0), vec2(1.0, 1.0)),
                vert(self.position + vec3(0.0, 0.0, 0.0), vec2(0.0, 0.0)),
                vert(self.position + vec3(0.0, 1.0, 0.0), vec2(1.0, 0.0)),
                vert(self.position + vec3(1.0, 1.0, 0.0), vec2(1.0, 1.0)),
                // back
                vert(self.position + vec3(0.0, 0.0, 1.0), vec2(0.0, 0.0)),
                vert(self.position + vec3(1.0, 0.0, 1.0), vec2(0.0, 1.0)),
                vert(self.position + vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)),
                vert(self.position + vec3(0.0, 0.0, 1.0), vec2(0.0, 0.0)),
                vert(self.position + vec3(0.0, 1.0, 1.0), vec2(1.0, 0.0)),
                vert(self.position + vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)),
                // left
                vert(self.position + vec3(0.0, 0.0, 0.0), vec2(0.0, 0.0)),
                vert(self.position + vec3(0.0, 0.0, 1.0), vec2(0.0, 1.0)),
                vert(self.position + vec3(0.0, 1.0, 1.0), vec2(1.0, 1.0)),
                vert(self.position + vec3(0.0, 0.0, 0.0), vec2(0.0, 0.0)),
                vert(self.position + vec3(0.0, 1.0, 0.0), vec2(1.0, 0.0)),
                vert(self.position + vec3(0.0, 1.0, 1.0), vec2(1.0, 1.0)),
                // right
                vert(self.position + vec3(1.0, 0.0, 0.0), vec2(0.0, 0.0)),
                vert(self.position + vec3(1.0, 0.0, 1.0), vec2(0.0, 1.0)),
                vert(self.position + vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)),
                vert(self.position + vec3(1.0, 0.0, 0.0), vec2(0.0, 0.0)),
                vert(self.position + vec3(1.0, 1.0, 0.0), vec2(1.0, 0.0)),
                vert(self.position + vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)),
            ],
            indices: vec![
                0, 1, 2, 3, 4, 5, // bottom
                6, 7, 8, 9, 10, 11, // top
                12, 13, 14, 15, 16, 17, // front
                18, 19, 20, 21, 22, 23, // back
                24, 25, 26, 27, 28, 29, // left
                30, 31, 32, 33, 34, 35, // right
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
    let atlas = load_texture("assets/textures/atlas.png").await.unwrap();
    atlas.set_filter(FilterMode::Nearest);

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

    let mut player = Player::new();
    loop {
        player.update();
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        clear_background(LIGHTGRAY);

        // Going 3d!
        draw_grid(100, 1., BLACK, GRAY);

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
                player.position.x, player.position.y, player.position.z
            )
            .as_str(),
            10.0,
            48.0 + 18.0,
            30.0,
            BLACK,
        );
        draw_text(
            format!("Press <TAB> to toggle mouse grab: {}", player.grabbed).as_str(),
            10.0,
            48.0 + 42.0,
            30.0,
            BLACK,
        );
        draw_text(
            format!(
                "Yaw: {:.2} Pitch: {:.2}",
                player.yaw.to_degrees(),
                player.pitch.to_degrees(),
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
