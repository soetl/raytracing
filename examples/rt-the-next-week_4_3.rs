use std::sync::Arc;

use ray_tracing::prelude::*;

fn main() {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let checkers = Arc::new(Lambertian::from(CheckersTexture::from((
        0.32,
        Color::new(0.1, 0.1, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ))));
    world.push(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        checkers.clone(),
    )));
    world.push(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        checkers,
    )));

    let bvh = BvhNode::new(world);

    match render(
        &bvh,
        "output/rt-the-next-week_4.3.png",
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
