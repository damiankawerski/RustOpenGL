use crate::geometry::Geometry;
use glam::{Vec2, Vec3};

pub fn compute_normals(verts: &[Vec3], indices: &[u32]) -> Vec<Vec3> {
    let mut normals = vec![Vec3::ZERO; verts.len()];

    for tri in indices.chunks(3) {
        let (i0, i1, i2) = (tri[0] as usize, tri[1] as usize, tri[2] as usize);
        let v10 = verts[i1] - verts[i0];
        let v21 = verts[i2] - verts[i1];
        let n = v10.cross(v21).normalize_or_zero();
        normals[i0] += n;
        normals[i1] += n;
        normals[i2] += n;
    }

    normals.iter().map(|n| n.normalize_or_zero()).collect()
}

pub enum Attributes {
    Position = 0,
    Color = 1,
    Normal = 2,
    UV = 7,
}

pub fn new_axes_geometry() -> Geometry {
    let mut geometry = Geometry::new();
    geometry.set_primitive_mode(gl::LINES);

    let verts = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    ];

    let colors = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, 1.0),
    ];

    geometry.set_vertices(Attributes::Position as u32, &verts);
    geometry.set_attribute(Attributes::Color as u32, &colors);

    geometry
}

pub fn new_plane_geometry(size: Vec2, color: Vec3) -> Geometry {
    let mut geometry = Geometry::new();
    geometry.set_primitive_mode(gl::TRIANGLES);

    let verts = [
        Vec3::new(-size.x / 2.0, -size.y / 2.0, 0.0),
        Vec3::new(size.x / 2.0, -size.y / 2.0, 0.0),
        Vec3::new(size.x / 2.0, size.y / 2.0, 0.0),
        Vec3::new(-size.x / 2.0, size.y / 2.0, 0.0),
    ];

    let uvs = [
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
    ];

    let colors = [color; 4];
    let normals = [Vec3::Z; 4];
    let indices: [u32; 6] = [0, 1, 2, 2, 3, 0];

    geometry.set_vertices(Attributes::Position as u32, &verts);
    geometry.set_attribute(Attributes::Color as u32, &colors);
    geometry.set_attribute(Attributes::Normal as u32, &normals);
    geometry.set_attribute_uv(Attributes::UV as u32, &uvs);
    geometry.set_indices(&indices);

    geometry
}

pub fn new_box_geometry(size: Vec3, color: Vec3) -> Geometry {
    let mut geometry = Geometry::new();
    geometry.set_primitive_mode(gl::TRIANGLES);

    let verts = [
        // Front face
        Vec3::new(-size.x / 2.0, -size.y / 2.0, size.z / 2.0),
        Vec3::new(size.x / 2.0, -size.y / 2.0, size.z / 2.0),
        Vec3::new(size.x / 2.0, size.y / 2.0, size.z / 2.0),
        Vec3::new(-size.x / 2.0, size.y / 2.0, size.z / 2.0),
        // Back face
        Vec3::new(-size.x / 2.0, -size.y / 2.0, -size.z / 2.0),
        Vec3::new(size.x / 2.0, -size.y / 2.0, -size.z / 2.0),
        Vec3::new(size.x / 2.0, size.y / 2.0, -size.z / 2.0),
        Vec3::new(-size.x / 2.0, size.y / 2.0, -size.z / 2.0),
    ];

    let colors = [color; 8];
    let indices: [u32; 36] = [
        // Front face
        0, 1, 2, 2, 3, 0, // Back face
        4, 5, 6, 6, 7, 4, // Left face
        4, 7, 3, 3, 0, 4, // Right face
        1, 5, 6, 6, 2, 1, // Top face
        3, 7, 6, 6, 2, 3, // Bottom face
        4, 5, 1, 1, 0, 4,
    ];

    geometry.set_vertices(Attributes::Position as u32, &verts);
    geometry.set_attribute(Attributes::Color as u32, &colors);
    geometry.set_attribute(Attributes::Normal as u32, &compute_normals(&verts, &indices));
    geometry.set_indices(&indices);

    geometry
}

