//! This module is the mesh for the table. It's a simple box with no top.

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;

use super::{X_NEGATIF, X_POSITIF, Y_POSITIF, Z_NEGATIF, Z_POSITIF};

pub(crate) struct Table {
    height: f32,
    width: f32,
    thickness: f32,
}

impl Table {
    pub(crate) fn new(height: f32, width: f32, wall_height: f32) -> Self {
        Self {
            height,
            width,
            thickness: wall_height,
        }
    }
}

impl Default for Table {
    fn default() -> Self {
        Self {
            height: 5.,
            width: 3.,
            thickness: 1.,
        }
    }
}

impl From<Table> for Mesh {
    fn from(value: Table) -> Self {
        // Center on the middle of the table in all axys
        let x_offset = -value.width / 2.;
        let y_offset = -value.thickness / 2.;
        let z_offset = -value.height / 2.;

        // Vertices are ordered carefully so that we can create
        // triangle indices with a for loop.
        #[rustfmt::skip]
        let vertices = vec![
            // Floor
            [x_offset,               y_offset ,                    z_offset               ],
            [x_offset,               y_offset ,                    z_offset + value.height],
            [x_offset + value.width, y_offset ,                    z_offset + value.height],
            [x_offset + value.width, y_offset ,                    z_offset               ],
            // Inside left wall
            [x_offset,               y_offset ,                    z_offset               ],
            [x_offset,               y_offset + value.thickness, z_offset               ],
            [x_offset,               y_offset + value.thickness, z_offset + value.height],
            [x_offset,               y_offset ,                    z_offset + value.height],
            // Outside left wall
            [x_offset,               y_offset ,                    z_offset               ],
            [x_offset,               y_offset ,                    z_offset + value.height],
            [x_offset,               y_offset + value.thickness, z_offset + value.height],
            [x_offset,               y_offset + value.thickness, z_offset               ],
            // Inside front wall
            [x_offset,               y_offset ,                    z_offset + value.height],
            [x_offset,               y_offset + value.thickness, z_offset + value.height],
            [x_offset + value.width, y_offset + value.thickness, z_offset + value.height],
            [x_offset + value.width, y_offset ,                    z_offset + value.height],
            // Outside front wall
            [x_offset,               y_offset ,                    z_offset + value.height],
            [x_offset + value.width, y_offset ,                    z_offset + value.height],
            [x_offset + value.width, y_offset + value.thickness, z_offset + value.height],
            [x_offset,               y_offset + value.thickness, z_offset + value.height],
            // Inside right wall
            [x_offset + value.width, y_offset ,                    z_offset + value.height],
            [x_offset + value.width, y_offset + value.thickness, z_offset + value.height],
            [x_offset + value.width, y_offset + value.thickness, z_offset               ],
            [x_offset + value.width, y_offset ,                    z_offset               ],
            // Ouside right wall
            [x_offset + value.width, y_offset ,                    z_offset + value.height],
            [x_offset + value.width, y_offset ,                    z_offset               ],
            [x_offset + value.width, y_offset + value.thickness, z_offset               ],
            [x_offset + value.width, y_offset + value.thickness, z_offset + value.height],
            // Inside back wall
            [x_offset,               y_offset ,                    z_offset               ],
            [x_offset + value.width, y_offset ,                    z_offset               ],
            [x_offset + value.width, y_offset + value.thickness, z_offset               ],
            [x_offset,               y_offset + value.thickness, z_offset               ],
            // Outside back wall
            [x_offset,               y_offset ,                    z_offset               ],
            [x_offset,               y_offset + value.thickness, z_offset               ],
            [x_offset + value.width, y_offset + value.thickness, z_offset               ],
            [x_offset + value.width, y_offset ,                    z_offset               ],
        ];

        let normals = vec![
            Y_POSITIF, Y_POSITIF, Y_POSITIF, Y_POSITIF, // Floor normales
            X_POSITIF, X_POSITIF, X_POSITIF, X_POSITIF, // Inside left wall normales
            X_NEGATIF, X_NEGATIF, X_NEGATIF, X_NEGATIF, // Outside left wall normales
            Z_NEGATIF, Z_NEGATIF, Z_NEGATIF, Z_NEGATIF, // Inside front wall normales
            Z_POSITIF, Z_POSITIF, Z_POSITIF, Z_POSITIF, // Outside front wall normales
            X_NEGATIF, X_NEGATIF, X_NEGATIF, X_NEGATIF, // Inside right wall normales
            X_POSITIF, X_POSITIF, X_POSITIF, X_POSITIF, // Outside right wall normales
            Z_POSITIF, Z_POSITIF, Z_POSITIF, Z_POSITIF, // Inside back wall normales
            Z_NEGATIF, Z_NEGATIF, Z_NEGATIF, Z_NEGATIF, // Outside back wall normales
        ];

        // Add triangle vertices indices counter-clockwise (vertices add been laid carefullly
        // so that we can create triangle in a simple loop).
        // See https://github.com/bevyengine/bevy/blob/main/examples/3d/generate_custom_mesh.rs
        let mut indices: Vec<u32> = Vec::with_capacity(18);
        for i in (0..vertices.len() as u32).step_by(4) {
            // Triangle one for side
            indices.push(i);
            indices.push(i + 1);
            indices.push(i + 2);
            // Triangle two for side
            indices.push(i + 2);
            indices.push(i + 3);
            indices.push(i);
        }

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_indices(Indices::U32(indices))
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    }
}
