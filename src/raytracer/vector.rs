use std::f32;
use std::ops;

#[derive(Debug,Clone,Copy)]
pub struct Vec3{
    e: [f32;3],
}

impl Vec3{
    pub fn new(e0:f32, e1:f32, e2:f32)-> Vec3{
        Vec3{e:[e0,e1,e2]}
    }

    pub fn x(&self)-> f32 {
        self.e[0]}
    pub fn y(&self)-> f32 {
        self.e[1]}
    pub fn z(&self)-> f32 {
        self.e[2]}
    pub fn r(&self)-> f32 {
        self.e[0]}
    pub fn g(&self)-> f32 {
        self.e[1]}
    pub fn b(&self)-> f32 {
        self.e[2]}

    pub fn length(&self)->f32{
        let sum:f32 = (self.e[0]).powi(2) + (self.e[1]).powi(2) + (self.e[2]).powi(2);
        sum.sqrt()
    }

    pub fn squared_length(&self)->f32{
        (self.length()).powi(2)
    }

    pub fn make_unit_vector(vector : Vec3)-> Vec3{
        vector/vector.length()
    } 

    pub fn dot(v1:&Vec3, v2:&Vec3)->f32{
        (v1.e[0]*v2.e[0])+(v1.e[1]*v2.e[1])+(v1.e[2]*v2.e[2])
    }
    
    pub fn cross(v1:&Vec3, v2:&Vec3)->Vec3{
        Vec3::new(v1.e[1]*v2.e[2] - v1.e[2]*v2.e[1],-(v1.e[0]*v2.e[2] - v1.e[2]*v2.e[0]), v1.e[0]*v2.e[1] - v1.e[1]*v2.e[0])
    }
}
// implementation of std::ops to perform operations on vectors
impl ops::Index<usize> for Vec3{
    type Output = f32;

    fn index(&self, index:usize)->&f32{
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3{
    fn index_mut(&mut self, index:usize)->&mut f32{
        &mut self.e[index]
    }
}

impl ops::Add<Vec3> for Vec3{
    type Output = Vec3;

    fn add(self, _rhs:Vec3)->Vec3{
        Vec3::new(self.e[0]+_rhs.e[0], self.e[1]+_rhs.e[1], self.e[2]+ _rhs.e[2])
    }
}

impl ops::Sub<Vec3> for Vec3{
    type Output = Vec3;

    fn sub(self, other:Vec3)->Vec3{
        Vec3::new(self.e[0]- other.e[0], self.e[1]- other.e[1], self.e[2]- other.e[2])
    }
}

impl ops::Mul<Vec3> for Vec3{
    type Output = Vec3;

    fn mul(self, _rhs:Vec3)->Vec3{
        Vec3::new(self.e[0]* _rhs.e[0], self.e[1]* _rhs.e[1], self.e[2]* _rhs.e[2])
    }
}
impl ops::Mul<Vec3> for f32{
    type Output = Vec3;

    fn mul(self, _rhs:Vec3)->Vec3{
        Vec3::new(self* _rhs.e[0], self* _rhs.e[1], self* _rhs.e[2])
    }
}

impl ops::Mul<f32> for Vec3{
    type Output = Vec3;

    fn mul(self, _rhs:f32)->Vec3{
        Vec3::new(self.e[0]* _rhs, self.e[1]* _rhs, self.e[2]* _rhs)
    }
}

impl ops::Div<Vec3> for Vec3{
    type Output = Vec3;

    fn div(self, _rhs:Vec3)->Vec3{
        Vec3::new(self.e[0]/ _rhs.e[0], self.e[1]/ _rhs.e[1], self.e[2]/ _rhs.e[2])
    }
}

impl ops::Div<f32> for Vec3{
    type Output = Vec3;

    fn div(self, _rhs:f32)->Vec3{
        Vec3::new(self.e[0]/ _rhs, self.e[1]/ _rhs, self.e[2]/ _rhs)
    }
}

impl ops::Div<Vec3> for f32{
    type Output = Vec3;

    fn div(self, _rhs:Vec3)->Vec3{
        Vec3::new(self/ _rhs.e[0], self/ _rhs.e[1], self/ _rhs.e[2])
    }
}

//assignment operators for addition, subtraction, multiplication and division

impl ops::AddAssign<Vec3> for Vec3{
    fn add_assign(&mut self, other:Vec3){
        self.e[0]+= other.e[0]; self.e[1]+= other.e[1]; self.e[2]+= other.e[2];
    }
}

impl ops::AddAssign<f32> for Vec3{
    fn add_assign(&mut self, other:f32){
        self.e[0]+=other; self.e[1]+=other; self.e[2]+=other;
    }
}

impl ops::SubAssign<f32> for Vec3{
    fn sub_assign(&mut self, other:f32){
        self.e[0]-=other; self.e[1]-=other; self.e[2]-=other;
    }
}

impl ops::MulAssign<Vec3> for Vec3{
    fn mul_assign(&mut self, other:Vec3){
        self.e[0]*=other.e[0]; self.e[1]*=other.e[1]; self.e[2]*=other.e[2];
    }
}

impl ops::MulAssign<f32> for Vec3{
    fn mul_assign(&mut self, other:f32){
        self.e[0]*=other; self.e[1]*=other; self.e[2]*=other;
    }
}

impl ops::DivAssign<Vec3> for Vec3{
    fn div_assign(&mut self, other:Vec3){
        self.e[0]/=other.e[0]; self.e[1]/=other.e[1]; self.e[2]/=other.e[2];
    }
}

impl ops::DivAssign<f32> for Vec3{
    fn div_assign(&mut self, other:f32){
        self.e[0]/=other; self.e[1]/=other; self.e[2]/=other;
    }
}
