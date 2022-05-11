use crate::vec3::Vec3;

pub struct Camera {
    aspect_ratio: f32,
    image_dim: (usize, usize),
    viewport_dim: (f32, f32),
    focal_len: f32,
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(width: usize, height: usize) -> Self {
        let aspect_ratio = (width as f32) / (height as f32);
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_len = 1.0;
        let origin = Vec3::from((0.0, 0.0, 0.0));
        let horizontal = Vec3::from((viewport_width, 0.0, 0.0));
        let vertical = Vec3::from((0.0, viewport_height, 0.0));
        let lower_left_corner =
            origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::from((0.0, 0.0, focal_len));

        Self {
            aspect_ratio,
            image_dim: (width, height),
            viewport_dim: (viewport_width, viewport_height),
            focal_len,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn image_width(&self) -> usize {
        self.image_dim.0
    }

    pub fn image_height(&self) -> usize {
        self.image_dim.1
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn llc(&self) -> Vec3 {
        self.lower_left_corner
    }

    pub fn horizontal(&self) -> Vec3 {
        self.horizontal
    }
    pub fn vertical(&self) -> Vec3 {
        self.vertical
    }
}
