use crate::geometry::Geometry;
use crate::primitives::Attributes;
use crate::textures_2d::Texture2d;
use glam::{Vec2, Vec3};
use std::path::Path;

pub struct ObjPart {
    pub geometry: Geometry,
    pub diffuse: Option<Texture2d>,
}

pub fn load_obj(obj_path: &str) -> Vec<ObjPart> {
    let load_options = tobj::LoadOptions {
        triangulate: true,
        single_index: true,
        ..Default::default()
    };

    let (models, materials_result) = match tobj::load_obj(obj_path, &load_options) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to load {}: {}", obj_path, e);
            return vec![];
        }
    };

    let materials = materials_result.unwrap_or_default();
    let obj_dir = Path::new(obj_path).parent().unwrap_or(Path::new("."));

    let (mut min, mut max) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
    for model in &models {
        for chunk in model.mesh.positions.chunks(3) {
            let p = Vec3::new(chunk[0], chunk[1], chunk[2]);
            min = min.min(p);
            max = max.max(p);
        }
    }
    let center = (min + max) * 0.5;
    let extent = (max - min).max_element();
    let scale = if extent > 0.0 { 2.0 / extent } else { 1.0 };

    models
        .into_iter()
        .map(|model| {
            let mesh = model.mesh;

            let positions: Vec<Vec3> = mesh
                .positions
                .chunks(3)
                .map(|c| (Vec3::new(c[0], c[1], c[2]) - center) * scale)
                .collect();

            let normals: Vec<Vec3> = if mesh.normals.is_empty() {
                let mut computed = vec![Vec3::ZERO; positions.len()];
                for tri in mesh.indices.chunks(3) {
                    let (a, b, c) = (
                        positions[tri[0] as usize],
                        positions[tri[1] as usize],
                        positions[tri[2] as usize],
                    );
                    let n = (b - a).cross(c - a).normalize_or_zero();
                    computed[tri[0] as usize] += n;
                    computed[tri[1] as usize] += n;
                    computed[tri[2] as usize] += n;
                }
                computed.iter().map(|n| n.normalize_or_zero()).collect()
            } else {
                mesh.normals
                    .chunks(3)
                    .map(|c| Vec3::new(c[0], c[1], c[2]))
                    .collect()
            };

            let uvs: Vec<Vec2> = if mesh.texcoords.is_empty() {
                vec![Vec2::ZERO; positions.len()]
            } else {
                mesh.texcoords
                    .chunks(2)
                    .map(|c| Vec2::new(c[0], 1.0 - c[1]))
                    .collect()
            };

            let mut geometry = Geometry::new();
            geometry.set_primitive_mode(gl::TRIANGLES);
            geometry.set_vertices(Attributes::Position as u32, &positions);
            geometry.set_attribute(Attributes::Normal as u32, &normals);
            geometry.set_attribute_uv(Attributes::UV as u32, &uvs);
            geometry.set_indices(&mesh.indices);

            let diffuse = mesh
                .material_id
                .and_then(|id| materials.get(id))
                .and_then(|mat| mat.diffuse_texture.as_ref())
                .and_then(|tex_file| {
                    let tex_path = obj_dir.join(tex_file);
                    let tex = Texture2d::new();
                    if tex.load_from_file(tex_path.to_str()?) {
                        Some(tex)
                    } else {
                        None
                    }
                });

            ObjPart { geometry, diffuse }
        })
        .collect()
}
