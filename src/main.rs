#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod systems;

use std::{rc::Rc, vec};

use macroquad::{
    prelude::*,
    ui::{
        self, hash,
        widgets::{self, Texture},
    },
};
use systems::{
    blocks::{Block, BlockType, RenderSides},
    chunk::Chunk,
    chunk_renderer::Renderer,
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
static ATLAS: &[u8] = include_bytes!("../assets/textures/atlas.png");

#[macroquad::main(conf)]
async fn main() {
    // let textures = load_tex().await;
    let stone_tex = Texture2D::from_file_with_format(STONE, Some(ImageFormat::Png));
    stone_tex.set_filter(FilterMode::Nearest);
    let grass_tex = Texture2D::from_file_with_format(GRASS, Some(ImageFormat::Png));
    grass_tex.set_filter(FilterMode::Nearest);
    let atlas_tex = Texture2D::from_file_with_format(ATLAS, Some(ImageFormat::Png));
    atlas_tex.set_filter(FilterMode::Nearest);

    // let mut demo = DemoFeatures::new(&stone_tex);
    let mut player = Player::new();
    let mut projection = 0;
    let LIGHTBLUE = Color {
        r: 135.0 / 255.0,
        g: 206.0 / 255.0,
        b: 250.0 / 255.0,
        a: 1.0,
    };
    let mut chunk = Chunk::populate();
    let mut renderer = Renderer::new(1024 * 1024 * 8);
    let mesh = renderer.render(&chunk, &atlas_tex);

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
            Some(&atlas_tex),
            WHITE,
        );

        draw_mesh(&mesh);

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
