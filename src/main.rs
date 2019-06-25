
pub mod raytracer;
use raytracer::vector::Vec3;
use raytracer::ray::Ray;


pub fn hit_sphere(center:Vec3, radius:f32, ray:Ray)->f32{
   let oc:Vec3 = ray.origin() - center;
    let a = Vec3::dot(&ray.direction(), &ray.direction());
    let b = 2.0*Vec3::dot(&oc, &ray.direction());
    let c = Vec3::dot(&oc, &oc) - radius.powf(2.0);
    let discriminant = b.powf(2.0) - 4.0*a*c;
    if discriminant < 0.0{
        -1.0
    }else{
        (-b-f32::sqrt(discriminant))/(2.0*a)
    }
}

pub fn color(r:Ray)->Vec3{
    let t:f32 = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);
   if t>0.0 {
         let n:Vec3 = Vec3::make_unit_vector(r.point_at_parameter(t)-Vec3::new(0.0,0.0,-1.0));
         return 0.5*Vec3::new(n.x()+1.0,n.y()+1.0,n.z()+1.0);
     }
    let unit_dir = Vec3::make_unit_vector(r.direction());
    let t:f32 = 0.5*(unit_dir.y()+1.0);
    (1.0-t)*Vec3::new(1.0,1.0,1.0) + t* Vec3::new(0.5,0.7,1.0)
}

fn main() {
    let nx:u16 = 800;
    let ny:u16 = 600;
    println!("P3\n {} {}\n 255 \n",nx,ny);
    let lower_left_corner :Vec3 = Vec3::new(-2.0,-1.0,-1.0);
    let horizontal: Vec3 = Vec3::new(4.0,0.0,0.0);
    let vertical: Vec3 = Vec3::new(0.0,2.0,0.0);
    let origin: Vec3 = Vec3::new(0.0,0.0,0.0);
    for j in (0..ny).rev(){
        for i in 0..nx{
            let u:f32 = (i as f32)/(nx as f32);
            let v:f32 = (j as f32)/(ny as f32);
            let r:Ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
            let color: Vec3  = color(r);
            let ir = (color[0]*255.99) as i32;
            let ig = (color[1]*255.99) as i32;
            let ib = (color[2]*255.99) as i32;
            println!("{} {} {}\n",ir,ig,ib);
        }
   }
}


