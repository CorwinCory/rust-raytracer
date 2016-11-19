use math::vector3d::*;
use math::ray::*;

pub struct Camera
{
    pub position: Vector3d,

    pub x_dir: Vector3d,
    pub y_dir: Vector3d,
    pub z_dir: Vector3d,

    pub focus_dist: f64,

    pub  screen_width: f64,
    pub  screen_height: f64,

    pub  pixels_width: u32,
    pub  pixels_height: u32
}

impl Camera
{
    /// Pixels are enumerated left to right, top to bottom
    pub fn ray_for_pixel(&self, x: u32, y: u32) -> Ray
    {
        let step_x = self.x_dir * self.screen_width / (self.pixels_width as f64);
        let step_y = self.y_dir * self.screen_height / (self.pixels_height as f64);
        return Ray::new(self.position,
                        self.z_dir * self.focus_dist
                         + step_x * (x as f64)
                         + step_y *( (self.pixels_height  - y) as f64));
    }
}
