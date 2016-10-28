use math::vector3d::*;

pub struct Camera
{
    pub position: Vector3d,
    pub x_dir: Vector3d,
    pub y_dir: Vector3d,
    pub z_dir: Vector3d,

    pub  screen_width: f64,
    pub  screen_height: f64,

    pub  pixels_width: i32,
    pub  pixels_height: i32
}
