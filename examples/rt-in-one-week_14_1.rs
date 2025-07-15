use std::sync::Arc;

use ray_tracing::{prelude::*, utils::Random};

fn main() {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.push(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(ground_material),
    )));

    (-11..11).for_each(|a| {
        (-11..11).for_each(|b| {
            let choose_material = f32::random();
            let center = Point3::new(
                a as f32 + 0.9 * f32::random(),
                0.2,
                b as f32 + 0.9 * f32::random(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = match choose_material {
                    x if x < 0.8 => {
                        let albedo = Color::from(Vec3::random() * Vec3::random());
                        Arc::new(Lambertian::new(albedo))
                    }
                    x if x < 0.95 => {
                        let albedo = Color::from(Vec3::random_range(&(0.5..1.0)));
                        let fuzz = f32::random_range(&(0.0..0.5));
                        Arc::new(Metal::new(albedo, fuzz))
                    }
                    _ => Arc::new(Dielectric::new(1.5)),
                };

                world.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        });
    });

    let material_1 = Dielectric::new(1.5);
    world.push(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(material_1),
    )));

    let material_2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.push(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(material_2),
    )));

    let material_3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.push(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(material_3),
    )));

    let bvh = BvhNode::new(world);

    match render(
        &bvh,
        "output/rt-in-one-week-14.1.png",
        &CameraConfig {
            vfov: 20.0,
            look_from: Point3::new(13.0, 2.0, 3.0),
            look_at: Point3::new(0.0, 0.0, 0.0),
            defocus_angle: 0.6,
            ..Default::default()
        },
    ) {
        Ok(_) => println!("Done."),
        Err(e) => println!("Error: {}", e),
    }
}
