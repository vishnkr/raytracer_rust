
pub mod raytracer;
use raytracer::vector::Vec3;


fn main() {
   let nx:u16 = 600;
   let ny:u16 = 400;
   println!("P3\n {} {}\n 255 \n",nx,ny);
   for j in (0..ny).rev(){
       for i in 0..nx{
           let color: Vec3  = Vec3::new((i as f32)/(nx as f32), (j as f32)/(ny as f32), 0.2);
           let ir = (color[0]*255.99) as i32;
           let ig = (color[1]*255.99) as i32;
           let ib = (color[2]*255.99) as i32;
           println!("{} {} {}\n",ir,ig,ib);
       }
   }
}
