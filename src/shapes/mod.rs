use bevy::prelude::{Mesh, Vec3};
use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
pub(crate) use elipse::*;
pub(crate) use flipper::*;
pub(crate) use table::*;

mod elipse;
mod flipper;
mod table;

const X_NEGATIF: [f32; 3] = [-1., 0., 0.];
const X_POSITIF: [f32; 3] = [1., 0., 0.];

const Y_NEGATIF: [f32; 3] = [0., -1., 0.];
const Y_POSITIF: [f32; 3] = [0., 1., 0.];

const Z_NEGATIF: [f32; 3] = [0., 0., -1.];
const Z_POSITIF: [f32; 3] = [0., 0., 1.];

#[derive(Clone, Debug)]
struct MeshElements {
    pub(crate) vertices: Vec<[f32; 3]>,
    pub(crate) normals: Vec<[f32; 3]>,
    pub(crate) indices: Vec<u32>,
}

impl std::ops::AddAssign<MeshElements> for MeshElements {
    fn add_assign(&mut self, rhs: MeshElements) {
        let vertices = self.vertices.len() as u32;

        self.vertices.try_reserve(rhs.vertices.len()).unwrap();
        self.vertices.extend(rhs.vertices);

        self.normals.try_reserve(rhs.normals.len()).unwrap();
        self.normals.extend(rhs.normals);

        self.indices.try_reserve(rhs.indices.len()).unwrap();
        for i in rhs.indices {
            self.indices.push(i + vertices);
        }
    }
}

impl std::ops::AddAssign<Vec3> for MeshElements {
    fn add_assign(&mut self, rhs: Vec3) {
        for i in 0..self.vertices.len() {
            let x = self.vertices[i][0];
            let y = self.vertices[i][1];
            let z = self.vertices[i][2];
            let _ = std::mem::replace(&mut self.vertices[i][0], x + rhs.x);
            let _ = std::mem::replace(&mut self.vertices[i][1], y + rhs.y);
            let _ = std::mem::replace(&mut self.vertices[i][2], z + rhs.z);
        }
    }
}

impl From<MeshElements> for Mesh {
    fn from(value: MeshElements) -> Self {
        let mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_indices(Indices::U32(value.indices))
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, value.vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, value.normals);

        mesh
    }
}
