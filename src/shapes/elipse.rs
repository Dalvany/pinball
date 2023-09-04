//! This shapes is a part of an ellipse "carved" into
//! a rectangle box.

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::render::{mesh::Indices, render_resource::PrimitiveTopology};
use thiserror::Error;

use super::{MeshElements, X_POSITIF, Y_NEGATIF, Y_POSITIF, Z_NEGATIF};

#[derive(Clone, Copy, Debug, Error)]
pub(crate) enum BanisterError {
    #[error("Angle must be between 0 and PI/2 (inclusive)")]
    OutsideRange,
}

/// Enum that allow to control where the
/// origin of the local coordinate is.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum Origin {
    #[default]
    Center,
    MinXMinZ,
    MaxXMinZ,
    MinXMaxZ,
    MaxXMaxZ,
}

pub(crate) struct Ellipse {
    /// Show the quarter of rect
    pub(crate) rectangle: bool,
    /// "Center" for easy placement of the element
    pub(crate) center: Origin,
    /// Starts from the left and turn counter clockwise. Must be between [0, PI/2].
    pub(crate) first_angle: f32,
    /// Starts from the left and turn counter clockwise. Must be between [0, PI/2].
    pub(crate) second_angle: f32,
    /// Number of vertices for the ellipse
    pub(crate) resolution: usize,
    ///  Ellipse half-axis in the X direction
    pub(crate) x: f32,
    /// Ellipse half-axis in the Z direction
    pub(crate) z: f32,
    /// Thickness (this the height in the upper direction)
    pub(crate) thickness: f32,
}

impl Default for Ellipse {
    fn default() -> Self {
        Self {
            rectangle: true,
            center: Origin::Center,
            first_angle: 0.,
            second_angle: PI / 2.,
            resolution: 20,
            x: 3.,
            z: 1.,
            thickness: 1.,
        }
    }
}

impl Ellipse {
    fn min_max_angle(&self) -> (f32, f32) {
        if self.first_angle < self.second_angle {
            (self.first_angle, self.second_angle)
        } else {
            (self.second_angle, self.first_angle)
        }
    }

    fn axis_offsets(&self) -> Vec3 {
        let center_y = self.thickness / 2.;
        let (min, max) = self.min_max_angle();

        let x1 = self.x * min.cos();
        let x2 = self.x * max.cos();
        let z1 = self.z * min.sin();
        let z2 = self.z * max.sin();

        let (left, right) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (bottom, top) = if z1 < z2 { (z2, z1) } else { (z1, z2) };

        match self.center {
            Origin::Center => Vec3::new(
                self.real_x() / 2. - right,
                -center_y,
                self.real_z() / 2. - bottom,
            ),
            Origin::MinXMinZ => Vec3::new(-left, -center_y, -top),
            Origin::MaxXMinZ => Vec3::new(-right, -center_y, -top),
            Origin::MinXMaxZ => Vec3::new(-left, -center_y, -bottom),
            Origin::MaxXMaxZ => Vec3::new(-right, -center_y, -bottom),
        }
    }

    pub(crate) fn real_x(&self) -> f32 {
        let (min, max) = self.min_max_angle();

        (self.x * max.cos() - self.x * min.cos()).abs()
    }

    pub(crate) fn real_z(&self) -> f32 {
        let (min, max) = self.min_max_angle();

        (self.z * max.sin() - self.z * min.sin()).abs()
    }

    /// Code for top and bottom
    fn top_bottom(
        &self,
        height: f32,
        angle_min: f32,
        angle_max: f32,
        angle_increment: f32,
    ) -> MeshElements {
        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(self.resolution + 2);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(self.resolution + 2);
        let mut indices: Vec<u32> = Vec::with_capacity(self.resolution * 3);

        let (normal, second_vertices, third_vertices) = if height == 0. {
            (Y_NEGATIF, 2, 1)
        } else {
            (Y_POSITIF, 1, 2)
        };

        vertices.push([self.x * angle_min.cos(), height, self.z * angle_max.sin()]);
        normals.push(normal);
        for i in 0..self.resolution + 1 {
            let x_current = self.x * (angle_min + i as f32 * angle_increment).cos();
            let z_current = self.z * (angle_min + i as f32 * angle_increment).sin();

            vertices.push([x_current, height, z_current]);
            normals.push(normal);
        }

        for i in 0..self.resolution as u32 {
            indices.push(0);
            indices.push(i + second_vertices);
            indices.push(i + third_vertices);
        }

        MeshElements {
            vertices,
            normals,
            indices,
        }
    }

