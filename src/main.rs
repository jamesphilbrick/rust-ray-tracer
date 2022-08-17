/*
Ray tracer implimented in rust
James Philbrick, August 2022

I don't know any rust and this is very definately not a great
way to learn it...but that's not going to stop me from trying.

following along to the python tutorial found here: 
https://raytracing.github.io/books/RayTracingInOneWeekend.html

* image processing using image-rs: https://github.com/image-rs/image & https://stackoverflow.com/questions/29836804/how-do-i-procedurally-generate-images-using-rust-image
*/

#![allow(dead_code, unused)]
// this line to be removed once all the features (traits/methods?) of the Vec3 struct are utilised in the main body of code

extern crate image;
use std::mem::discriminant;

use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

// may be beneficial to read in the setup parameters from a json file
// image setup
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMG_WIDTH: u32 = 400;
const IMG_HEIGHT: u32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as u32;

// camera definition
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

fn ray_colour(r: Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    else {
        let unit_direction: Vec3 = Vec3::normalise(&r.direction);
        let t: f64 = 0.5 * (unit_direction.y + 1.0); 
        return Vec3::new(1.0, 1.0, 1.0)*(1.0 - t) + Vec3::new(0.5, 0.7, 1.0)*t;
    }
}

fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> bool {
    let oc: Vec3 = r.origin - center;
    let a: f64 = Vec3::dot(&r.direction, &r.direction);
    let b: f64 = Vec3::dot(&oc, &r.direction) * 2.0;
    let c: f64 = Vec3::dot(&oc, &oc) - (radius*radius);
    let discrim = b*b - a*c*4.0;
    return (discrim > 0.0);
}

fn render() {
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner: Vec3 = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    let mut image: RgbImage = ImageBuffer::new(IMG_WIDTH, IMG_HEIGHT);

    println!("** Lines rendered **");
    let bar = ProgressBar::new(IMG_HEIGHT.into());

    for j in (0..IMG_HEIGHT).rev() {
        for i in 0..IMG_WIDTH {

            let u: f64 = i as f64 / (IMG_WIDTH as f64 - 1.0);
            let v: f64 = j as f64 / (IMG_HEIGHT as f64 - 1.0);

            let r: Ray = Ray::new(origin, 
                lower_left_corner + horizontal*u + vertical*v - origin);
            let colour: Vec3 = ray_colour(r);

            *image.get_pixel_mut(i, IMG_HEIGHT - j - 1) = image::Rgb(Vec3::to_colour(&colour));
        } 
        bar.inc(1)
    }
    bar.finish();
    image.save("testImage.png").unwrap();
}

fn main() {


    // render
    render();
    // let v1 = Vec3::new(1.1, 2.2, 3.3);
    // let v2 = Vec3::new(3.3, 2.2, 1.1);
    // let v3 = Vec3::dot(&v1, &v2);
    // let v4 = Vec3::normalise(&v2);
    // println!("{}\n{}\n{}", Vec3::show(&v2), v3, Vec3::show(&v4));
    // println!("{:?}", Vec3::to_array(&v1));
}