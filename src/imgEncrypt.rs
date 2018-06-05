extern crate image;
use image::{GenericImage, Pixel};
use std::f64;


/// Takes a vector of angles to do many encryptions on the image saves to the given directory.
/// As you add more numbers the image will be harder to recognize when decrypted.
/// Panics if the given vector of degrees is empty.
/// Filename: &str, export_directory: &str, degrees Vec<u32> -> puts image into given directory as
/// encrImg.jpg
pub fn encrypt_many (filename: &str, export_directory: &str, mut degrees: Vec<u32>){
    if degrees.len()==0{
        panic!("Need at least one degree");
    }
    let first_degree = degrees.pop().unwrap();
    let export = export_directory.to_string()+"\\encrImg.jpg";
    encrypt(filename,export_directory,first_degree);
    for degree in degrees{
        encrypt(&export,export_directory,degree);
    }
}

/// Takes a vector of angles to try and reverse many encryptions to go back to the original saves
/// it to the given directory. Panics if the given vector of degrees is empty.
/// Filename: &str, export_directory: &str, degrees Vec<u32> -> puts image into directory as
/// decrImg.jpg
pub fn decrypt_many (filename: &str, export_directory: &str, mut degrees : Vec<u32>) {
    if degrees.len()==0{
        panic!("Need at least one degree");
    }
    let first_degree = degrees.pop().unwrap();
    let export = export_directory.to_string()+"\\decrImg.jpg";
    decrypt(filename,export_directory,first_degree);
    for degree in degrees{
        decrypt(&export,export_directory,degree);
    }
}

/// Encrypts the given image with the given angle of swirl in order to disguise image
/// saves the image in the given directory
/// Filename: &str, export_directory: &str, degrees u32 -> puts image into directory as encrImg.jpg
pub fn encrypt(filename: &str, export_directory: &str, degrees: u32){
    let img = image::open(filename).unwrap();
    let (dim_x, dim_y) = img.dimensions();
    let x_mid =  (dim_x - 1) / 2;
    let y_mid =  (dim_y - 1) / 2;
    let mut new_img = image::ImageBuffer::new(512, 512);

    for i in 0..dim_x {
         for j in 0..dim_y {
             let temp_x = i as f64 - x_mid as f64;
             let temp_y = j as f64 - y_mid as f64;

             let square = (temp_x * temp_x + temp_y * temp_y) as f64;
             let radius = square.sqrt();
             let trans_angle = f64::consts::PI / degrees as f64 * radius;
             let mut final_x = (temp_x * trans_angle.cos() - temp_y * trans_angle.sin() + x_mid as f64) as u32;
             let mut final_y = (temp_x * trans_angle.sin() + temp_y * trans_angle.cos() + y_mid as f64) as u32;
             if final_x >= dim_x{
                 final_x=dim_x - 1;
             }
             if final_x < 0 {
                 final_x = 0;
             }
             if final_y >= dim_y{
                 final_y=dim_y - 1;
             }
             if final_y < 0 {
                 final_y = 0;
             }
             new_img.put_pixel(i,j,img.get_pixel(final_x, final_y));

            }
    }
    let export = export_directory.to_string()+"\\encrImg.jpg";

    new_img.save(export);
}


/// Decrypts the given image with the given swirl in order to restore the orignial image
/// saves to the given directory
/// Filename: &str, export_directory: &str, degrees u32 -> puts image into directory as decrImg.jpg
pub fn decrypt(filename: &str, export_directory: &str, degrees: u32){
    let img = image::open(filename).unwrap();
    let (dim_x, dim_y) = img.dimensions();
    let x_mid =  (dim_x - 1) / 2;
    let y_mid =  (dim_y - 1) / 2;
    let mut new_img = image::ImageBuffer::new(512, 512);

    for i in 0..dim_x {
        for j in 0..dim_y {
            let temp_x = i as f64 - x_mid as f64;
            let temp_y = j as f64 - y_mid as f64;

            let square = (temp_x * temp_x + temp_y * temp_y) as f64;
            let radius = square.sqrt();
            let trans_angle = f64::consts::PI / degrees as f64 * radius;
            let mut final_x = (temp_x * trans_angle.cos() - temp_y * trans_angle.sin() + x_mid as f64) as u32;
            let mut final_y = (temp_x * trans_angle.sin() + temp_y * trans_angle.cos() + y_mid as f64) as u32;
            if final_x >= dim_x{
                final_x=dim_x - 1;
            }
            if final_x < 0 {
                final_x = 0;
            }
            if final_y >= dim_y{
                final_y=dim_y - 1;
            }
            if final_y < 0 {
                final_y = 0;
            }
            new_img.put_pixel(final_x,final_y,img.get_pixel(i, j));

        }
    }
    let export = export_directory.to_string()+"\\decrImg.jpg";
    new_img.save(export);
}