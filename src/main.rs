mod systems;

use std::{fs, vec};

use macroquad::{models::Vertex, prelude::*};
use miniquad::TextureParams;
use systems::{chunks::{Block, BlockType}, devInfo::dev_info_system};

use crate::systems::controls::*;

const CHUNK_SIZE_16: i32 = 16;
fn conf() -> Conf {
    Conf {
        window_title: String::from("Minequad"),
        window_width: 1572,
        window_height: 960,
        fullscreen: false,
        ..Default::default()
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
    let mut blocks = Vec::new();
    for x in 0..CHUNK_SIZE_16 {
        for z in 0..CHUNK_SIZE_16 {
            blocks.push(Block::new(BlockType::Stone, vec3(x as f32, 0.0, z as f32), textures[0].clone()));
        }
    }

    let mut player = Player::new();
    loop {
        clear_background(LIGHTGRAY);
        player.update();
        for b in &mut blocks {
            b.make_mesh();
        }
        // Going 3d!
        draw_grid(100, 1., BLACK, GRAY);

        // Back to screen space, render some text

        set_default_camera();
        
        dev_info_system(&player);
        next_frame().await
    }
}
