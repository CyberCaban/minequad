#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod systems;

use std::{rc::Rc, vec};

use macroquad::{
    prelude::*,
    ui::{self, hash},
};
use ::rand::random;
use systems::chunk::{Chunk, ChunkRenderer, CHUNK_D, CHUNK_H, CHUNK_W};

use crate::systems::controls::*;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Minequad"),
        window_width: 1572,
        window_height: 960,
        fullscreen: false,

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

static ATLAS: &[u8] = include_bytes!("../assets/textures/atlas.png");

fn create_chunks(length: usize, width: usize, height: usize, seed: i32) -> Vec<Vec<Vec<Chunk>>> {
    let mut chunks: Vec<Vec<Vec<Chunk>>> = Vec::new();
    for _ in 0..length {
        let mut rows: Vec<Vec<Chunk>> = Vec::new();
        for _ in 0..width {
            let mut column: Vec<Chunk> = Vec::new();
            for _ in 0..height {
                column.push(Chunk::new(seed));
            }
            rows.push(column);
        }
        chunks.push(rows);
    }
    for x in 0..length {
        for z in 0..width {
            for y in 0..height {
                chunks[x][z][y].populate(((x * CHUNK_W) as f32, (y * CHUNK_H) as f32, (z * CHUNK_D) as f32));
            }
        }
    }
    chunks
}

fn create_renderers(chunks: &Vec<Vec<Vec<Chunk>>>, atlas: &Texture2D) -> Vec<ChunkRenderer> {
    let mut renderers: Vec<ChunkRenderer> = vec![];
    let length = chunks.len();
    let width = chunks[0].len();

    for x in 0..length {
        for z in 0..width {
            let mut renderer = ChunkRenderer::new();
            renderer.gen_mesh(&chunks[x][z][0], atlas);
            renderers.push(renderer);
        }
    }
    renderers
}

#[macroquad::main(conf)]
async fn main() {
    let atlas = Texture2D::from_file_with_format(ATLAS, Some(ImageFormat::Png));
    atlas.set_filter(FilterMode::Nearest);

    let mut player = Player::new();
    let mut projection = 0;
    let lightblue = Color {
        r: 135.0 / 255.0,
        g: 206.0 / 255.0,
        b: 250.0 / 255.0,
        a: 1.0,
    };

    let seed = random();
    let chunks = create_chunks(16,16, 1, seed);
    let mut renderers = create_renderers(&chunks, &atlas);

    loop {
        clear_background(lightblue);
        if projection == 0 {
            player.projection = Projection::Perspective;
        } else if projection == 1 {
            player.projection = Projection::Orthographics;
        }
        player.update();

        for r in renderers.iter_mut() {
            r.render_mesh();
        }


        ui::root_ui().group(
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

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        set_default_camera();
        next_frame().await
    }
}
