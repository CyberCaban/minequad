use core::fmt;

use macroquad::prelude::*;

use super::render_utils::{mesh_back, mesh_bottom, mesh_front, mesh_left, mesh_right, mesh_top};

pub enum BlockSides {
    Top,
    Bottom,
    Front,
    Back,
    Left,
    Right,
}

pub struct RenderSides {
    pub top: Option<()>,
    pub bottom: Option<()>,
    pub front: Option<()>,
    pub back: Option<()>,
    pub left: Option<()>,
    pub right: Option<()>,
}
impl Default for RenderSides {
    fn default() -> Self {
        RenderSides {
            top: None,
            bottom: None,
            front: None,
            back: None,
            left: None,
            right: None,
        }
    }
}
impl RenderSides {
    pub fn all() -> Self {
        RenderSides {
            top: Some(()),
            bottom: Some(()),
            front: Some(()),
            back: Some(()),
            left: Some(()),
            right: Some(()),
        }
    }
}

pub struct BlockMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub texture: Option<Texture2D>,
    pub idx_counter: u32,
}
impl BlockMesh {
    pub fn new() -> Self {
        BlockMesh {
            vertices: vec![],
            indices: vec![],
            texture: None,
            idx_counter: 0,
        }
    }
}
impl Clone for BlockMesh {
    fn clone(&self) -> Self {
        BlockMesh {
            vertices: self.vertices.clone(),
            indices: self.indices.clone(),
            texture: self.texture.clone(),
            idx_counter: self.idx_counter,
        }
    }
}
impl fmt::Debug for BlockMesh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BlockMesh {{ \nvertices: {:?}, \nindices: {:?}, \ntexture: {:?}, \nidx_counter: {:?} }}", self.vertices, self.indices, self.texture, self.idx_counter)
    }
}

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
    mesh: BlockMesh,
}

impl Block {
    pub fn new(block_type: BlockType, position: Vec3, texture: &Texture2D) -> Self {
        // assert that position is integer
        // debug_assert!(
        //     position.x.floor() == position.x.ceil()
        //         && position.y.floor() == position.y.ceil()
        //         && position.z.floor() == position.z.ceil()
        // );
        let texture = BlockTexture {
            top: texture.clone(),
            bottom: texture.clone(),
            front: texture.clone(),
            back: texture.clone(),
            left: texture.clone(),
            right: texture.clone(),
        };

        Self {
            block_type,
            texture,
            position,
            mesh: BlockMesh::new(),
        }
    }

    pub fn make_mesh(&mut self, sides: &RenderSides) {
        self.mesh.vertices = vec![];
        self.mesh.indices = vec![];
        self.mesh.idx_counter = 0;

        if sides.top.is_some() {
            mesh_top(&mut self.mesh, self.position, self.texture.top.clone());
        }
        if sides.bottom.is_some() {
            mesh_bottom(&mut self.mesh, self.position, self.texture.bottom.clone());
        }
        if sides.front.is_some() {
            mesh_front(&mut self.mesh, self.position, self.texture.front.clone());
        }
        if sides.back.is_some() {
            mesh_back(&mut self.mesh, self.position, self.texture.back.clone());
        }
        if sides.left.is_some() {
            mesh_left(&mut self.mesh, self.position, self.texture.left.clone());
        }
        if sides.right.is_some() {
            mesh_right(&mut self.mesh, self.position, self.texture.right.clone());
        }
    }
    // self.mesh = BlockMesh::new();

    pub fn render_mesh(&mut self) {
        // println!("Mesh: {:?}", self.mesh);

        draw_mesh(&Mesh {
            vertices: self.mesh.vertices.clone(),
            indices: self.mesh.indices.clone(),
            texture: self.mesh.texture.clone(),
        });
    }

