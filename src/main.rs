fn main() {
   let nx:u8 = 200;
   let ny:u8 = 100;
   println!("P3\n {} {}\n 255 \n",nx,ny);
   for j in (0..ny).rev(){
       for i in 0..nx{
           let r = (i as f32 /nx as f32) as f32;
           let g = (j as f32 /ny as f32) as f32;
           let b = 0.2;
           let ir = (r*255.99) as i32;
           let ig = (g*255.99) as i32;
           let ib = (b*255.99) as i32;
           println!("{} {} {}\n",ir,ig,ib);
       }
   }
}
