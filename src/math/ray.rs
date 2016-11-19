use math::vector3d::Vector3d;

#[derive(Debug, Copy,  Clone)]
pub struct Ray
{
    origin: Vector3d,
    direction: Vector3d
}

impl Ray
{
    pub fn new(org: Vector3d, dir: Vector3d) -> Ray
    {
        let mut normed = dir.clone();
        normed.normalize();
        return Ray {origin: org, direction: normed}
    }

    pub fn origin(& self) -> Vector3d
    {
        return self.origin;
    }

    pub fn direction(& self) -> Vector3d
    {
        return self.direction;
    }
}

pub trait Intersectable
{
    fn intersect(&self, ray: Ray) -> Option<Vector3d>;

    //fn get_intersect_func(&self) -> Box<Fn(Ray) ->  Option<Vector3d> >;
}
