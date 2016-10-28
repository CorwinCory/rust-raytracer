use std::ops::{Add, Sub, AddAssign, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

/*
    Copy is intended to simplify usage of arithmetic operators.
    Lets hope that will not create too much an overhead.
*/
#[derive(Debug, Copy,  Clone)]
pub struct Vector3d
{
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub fn abs(vec: Vector3d) -> f64
{
    return (vec.x * vec.x + vec.y * vec.y + vec.z * vec.z).sqrt()
}

impl Vector3d
{
    pub fn zero() -> Vector3d
    {
        return Vector3d {x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn normalize(&mut self)
    {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

}

/*
    Arithmnetic overloads
*/
impl Add for Vector3d
{
    type Output = Vector3d;

    fn add(self, other: Vector3d) -> Vector3d
    {
        return Vector3d{ x: self.x + other.x,
                         y: self.y + other.y,
                         z: self.z + other.z}
    }
}

impl Sub for Vector3d
{
    type Output = Vector3d;

    fn sub(self, other: Vector3d) -> Vector3d
    {
        return Vector3d{ x: self.x - other.x,
                         y: self.y - other.y,
                         z: self.z - other.z}
    }
}

impl AddAssign for Vector3d
{
    fn add_assign(&mut self, other: Vector3d)
    {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vector3d
{
    fn sub_assign(&mut self, other: Vector3d)
    {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

// Dot product
impl Mul<Vector3d> for Vector3d
{
    type Output = f64;

    fn mul(self, vec: Vector3d) -> f64
    {
        return self.x * vec.x + self.y * vec.y + self.z * vec.z;
    }
}

/*
    Scalar stuff
*/
impl Mul<f64> for Vector3d
{
    type Output = Vector3d;

    fn mul(self, m: f64) -> Vector3d
    {
        return Vector3d {x: self.x * m, y: self.y *m, z: self.z *m};
    }
}

impl Div<f64> for Vector3d
{
    type Output = Vector3d;

    fn div(self, m: f64) -> Vector3d
    {
        return Vector3d {x: self.x / m, y: self.y / m, z: self.z / m};
    }
}

impl Neg for Vector3d
{
    type Output = Vector3d;

    fn neg(self) -> Vector3d
    {
        return Vector3d {x: -self.x, y: -self.y, z: -self.z};
    }
}

impl MulAssign<f64> for Vector3d
{

    fn mul_assign(& mut self, m: f64)
    {
        self.x *= m;
        self.y *= m;
        self.z *= m;
    }
}

impl DivAssign<f64> for Vector3d
{

    fn div_assign(& mut self, m: f64)
    {
        self.x /= m;
        self.y /= m;
        self.z /= m;
    }
}
