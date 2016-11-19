mod math;
mod scenery;
use math::vector3d::*;
use math::sphere::*;
use math::ray::*;
use scenery::camera::*;
ise scenery::color::*;

extern crate image;

use image::{ImageBuffer, Rgb};

fn main()
{
    // asd
    let z = Vector3d::zero();
    let e1 = Vector3d {x: 1.0, y: 0.0, z: 0.0};
    let e2 = Vector3d {x: 0.0, y: 1.0, z: 0.0};
    let e3 = Vector3d {x: 0.0, y: 0.0, z: 1.0};

    let camera_pos = Vector3d {x: 2.0, y: 0.0, z: 0.0};

    let camera = Camera {position: camera_pos,
                         y_dir: e3,
                         x_dir: e2,
                         z_dir: -e1,
                         focus_dist: 2.0,
                         screen_height: 1.0,
                         screen_width: 1.0,
                         pixels_width: 512,
                         pixels_height: 512};

    // define the level here
    let rad = 0.1;
    let ball1 = Sphere {center: Vector3d {x: 0.0, y: rad/2.0, z: rad/2.0 },  radius: rad};

    let mut objects: Vec<Box<Intersectable> > = Vec::new();
    objects.push(Box::new(ball1));
    // actual job
    //let mut result = ImageBuffer::new(camera.pixels_width as u32, camera.pixels_height as u32);
    let mut result = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(camera.pixels_width as u32, camera.pixels_height as u32);
    //
    for (x, y, pixel) in result.enumerate_pixels_mut()
    {
        let mut hit = false;
        let ray = camera.ray_for_pixel(x,y);
        //println!("{:?}", ray);
        for object in &objects
        {
            if let Some(x) = object.intersect(ray)
            {
                hit = true;
            }
        }

        if hit
        {
            println!("hit");
            *pixel = image::Rgb([255 as u8, 200 as u8, 200 as u8]);
        }
        else
        {
            *pixel = image::Rgb([0 as u8, 0 as u8, 0 as u8]);
        }
    }
    // Save the image as “fractal.png”
   let ref mut fout = std::fs::File::create(&std::path::Path::new("frame.png")).unwrap();
   // We must indicate the image’s color type and what format to save as
   let _ = image::ImageRgb8(result).save(fout, image::PNG);

    println!("Test!");
}
