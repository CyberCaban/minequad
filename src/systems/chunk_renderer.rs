use macroquad::{
    models::{draw_mesh, Mesh},
    texture::Texture2D,
    ui::Vertex,
};

use super::chunk;
use macroquad::prelude::*;

const VERTEX_SIZE: usize = 6;
const CHUNK_SIZE_W: i32 = 16;
const CHUNK_SIZE_H: i32 = 16;
const CHUNK_SIZE_D: i32 = 16;
const CHUNK_VOLUME: i32 = CHUNK_SIZE_W * CHUNK_SIZE_H * CHUNK_SIZE_D;


macro_rules! get_block {
    ($chunk: ident, $x: expr, $y: expr, $z: expr) => {
        $chunk.blocks[$y as usize][$z as usize][$x as usize]
    };
}
macro_rules! in_chunk {
    ($x: expr, $y: expr, $z: expr) => {
        $x >= 0 && $x < CHUNK_SIZE_W && $y >= 0 && $y < CHUNK_SIZE_H && $z >= 0 && $z < CHUNK_SIZE_D
    };
}
macro_rules! is_blocked {
    ($chunk: ident, $x : expr, $y: expr, $z: expr) => {
        !in_chunk!($x, $y, $z) || get_block!($chunk, $x, $y, $z).id as i32 != 0
    };
}
macro_rules! vertex {
    ($buf: expr, $idx: expr, $x: expr, $y: expr, $z: expr, $u: expr, $v: expr, $l: expr) => {
        $buf.push(Vertex { position: vec3($x, $y, $z), uv: vec2($u, -$v), color: [($l * 255.0) as u8, ($l * 255.0) as u8, ($l * 255.0) as u8, 255], normal: vec4(0.0, 0.0, 0.0, 0.0) });

        $idx += VERTEX_SIZE;
    };
}
pub struct Renderer {
    buffer: Vec<Vertex>,
}

