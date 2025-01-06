use image::RgbImage;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

use crate::{
    color::{Color, Linear},
    hittable::Hittable,
    point::Point3,
    ray::Ray,
    utils::{Random, INFINITY},
    vec::{Vec3, VecExt},
};

pub struct Camera {
    image_width: u32,
    image_height: u32,
    pixel00_origin: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u32,
    max_depth: u32,
    look_from: Point3,
    defocus_angle: f32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        &CameraConfig {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            look_from,
            look_at,
            vup,
            defocus_angle,
            focus_distance,
        }: &CameraConfig,
    ) -> Self {
        let image_height = (image_width as f32 / aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left =
            look_from - focus_distance * w - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_origin = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_angle = if defocus_angle < 0.0 {
            0.0
        } else {
            (defocus_angle / 2.0).to_radians()
        };
        let defocus_radius = focus_distance * defocus_angle.tan();
        let defocus_disk_u = defocus_radius * u;
        let defocus_disk_v = defocus_radius * v;

        Self {
            image_width,
            image_height,
            pixel00_origin,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,
            look_from,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &impl Hittable) -> RgbImage {
        let mut output = RgbImage::new(self.image_width, self.image_height);
        output
            .par_enumerate_pixels_mut()
            .progress()
            .for_each(|(x, y, pixel)| {
                let mut color: Color<Linear> =
                    (0..self.samples_per_pixel).fold(Color::from(Vec3::ZERO), |acc, _| {
                        let ray = self.get_ray(x, y);
                        Color::from(acc.v + Self::ray_color(&ray, world, self.max_depth).v)
                    });

                color.v *= 1.0 / self.samples_per_pixel as f32;
                *pixel = color.to_srgb().into();
            });

        output
    }

    fn ray_color(ray: &Ray, world: &impl Hittable, depth: u32) -> Color<Linear> {
        if depth == 0 {
            return Color::from(Vec3::ZERO);
        }

        if let Some(hit) = world.hit(ray, 0.001..INFINITY) {
            let Some((scattered, attenuation)) = hit.material.scatter(ray, &hit) else {
                return Color::from(Vec3::ZERO);
            };
            return Color::from(attenuation.v * Self::ray_color(&scattered, world, depth - 1).v);
        }

        let unit_direction = ray.direction().normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        Color::from(Vec3::lerp(Vec3::ONE, Vec3::new(0.5, 0.7, 1.0), a))
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_origin
            + (((i as f32) + offset.x) * self.pixel_delta_u)
            + (((j as f32) + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.look_from
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction).with_time(f32::random())
    }

    fn sample_square() -> Vec3 {
        Vec3::new(f32::random() - 0.5, f32::random() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.look_from + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}

pub struct CameraConfig {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f32,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pub defocus_angle: f32,
    pub focus_distance: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 1200,
            samples_per_pixel: 500,
            max_depth: 50,
            vfov: 90.0,
            look_from: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::Y,
            defocus_angle: 0.0,
            focus_distance: 10.0,
        }
    }
}
