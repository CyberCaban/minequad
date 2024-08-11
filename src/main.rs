#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod systems;

use std::{rc::Rc, vec};

use macroquad::{
    prelude::*,
    ui::{
        hash, root_ui,
        widgets::{self, Texture},
    },
};
use systems::{
    blocks::{Block, BlockType, RenderSides},
    demo_features::DemoFeatures,
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

async fn load_tex() -> Vec<Rc<Texture2D>> {
    let mut textures: Vec<Rc<Texture2D>> = vec![];

    let stone = load_texture("assets/textures/stone.png").await.unwrap();
    stone.set_filter(FilterMode::Nearest);

    let grass = load_texture("assets/textures/grass.png").await.unwrap();
    grass.set_filter(FilterMode::Nearest);

    textures.push(stone.into());
    textures.push(grass.into());

    textures
}

static STONE: &[u8] = include_bytes!("../assets/textures/stone.png");
static GRASS: &[u8] = include_bytes!("../assets/textures/grass.png");

#[macroquad::main(conf)]
async fn main() {
    // let textures = load_tex().await;
    let stone_tex = Texture2D::from_file_with_format(STONE, Some(ImageFormat::Png));
    stone_tex.set_filter(FilterMode::Nearest);
    let grass_tex = Texture2D::from_file_with_format(GRASS, Some(ImageFormat::Png));
    grass_tex.set_filter(FilterMode::Nearest);
    println!("Textures: {:?}", stone_tex);

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
                // let block_tex = &textures[match block_type {
                //     BlockType::Stone => 0,
                //     BlockType::Grass => 1,
                // }];
                let block_tex = match block_type {
                    BlockType::Stone => &stone_tex,
                    BlockType::Grass => &grass_tex,
                };
                let mut block = Block::new(block_type, block_pos, block_tex);
                block.make_mesh(&RenderSides::all());
                row.push(block);
            }
            layer.push(row);
        }
        arr.push(layer);
    }

    let mut demo = DemoFeatures::new(&stone_tex);
    let mut player = Player::new();
    let mut projection = 0;
    let LIGHTBLUE = Color {
        r: 135.0 / 255.0,
        g: 206.0 / 255.0,
        b: 250.0 / 255.0,
        a: 1.0,
    };

    loop {
        clear_background(LIGHTBLUE);
        if projection == 0 {
            player.projection = Projection::Perspective;
        } else if projection == 1 {
            player.projection = Projection::Orthographics;
        }
        player.update();

        draw_grid(100, 1., BLACK, GRAY);

        // Going 3d!

        draw_cube(
            vec3(-4.5, 0.5, -2.5),
            vec3(1.0, 1.0, 1.0),
            Some(&stone_tex),
            WHITE,
        );

        for x in 0..CHUNK_SIZE_16 {
            for y in 0..arr[1].len() {
                for z in 0..CHUNK_SIZE_16 {
                    arr[x as usize][y as usize][z as usize].render_mesh();
                }
            }
        }
        demo.render();

        root_ui().group(
            hash!(),
            vec2(screen_width() / 4.0, screen_height() / 4.0),
            |ui| {
                ui.slider(hash!(), "Fovy", 38.0..47.0, &mut player.fovy);
                ui.combo_box(
                    hash!(),
                    "Projection",
                    &["Perspective", "Orthographic"],
                    &mut projection,
                );
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
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await
    }
}
