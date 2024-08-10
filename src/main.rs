#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod systems;

use std::{fs, vec};

use macroquad::{models::Vertex, prelude::*};
use systems::{
    chunks::{Block, BlockType},
    devInfo::dev_info_system,
};

use crate::systems::controls::*;

const CHUNK_SIZE_16: i32 = 16;
fn conf() -> Conf {
    Conf {
        window_title: String::from("Minequad"),
        window_width: 1572,
        window_height: 960,
        fullscreen: false,
        high_dpi: true,
        sample_count: 4,
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

    let mut arr = Vec::new();
    for y in 0..CHUNK_SIZE_16 {
        let mut layer = Vec::new();
        for x in 0..3 {
            let mut row = Vec::new();
            for z in 0..CHUNK_SIZE_16 {
                let block_pos = vec3(x as f32, y as f32, z as f32);
                let block_type =
                    if x == 0 || z == 0 || x == CHUNK_SIZE_16 - 1 || z == CHUNK_SIZE_16 - 1 {
                        BlockType::Stone
                    } else {
                        BlockType::Grass
                    };
                let block_tex = &textures[match block_type {
                    BlockType::Stone => 0,
                    BlockType::Grass => 1,
                }];
                row.push(Block::new(block_type, block_pos, block_tex.clone()));
            }
            layer.push(row);
        }
        arr.push(layer);
    }

    let mut player = Player::new();
    loop {
        clear_background(WHITE);
        player.update();

        for x in 0..CHUNK_SIZE_16 {
            for y in 0..arr[1].len() {
                for z in 0..CHUNK_SIZE_16 {
                    arr[x as usize][y as usize][z as usize].make_mesh();
                }
            }
        }

        // Going 3d!
        draw_grid(100, 1., BLACK, GRAY);
        // draw_cube(vec3(0.0, 6.0, 0.0), vec3(10.0, 10.0, 10.0), Some(&textures[1].clone()), WHITE);

        // Back to screen space, render some text

        set_default_camera();

        dev_info_system(&player);
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        
        next_frame().await
    }
}
