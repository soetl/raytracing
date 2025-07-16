use std::sync::Arc;

use ray_tracing::prelude::*;

fn main() {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let perlin = Arc::new(Lambertian::new(Arc::new(PerlinNoise::new(4.0))));
    world.push(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin.clone(),
    )));
    world.push(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin,
    )));

    let bvh = BvhNode::new(world);

    match render(
        &bvh,
        "output/rt-the-next-week_5.7.png",
        &CameraConfig {
            vfov: 20.0,
            look_from: Point3::new(13.0, 2.0, 3.0),
            look_at: Point3::new(0.0, 0.0, 0.0),
            defocus_angle: 0.0,
            ..Default::default()
        },
    ) {
        Ok(_) => println!("Done."),
        Err(e) => println!("Error: {}", e),
    }
}
