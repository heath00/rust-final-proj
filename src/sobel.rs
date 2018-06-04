extern crate image;
use image::{GenericImage, Pixel};


pub fn sobel_detector(filename: &str, export_directory: &str, threshold: u8) {
    let img = image::open(filename).unwrap();


    let (dim_x, dim_y) = img.dimensions();

    let gray_im = img.grayscale();

    let filt_x: &[f32] = &[-1f32,0f32,1f32,-2f32,0f32,2f32,-1f32,0f32,1f32];
    let filt_y: &[f32] = &[-1f32,-2f32, -1f32, 0f32,0f32, 0f32, 1f32, 2f32,1f32];
    let gray_x = gray_im.filter3x3(filt_x);
    let gray_y = gray_im.filter3x3(filt_y);

    let mut sobeled_im = image::ImageBuffer::new(dim_x, dim_y);


    for i in 0..dim_x {
        for j in 0..dim_y {
            let mut px1_sq = gray_x.get_pixel(i, j).channels()[0] as f32;
            let mut px2_sq = gray_y.get_pixel(i, j).channels()[0] as f32;

            px1_sq = px1_sq.powf(2.);
            px2_sq = px2_sq.powf(2.);

            let new_val = (px1_sq + px2_sq).sqrt().ceil() as u8;
            let new_px = gray_x.get_pixel(i, j).map(|_v| if new_val > threshold {255} else {0});
            sobeled_im.put_pixel(i, j, new_px);
        }
    }

    let export = export_directory.to_string();
    let borrowed_x: &str = "\\grey_x.jpg";
    let borrowed_y: &str = "\\grey_y.jpg";
    let borrowed_sobel: &str = "\\sobel.jpg";
    let export_x = export.clone() + borrowed_x;
    let export_y = export.clone() + borrowed_y;
    let export_sobel = export.clone() + borrowed_sobel;

    gray_x.save(export_x);
    gray_y.save(export_y);
    sobeled_im.save(export_sobel);


    
}

