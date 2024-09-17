#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod systems;

use std::{rc::Rc, vec};

use macroquad::{
    prelude::*,
    ui::{self, hash},
};
use systems::chunk::{Chunk, ChunkRenderer, CHUNK_D, CHUNK_W};

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

#[macroquad::main(conf)]
async fn main() {
    let atlas = Texture2D::from_file_with_format(ATLAS, Some(ImageFormat::Png));
    atlas.set_filter(FilterMode::Nearest);

    let mut player = Player::new();
    let mut projection = 0;
    let LIGHTBLUE = Color {
        r: 135.0 / 255.0,
        g: 206.0 / 255.0,
        b: 250.0 / 255.0,
        a: 1.0,
    };

    let mut chunks: Vec<Vec<Chunk>> = Vec::new();
    for _ in 0..6 {
        chunks.push(vec![
            Chunk::new(),
            Chunk::new(),
            Chunk::new(),
            Chunk::new(),
            Chunk::new(),
            Chunk::new(),
        ]);
    }
    for x in 0..6 {
        for z in 0..6 {
            chunks[x][z].populate(((x * CHUNK_W) as f32, 0.0, (z * CHUNK_D) as f32));
        }
    }

    let mut renderers: Vec<ChunkRenderer> = vec![];

    for x in 0..6 {
        for z in 0..6 {
            renderers.push(ChunkRenderer::new());
            renderers[x * 6 + z].gen_mesh(&chunks[x][z], &atlas);
        }
    }

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
            Some(&atlas),
            WHITE,
        );

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
