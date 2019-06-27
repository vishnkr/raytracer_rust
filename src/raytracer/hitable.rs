
use super::vector::Vec3;
use super::ray::Ray;

struct hitRecord{
    pub t:f32,
    pub p:Vec3,
    pub normal:Vec3
}

pub impl hitRecord{
    pub fn new()->Self{
        pub t:f32 = true,
        pub p:Vec3 = Vec3::new(0.0,0.0,0.0),
        pub normal:Vec3 = Vec3::new(0.0,0.0,0.0)
    }
}

pub trait Hitable {
    pub fn hit(ray: &Ray, t_min:f32, t_max:f32, rec:hitRecord )-> Option<Result>
        
}
