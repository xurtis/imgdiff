extern crate image;
extern crate rayon;

use std::process::exit;
use std::str::FromStr;
use std::env::args;
use rayon::prelude::*;
use image::Rgb;

const NUM_PIXELS: u32 = 1920 * 1080;
const MIN_DELTA: u32 = 48;

fn main() {
    let mut args = args();
    args.next();
    let l_path = args.next().expect("Need 2 filenames");
    let r_path = args.next().expect("Need 2 filenames");
    let pixel_proportion = args.next().expect("Need pixel proportion");
    let max_diff_pixels = ((NUM_PIXELS as f32) * f32::from_str(&pixel_proportion).unwrap()) as u32;

    let left = image::open(&l_path).expect("Could not open first image");
    let right = image::open(&r_path).expect("Could not open second image");

    let left_pixels = left.as_rgb8().unwrap().enumerate_pixels();
    let right_pixels = right.as_rgb8().unwrap().enumerate_pixels();

    let pairs = left_pixels.zip(right_pixels)
        .map(|((_, _, l), (_, _, r))| (l, r))
        .collect::<Vec<(&Rgb<u8>, &Rgb<u8>)>>();

    let diff_pixels = pairs.into_par_iter()
        .map(|(l, r)| pixel_diff(l, r))
        .fold_with(0, |count, delta| {
            if delta > MIN_DELTA {
                count + 1
            } else {
                count
            }
        })
        .sum::<u32>();

    if diff_pixels < max_diff_pixels {
        exit(1);
    }
}

fn pixel_diff(left: &Rgb<u8>, right: &Rgb<u8>) -> u32 {
    let red = (left.data[0] as i32 - right.data[0] as i32 ).abs();
    let green = (left.data[1] as i32 - right.data[1] as i32 ).abs();
    let blue = (left.data[2] as i32 - right.data[2] as i32 ).abs();
    let sqaredist = ((red * red) + (green * green) + (blue * blue)) as u32;

    let mut root = 0u32;
    let mut flag = 1u32 << 10;
    while flag > 0 {
        root |= flag;
        if (root * root) > sqaredist {
            root &= !flag;
        }
        flag >>= 1;
    }

    return root;
}
