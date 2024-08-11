#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod systems;

use std::vec;

use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets},
};
use systems::chunks::{Block, BlockSides, BlockType, RenderSides};

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
    let mut projection = 0;

    let mut draw_sides: Vec<bool> = vec![false; 6];
    let mut sides: RenderSides = RenderSides::default();
    let mut test_block = Block::new(BlockType::Stone, vec3(-2.0, 0.0, -3.0), textures[0].clone());

    loop {
        clear_background(WHITE);
        if projection == 0 {
            player.projection = Projection::Perspective;
        } else if projection == 1 {
            player.projection = Projection::Orthographics;
        }
        player.update();

        // for x in 0..CHUNK_SIZE_16 {
        //     for y in 0..arr[1].len() {
        //         for z in 0..CHUNK_SIZE_16 {
        //             arr[x as usize][y as usize][z as usize].render_mesh();
        //         }
        //     }
        // }

        draw_cube(
            vec3(-4.5, 0.5, -2.5),
            vec3(1.0, 1.0, 1.0),
            Some(&textures[0].clone()),
            WHITE,
        );

        // Going 3d!
        draw_grid(100, 1., BLACK, GRAY);

        // Back to screen space, render some text

        test_block.make_mesh(&sides);
        test_block.construct_mesh();
        if draw_sides[0] {
            sides.top = Some(&BlockSides::Top);
        } else {
            sides.top = None;
        }
        if draw_sides[1] {
            sides.bottom = Some(&BlockSides::Bottom);
        } else {
            sides.bottom = None;
        }
        if draw_sides[2] {
            sides.front = Some(&BlockSides::Front);
        } else {
            sides.front = None;
        }
        if draw_sides[3] {
            sides.back = Some(&BlockSides::Back);
        } else {
            sides.back = None;
        }
        if draw_sides[4] {
            sides.left = Some(&BlockSides::Left);
        } else {
            sides.left = None;
        }
        if draw_sides[5] {
            sides.right = Some(&BlockSides::Right);
        } else {
            sides.right = None;
        }
        
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        
        root_ui().group(
            hash!(),
            vec2(screen_width() / 4.0, screen_height() / 4.0),
            |ui| {
                ui.checkbox(hash!(), "Render top", &mut draw_sides[0]);
                ui.checkbox(hash!(), "Render bottom", &mut draw_sides[1]);
                ui.checkbox(hash!(), "Render front", &mut draw_sides[2]);
                ui.checkbox(hash!(), "Render back", &mut draw_sides[3]);
                ui.checkbox(hash!(), "Render left", &mut draw_sides[4]);
                ui.checkbox(hash!(), "Render right", &mut draw_sides[5]);
                
                ui.slider(hash!(), "Fovy", 38.0..47.0, &mut player.fovy);
                ui.combo_box(hash!(), "Projection", &["Perspective", "Orthographic"],   &mut projection);
                ui.label(None, format!("FPS: {}", get_fps()).as_str());
                ui.label(
                    None,
                    format!(
                        "X: {:.2} Y: {:.2} Z: {:.2}",
                        player.position.x, player.position.y, player.position.z
                    )
                    .as_str(),
                );
                ui.label(
                    None,
                    format!("Yaw: {:.2} Pitch: {:.2}", player.yaw, player.pitch).as_str(),
                );
            },
        );
        set_default_camera();

        next_frame().await
    }
}
