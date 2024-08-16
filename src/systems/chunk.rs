use macroquad::{
    color::Color,
    math::{vec2, vec3},
    models::{draw_mesh, Mesh, Vertex},
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
                    let mut id = if (block_pos.0 + block_pos.2).sin() > 0.0
                        && (block_pos.1 + block_pos.2.cos()).sin() > 0.0
                    {
                        BlockType::Stone
                    } else {
                        BlockType::Air
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

fn vert(x: f32, y: f32, z: f32, u: f32, v: f32) -> Vertex {
    Vertex {
        position: vec3(x + 0.5, y - 0.5, z + 0.5),
        uv: vec2(u, v),
        color: Color::new(1.0, 1.0, 1.0, 1.0),
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

                    match block.id {
                        BlockType::Air => {
                            continue;
                        }
                        _ => {
                            if !is_blocking!(chunk, x, y + 1, z) {
                                let (x, y, z) = block_pos;
                                vertices.push(vert(x - 0.5, y + 0.5, z + 0.5, 0.0, 0.0));
                                vertices.push(vert(x + 0.5, y + 0.5, z + 0.5, 1.0, 0.0));
                                vertices.push(vert(x + 0.5, y + 0.5, z - 0.5, 1.0, 1.0));
                                vertices.push(vert(x - 0.5, y + 0.5, z - 0.5, 0.0, 1.0));

                                index(&mut indices, &mut idx);
                            }

                            if !is_blocking!(chunk, x, y - 1, z) {
                                let (x, y, z) = block_pos;
                                vertices.push(vert(x - 0.5, y - 0.5, z + 0.5, 0.0, 0.0));
                                vertices.push(vert(x + 0.5, y - 0.5, z + 0.5, 1.0, 0.0));
                                vertices.push(vert(x + 0.5, y - 0.5, z - 0.5, 1.0, 1.0));
                                vertices.push(vert(x - 0.5, y - 0.5, z - 0.5, 0.0, 1.0));

                                index(&mut indices, &mut idx);
                            }

                            if !is_blocking!(chunk, x, y, z + 1) {
                                let (x, y, z) = block_pos;
                                vertices.push(vert(x - 0.5, y - 0.5, z + 0.5, 0.0, 1.0));
                                vertices.push(vert(x + 0.5, y - 0.5, z + 0.5, 1.0, 1.0));
                                vertices.push(vert(x + 0.5, y + 0.5, z + 0.5, 1.0, 0.0));
                                vertices.push(vert(x - 0.5, y + 0.5, z + 0.5, 0.0, 0.0));

                                index(&mut indices, &mut idx);
                            }

                            if !is_blocking!(chunk, x, y, z - 1) {
                                let (x, y, z) = block_pos;
                                vertices.push(vert(x - 0.5, y - 0.5, z - 0.5, 0.0, 1.0));
                                vertices.push(vert(x + 0.5, y - 0.5, z - 0.5, 1.0, 1.0));
                                vertices.push(vert(x + 0.5, y + 0.5, z - 0.5, 1.0, 0.0));
                                vertices.push(vert(x - 0.5, y + 0.5, z - 0.5, 0.0, 0.0));

                                index(&mut indices, &mut idx);
                            }

                            if !is_blocking!(chunk, x + 1, y, z) {
                                let (x, y, z) = block_pos;
                                vertices.push(vert(x + 0.5, y - 0.5, z + 0.5, 1.0, 1.0));
                                vertices.push(vert(x + 0.5, y - 0.5, z - 0.5, 0.0, 1.0));
                                vertices.push(vert(x + 0.5, y + 0.5, z - 0.5, 0.0, 0.0));
                                vertices.push(vert(x + 0.5, y + 0.5, z + 0.5, 1.0, 0.0));

                                index(&mut indices, &mut idx);
                            }

                            if !is_blocking!(chunk, x - 1, y, z) {
                                let (x, y, z) = block_pos;
                                vertices.push(vert(x - 0.5, y - 0.5, z - 0.5, 0.0, 1.0));
                                vertices.push(vert(x - 0.5, y - 0.5, z + 0.5, 1.0, 1.0));
                                vertices.push(vert(x - 0.5, y + 0.5, z + 0.5, 1.0, 0.0));
                                vertices.push(vert(x - 0.5, y + 0.5, z - 0.5, 0.0, 0.0));

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
