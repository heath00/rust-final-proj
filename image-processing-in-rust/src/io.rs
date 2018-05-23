extern crate image;
extern crate rust_im;

use image::{GenericImage, ImageBuffer};
use std::io::File;
use dimensions::ImageInfo;

fn read_image(&str: filePath){

    // Read in Image
    let mut img = image::open(filePath).ok().expect("Opening image failed");
    img = img.grayscale();
    /*let info = ImageInfo{
        data: img.raw_pixels(),
        width: 0,
        height: 0,
    };*/

}