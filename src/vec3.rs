#[derive(Clone, Copy)]
pub struct Vec3(f32, f32, f32);

impl Vec3 {
    #[inline]
    pub fn x(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.1
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.2
    }

    #[inline]
    pub fn dot(self, rhs: &Vec3) -> f32 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    #[inline]
    pub fn cross(self, rhs: &Vec3) -> Vec3 {
        Self(
            self.y()*rhs.z() - self.z()*rhs.y(),
            self.z()*rhs.x() - self.x()*rhs.z(),
            self.x()*rhs.y() - self.y()*rhs.x()
        )
    }

    #[inline]
    pub fn unit(self) -> Vec3 {
        self / self.len()
    }

    #[inline]
    pub fn len(&self) -> f32 {
        self.len_sqr().sqrt()
    }

    #[inline]
    pub fn len_sqr(&self) -> f32 {
        self.0*self.0 + self.1*self.1 + self.2*self.2
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, val: f32) -> Vec3 {
        Vec3(self.0 * val, self.1 * val, self.2 * val)
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, val: f32) -> Vec3 {
        self * (1.0 / val)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0+rhs.x(), self.1+rhs.y(), self.2+rhs.z())
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        self + (-rhs)
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.x();
        self.1 += rhs.y();
        self.2 += rhs.z();
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(val: (f32, f32, f32)) -> Self {
        Self(val.0, val.1, val.2)
    }
}
