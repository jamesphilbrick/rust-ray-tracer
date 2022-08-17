// vector functions

use std::vec;
// use std::num;
// use num_integer::roots::Roots;

pub fn vec3_add(v1: Vec<f64>, v2: Vec<f64>) -> Vec<f64> {
    let mut v3 = vec![];
    if v1.len() != v2.len() {
        println!("Vector lengths are not compatible!");
    } else {
        for i in 0..v1.len() {
            v3.push(v1[i] + v2[i]);
        }
    }
    return v3;
}

pub fn vec3_mult(v1: Vec<f64>, v2: Vec<f64>) -> Vec<f64> {
    let mut v3 = vec![];
    if v1.len() != v2.len() {
        println!("Vector lengths are not compatible!");
    } else {
        for i in 0..v1.len() {
            v3.push(v1[i] * v2[i]);
        }
    }
    return v3;
}

pub fn vec3_div(v: Vec<f64>, t: f64) -> Vec<f64> {
    let mut result = vec![];
    for i in 0..v.len() {
        result.push(v[i] / t);
    }
    return result;
}

pub fn vec3_mag(v: Vec<f64>) -> f64 {
    let mut result: f64 = 0.0;
    for i in 0..v.len() {
        result += v[i] * v[i];
    }
    result = result.sqrt();
    return result;
}

pub fn vec3_dot(v1: Vec<f64>, v2: Vec<f64>) -> f64 {
    let mut result: f64 = 0.0;
    for i in 0..v1.len() {
        result += v1[i] * v2[i];
    }
    return result;
}