#![allow(dead_code)]
mod controller;
mod vec3;
mod ray;
mod camera;

use controller::*;

fn main() {
    use std::io::prelude::*;
    use std::fs;
    use std::time::*;

    let mut file = fs::File::options()
        .write(true)
        .open("output.ppm")
        .unwrap_or_else(|_| fs::File::create("output.ppm").unwrap());

    let (width, height) = (500, 250);
    let mut ctl = Controller::new(width, height);
    let start = Instant::now();



    render(&mut ctl);



    let end = Instant::now();
    let dur = end.duration_since(start);
    println!("Render complete\ntime:\t{}ms\nfile:\toutput.txt\nres:\t{}x{}", dur.as_millis(), width, height);

    file.write(ctl.output()).expect("writing to file");
}

fn render(ctl: &mut Controller) {
    let width = ctl.width() as f32;
    let height = ctl.height() as f32;
    for j in (0..ctl.height()).rev() {
        for i in 0..ctl.width() {
            let u = (i as f32) / ((width - 1.0) as f32);
            let v = (j as f32) / ((height - 1.0) as f32);
            let ray = {
                let cam = ctl.camera();
                let o = cam.origin();
                let dir = cam.llc() + cam.horizontal()*u + cam.vertical()*v - cam.origin();
                ray::Ray::from((o, dir))
            };
            let color = ray_color(&ray);
            ctl.write_color(&color);
        }
    }
}

fn ray_color(ray: &ray::Ray) -> vec3::Vec3 {
    let dir_unit = ray.direction().unit();
    let t = 0.5 * (dir_unit.y() + 1.0);
    vec3::Vec3::from((1.0, 1.0, 1.0))*(1.0-t) + vec3::Vec3::from((0.5, 0.7, 1.0))*t
}

