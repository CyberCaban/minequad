const CHUNK_SIZE_W: i32 = 16;
const CHUNK_SIZE_H: i32 = 16;
const CHUNK_SIZE_D: i32 = 16;

const CHUNK_VOLUME: i32 = CHUNK_SIZE_W * CHUNK_SIZE_H * CHUNK_SIZE_D;

const VERTEX_SIZE: usize = 6;

#[derive(Clone, Copy)]
pub enum BlockId {
    Air,
    Stone,
    Grass,
}

#[derive(Clone, Copy)]
pub struct Block {
    pub id: BlockId,
}

pub struct Chunk {
    pub blocks: [[[Block; CHUNK_SIZE_W as usize]; CHUNK_SIZE_H as usize]; CHUNK_SIZE_D as usize],
    pub position: (i32, i32, i32),
}

impl Chunk {
    pub fn new(position: (i32, i32, i32)) -> Self {
        Self {
            blocks: [[[Block { id: BlockId::Air }; CHUNK_SIZE_W as usize]; CHUNK_SIZE_H as usize];
                CHUNK_SIZE_D as usize],
            position,
        }
    }
    pub fn populate() -> Self {
        let mut arr = [[[Block { id: BlockId::Air }; CHUNK_SIZE_W as usize]; CHUNK_SIZE_H as usize];
            CHUNK_SIZE_D as usize];
        for y in 0..CHUNK_SIZE_H {
            let mut layer =
                [[Block { id: BlockId::Air }; CHUNK_SIZE_W as usize]; CHUNK_SIZE_H as usize];
            for z in 0..CHUNK_SIZE_D {
                let mut row = [Block { id: BlockId::Air }; CHUNK_SIZE_W as usize];
                for x in 0..CHUNK_SIZE_W {
                    let not_air = y <= (((x as f32 * 0.3).sin() * 0.5 + 0.5) * 10.0) as usize;
                    let id = if not_air {
                        BlockId::Stone
                    } else {
                        BlockId::Air
                    };
                    // arr[((y * CHUNK_SIZE_D + z) * CHUNK_SIZE_W + (x)) as usize] = Block { id };
                    row[x as usize] = Block { id };
                }
                layer[z as usize] = row;
            }
            arr[y as usize] = layer;
        }

        Self {
            blocks: arr,
            position: (0, 0, 0),
        }
    }
}
