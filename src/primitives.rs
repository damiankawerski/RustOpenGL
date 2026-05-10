use crate::geometry::Geometry;

pub enum Attributes {
    Position = 0,
    Color = 1,
}

pub fn new_axes_geometry() -> Geometry {
    let mut geometry = Geometry::new();
    geometry.set_primitive_mode(gl::LINES);

    let verts = [
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(1.0, 0.0, 0.0),
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(0.0, 1.0, 0.0),
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(0.0, 0.0, 1.0),
    ];

    let colors = [
        glm::vec3(1.0, 0.0, 0.0),
        glm::vec3(1.0, 0.0, 0.0),
        glm::vec3(0.0, 1.0, 0.0),
        glm::vec3(0.0, 1.0, 0.0),
        glm::vec3(0.0, 0.0, 1.0),
        glm::vec3(0.0, 0.0, 1.0),
    ];

    geometry.set_vertices(Attributes::Position as u32, &verts);
    geometry.set_attribute(Attributes::Color as u32, &colors);

    geometry
}

pub fn new_plane_geometry(size: glm::Vec2, color: glm::Vec3) -> Geometry {
    let mut geometry = Geometry::new();
    geometry.set_primitive_mode(gl::TRIANGLES);

    let verts = [
        glm::vec3(-size.x / 2.0, -size.y / 2.0, 0.0),
        glm::vec3(size.x / 2.0, -size.y / 2.0, 0.0),
        glm::vec3(size.x / 2.0, size.y / 2.0, 0.0),
        glm::vec3(-size.x / 2.0, size.y / 2.0, 0.0),
    ];

    let colors = [color, color, color, color];
    let indices: [u32; 6] = [0, 1, 2, 2, 3, 0];

    geometry.set_vertices(Attributes::Position as u32, &verts);
    geometry.set_attribute(Attributes::Color as u32, &colors);
    geometry.set_indices(&indices);

    geometry
}

pub fn new_box_geometry(size: glm::Vec3, color: glm::Vec3) -> Geometry {
    let mut geometry = Geometry::new();
    geometry.set_primitive_mode(gl::TRIANGLES);

    let verts = [
        // Front face
        glm::vec3(-size.x / 2.0, -size.y / 2.0, size.z / 2.0),
        glm::vec3(size.x / 2.0, -size.y / 2.0, size.z / 2.0),
        glm::vec3(size.x / 2.0, size.y / 2.0, size.z / 2.0),
        glm::vec3(-size.x / 2.0, size.y / 2.0, size.z / 2.0),
        // Back face
        glm::vec3(-size.x / 2.0, -size.y / 2.0, -size.z / 2.0),
        glm::vec3(size.x / 2.0, -size.y / 2.0, -size.z / 2.0),
        glm::vec3(size.x / 2.0, size.y / 2.0, -size.z / 2.0),
        glm::vec3(-size.x / 2.0, size.y / 2.0, -size.z / 2.0),
    ];

    let colors = [color; 8];
    let indices: [u32; 36] = [
        // Front face
        0, 1, 2, 2, 3, 0,
        // Back face
        4, 5, 6, 6, 7, 4,
        // Left face
        4, 7, 3, 3, 0, 4,
        // Right face
        1, 5, 6, 6, 2, 1,
        // Top face
        3, 7, 6, 6, 2, 3,
        // Bottom face
        4, 5, 1, 1, 0, 4,
    ];

    geometry.set_vertices(Attributes::Position as u32, &verts);
    geometry.set_attribute(Attributes::Color as u32, &colors);
    geometry.set_indices(&indices);

    geometry
}

pub fn new_circle_geometry(radius: f32, n_segments: i32, color: glm::Vec3) -> Geometry {
    let mut geometry = Geometry::new();
    geometry.set_primitive_mode(gl::TRIANGLE_FAN);

    let mut verts = Vec::new();
    let mut colors = Vec::new();

    verts.push(glm::vec3(0.0, 0.0, 0.0));
    colors.push(color);

    for i in 0..=n_segments {
        let angle = (i as f32) / (n_segments as f32) * 2.0 * std::f32::consts::PI;
        verts.push(glm::vec3(radius * angle.cos(), radius * angle.sin(), 0.0));
        colors.push(color);
    }

    geometry.set_vertices(Attributes::Position as u32, &verts);
    geometry.set_attribute(Attributes::Color as u32, &colors);

    geometry
}

pub fn new_sphere_geometry(radius: f32, n_segments: i32, color: glm::Vec3) -> Geometry {
    let mut geometry = Geometry::new();
    geometry.set_primitive_mode(gl::TRIANGLES);

    let mut verts = Vec::new();
    let mut colors = Vec::new();

    for i in 0..=n_segments {
        let lat = (i as f32) / (n_segments as f32) * std::f32::consts::PI;
        for j in 0..=n_segments {
            let lon = (j as f32) / (n_segments as f32) * 2.0 * std::f32::consts::PI;
            verts.push(glm::vec3(
                radius * lat.sin() * lon.cos(),
                radius * lat.sin() * lon.sin(),
                radius * lat.cos(),
            ));
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

    geometry.set_vertices(Attributes::Position as u32, &verts);
    geometry.set_attribute(Attributes::Color as u32, &colors);
    geometry.set_indices(&indices);

    geometry
}