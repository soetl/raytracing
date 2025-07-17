use std::sync::Arc;

use ray_tracing::prelude::*;

fn main() {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    // Cornell box materials
    let red = Arc::new(Lambertian::from(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(Diffuse::from(Color::new(15.0, 15.0, 15.0)));

    // Right wall (green)
    world.push(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));

    // Left wall (red)
    world.push(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));

    // Light
    world.push(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    )));

    // Floor (white)
    world.push(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    // Ceiling (white)
    world.push(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));

    // Back wall (white)
    world.push(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white,
    )));

    let bvh = BvhNode::new(world);

    match render(
        &bvh,
        "output/rt-the-next-week_7.4.png",
        &CameraConfig {
            aspect_ratio: 1.0,
            vfov: 40.0,
            look_from: Point3::new(278.0, 278.0, -800.0),
            look_at: Point3::new(278.0, 278.0, 0.0),
            defocus_angle: 0.0,
            background: Color::new(0.0, 0.0, 0.0),
            max_depth: 15,
            ..Default::default()
        },
    ) {
        Ok(_) => println!("Done."),
        Err(e) => println!("Error: {}", e),
    }
}
