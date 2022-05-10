#![allow(dead_code)]
mod controller;
mod vec3;
mod ray;
mod camera;
mod object;

use controller::*;

fn main() {
    use std::io::prelude::*;
    use std::fs;
    use std::time::*;

    let mut file = fs::File::options()
        .write(true)
        .open("output.ppm")
        .unwrap_or_else(|_| fs::File::create("output.ppm").unwrap());

    //let (width, height) = (600, 338);
    //let (width, height) = (1920, 1080);
    let (width, height) = (2560, 1600);
    let mut ctl = Controller::new(width, height);
    let start = Instant::now();


    ctl.add_object(object::Sphere::new((0.0, 0.0, -1.0).into(), 0.5));
    ctl.add_object(object::Sphere::new((0.0, 100.5, -1.0).into(), 100.0));
    ctl.render();


    let end = Instant::now();
    let dur = end.duration_since(start);
    println!("Render complete\ntime:\t{}ms\nfile:\toutput.txt\nres:\t{}x{}", dur.as_millis(), width, height);

    file.write(ctl.output()).expect("writing to file");
}

