use std::sync::Arc;

use crate::{
    material::Material,
    math::{Hittable, Point3, Vec3},
    mesh::Mesh,
    primitives::Quad,
};

pub struct Cuboid {
    mesh: Vec<Arc<dyn Hittable>>,
}

impl Cuboid {
    pub fn new(a: Point3, b: Point3, material: Arc<dyn Material>) -> Self {
        let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

        let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y - min.y, 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z - min.z);

        let sides = [
            Quad::new(Point3::new(min.x, min.y, max.z), dx, dy, material.clone()), // front
            Quad::new(Point3::new(max.x, min.y, max.z), -dz, dy, material.clone()), // right
            Quad::new(Point3::new(max.x, min.y, min.z), -dx, dy, material.clone()), // back
            Quad::new(Point3::new(min.x, min.y, min.z), dz, dy, material.clone()), // left
            Quad::new(Point3::new(min.x, max.y, max.z), dx, -dz, material.clone()), // top
            Quad::new(Point3::new(min.x, min.y, min.z), dx, dz, material.clone()), // bottom
        ];

        let mesh: Vec<Arc<dyn Hittable>> = sides
            .into_iter()
            .map(|quad| Arc::new(quad) as Arc<dyn Hittable>)
            .collect();

        Self { mesh }
    }
}

impl Mesh for Cuboid {
    fn mesh(&self) -> Vec<Arc<dyn Hittable>> {
        self.mesh.clone()
    }
}
