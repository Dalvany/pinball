use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::shapes::{MeshElements, Y_NEGATIF, Y_POSITIF};

pub(crate) struct Flipper {
    x: f32,
    radius_min: f32,
    radius_max: f32,
    thickness: f32,
    resolution: usize,
}

impl Flipper {
    pub(crate) fn new(x: f32, min_z: f32, max_z: f32, thickness: f32, resolution: usize) -> Self {
        Self {
            x,
            radius_min: min_z,
            radius_max: max_z,
            thickness,
            resolution,
        }
    }

    fn border(&self) -> MeshElements {
        let x_distance_centers = self.x - (self.radius_min + self.radius_max);
        let tangente = (self.radius_max - self.radius_min) / (2. * x_distance_centers);
        let alpha = tangente.atan() * 2.;

        let mut vertices = Vec::with_capacity(self.resolution * 2 + 2);
        let mut normals = Vec::with_capacity(self.resolution * 2 + 2);
        let mut indices = Vec::with_capacity(self.resolution * 3 * 2);

        let angle_incremente = (PI + alpha) / self.resolution as f32;
        let start_angle = PI / 2.;
        for i in 0..=self.resolution {
            for height in [0., self.thickness] {
                let angle1 = start_angle + angle_incremente * i as f32;
                let x = self.radius_max * angle1.cos();
                let z = self.radius_max * angle1.sin();
                vertices.push([x, height, z]);
                let normal = Vec3::new(x, 0., z).normalize();
                normals.push([normal.x, normal.y, normal.z]);
            }
        }

        let angle_incremente = (PI - alpha) / self.resolution as f32;
        let start_angle = -PI / 2. + alpha;
        for i in 0..=self.resolution {
            for height in [0., self.thickness] {
                let angle1 = start_angle + angle_incremente * i as f32;
                let x = self.radius_min * angle1.cos();
                let z = self.radius_min * angle1.sin();
                vertices.push([
                    x + x_distance_centers,
                    height,
                    z + (self.radius_max - self.radius_min) / 2.,
                ]);
                let normal = Vec3::new(x, 0., z).normalize();
                normals.push([normal.x, normal.y, normal.z]);
            }
        }

        for i in (0..vertices.len() as u32 - 2).step_by(2) {
            // Triangle 1
            indices.push(i);
            indices.push(i + 1);
            indices.push(i + 3);
            // Triangle 2
            indices.push(i + 3);
            indices.push(i + 2);
            indices.push(i);
        }

        indices.push(vertices.len() as u32 - 2);
        indices.push(vertices.len() as u32 - 1);
        indices.push(1);
        indices.push(1);
        indices.push(0);
        indices.push(vertices.len() as u32 - 2);

        MeshElements {
            vertices,
            normals,
            indices,
        }
    }

    fn left_arc(&self, height: f32) -> MeshElements {
        let distance_centers = self.x - (self.radius_min + self.radius_max);
        let tangente = (self.radius_max - self.radius_min) / (2. * distance_centers);
        let alpha = tangente.atan() * 2.;

        let mut vertices = Vec::with_capacity(self.resolution + 1);
        let mut normals = Vec::with_capacity(self.resolution + 1);
        let mut indices = Vec::with_capacity(self.resolution * 3);

        let normal = if height == 0. { Y_NEGATIF } else { Y_POSITIF };

        vertices.push([0., height, 0.]);
        normals.push(normal);

        let angle_incremente = (PI + alpha) / self.resolution as f32;
        let start_angle = PI / 2.;
        for i in 0..=self.resolution {
            let angle1 = start_angle + angle_incremente * i as f32;
            vertices.push([
                self.radius_max * angle1.cos(),
                height,
                self.radius_max * angle1.sin(),
            ]);
            normals.push(normal);
        }

        let (offset1, offset2) = if height == 0. { (0, 1) } else { (1, 0) };

        for i in 1..(vertices.len() as u32 - 1) {
            indices.push(0);
            indices.push(i + offset1);
            indices.push(i + offset2);
        }

        MeshElements {
            vertices,
            normals,
            indices,
        }
    }

    fn right_arc(&self, height: f32) -> MeshElements {
        let distance_centers = self.x - (self.radius_min + self.radius_max);
        let tangente = (self.radius_max - self.radius_min) / (2. * distance_centers);
        let alpha = tangente.atan() * 2.;

        let mut vertices = Vec::with_capacity(self.resolution + 1);
        let mut normals = Vec::with_capacity(self.resolution + 1);
        let mut indices = Vec::with_capacity(self.resolution * 3);

        let normal = if height == 0. { Y_NEGATIF } else { Y_POSITIF };

        vertices.push([0., height, 0.]);
        normals.push(normal);

        let angle_incremente = (PI - alpha) / self.resolution as f32;
        let start_angle = -PI / 2. + alpha;
        for i in 0..=self.resolution {
            let angle1 = start_angle + angle_incremente * i as f32;
            vertices.push([
                self.radius_min * angle1.cos(),
                height,
                self.radius_min * angle1.sin(),
            ]);
            normals.push(normal);
        }

        let (offset1, offset2) = if height == 0. { (0, 1) } else { (1, 0) };

        for i in 1..(vertices.len() as u32 - 1) {
            indices.push(0);
            indices.push(i + offset1);
            indices.push(i + offset2);
        }

        let mut result = MeshElements {
            vertices,
            normals,
            indices,
        };

        let offset = Vec3::new(
            distance_centers,
            0.,
            (self.radius_max - self.radius_min) / 2.,
        );

        result += offset;

        result
    }
}

impl From<Flipper> for Mesh {
    fn from(value: Flipper) -> Self {
        // Down
        let mut down = value.left_arc(0.);

        let right_arc = value.right_arc(0.);

        let vertices = vec![
            down.vertices[0],
            down.vertices[1],
            down.vertices[down.vertices.len() - 1],
            right_arc.vertices[0],
            right_arc.vertices[1],
            right_arc.vertices[right_arc.vertices.len() - 1],
        ];
        let normals = vec![
            Y_NEGATIF, Y_NEGATIF, Y_NEGATIF, Y_NEGATIF, Y_NEGATIF, Y_NEGATIF,
        ];
        let indices = vec![0, 2, 4, 0, 4, 3, 1, 0, 3, 1, 3, 5];

        down += MeshElements {
            vertices,
            normals,
            indices,
        };

        down += right_arc;

        // Up
        let mut up = value.left_arc(value.thickness);

        let right_arc = value.right_arc(value.thickness);

        let vertices = vec![
            up.vertices[0],
            up.vertices[1],
            up.vertices[up.vertices.len() - 1],
            right_arc.vertices[0],
            right_arc.vertices[1],
            right_arc.vertices[right_arc.vertices.len() - 1],
        ];
        let normals = vec![
            Y_POSITIF, Y_POSITIF, Y_POSITIF, Y_POSITIF, Y_POSITIF, Y_POSITIF,
        ];
        let indices = vec![0, 4, 2, 0, 3, 4, 1, 3, 0, 1, 5, 3];

        up += MeshElements {
            vertices,
            normals,
            indices,
        };

        up += right_arc;

        down += up;

        down += value.border();

        down.into()
    }
}
