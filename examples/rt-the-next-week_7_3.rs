use std::sync::Arc;

use ray_tracing::prelude::*;

fn main() {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let perlin = Arc::new(Lambertian::from(PerlinNoise::new(4.0)));
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

    let light = Arc::new(Diffuse::from(Color::new(4.0, 4.0, 4.0)));
    world.push(Arc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        light.clone(),
    )));
    world.push(Arc::new(Quad::new(
        (3.0, 1.0, -2.0).into(),
        (2.0, 0.0, 0.0).into(),
        (0.0, 2.0, 0.0).into(),
        light,
    )));

    let bvh = BvhNode::new(world);

    match render(
        &bvh,
        "output/rt-the-next-week_7.3.png",
        &CameraConfig {
            vfov: 20.0,
            look_from: Point3::new(26.0, 3.0, 6.0),
            look_at: Point3::new(0.0, 2.0, 0.0),
            defocus_angle: 0.0,
            background: Color::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
    ) {
        Ok(_) => println!("Done."),
        Err(e) => println!("Error: {}", e),
    }
}
