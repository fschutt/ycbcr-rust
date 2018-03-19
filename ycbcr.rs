#![feature(stdsimd)]

use std::simd::{f32x4};

pub fn clamp(val: f32) -> u8 {
    if val < 0.0 { 
        0
    } else if val > 255.0 { 
        255 
    } else { 
        val.round() as u8 
    }
}

pub fn rgb_to_ycbcr((r, g, b): (u8, u8, u8)) -> (u8, u8, u8) {
    let rgb = f32x4::new(r as f32, g as f32, b as f32, 1.0);
    let y  = (rgb * f32x4::new( 0.299000,  0.587000,  0.114000,   0.0)).sum();
    let cb = (rgb * f32x4::new(-0.168736, -0.331264,  0.500000, 128.0)).sum();
    let cr = (rgb * f32x4::new( 0.500000, -0.418688, -0.081312, 128.0)).sum();

    (clamp(y), clamp(cb), clamp(cr))
}

pub fn ycbcr_to_rgb((y, cb, cr): (u8, u8, u8)) -> (u8, u8, u8) {
    let ycbcr = f32x4::new(y as f32, cb as f32 - 128.0f32, cr as f32 - 128.0f32, 0.0);
    let r = (ycbcr * f32x4::new(1.0,  0.00000,  1.40200, 0.0)).sum();
    let g = (ycbcr * f32x4::new(1.0, -0.34414, -0.71414, 0.0)).sum();
    let b = (ycbcr * f32x4::new(1.0,  1.77200,  0.00000, 0.0)).sum();

    (clamp(r), clamp(g), clamp(b))
}

fn main() {
    let mut rgb = (0, 71, 16);
    println!("{:?}", rgb);

    for _ in 0..100 {
        let yuv = rgb_to_ycbcr(rgb);
        rgb = ycbcr_to_rgb(yuv);

        println!("{:?}", rgb);
    }
}
