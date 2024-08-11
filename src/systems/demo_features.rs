use super::blocks::{Block, BlockSides, BlockType, RenderSides};
use macroquad::{
    prelude::*,
    texture,
    ui::{hash, root_ui},
};

struct PartialBlockRender {
    block: Block,
    sides: RenderSides,
    draw_sides: [bool; 6],
}
pub struct DemoFeatures {
    block_demo: PartialBlockRender
}

impl DemoFeatures {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            block_demo: PartialBlockRender {
                block: Block::new(BlockType::Stone, vec3(-5.0, 0.0, -5.0), texture.clone()),
                sides: RenderSides::default(),
                draw_sides: [false; 6],
            },
        }
    }

    pub fn render(&mut self) {
        let demo = &mut self.block_demo;
        demo.block.make_mesh(&mut demo.sides);
        demo.block.render_mesh();
        if demo.draw_sides[0] {
            demo.sides.top = Some(());
        } else {
            demo.sides.top = None;
        }
        if demo.draw_sides[1] {
            demo.sides.bottom = Some(());
        } else {
            demo.sides.bottom = None;
        }
        if demo.draw_sides[2] {
            demo.sides.front = Some(());
        } else {
            demo.sides.front = None;
        }
        if demo.draw_sides[3] {
            demo.sides.back = Some(());
        } else {
            demo.sides.back = None;
        }
        if demo.draw_sides[4] {
            demo.sides.left = Some(());
        } else {
            demo.sides.left = None;
        }
        if demo.draw_sides[5] {
            demo.sides.right = Some(());
        } else {
            demo.sides.right = None;
        }

        root_ui().group(
            hash!(),
            vec2(160.0, 150.0),
            |ui| {
                ui.checkbox(hash!(), "Render top", &mut demo.draw_sides[0]);
                ui.checkbox(hash!(), "Render bottom", &mut demo.draw_sides[1]);
                ui.checkbox(hash!(), "Render front", &mut demo.draw_sides[2]);
                ui.checkbox(hash!(), "Render back", &mut demo.draw_sides[3]);
                ui.checkbox(hash!(), "Render left", &mut demo.draw_sides[4]);
                ui.checkbox(hash!(), "Render right", &mut demo.draw_sides[5]);
            },
        );
    }
}
