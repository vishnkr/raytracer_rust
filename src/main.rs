
pub mod raytracer;
use raytracer::vector::Vec3;
use raytracer::ray::Ray;

pub fn color(r:Ray)->Vec3{
    let unit_dir = Vec3::make_unit_vector(r.direction());
    let t:f32 = 0.5*(unit_dir.y()+1.0);
    (1.0-t)*Vec3::new(1.0,1.0,1.0) + t* Vec3::new(0.5,0.7,1.0)
}

fn main() {
    let nx:u16 = 600;
    let ny:u16 = 400;
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


