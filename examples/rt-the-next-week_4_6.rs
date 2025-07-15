use std::sync::Arc;

use ray_tracing::{prelude::*, texture::image::ImageTexture};

fn main() {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let image = image::open("examples/assets/images/earthmap.jpg")
        .unwrap()
        .into_rgb8();

    let earth_material = Arc::new(Lambertian::new(Arc::new(ImageTexture::new(image))));
    world.push(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_material,
    )));

    let bvh = BvhNode::new(world);

    match render(
        &bvh,
        "output/rt-the-next-week_4.6.png",
        &CameraConfig {
            vfov: 20.0,
            look_from: Point3::new(0.0, 0.0, 12.0),
            look_at: Point3::new(0.0, 0.0, 0.0),
            defocus_angle: 0.0,
            ..Default::default()
        },
    ) {
        Ok(_) => println!("Done."),
        Err(e) => println!("Error: {}", e),
    }
}