impl Renderer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity * VERTEX_SIZE * 6),
        }
    }
    pub fn render(&mut self, chunk: &chunk::Chunk, atlas: &Texture2D) -> Mesh {
        let mut buf = &mut self.buffer;
        let mut idx = 0;
        for y in 0..CHUNK_SIZE_H {
            for z in 0..CHUNK_SIZE_D {
                for x in 0..CHUNK_SIZE_W {
                    let block = chunk.blocks[y as usize][z as usize][x as usize];
                    let id = block.id as i32;

                    if id == 0 {
                        continue;
                    }

                    let mut l: f32;
                    let uvsize = 1.0 / 16.0;
                    let u = (id % 16) as f32 * uvsize;
                    let v: f32 = 1.0-  ((1.0 + (id as f32 / 16.0)) * uvsize);

                    if !is_blocked!(chunk, x, y + 1, z) {
                        l = 1.0;
                        vertex!(
                            buf,
                            idx,
                            x as f32 - 0.5,
                            y as f32 + 0.5,
                            z as f32 - 0.5,
                            u + uvsize,
                            v,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 - 0.5,
                            y as f32 + 0.5,
                            z as f32 + 0.5,
                            u + uvsize,
                            v + uvsize,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 + 0.5,
                            y as f32 + 0.5,
                            z as f32 + 0.5,
                            u,
                            v + uvsize,
                            l
                        );

                        vertex!(
                            buf,
                            idx,
                            x as f32 - 0.5,
                            y as f32 + 0.5,
                            z as f32 - 0.5,
                            u + uvsize,
                            v,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 + 0.5,
                            y as f32 + 0.5,
                            z as f32 + 0.5,
                            u,
                            v + uvsize,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 + 0.5,
                            y as f32 + 0.5,
                            z as f32 - 0.5,
                            u,
                            v,
                            l
                        );
                    }
                    if !is_blocked!(chunk, x, y - 1, z) {
                        l = 0.75;
                        vertex!(
                            buf,
                            idx,
                            x as f32 - 0.5,
                            y as f32 - 0.5,
                            z as f32 - 0.5,
                            u,
                            v,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 + 0.5,
                            y as f32 - 0.5,
                            z as f32 + 0.5,
                            u + uvsize,
                            v + uvsize,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 - 0.5,
                            y as f32 - 0.5,
                            z as f32 + 0.5,
                            u,
                            v + uvsize,
                            l
                        );

                        vertex!(
                            buf,
                            idx,
                            x as f32 - 0.5,
                            y as f32 - 0.5,
                            z as f32 - 0.5,
                            u,
                            v,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 + 0.5,
                            y as f32 - 0.5,
                            z as f32 - 0.5,
                            u + uvsize,
                            v,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 + 0.5,
                            y as f32 - 0.5,
                            z as f32 + 0.5,
                            u + uvsize,
                            v + uvsize,
                            l
                        );
                    }
                    if !is_blocked!(chunk, x + 1, y, z) {
                        l = 0.5;
                        vertex!(
                            buf,
                            idx,
                            x as f32 + 0.5,
                            y as f32 - 0.5,
                            z as f32 - 0.5,
                            u + uvsize,
                            v,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 + 0.5,
                            y as f32 + 0.5,
                            z as f32 - 0.5,
                            u + uvsize,
                            v + uvsize,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 + 0.5,
                            y as f32 + 0.5,
                            z as f32 + 0.5,
                            u,
                            v + uvsize,
                            l
                        );

                        vertex!(
                            buf,
                            idx,
                            x as f32 + 0.5,
                            y as f32 - 0.5,
                            z as f32 - 0.5,
                            u + uvsize,
                            v,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 + 0.5,
                            y as f32 + 0.5,
                            z as f32 + 0.5,
                            u,
                            v + uvsize,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 + 0.5,
                            y as f32 - 0.5,
                            z as f32 + 0.5,
                            u,
                            v,
                            l
                        );
                    }
                    if !is_blocked!(chunk, x - 1, y, z) {
                        l = 0.25;
                        vertex!(
                            buf,
                            idx,
                            x as f32 - 0.5,
                            y as f32 - 0.5,
                            z as f32 - 0.5,
                            u,
                            v,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 - 0.5,
                            y as f32 + 0.5,
                            z as f32 + 0.5,
                            u + uvsize,
                            v + uvsize,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 - 0.5,
                            y as f32 + 0.5,
                            z as f32 - 0.5,
                            u,
                            v + uvsize,
                            l
                        );

                        vertex!(
                            buf,
                            idx,
                            x as f32 - 0.5,
                            y as f32 - 0.5,
                            z as f32 - 0.5,
                            u,
                            v,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 - 0.5,
                            y as f32 - 0.5,
                            z as f32 + 0.5,
                            u + uvsize,
                            v,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 - 0.5,
                            y as f32 + 0.5,
                            z as f32 + 0.5,
                            u + uvsize,
                            v + uvsize,
                            l
                        );
                    }
                    if !is_blocked!(chunk, x, y, z + 1) {
                        l = 0.0;
                        vertex!(
                            buf,
                            idx,
                            x as f32 - 0.5,
                            y as f32 - 0.5,
                            z as f32 + 0.5,
                            u,
                            v,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 + 0.5,
                            y as f32 + 0.5,
                            z as f32 + 0.5,
                            u + uvsize,
                            v + uvsize,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 - 0.5,
                            y as f32 + 0.5,
                            z as f32 + 0.5,
                            u,
                            v + uvsize,
                            l
                        );

                        vertex!(
                            buf,
                            idx,
                            x as f32 - 0.5,
                            y as f32 - 0.5,
                            z as f32 + 0.5,
                            u,
                            v,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 + 0.5,
                            y as f32 - 0.5,
                            z as f32 - 0.5,
                            u + uvsize,
                            v,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 + 0.5,
                            y as f32 + 0.5,
                            z as f32 + 0.5,
                            u + uvsize,
                            v + uvsize,
                            l
                        );
                    }
                    if !is_blocked!(chunk, x, y, z - 1) {
                        l = 0.75;
                        vertex!(
                            buf,
                            idx,
                            x as f32 - 0.5,
                            y as f32 - 0.5,
                            z as f32 - 0.5,
                            u + uvsize,
                            v,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 + 0.5,
                            y as f32 + 0.5,
                            z as f32 - 0.5,
                            u + uvsize,
                            v + uvsize,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 - 0.5,
                            y as f32 + 0.5,
                            z as f32 - 0.5,
                            u,
                            v + uvsize,
                            l
                        );

                        vertex!(
                            buf,
                            idx,
                            x as f32 - 0.5,
                            y as f32 - 0.5,
                            z as f32 - 0.5,
                            u + uvsize,
                            v,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 + 0.5,
                            y as f32 + 0.5,
                            z as f32 - 0.5,
                            u,
                            v + uvsize,
                            l
                        );
                        vertex!(
                            buf,
                            idx,
                            x as f32 + 0.5,
                            y as f32 - 0.5,
                            z as f32 - 0.5,
                            u,
                            v,
                            l
                        );
                    }
                }
            }
        }
        let mut indices: Vec<u16> = Vec::with_capacity(idx + 1);
        for i in 0..idx {
            indices.push(i as u16);
        }
        println!("idx: {}", idx);
        let mesh = Mesh {
            vertices: buf.to_vec(),
            indices,
            texture: Some(atlas.clone()),
        };

        mesh
    }
}
