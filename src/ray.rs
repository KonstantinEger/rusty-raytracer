use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn origin(&self) -> Vec3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    
    pub fn at(&self, pos: f32) -> Vec3 {
        self.orig + (self.dir * pos)
    }
}

impl From<(Vec3, Vec3)> for Ray {
    fn from(val: (Vec3, Vec3)) -> Self {
        Self {
            orig: val.0,
            dir: val.1,
        }
    }
}
