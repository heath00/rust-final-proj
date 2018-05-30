extern crate image;
use image::{GenericImage, Pixel};
use std::fs::read_dir;

extern crate nalgebra as na;
use self::na::{Dynamic, MatrixArray, MatrixVec, DMatrix, DVector,};

pub fn handler() {
    let mut my_mat = create_column_vecs_na();
    let mean_vec = calculate_mean_vec_na(&my_mat);
    reduce_matrix_na(&mut my_mat, mean_vec);

    let cov_mat_small = get_covariant_mat_small(my_mat);
//    let cov_mat_large = get_covariant_mat_large(my_mat);

    let svd_mat = na::linalg::SVD::new(cov_mat_small, true, true);
    let eigen_vals_pwr_N_hlf = svd_mat.clone().v_t.unwrap().map(|v| v.powf(-0.5));

    println!("{:?}", svd_mat.v_t);
//    for i in 0..eigen_vals_pwr_N_hlf.nrows() {
//        let mut this_row = actual_eigens.column_mut(i);
//
//    }



}


fn get_covariant_mat_large(inp: DMatrix<f32>) -> DMatrix<f32> {
    let transposed: DMatrix<f32> = inp.transpose();
    inp * transposed
}

fn get_covariant_mat_small(inp: DMatrix<f32>) -> DMatrix<f32> {
    let transposed: DMatrix<f32> = inp.transpose();
    transposed * inp
}

fn img_to_1d(filename: &str, is_grayscale: bool) -> Vec<f32> {
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

    vec_1d
}

fn create_column_vecs_na() -> DMatrix<f32> {

    let paths = read_dir("eigen_training").unwrap();

    let mut col_vec = Vec::new();

    let mut first_im = false;
    let mut im_size= 0;
    let mut num_ims =  0;

    for item in  paths {
        col_vec.append(&mut img_to_1d(item.unwrap().path().display().to_string().as_str(), false));
        num_ims += 1;

        if !first_im {
            im_size += col_vec.len();
            first_im = true;
        }
    }

    let row_slice: &[f32] = &col_vec;
    DMatrix::from_row_slice(im_size, num_ims, row_slice)
}

fn calculate_mean_vec_na(inp: &DMatrix<f32>) -> DVector<f32> {

    let mut mean_vec = Vec::with_capacity(inp.nrows());
    let num_images = inp.len() as f32;

    for i in 0..inp.nrows() {
        let mut running_sum = 0f32;
        for j in 0..inp.ncols() {
            running_sum += inp[(i,j)];
        }

        let mean_val: f32 = running_sum / num_images;
        mean_vec.push(mean_val.ceil());
    }

    let row_slice: &[f32] = &mean_vec;
    DVector::from_row_slice(mean_vec.len(), row_slice)

}

fn reduce_matrix_na(inp_mat: &mut DMatrix<f32>, mean_vec: DVector<f32>) {
    for i in 0..inp_mat.ncols() {
        let mut this_col = inp_mat.column_mut(i);
        this_col -= &mean_vec;
    }
}

























pub fn create_column_vecs() -> Vec<Vec<f32>> {

    let paths = read_dir("eigen_training").unwrap();

    let mut col_vec = Vec::new();

    for item in  paths {
        col_vec.push(img_to_1d(item.unwrap().path().display().to_string().as_str(), false));
    }

    col_vec
}

pub fn calculate_mean_vec(inp: Vec<Vec<f32>>) -> Vec<f32> {

    let mut mean_vec = Vec::with_capacity(inp[0].len());
    let num_images = inp.len() as f32;

    for i in 0..inp[0].len() {
        let mut running_sum = 0f32;
        for j in 0..inp.len() {
            running_sum += inp[j][i] as f32;
        }

        let mean_val: f32 = running_sum / num_images;
        mean_vec.push(mean_val.ceil());
    }

    mean_vec
}


pub fn reduce_matrix(inp_vec: &mut Vec<Vec<f32>>, mean_vec: Vec<f32>) {
    for i in 0..inp_vec[0].len() {
        for j in 0..inp_vec.len() {
            inp_vec[j][i] -= mean_vec[i];
        }
    }
}
