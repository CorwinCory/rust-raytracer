mod math;

#[test]
fn check_vectors()
{
    use math::vector3d::*;

    let eps = 1e-6;

    let z = Vector3d::zero();
    let x = Vector3d {x: 1.0,  y:0.0, z:0.0};
    let y = Vector3d {x:-1.0,  y:0.0, z:0.0};
    let w = Vector3d {x: 0.0,  y:1.0, z:0.0};

    assert!(abs(z) <= eps);
    assert!(abs(x + y) <= eps);
    assert!( (x*w).abs() <= eps );
    assert!( abs(x*(-1.0) -y ) <= eps );
    assert!( abs(-x + x) <= eps );

}

#[test]
fn check_rays()
{
    use math::vector3d::*;
    use math::ray::*;

    let o = Vector3d {x:2.0, y:0.0, z:0.0}; ;
    let d = Vector3d {x:-1.0, y:0.0, z:0.0};

    let ray = Ray::new(o, d);
    let s = math::sphere::Sphere {center: Vector3d::zero(), radius: 1.0};
    assert!(s.intersect(ray).is_some());
}


#[test]
fn check_camera()
{

}
