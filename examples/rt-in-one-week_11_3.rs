use std::sync::Arc;

use ray_tracing::prelude::*;

fn main() {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.00 / 1.33);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.push(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Arc::new(material_ground),
    )));
    world.push(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        Arc::new(material_center),
    )));
    world.push(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Arc::new(material_left),
    )));
    world.push(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Arc::new(material_right),
    )));

    let bvh = BvhNode::new(world);

    match render(
        &bvh,
        "output/rt-in-one-week-11.3.png",
        &CameraConfig::default(),
    ) {
        Ok(_) => println!("Done."),
        Err(e) => println!("Error: {}", e),
    }
}
