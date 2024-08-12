use std::ops::BitAnd;

use crate::{systems::blocks::*, CHUNK_SIZE_16};
use macroquad::prelude::*;

pub struct Chunk {
    blocks: Vec<Vec<Vec<Option<Block>>>>,
    position: (f32, f32, f32),
}

fn proper_mod(a: i32, b: i32) -> i32 {
    (a % b + b) % b
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

    pub fn connected_blocks(&mut self) {
        for y in 0..CHUNK_SIZE_16 {
            for x in 0..CHUNK_SIZE_16 {
                for z in 0..CHUNK_SIZE_16 {
                    let mut sides = RenderSides::default();

                    if self.blocks[y as usize][x as usize][z as usize].is_some() {
                        let top = y + 1 >= CHUNK_SIZE_16
                            || self.blocks[y as usize + 1][x as usize][z as usize].is_none();
                        if top {
                            sides.top = Some(());
                        }
                        let bottom =
                            y <= 0 || self.blocks[y as usize - 1][x as usize][z as usize].is_none();
                        if bottom {
                            sides.bottom = Some(());
                        }
                        let back = z + 1 >= CHUNK_SIZE_16
                            || self.blocks[y as usize][x as usize][z as usize + 1].is_none();
                        if back {
                            sides.back = Some(());
                        }
                        let front =
                            z <= 0 || self.blocks[y as usize][x as usize][z as usize - 1].is_none();
                        if front {
                            sides.front = Some(());
                        }
                        let left = x + 1 >= CHUNK_SIZE_16
                            || self.blocks[y as usize][x as usize + 1][z as usize].is_none();
                        if left {
                            sides.left = Some(());
                        }
                        let right =
                            x <= 0 || self.blocks[y as usize][x as usize - 1][z as usize].is_none();
                        if right {
                            sides.right = Some(());
                        }
                    }

                    if let Some(block) = &mut self.blocks[y as usize][x as usize][z as usize] {
                        block.make_mesh(&sides);
                    }
                }
            }
        }
    }

    pub fn populate(&mut self, texture: &Texture2D) {
        for y in 0..CHUNK_SIZE_16 {
            let mut layer = Vec::new();
            for x in 0..CHUNK_SIZE_16 {
                let mut row = Vec::new();
                for z in 0..CHUNK_SIZE_16 {
                    let block_pos = vec3(x as f32, y as f32, z as f32);
                    let block = if 1 == 1 {
                        Some(Block::new(BlockType::Stone, block_pos, &texture))
                    } else {
                        None
                    };
                    row.push(block);
                }
                layer.push(row);
            }
            self.blocks.push(layer);
        }
    }
    pub fn from_fn(&mut self, texture: &Texture2D, f: impl Fn(i32, i32, i32) -> bool) {
        for y in 0..CHUNK_SIZE_16 {
            let mut layer = Vec::new();
            for x in 0..CHUNK_SIZE_16 {
                let mut row = Vec::new();
                for z in 0..CHUNK_SIZE_16 {
                    let block_pos = vec3(x as f32, y as f32, z as f32);
                    let block = if f(x, y, z) {
                        Some(Block::new(BlockType::Stone, block_pos, &texture))
                    } else {
                        None
                    };
                    row.push(block);
                }
                layer.push(row);
            }
            self.blocks.push(layer);
        }
    }
    pub fn render(&mut self) {
        for x in 0..CHUNK_SIZE_16 {
            for y in 0..CHUNK_SIZE_16 {
                for z in 0..CHUNK_SIZE_16 {
                    if let Some(block) = &mut self.blocks[x as usize][y as usize][z as usize] {
                        block.render_mesh();
                    }
                }
            }
        }
    }
}
