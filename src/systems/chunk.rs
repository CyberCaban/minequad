use macroquad::{
    color::Color,
    math::{vec2, vec3, vec4},
    models::{draw_mesh, Mesh, Vertex},
    rand,
    texture::Texture2D,
};

pub const CHUNK_W: usize = 16;
pub const CHUNK_H: usize = 16;
pub const CHUNK_D: usize = 16;
pub const CHUCK_VOLUME: usize = CHUNK_W * CHUNK_H * CHUNK_D;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum BlockType {
    Air,
    Stone,
    Grass,
    Bricks,
}
#[derive(Clone, Debug, Copy)]
pub struct Block {
    id: BlockType,
}

pub struct Chunk {
    pub blocks: [[[Block; CHUNK_W]; CHUNK_H]; CHUNK_D],
    pub position: (f32, f32, f32),
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: [[[Block { id: BlockType::Air }; CHUNK_W]; CHUNK_H]; CHUNK_D],
            position: (0.0, 0.0, 0.0),
        }
    }
    pub fn populate(&mut self, position: (f32, f32, f32)) {
        self.position = position;
        let mut blocks = [[[Block { id: BlockType::Air }; CHUNK_W]; CHUNK_H]; CHUNK_D];

        for y in 0..CHUNK_H {
            for z in 0..CHUNK_D {
                for x in 0..CHUNK_W {
                    let block_pos = (x as f32, y as f32, z as f32);
                    // let mut id = if block_pos.0.sin() + block_pos.1.sin() + block_pos.2.sin() > 0.0 {
                    //     BlockType::Stone
                    // } else {
                    //     BlockType::Air
                    // };
                    // let mut id = if (block_pos.0 + block_pos.2).sin() > 0.0
                    //     && (block_pos.1 + block_pos.2.cos()).sin() > 0.0
                    // {
                    //     if block_pos.0.sin() <= 0.0 {
                    //         if block_pos.1.cos().sin() > 0.0 {
                    //             BlockType::Bricks
                    //         } else {
                    //             BlockType::Grass
                    //         }
                    //     } else {
                    //         BlockType::Stone
                    //     }
                    // } else {
                    //     BlockType::Air
                    // };
                    let id = if block_pos.1 >= 4.0 {
                        BlockType::Air
                    } else {
                        match rand::gen_range(0, 8) {
                            0 => BlockType::Stone,
                            1 => BlockType::Grass,
                            2 => BlockType::Bricks,
                            _ => BlockType::Air,
                        }
                    };

                    // if block_pos.0 == 7.0 && block_pos.1 == 14.0 && block_pos.2 == 7.0 {
                    //     id = BlockType::Stone;
                    // }
                    // if block_pos.1 <= 2.0 {
                    //     id = BlockType::Stone
                    // } else {
                    //     id = BlockType::Air
                    // };

                    blocks[y][z][x] = Block { id };
                }
            }
        }

        self.blocks = blocks;
    }
}

macro_rules! in_chunk {
    ($x: expr, $y: expr, $z: expr) => {
        $x >= 0
            && $x < CHUNK_W as i32
            && $y >= 0
            && $y < CHUNK_H as i32
            && $z >= 0
            && $z < CHUNK_D as i32
    };
}

macro_rules! get_block {
    ($chunk: expr, $x: expr, $y: expr, $z: expr) => {
        $chunk.blocks[$y as usize][$z as usize][$x as usize]
    };
}

macro_rules! is_blocking {
    ($chunk: expr, $x: expr, $y: expr, $z: expr) => {
        in_chunk!($x, $y, $z) && get_block!($chunk, $x, $y, $z).id != BlockType::Air
    };
}

fn vert(x: f32, y: f32, z: f32, u: f32, v: f32, l: f32) -> Vertex {
    Vertex {
        position: vec3(x + 0.5, y - 0.5, z + 0.5),
        uv: vec2(u, v),
        color: Color::new(l, l, l, 1.0).into(),
        normal: vec4(0.0, 1.0, 0.0, 1.0),
    }
}
fn index(indices: &mut Vec<u16>, idx: &mut u16) {
    indices.push(*idx);
    indices.push(*idx + 1);
    indices.push(*idx + 2);
    indices.push(*idx + 2);
    indices.push(*idx + 3);
    indices.push(*idx);
    *idx += 4;
}

pub struct ChunkRenderer {
    pub mesh: Mesh,
}

