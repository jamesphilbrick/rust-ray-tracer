// Vec3 class - a useful way to store and manipulate vectors and colours

use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {

    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const ONES: Vec3 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { 
            x: x, 
            y: y, 
            z: z,
        }
    }

    pub fn show(v: &Vec3) -> String {
        return format!("vec3[{}, {}, {}]", v.x, v.y, v.z);
    }

    pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
        return a.x * b.x + a.y * b.y + a.z * b.z;
    }

    pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub fn magnitude(v: &Vec3) -> f64 {
        return (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    }

    pub fn normalise(v: &Vec3) -> Vec3 {
        // let mag = Vec3::magnitude(&v).abs();
        let mag = Vec3::magnitude(&v);
        Vec3 {
            x: v.x / mag,
            y: v.y / mag,
            z: v.z / mag,
        }
    
    }

    pub fn to_array(v: &Vec3) -> [f64; 3] {
        [v.x, v.y, v.z]
    }

    pub fn to_colour(v: &Vec3) -> [u8; 3] {
        [(225.999 * v.x) as u8, 
        (225.999 * v.y) as u8, 
        (225.999 * v.z )as u8]
    }
}

// impliment operator overloading to define behaviour for basic arithmetic operators
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}