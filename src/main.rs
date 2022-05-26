#![allow(dead_code)]
mod camera;
mod controller;
mod object;
mod ray;
mod vec3;

use controller::*;

fn main() {
    use std::fs;
    use std::time::*;

    let file = fs::File::options()
        .write(true)
        .open("output.ppm")
        .unwrap_or_else(|_| fs::File::create("output.ppm").unwrap());
    let mut file = std::io::BufWriter::new(file);

    //let (width, height) = (600, 338);
    //let (width, height) = (1920, 1080);
    let (width, height) = (2560, 1600);
    let mut ctl = Controller::new(width, height);
    let start = Instant::now();

    ctl.add_object(object::Sphere::new((0.0, 0.0, -1.0).into(), 0.5));
    ctl.add_object(object::Sphere::new((0.0, 100.5, -1.0).into(), 100.0));
    ctl.render(&mut file).expect("rendering");

    let end = Instant::now();
    let dur = end.duration_since(start);
    println!(
        "Render complete\ntime:\t{}ms\nfile:\toutput.txt\nres:\t{}x{}",
        dur.as_millis(),
        width,
        height
    );
}
