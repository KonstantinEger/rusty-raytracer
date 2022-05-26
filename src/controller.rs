use std::io::Write;

use crate::camera::Camera;
use crate::object::Hittable;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Controller {
    camera: Camera,
    objects: Vec<Box<dyn Hittable>>,
}
impl Controller {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            camera: Camera::new(width, height),
            objects: vec![],
        }
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

    pub fn add_object(&mut self, obj: impl Hittable + 'static) {
        self.objects.push(Box::new(obj));
    }

    pub fn write_color(&mut self, output: &mut dyn Write, color: &Vec3) -> std::io::Result<()> {
        let r = (color.x() * 255.0) as usize;
        let g = (color.y() * 255.0) as usize;
        let b = (color.z() * 255.0) as usize;
        self.write_pixel(output, r, g, b)?;
        Ok(())
    }

    pub fn write_pixel(&mut self, output: &mut dyn Write, r: usize, g: usize, b: usize) -> std::io::Result<()> {
        let _ = output.write(r.to_string().as_bytes())?;
        let _ = output.write(b" ")?;
        let _ = output.write(g.to_string().as_bytes())?;
        let _ = output.write(b" ")?;
        let _ = output.write(b.to_string().as_bytes())?;
        let _ = output.write(b" \n")?;
        Ok(())
    }

    pub fn render(&mut self, output: &mut dyn Write) -> std::io::Result<()> {
        let width = self.width() as f32;
        let height = self.height() as f32;
        let _ = output.write(format!("P3\n{} {}\n255\n", width, height).as_bytes())?;
        for j in (0..self.height()).rev() {
            for i in 0..self.width() {
                let u = (i as f32) / ((width - 1.0) as f32);
                let v = (j as f32) / ((height - 1.0) as f32);
                let ray = {
                    let cam = self.camera();
                    let o = cam.origin();
                    let dir = cam.llc() + cam.horizontal() * u + cam.vertical() * v - cam.origin();
                    Ray::from((o, dir))
                };

                let mut hit = None;
                let mut t_closest = f32::MAX;
                for obj in self.objects.iter() {
                    if let Some(h) = obj.hit_by(&ray, 0.0, t_closest) {
                        t_closest = h.t;
                        hit = Some(h);
                    }
                }

                if let Some(hit) = hit {
                    let unit_normal = hit.normal.unit();
                    let color = (unit_normal + Vec3::from((1.0, 1.0, 1.0))) * 0.5;
                    self.write_color(output, &color)?;
                } else {
                    self.write_color(output, &(0.2, 0.2, 0.2).into())?;
                }
            }
        }
        Ok(())
    }
}
