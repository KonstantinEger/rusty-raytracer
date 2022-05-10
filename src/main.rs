#![allow(dead_code)]
mod controller;
mod vec3;

use controller::*;
use vec3::*;

fn main() {
    use std::io::prelude::*;
    use std::fs;
    use std::time::*;

    let mut file = fs::File::options()
        .write(true)
        .open("output.ppm")
        .unwrap_or_else(|_| fs::File::create("output.ppm").unwrap());

    let (width, height) = (500, 500);
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
            let color = (i as f32 / width, j as f32 / height, 0.25).into();
            ctl.write_color(&color);
        }
    }
}




#[derive(Clone, Copy)]
struct Ray {
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
        &self.orig + &(&self.dir * pos)
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

