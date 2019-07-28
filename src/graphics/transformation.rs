use nalgebra::{Matrix4, Point3, Vector3};
use std::ops::Mul;

use crate::graphics::point::Point;
use crate::graphics::vector::Vector;

/// A 2D transformation matrix.
///
/// It can be used to apply a transformation to a [`Target`].
///
/// [`Target`]: struct.Target.html
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transformation(Matrix4<f32>);

impl Transformation {
    /// Get the identity transformation.
    pub fn identity() -> Transformation {
        Transformation(Matrix4::identity())
    }

    /// Creates an orthographic projection.
    ///
    /// You should rarely need this. On creation, a [`Target`] is automatically
    /// set up with the correct orthographic projection.
    ///
    /// [`Target`]: struct.Target.html
    pub fn orthographic(width: f32, height: f32) -> Transformation {
        Transformation(nalgebra::Matrix4::new_orthographic(
            0.0, width, 0.0, height, -1.0, 1.0,
        ))
    }

    /// Creates a translate transformation.
    ///
    /// You can use this to pan your camera, for example.
    pub fn translate(translation: Vector) -> Transformation {
        Transformation(Matrix4::new_translation(&Vector3::new(
            translation.x,
            translation.y,
            0.0,
        )))
    }

    /// Creates a uniform scale transformation.
    ///
    /// You can use this to zoom your camera, for example.
    pub fn scale(scale: f32) -> Transformation {
        Transformation(Matrix4::new_scaling(scale))
    }

    /// Creates a non-uniform scale transformation.
    ///
    /// It allows you to scale each axis independently. You should rarely need
    /// this.
    pub fn nonuniform_scale(scale: Vector) -> Transformation {
        Transformation(Matrix4::new_nonuniform_scaling(&Vector3::new(
            scale.x, scale.y, 1.0,
        )))
    }

    /// Creates a rotation transformation (in radians).
    ///
    /// You can use this to rotate your camera, for example.
    pub fn rotate(rotation: f32) -> Transformation {
        Transformation(Matrix4::new_rotation(Vector3::new(0.0, 0.0, rotation)))
    }

    /// Transforms the given point by this transformation.
    pub fn transform_point(self, point: Point) -> Point {
        let point = self.0.transform_point(&Point3::new(point.x, point.y, 0.0));
        Point::new(point.x, point.y)
    }

    /// Transforms the given vector by this transformation.
    pub fn transform_vector(self, vector: Vector) -> Vector {
        let vector = self
            .0
            .transform_vector(&Vector3::new(vector.x, vector.y, 0.0));
        Vector::new(vector.x, vector.y)
    }

    /// Transforms the given point by the inverse of this transformation.
    pub fn inverse_transform_point(self, point: Point) -> Point {
        let point = self.0
            .try_inverse()
            .expect("Transformation matrix should only contain invertible operations")
            .transform_point(&Point3::new(point.x, point.y, 0.0));
        Point::new(point.x, point.y)
    }

    /// Transforms the given vector by the inverse of this transformation.
    pub fn inverse_transform_vector(self, vector: Vector) -> Vector {
        let vector = self.0
            .try_inverse()
            .expect("Transformation matrix should only contain invertible operations")
            .transform_vector(&Vector3::new(vector.x, vector.y, 0.0));
        Vector::new(vector.x, vector.y)
    }
}

impl Mul for Transformation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Transformation(self.0 * rhs.0)
    }
}

impl From<Transformation> for [[f32; 4]; 4] {
    fn from(transformation: Transformation) -> [[f32; 4]; 4] {
        transformation.0.into()
    }
}

impl From<Transformation> for [f32; 16] {
    fn from(t: Transformation) -> [f32; 16] {
        [
            t.0[0], t.0[1], t.0[2], t.0[3], t.0[4], t.0[5], t.0[6], t.0[7],
            t.0[8], t.0[9], t.0[10], t.0[11], t.0[12], t.0[13], t.0[14],
            t.0[15],
        ]
    }
}

impl From<Matrix4<f32>> for Transformation {
    fn from(matrix: Matrix4<f32>) -> Self {
        Transformation(matrix)
    }
}

impl Into<Matrix4<f32>> for Transformation {
    fn into(self) -> Matrix4<f32> {
        self.0
    }
}
