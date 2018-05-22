extern crate image;
use std::fs::File;
use image::GenericImage;

pub fn sobel_detector_test() {
    let img = image::open("test.jpg").unwrap();

    println!("dimensions: {:?}", img.dimensions());
    println!("color : {:?}", img.grayscale().color());

    let mut gray_im = img.grayscale();

    let filt_x: &[f32] = &[-1f32,0f32,1f32,-2f32,0f32,2f32,-1f32,0f32,1f32];
    let filt: &[f32] = &[-1f32,-2f32, -1f32, 0f32,0f32, 0f32, 1f32, 2f32,1f32];
    let gray_x = gray_im.filter3x3(filt_x);
    let gray_y = gray_im.filter3x3(filt_y);

    let mut sobeled_im = image::ImageBuffer::new(512, 512);

    for i in 0..512 {
        for j in 0..512 {
            sobeled_im.get_pixel_mut(i, j) = (gray_x.get_pixel(i, j).pow(2));
        }
    }

    gray_im.save("sobel.jpg");
}
