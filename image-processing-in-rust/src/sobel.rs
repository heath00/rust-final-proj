extern crate image;
use image::{GenericImage, Pixel};

pub fn sobel_detector(filename: &str, threshold: u8) {
    let img = image::open(filename).unwrap();

    let (dim_x, dim_y) = img.dimensions();

    let gray_im = img.grayscale();

    let filt_x: &[f32] = &[-1f32,0f32,1f32,-2f32,0f32,2f32,-1f32,0f32,1f32];
    let filt_y: &[f32] = &[-1f32,-2f32, -1f32, 0f32,0f32, 0f32, 1f32, 2f32,1f32];
    let gray_x = gray_im.filter3x3(filt_x);
    let gray_y = gray_im.filter3x3(filt_y);

    let mut sobeled_im = image::ImageBuffer::new(512, 512);


    for i in 0..dim_x {
        for j in 0..dim_y {
            let mut px1_sq = gray_x.get_pixel(i, j).channels()[0] as f32;
            let mut px2_sq = gray_y.get_pixel(i, j).channels()[0] as f32;

            px1_sq = px1_sq.powf(2.);
            px2_sq = px2_sq.powf(2.);

            let new_val = (px1_sq + px2_sq).sqrt().ceil() as u8;
            let new_px = gray_x.get_pixel(i, j).map(|_v| if new_val > threshold {new_val} else {0});
            sobeled_im.put_pixel(i, j, new_px);
        }
    }

    gray_x.save("gray_x.jpg");
    gray_y.save("gray_y.jpg");
    sobeled_im.save("sobel.jpg");
    
}
