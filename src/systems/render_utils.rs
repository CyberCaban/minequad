use macroquad::{color::WHITE, math::{vec2, vec3, Vec2, Vec3}, models::Vertex, texture::Texture2D};

use crate::systems::blocks::BlockMesh;

fn vert(pos: Vec3, uv: Vec2) -> Vertex {
    Vertex {
        position: pos,
        uv,
        color: WHITE.into(),
    }
}
pub fn mesh_top(mesh: &mut BlockMesh, pos: Vec3, texture: Texture2D) {
    mesh.vertices
        .push(vert(pos + vec3(0.0, 1.0, 0.0), vec2(1.0, 1.0)));
    mesh.vertices
        .push(vert(pos + vec3(1.0, 1.0, 0.0), vec2(0.0, 1.0)));
    mesh.vertices
        .push(vert(pos + vec3(1.0, 1.0, 1.0), vec2(0.0, 0.0)));
    mesh.vertices
        .push(vert(pos + vec3(0.0, 1.0, 0.0), vec2(1.0, 1.0)));
    mesh.vertices
        .push(vert(pos + vec3(0.0, 1.0, 1.0), vec2(1.0, 0.0)));
    mesh.vertices
        .push(vert(pos + vec3(1.0, 1.0, 1.0), vec2(0.0, 0.0)));

    for _ in 0..6 {
        mesh.indices.push(mesh.idx_counter as u16);
        mesh.idx_counter += 1;
    }

    mesh.texture = Some(texture);
}

pub fn mesh_bottom(mesh: &mut BlockMesh, pos: Vec3, texture: Texture2D) {
    mesh.vertices
        .push(vert(pos + vec3(0.0, 0.0, 0.0), vec2(1.0, 1.0)));
    mesh.vertices
        .push(vert(pos + vec3(1.0, 0.0, 0.0), vec2(0.0, 1.0)));
    mesh.vertices
        .push(vert(pos + vec3(1.0, 0.0, 1.0), vec2(0.0, 0.0)));
    mesh.vertices
        .push(vert(pos + vec3(0.0, 0.0, 0.0), vec2(1.0, 1.0)));
    mesh.vertices
        .push(vert(pos + vec3(0.0, 0.0, 1.0), vec2(1.0, 0.0)));
    mesh.vertices
        .push(vert(pos + vec3(1.0, 0.0, 1.0), vec2(0.0, 0.0)));

    for _ in 0..6 {
        mesh.indices.push(mesh.idx_counter as u16);
        mesh.idx_counter += 1;
    }

    mesh.texture = Some(texture);
}

pub fn mesh_front(mesh: &mut BlockMesh, pos: Vec3, texture: Texture2D) {
    mesh.vertices
        .push(vert(pos + vec3(0.0, 0.0, 0.0), vec2(0.0, 0.0)));
    mesh.vertices
        .push(vert(pos + vec3(1.0, 0.0, 0.0), vec2(1.0, 0.0)));
    mesh.vertices
        .push(vert(pos + vec3(1.0, 1.0, 0.0), vec2(1.0, 1.0)));
    mesh.vertices
        .push(vert(pos + vec3(0.0, 0.0, 0.0), vec2(0.0, 0.0)));
    mesh.vertices
        .push(vert(pos + vec3(0.0, 1.0, 0.0), vec2(0.0, 1.0)));
    mesh.vertices
        .push(vert(pos + vec3(1.0, 1.0, 0.0), vec2(1.0, 1.0)));

    for _ in 0..6 {
        mesh.indices.push(mesh.idx_counter as u16);
        mesh.idx_counter += 1;
    }

    mesh.texture = Some(texture);
}

pub fn mesh_back(mesh: &mut BlockMesh, pos: Vec3, texture: Texture2D) {
    mesh.vertices
        .push(vert(pos + vec3(0.0, 0.0, 1.0), vec2(0.0, 0.0)));
    mesh.vertices
        .push(vert(pos + vec3(1.0, 0.0, 1.0), vec2(1.0, 0.0)));
    mesh.vertices   
        .push(vert(pos + vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)));
    mesh.vertices
        .push(vert(pos + vec3(0.0, 0.0, 1.0), vec2(0.0, 0.0)));
    mesh.vertices
        .push(vert(pos + vec3(0.0, 1.0, 1.0), vec2(0.0, 1.0)));
    mesh.vertices
        .push(vert(pos + vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)));

    for _ in 0..6 {
        mesh.indices.push(mesh.idx_counter as u16);
        mesh.idx_counter += 1;
    }

    mesh.texture = Some(texture);
} 

pub fn mesh_left(mesh: &mut BlockMesh, pos: Vec3, texture: Texture2D) {
    mesh.vertices
        .push(vert(pos + vec3(1.0, 0.0, 0.0), vec2(0.0, 0.0)));
    mesh.vertices
        .push(vert(pos + vec3(1.0, 0.0, 1.0), vec2(0.0, 1.0)));
    mesh.vertices
        .push(vert(pos + vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)));
    mesh.vertices
        .push(vert(pos + vec3(1.0, 0.0, 0.0), vec2(0.0, 0.0)));
    mesh.vertices
        .push(vert(pos + vec3(1.0, 1.0, 0.0), vec2(1.0, 0.0)));
    mesh.vertices
        .push(vert(pos + vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0)));

    for _ in 0..6 {
        mesh.indices.push(mesh.idx_counter as u16);
        mesh.idx_counter += 1;
    }

    mesh.texture = Some(texture);
}

pub fn mesh_right(mesh: &mut BlockMesh, pos: Vec3, texture: Texture2D) {
    mesh.vertices
        .push(vert(pos + vec3(0.0, 0.0, 1.0), vec2(0.0, 1.0)));
    mesh.vertices
        .push(vert(pos + vec3(0.0, 0.0, 0.0), vec2(0.0, 0.0)));
    mesh.vertices
        .push(vert(pos + vec3(0.0, 1.0, 0.0), vec2(1.0, 0.0)));
    mesh.vertices
        .push(vert(pos + vec3(0.0, 0.0, 1.0), vec2(0.0, 1.0)));
    mesh.vertices
        .push(vert(pos + vec3(0.0, 1.0, 1.0), vec2(1.0, 1.0)));
    mesh.vertices
        .push(vert(pos + vec3(0.0, 1.0, 0.0), vec2(1.0, 0.0)));

    for _ in 0..6 {
        mesh.indices.push(mesh.idx_counter as u16);
        mesh.idx_counter += 1;
    }

    mesh.texture = Some(texture);
}