use math::vector3d::*;
use math::ray::*;

pub struct Sphere
{
    pub center: Vector3d,
    pub radius: f64
}


impl Intersectable for Sphere
{
    fn intersect(&self, ray: Ray) -> Option<Vector3d>
    {
        let dv = ray.origin() - self.center;
        let c = abs_sq(dv) - self.radius*self.radius;
        let b =  dv * ray.direction() * 2.0;

        let d = b*b  - 4.0 * c; // a=1 because the direction is normed
        if d < 0.0
        {
            return None;
        }
        let lambda : f64;
        let lambda_1 = (-b - d.sqrt()) / 2.0;
        let lambda_2 = (-b + d.sqrt()) / 2.0;

        if lambda_1 > 0.0
        {
            lambda = lambda_1;
        }
        else if lambda_2 > 0.0
        {
            lambda = lambda_2;
        }
        else
        {
            return None;
        }

        return Some(ray.origin() + ray.direction() * lambda);
    }

    /*fn get_intersect_func(& self) -> Box<Fn(Ray) ->  Option<Vector3d> >
    {
        //return Box::new( |ray| self.intersect(ray));
    }*/
}
