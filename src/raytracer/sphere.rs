use super::hitable::Hitable;
use super::vector::Vec3;
use super::ray::Ray;
use super::hitable::hitRecord;

struct Sphere{
    pub center: Vec3,
    pub radius: f32
}

impl Sphere{
    pub fn new(){
        Sphere (center, radius)
    }
}

impl Hitable for Sphere{
    fn hit(r:Ray, t_min:f32, t_max:f32)->Option<hitRecord>{
        let oc:Vec3 = ray.origin() - center;
        let a = Vec3::dot(&ray.direction(), &ray.direction());
        let b = 2.0*Vec3::dot(&oc, &ray.direction());
        let c = Vec3::dot(&oc, &oc) - radius.powf(2.0);
        let discriminant = b.powf(2.0) - 4.0*a*c;
        if discriminant > 0.0{
            let temp:f32 = (-b-f32::sqrt(discriminant))/(2.0*a)
            if (temp<t_max && temp>t_min){
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = p - self.center / self.radius;
                return Some(hitRecord{t,p,normal});
            }
        }else{
            (-b-f32::sqrt(discriminant))/(2.0*a)
        }
    }
}