    // pub fn render_full_block(&self) {
    //     let mesh = Mesh {
    //         vertices: vec![
    //             // bottom
    //             vert(self.position + vec3(0.0, 0.0, 0.0), vec2(1.0, 1.0)),
    //             vert(self.position + vec3(1.0, 0.0, 0.0), vec2(0.0, 1.0)),
    //             vert(self.position + vec3(1.0, 0.0, 1.0), vec2(0.0, 0.0)),
    //             vert(self.position + vec3(0.0, 0.0, 0.0), vec2(1.0, 1.0)),
    //             vert(self.position + vec3(0.0, 0.0, 1.0), vec2(1.0, 0.0)),
    //             vert(self.position + vec3(1.0, 0.0, 1.0), vec2(0.0, 0.0)),
    //             // front
    //             vert(self.position + vec3(0.0, 0.0, 0.0), vec2(1.0, 1.0)),
    //             vert(self.position + vec3(1.0, 0.0, 0.0), vec2(0.0, 1.0)),
    //             vert(self.position + vec3(1.0, 1.0, 0.0), vec2(0.0, 0.0)),
    //             vert(self.position + vec3(0.0, 0.0, 0.0), vec2(1.0, 1.0)),
    //             vert(self.position + vec3(0.0, 1.0, 0.0), vec2(1.0, 0.0)),
    //             vert(self.position + vec3(1.0, 1.0, 0.0), vec2(0.0, 0.0)),
    //             // right
    //             vert(self.position + vec3(0.0, 0.0, 1.0), vec2(0.0, 1.0)),
    //             vert(self.position + vec3(0.0, 0.0, 0.0), vec2(0.0, 0.0)),
    //             vert(self.position + vec3(0.0, 1.0, 0.0), vec2(1.0, 0.0)),
    //             vert(self.position + vec3(0.0, 0.0, 1.0), vec2(0.0, 1.0)),
    //             vert(self.position + vec3(0.0, 1.0, 1.0), vec2(1.0, 1.0)),
    //             vert(self.position + vec3(0.0, 1.0, 0.0), vec2(1.0, 0.0)),
    //             // top
    //             vert(self.position + vec3(0.0, 1.0, 0.0), vec2(0.0, 0.0)),
    //             vert(self.position + vec3(1.0, 1.0, 0.0), vec2(0.0, 1.0)),
    //             vert(self.position + vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)),
    //             vert(self.position + vec3(0.0, 1.0, 0.0), vec2(0.0, 0.0)),
    //             vert(self.position + vec3(0.0, 1.0, 1.0), vec2(1.0, 0.0)),
    //             vert(self.position + vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)),
    //             // back
    //             vert(self.position + vec3(0.0, 0.0, 1.0), vec2(0.0, 1.0)),
    //             vert(self.position + vec3(1.0, 0.0, 1.0), vec2(0.0, 0.0)),
    //             vert(self.position + vec3(1.0, 1.0, 1.0), vec2(1.0, 0.0)),
    //             vert(self.position + vec3(0.0, 0.0, 1.0), vec2(0.0, 1.0)),
    //             vert(self.position + vec3(0.0, 1.0, 1.0), vec2(1.0, 1.0)),
    //             vert(self.position + vec3(1.0, 1.0, 1.0), vec2(1.0, 0.0)),
    //             // left
    //             vert(self.position + vec3(1.0, 0.0, 0.0), vec2(0.0, 0.0)),
    //             vert(self.position + vec3(1.0, 0.0, 1.0), vec2(0.0, 1.0)),
    //             vert(self.position + vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)),
    //             vert(self.position + vec3(1.0, 0.0, 0.0), vec2(0.0, 0.0)),
    //             vert(self.position + vec3(1.0, 1.0, 0.0), vec2(1.0, 0.0)),
    //             vert(self.position + vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)),
    //         ],
    //         indices: vec![
    //             0, 1, 2, 3, 4, 5, // bottom
    //             6, 7, 8, 9, 10, 11, // top
    //             12, 13, 14, 15, 16, 17, // front
    //             18, 19, 20, 21, 22, 23, // back
    //             24, 25, 26, 27, 28, 29, // right
    //             30, 31, 32, 33, 34, 35, // left
    //         ],
    //         texture: Some(self.texture.top.clone()),
    //     };
    //     draw_mesh(&mesh);
    // }
}