impl ChunkRenderer {
    pub fn new() -> Self {
        Self {
            mesh: Mesh {
                vertices: vec![],
                indices: vec![],
                texture: None,
            },
        }
    }
    pub fn gen_mesh(&mut self, chunk: &Chunk, texture: &Texture2D) {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut idx: u16 = 0;
        let mut indices: Vec<u16> = Vec::new();

        for y in 0..CHUNK_H {
            for z in 0..CHUNK_D {
                for x in 0..CHUNK_W {
                    let (x, y, z) = (x as i32, y as i32, z as i32);
                    let block = get_block!(chunk, x, y, z);
                    let block_pos = (
                        x as f32 + chunk.position.0,
                        y as f32 + chunk.position.1,
                        z as f32 + chunk.position.2,
                    );
                    let id = block.id as u32 as f32;

                    match block.id {
                        BlockType::Air => {
                            continue;
                        }
                        _ => {
                            let mut l;
                            let atlas_size = 8.0;
                            let size = 1.0 / atlas_size;
                            let u = (id % atlas_size) * size;
                            let v = (1.0 - (id / atlas_size + 1.0)) * size + size / 8.0 * id;
                            if !is_blocking!(chunk, x, y + 1, z) {
                                l = 1.0;
                                let (x, y, z) = block_pos;
                                vertices.push(vert(x - 0.5, y + 0.5, z + 0.5, u, v,l));
                                vertices.push(vert(x + 0.5, y + 0.5, z + 0.5, u + size, v, l));
                                vertices.push(vert(x + 0.5, y + 0.5, z - 0.5, u + size, v + size, l));
                                vertices.push(vert(x - 0.5, y + 0.5, z - 0.5, u, v + size, l));

                                index(&mut indices, &mut idx);
                            }

                            if !is_blocking!(chunk, x, y - 1, z) {
                                l = 0.75;
                                let (x, y, z) = block_pos;
                                vertices.push(vert(x - 0.5, y - 0.5, z + 0.5, u, v, l));
                                vertices.push(vert(x + 0.5, y - 0.5, z + 0.5, u + size, v, l));
                                vertices.push(vert(x + 0.5, y - 0.5, z - 0.5, u + size, v + size, l));
                                vertices.push(vert(x - 0.5, y - 0.5, z - 0.5, u, v + size, l));

                                index(&mut indices, &mut idx);
                            }

                            if !is_blocking!(chunk, x, y, z + 1) {
                                l = 0.9;
                                let (x, y, z) = block_pos;
                                vertices.push(vert(x - 0.5, y - 0.5, z + 0.5, u, v + size,l ));
                                vertices.push(vert(x + 0.5, y - 0.5, z + 0.5, u + size, v + size,l ));
                                vertices.push(vert(x + 0.5, y + 0.5, z + 0.5, u + size, v,l ));
                                vertices.push(vert(x - 0.5, y + 0.5, z + 0.5, u, v,l ));

                                index(&mut indices, &mut idx);
                            }

                            if !is_blocking!(chunk, x, y, z - 1) {
                                l = 0.8;
                                let (x, y, z) = block_pos;
                                vertices.push(vert(x - 0.5, y - 0.5, z - 0.5, u, v + size, l ));
                                vertices.push(vert(x + 0.5, y - 0.5, z - 0.5, u + size, v + size, l ));
                                vertices.push(vert(x + 0.5, y + 0.5, z - 0.5, u + size, v, l ));
                                vertices.push(vert(x - 0.5, y + 0.5, z - 0.5, u, v, l ));

                                index(&mut indices, &mut idx);
                            }

                            if !is_blocking!(chunk, x + 1, y, z) {
                                l = 0.95;
                                let (x, y, z) = block_pos;
                                vertices.push(vert(x + 0.5, y - 0.5, z + 0.5, u + size, v + size, l ));
                                vertices.push(vert(x + 0.5, y - 0.5, z - 0.5, u, v + size, l ));
                                vertices.push(vert(x + 0.5, y + 0.5, z - 0.5, u, v, l ));
                                vertices.push(vert(x + 0.5, y + 0.5, z + 0.5, u + size, v, l ));

                                index(&mut indices, &mut idx);
                            }

                            if !is_blocking!(chunk, x - 1, y, z) {
                                l = 0.85;
                                let (x, y, z) = block_pos;
                                vertices.push(vert(x - 0.5, y - 0.5, z - 0.5, u, v + size,l));
                                vertices.push(vert(x - 0.5, y - 0.5, z + 0.5, u + size, v + size,l));
                                vertices.push(vert(x - 0.5, y + 0.5, z + 0.5, u + size, v,l));
                                vertices.push(vert(x - 0.5, y + 0.5, z - 0.5, u, v,l));

                                index(&mut indices, &mut idx);
                            }
                        }
                    }
                }
            }
        }
        self.mesh = Mesh {
            vertices,
            indices,
            texture: Some(texture.clone()),
        };
    }

    pub fn render_mesh(&mut self) {
        draw_mesh(&self.mesh);
    }
}
