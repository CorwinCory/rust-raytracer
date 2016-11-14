mod math;
mod scenery;
use math::vector3d::*;
use math::sphere::*;
use math::ray::*;
use scenery::camera::*;

extern crate image;

use image::{ImageBuffer, Rgb};

fn main()
{

    let z = Vector3d::zero();
    let e1 = Vector3d {x: 1.0, y: 0.0, z: 0.0};
    let e2 = Vector3d {x: 0.0, y: 1.0, z: 0.0};
    let e3 = Vector3d {x: 0.0, y: 0.0, z: 1.0};

    let camera_pos = e3*2.0;

    let camera = Camera {position: camera_pos,
                         y_dir: e3,
                         x_dir: e2,
                         z_dir: -e1,
                         screen_height: 1.0,
                         screen_width: 1.0,
                         pixels_width: 512,
                         pixels_height: 512};

    // define the level here
    let ball1 = Sphere {center: Vector3d::zero(), radius: 0.1};

    let mut objects: Vec<Box<Intersectable> > = Vec::new();
    objects.push(Box::new(ball1));
    // actual job
    let mut result = ImageBuffer::new(camera.pixels_width as u32, camera.pixels_height as u32);
    for (x, y, pixel) in result.enumerate_pixels_mut()
    {
        let mut hit = false;
        let step_x = camera.x_dir / (camera.pixels_width as f64);
        let step_y = camera.y_dir / (camera.pixels_height as f64);
        let ray = Ray::new(camera.position, camera.z_dir + step_x*(x as f64) + step_y *(y as f64));
        for object in &objects
        {
            if let Some(x) = object.intersect(ray)
            {
                hit = true;
            }
        }

        if hit
        {
            *pixel = image::Luma([255 as u8]);
        }
        else
        {
            *pixel = image::Luma([255 as u8]);
        }
    }
    // Save the image as “fractal.png”
   let ref mut fout = std::fs::File::create(&std::path::Path::new("frame.png")).unwrap();
   // We must indicate the image’s color type and what format to save as
   let _ = image::ImageLuma8(result).save(fout, image::PNG);

    println!("Test!");
}