pub fn new_circle_geometry(radius: f32, n_segments: i32, color: Vec3) -> Geometry {
    let mut geometry = Geometry::new();
    geometry.set_primitive_mode(gl::TRIANGLES);

    let mut verts = Vec::new();
    let mut colors = Vec::new();

    verts.push(Vec3::new(0.0, 0.0, 0.0));
    colors.push(color);

    for i in 0..=n_segments {
        let angle = (i as f32) / (n_segments as f32) * 2.0 * std::f32::consts::PI;
        verts.push(Vec3::new(radius * angle.cos(), radius * angle.sin(), 0.0));
        colors.push(color);
    }

    let mut indices = Vec::new();
    for i in 1..n_segments {
        indices.push(0u32);
        indices.push(i as u32);
        indices.push((i + 1) as u32);
    }

    let normals = compute_normals(&verts, &indices);

    geometry.set_vertices(Attributes::Position as u32, &verts);
    geometry.set_attribute(Attributes::Color as u32, &colors);
    geometry.set_attribute(Attributes::Normal as u32, &normals);
    geometry.set_indices(&indices);

    geometry
}

pub fn new_sphere_geometry(radius: f32, n_segments: i32, color: Vec3) -> Geometry {
    let mut geometry = Geometry::new();
    geometry.set_primitive_mode(gl::TRIANGLES);

    let mut verts = Vec::new();
    let mut colors = Vec::new();

    for i in 0..=n_segments {
        let lat = (i as f32) / (n_segments as f32) * std::f32::consts::PI;
        for j in 0..=n_segments {
            let lon = (j as f32) / (n_segments as f32) * 2.0 * std::f32::consts::PI;
            let n = Vec3::new(lat.sin() * lon.cos(), lat.sin() * lon.sin(), lat.cos());
            verts.push(n * radius);
            colors.push(color);
        }
    }

    let mut indices = Vec::new();
    for i in 0..n_segments {
        for j in 0..n_segments {
            let first = i * (n_segments + 1) + j;
            let second = first + n_segments + 1;
            indices.push(first as u32);
            indices.push(second as u32);
            indices.push((first + 1) as u32);
            indices.push(second as u32);
            indices.push((second + 1) as u32);
            indices.push((first + 1) as u32);
        }
    }

    let normals = compute_normals(&verts, &indices);

    geometry.set_vertices(Attributes::Position as u32, &verts);
    geometry.set_attribute(Attributes::Color as u32, &colors);
    geometry.set_attribute(Attributes::Normal as u32, &normals);
    geometry.set_indices(&indices);

    geometry
}

pub fn new_cylinder_geometry(radius: f32, height: f32, n_segments: i32, color: Vec3) -> Geometry {
    let mut geometry = Geometry::new();
    geometry.set_primitive_mode(gl::TRIANGLES);

    let mut verts = Vec::new();
    let mut colors = Vec::new();

    for i in 0..=n_segments {
        let angle = (i as f32) / (n_segments as f32) * 2.0 * -std::f32::consts::PI;
        verts.push(Vec3::new(radius * angle.cos(), radius * angle.sin(), 0.0));
        colors.push(color);
        verts.push(Vec3::new(
            radius * angle.cos(),
            radius * angle.sin(),
            height,
        ));
        colors.push(color);
    }

    let mut indices = Vec::new();
    for i in 0..n_segments {
        let first = i * 2;
        let second = first + 1;
        let third = ((i + 1) % (n_segments + 1)) * 2;
        let fourth = third + 1;
        indices.push(first as u32);
        indices.push(second as u32);
        indices.push(third as u32);
        indices.push(second as u32);
        indices.push(fourth as u32);
        indices.push(third as u32);
    }

    let normals = compute_normals(&verts, &indices);

    geometry.set_vertices(Attributes::Position as u32, &verts);
    geometry.set_attribute(Attributes::Color as u32, &colors);
    geometry.set_attribute(Attributes::Normal as u32, &normals);
    geometry.set_indices(&indices);

    geometry
}

pub fn new_cone_geometry(radius: f32, height: f32, n_segments: i32, color: Vec3) -> Geometry {
    let mut geometry = Geometry::new();
    geometry.set_primitive_mode(gl::TRIANGLES);

    let mut verts = Vec::new();
    let mut colors = Vec::new();

    verts.push(Vec3::new(0.0, 0.0, height));
    colors.push(color);

    for i in 0..=n_segments {
        let angle = (i as f32) / (n_segments as f32) * 2.0 * std::f32::consts::PI;
        verts.push(Vec3::new(radius * angle.cos(), radius * angle.sin(), 0.0));
        colors.push(color);
    }

    let mut indices = Vec::new();
    for i in 1..=n_segments {
        indices.push(0);
        indices.push(i as u32);
        indices.push(((i % n_segments) + 1) as u32);
    }

    let normals = compute_normals(&verts, &indices);

    geometry.set_vertices(Attributes::Position as u32, &verts);
    geometry.set_attribute(Attributes::Color as u32, &colors);
    geometry.set_attribute(Attributes::Normal as u32, &normals);
    geometry.set_indices(&indices);

    geometry
}
