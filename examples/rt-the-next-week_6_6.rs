use std::sync::Arc;

use ray_tracing::prelude::*;

fn main() {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let red = Lambertian::from(Color::new(1.0, 0.2, 0.2));
    let green = Lambertian::from(Color::new(0.2, 1.0, 0.2));
    let blue = Lambertian::from(Color::new(0.2, 0.2, 1.0));
    let orange = Lambertian::from(Color::new(1.0, 0.5, 0.0));
    let teal = Lambertian::from(Color::new(0.2, 0.8, 0.8));

    world.push(Arc::new(Quad::new(
        (-3.0, -2.0, 5.0).into(),
        (0.0, 0.0, -4.0).into(),
        (0.0, 4.0, 0.0).into(),
        Arc::new(red),
    )));
    world.push(Arc::new(Quad::new(
        (-2.0, -2.0, 0.0).into(),
        (4.0, 0.0, 0.0).into(),
        (0.0, 4.0, 0.0).into(),
        Arc::new(green),
    )));
    world.push(Arc::new(Quad::new(
        (3.0, -2.0, 1.0).into(),
        (0.0, 0.0, 4.0).into(),
        (0.0, 4.0, 0.0).into(),
        Arc::new(blue),
    )));
    world.push(Arc::new(Quad::new(
        (-2.0, 3.0, 1.0).into(),
        (4.0, 0.0, 0.0).into(),
        (0.0, 0.0, 4.0).into(),
        Arc::new(orange),
    )));
    world.push(Arc::new(Quad::new(
        (-2.0, -3.0, 5.0).into(),
        (4.0, 0.0, 0.0).into(),
        (0.0, 0.0, -4.0).into(),
        Arc::new(teal),
    )));

    let bvh = BvhNode::new(world);

    match render(
        &bvh,
        "output/rt-the-next-week_6.6.png",
        &CameraConfig {
            aspect_ratio: 1.0,
            vfov: 80.0,
            look_from: Point3::new(0.0, 0.0, 9.0),
            look_at: Point3::new(0.0, 0.0, 0.0),
            defocus_angle: 0.0,
            ..Default::default()
        },
    ) {
        Ok(_) => println!("Done."),
        Err(e) => println!("Error: {}", e),
    }
}