    fn back(&self, angle_min: f32, angle_max: f32) -> MeshElements {
        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(4);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(4);
        let mut indices: Vec<u32> = Vec::with_capacity(6);

        vertices.push([self.x * angle_min.cos(), 0., self.z * angle_max.sin()]);
        vertices.push([
            self.x * angle_min.cos(),
            self.thickness,
            self.z * angle_max.sin(),
        ]);
        vertices.push([
            self.x * angle_max.cos(),
            self.thickness,
            self.z * angle_max.sin(),
        ]);
        vertices.push([self.x * angle_max.cos(), 0., self.z * angle_max.sin()]);
        normals.push(Z_NEGATIF);
        normals.push(Z_NEGATIF);
        normals.push(Z_NEGATIF);
        normals.push(Z_NEGATIF);

        indices.push(0);
        indices.push(1);
        indices.push(2);
        indices.push(2);
        indices.push(3);
        indices.push(0);

        MeshElements {
            vertices,
            normals,
            indices,
        }
    }

    fn right(&self, angle_min: f32, angle_max: f32) -> MeshElements {
        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(4);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(4);
        let mut indices: Vec<u32> = Vec::with_capacity(6);

        vertices.push([self.x * angle_min.cos(), 0., self.z * angle_min.sin()]);
        vertices.push([
            self.x * angle_min.cos(),
            self.thickness,
            self.z * angle_min.sin(),
        ]);
        vertices.push([
            self.x * angle_min.cos(),
            self.thickness,
            self.z * angle_max.sin(),
        ]);
        vertices.push([self.x * angle_min.cos(), 0., self.z * angle_max.sin()]);

        normals.push(X_POSITIF);
        normals.push(X_POSITIF);
        normals.push(X_POSITIF);
        normals.push(X_POSITIF);

        indices.push(0);
        indices.push(1);
        indices.push(2);
        indices.push(2);
        indices.push(3);
        indices.push(0);

        MeshElements {
            vertices,
            normals,
            indices,
        }
    }

    fn ellipse(&self, angle_min: f32, angle_increment: f32, revert: bool) -> MeshElements {
        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(self.resolution * 4);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(self.resolution * 4);
        let mut indices: Vec<u32> = Vec::with_capacity(self.resolution * 6);

        for i in 0..self.resolution {
            let x_current = self.x * (angle_min + i as f32 * angle_increment).cos();
            let z_current = self.z * (angle_min + i as f32 * angle_increment).sin();

            let x_next = self.x * (angle_min + (i + 1) as f32 * angle_increment).cos();
            let z_next = self.z * (angle_min + (i + 1) as f32 * angle_increment).sin();

            vertices.push([x_current, 0., z_current]);
            vertices.push([x_current, self.thickness, z_current]);
            vertices.push([x_next, self.thickness, z_next]);
            vertices.push([x_next, 0., z_next]);

            let direction = if revert { 1. } else { -1. };
            // Compute normal to the surface delimited by four vertices
            let v = Vec3::new(direction * x_current, 0., direction * z_current)
                // Normalize because it is required to put in normal attribut
                .normalize();

            for _ in 0..4 {
                normals.push([v.x, v.y, v.z]);
            }
        }

        let (offset1, offset2, offset3, offset4, offset5, offset6) = if revert {
            (0, 1, 2, 2, 3, 0)
        } else {
            (0, 2, 1, 2, 0, 3)
        };
        for i in (0..vertices.len() as u32).step_by(4) {
            // Triangle one for side
            indices.push(i + offset1);
            indices.push(i + offset2);
            indices.push(i + offset3);
            // Triangle two for side
            indices.push(i + offset4);
            indices.push(i + offset5);
            indices.push(i + offset6);
        }

        MeshElements {
            vertices,
            normals,
            indices,
        }
    }
}

impl TryFrom<Ellipse> for Mesh {
    type Error = BanisterError;

    fn try_from(value: Ellipse) -> Result<Self, Self::Error> {
        if value.first_angle < 0.
            || value.first_angle > PI / 2.
            || value.second_angle < 0.
            || value.second_angle > PI / 2.
        {
            return Err(BanisterError::OutsideRange);
        }

        let angle_increment =
            (value.first_angle - value.second_angle).abs() / value.resolution as f32;
        let (angle_min, angle_max) = value.min_max_angle();

        let mut ellipse = value.ellipse(angle_min, angle_increment, false);

        if value.rectangle {
            // Top
            ellipse += value.top_bottom(value.thickness, angle_min, angle_max, angle_increment);

            // Bottom
            ellipse += value.top_bottom(0., angle_min, angle_max, angle_increment);

            // Back
            ellipse += value.back(angle_min, angle_max);

            // Right
            ellipse += value.right(angle_min, angle_max);
        } else {
            ellipse += value.ellipse(angle_min, angle_increment, true);
        }

        // Center the origin
        ellipse += value.axis_offsets();

        Ok(ellipse.into())
    }
}
