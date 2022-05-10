use crate::vec3::Vec3;
use crate::camera::Camera;

pub struct Controller {
    camera: Camera,
    output: Vec<u8>,
}
impl Controller {
    pub fn new(width: usize, height: usize) -> Self {
        let mut res = Self {
            camera: Camera::new(width, height),
            output: Vec::with_capacity(width * height * 12 + 20),
        };
        res.output.extend_from_slice(&format!("P3\n{} {}\n255\n", width, height).as_bytes());
        res
    }

    #[inline(always)]
    pub fn width(&self) -> usize {
        self.camera.image_width()
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        self.camera.image_height()
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn write_color(&mut self, color: &Vec3) {
        let r = (color.x() * 255.0) as usize;
        let g = (color.y() * 255.0) as usize;
        let b = (color.z() * 255.0) as usize;
        self.write_pixel(r, g, b);
    }

    pub fn write_pixel(&mut self, r: usize, g: usize, b: usize) {
        self.output.extend_from_slice(r.to_string().as_bytes());
        self.output.push(b' ');
        self.output.extend_from_slice(g.to_string().as_bytes());
        self.output.push(b' ');
        self.output.extend_from_slice(b.to_string().as_bytes());
        self.output.push(b'\n');
    }

    pub fn output(&self) -> &[u8] {
        &self.output
    }
}
