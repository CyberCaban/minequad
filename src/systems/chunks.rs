use crate::{systems::blocks::*, CHUNK_SIZE_16};
use macroquad::prelude::*;

struct Chunk {
    blocks: Vec<Vec<Vec<Option<Block>>>>,
    position: (f32, f32, f32),
}

impl Chunk {
    pub fn new(position: (f32, f32, f32)) -> Self {
        // debug_assert!(position.0.floor() == position.0.ceil());
        // debug_assert!(position.1.floor() == position.1.ceil());
        // debug_assert!(position.2.floor() == position.2.ceil());

        Self {
            blocks: Vec::new(),
            position,
        }
    }

    pub fn render(&mut self) {
        for y in 0..CHUNK_SIZE_16 {
            let mut layer = Vec::new();
            for x in 0..CHUNK_SIZE_16 {
                let mut row = Vec::new();
                for z in 0..CHUNK_SIZE_16 {
                    let block_pos = vec3(x as f32, y as f32, z as f32);
                    let block = Block::new(BlockType::Stone, block_pos, &Texture2D::empty());
                    row.push(Some(block));
                }
                layer.push(row);
            }
            self.blocks.push(layer);
        }
    }
}