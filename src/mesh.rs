
use crate::vector::Vector3;
use crate::matrix::Matrix4;
use crate::camera::Camera;
use std::f64;

use sfml::graphics::Color;

pub struct Vertex {
    pub pt: Vector3,
    pub color: Option<Color>,
}

pub struct Face {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub color: Option<Color>,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
    pub translation: Vector3,
}

macro_rules! face {
    ($a:expr, $b:expr, $c:expr) => (Face { a: $a, b: $b, c: $c, color: None })
}


impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            faces: Vec::new(),
            rot_x: 0.,
            rot_y: 0.,
            rot_z: 0.,
            translation: Vector3::zero(),
        }
    }

    fn pt(&self, index: usize) -> Vector3 {
        self.vertices[index].pt
    }

    pub fn rot_x(&mut self, theta: f32) {
        self.rot_x += theta
    }

    pub fn rot_y(&mut self, theta: f32) {
        self.rot_y += theta
    }

    pub fn rot_z(&mut self, theta: f32) {
        self.rot_z += theta
    }

    pub fn translate(&mut self, vec: Vector3) {
        self.translation = self.translation + vec
    }

    pub fn get_mat(&self) -> Matrix4 {
        Matrix4::rot_and_translate(self.rot_x, self.rot_y, self.rot_z,
            self.translation)
    }
}
