extern crate image;
extern crate nalgebra as na;
use image::{GenericImage, Pixel};
use std::fs::read_dir;

use self::na::{Dynamic, MatrixArray, MatrixVec, DMatrix, DVector};

pub fn img_to_1d_na(filename: &str, is_grayscale: bool) -> DVector<f32> {
    let mut img = image::open(filename).unwrap();
    let (x,y) = img.dimensions();
    if (!is_grayscale) {
        img = img.grayscale();
    }

    let mut vec_1d = Vec::with_capacity((x * y) as usize);

    for i in 0..y {
        for j in 0..x {
            vec_1d.push(img.get_pixel(i,j).channels()[0] as f32);
        }
    }

    let row_slice: &[f32] = &vec_1d;
    DVector::from_row_slice(vec_1d.len(), row_slice)
}

