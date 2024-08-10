use macroquad::{color::WHITE, math::{vec2, vec3, Vec2, Vec3}, models::{draw_mesh, Mesh, Vertex}, texture::Texture2D};

#[derive(Clone, Debug)]
struct BlockTexture {
    top: Texture2D,
    bottom: Texture2D,
    front: Texture2D,
    back: Texture2D,
    left: Texture2D,
    right: Texture2D,
}

#[derive(Clone, Debug)]
pub enum BlockType {
    Stone,
    Grass,
}

#[derive(Clone, Debug)]
pub struct Block {
    block_type: BlockType,
    texture: BlockTexture,
    pub position: Vec3,
}

fn vert(pos: Vec3, uv: Vec2) -> Vertex {
    Vertex {
        position: pos,
        uv,
        color: WHITE,
    }
}

impl Block {
    pub fn new(block_type: BlockType, position: Vec3, texture: Texture2D) -> Self {
        let texture = match block_type {
            BlockType::Stone => BlockTexture {
                top: texture.clone(),
                bottom: texture.clone(),
                front: texture.clone(),
                back: texture.clone(),
                left: texture.clone(),
                right: texture.clone(),
            },
            BlockType::Grass => BlockTexture {
                top: texture.clone(),
                bottom: texture.clone(),
                front: texture.clone(),
                back: texture.clone(),
                left: texture.clone(),
                right: texture.clone(),
            },
        };

        Self {
            block_type,
            texture,
            position,
        }
    }

    pub fn make_mesh(&self) {
        let mesh = Mesh {
            vertices: vec![
                // bottom
                vert(self.position + vec3(0.0, 0.0, 0.0), vec2(1.0, 1.0)),
                vert(self.position + vec3(1.0, 0.0, 0.0), vec2(0.0, 1.0)),
                vert(self.position + vec3(1.0, 0.0, 1.0), vec2(0.0, 0.0)),
                vert(self.position + vec3(0.0, 0.0, 0.0), vec2(1.0, 1.0)),
                vert(self.position + vec3(0.0, 0.0, 1.0), vec2(1.0, 0.0)),
                vert(self.position + vec3(1.0, 0.0, 1.0), vec2(0.0, 0.0)),
                // front
                vert(self.position + vec3(0.0, 0.0, 0.0), vec2(1.0, 1.0)),
                vert(self.position + vec3(1.0, 0.0, 0.0), vec2(0.0, 1.0)),
                vert(self.position + vec3(1.0, 1.0, 0.0), vec2(0.0, 0.0)),
                vert(self.position + vec3(0.0, 0.0, 0.0), vec2(1.0, 1.0)),
                vert(self.position + vec3(0.0, 1.0, 0.0), vec2(1.0, 0.0)),
                vert(self.position + vec3(1.0, 1.0, 0.0), vec2(0.0, 0.0)),
                // right
                vert(self.position + vec3(0.0, 0.0, 1.0), vec2(0.0, 1.0)),
                vert(self.position + vec3(0.0, 0.0, 0.0), vec2(0.0, 0.0)),
                vert(self.position + vec3(0.0, 1.0, 0.0), vec2(1.0, 0.0)),
                vert(self.position + vec3(0.0, 0.0, 1.0), vec2(0.0, 1.0)),
                vert(self.position + vec3(0.0, 1.0, 1.0), vec2(1.0, 1.0)),
                vert(self.position + vec3(0.0, 1.0, 0.0), vec2(1.0, 0.0)),
                // top
                vert(self.position + vec3(0.0, 1.0, 0.0), vec2(0.0, 0.0)),
                vert(self.position + vec3(1.0, 1.0, 0.0), vec2(0.0, 1.0)),
                vert(self.position + vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)),
                vert(self.position + vec3(0.0, 1.0, 0.0), vec2(0.0, 0.0)),
                vert(self.position + vec3(0.0, 1.0, 1.0), vec2(1.0, 0.0)),
                vert(self.position + vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)),
                // back
                vert(self.position + vec3(0.0, 0.0, 1.0), vec2(0.0, 1.0)),
                vert(self.position + vec3(1.0, 0.0, 1.0), vec2(0.0, 0.0)),
                vert(self.position + vec3(1.0, 1.0, 1.0), vec2(1.0, 0.0)),
                vert(self.position + vec3(0.0, 0.0, 1.0), vec2(0.0, 1.0)),
                vert(self.position + vec3(0.0, 1.0, 1.0), vec2(1.0, 1.0)),
                vert(self.position + vec3(1.0, 1.0, 1.0), vec2(1.0, 0.0)),
                // left
                vert(self.position + vec3(1.0, 0.0, 0.0), vec2(0.0, 0.0)),
                vert(self.position + vec3(1.0, 0.0, 1.0), vec2(0.0, 1.0)),
                vert(self.position + vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)),
                vert(self.position + vec3(1.0, 0.0, 0.0), vec2(0.0, 0.0)),
                vert(self.position + vec3(1.0, 1.0, 0.0), vec2(1.0, 0.0)),
                vert(self.position + vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)),
            ],
            indices: vec![
                0, 1, 2, 3, 4, 5, // bottom
                6, 7, 8, 9, 10, 11, // top
                12, 13, 14, 15, 16, 17, // front
                18, 19, 20, 21, 22, 23, // back
                24, 25, 26, 27, 28, 29, // right
                30, 31, 32, 33, 34, 35, // left
            ],
            texture: Some(self.texture.top.clone()),
        };
        draw_mesh(&mesh);
    }
}
