extern crate image;
use image::{GenericImage, Pixel, Luma};
use std::fs::{read_dir, File};
use std::path::Path;

extern crate nalgebra as na;
use self::na::{Dynamic, MatrixArray, MatrixVec, DMatrix, DVector,};


pub fn handler() {
    let mut my_mat = create_column_vecs_na();
    let mean_vec = calculate_mean_vec_na(&my_mat);
    reduce_matrix_na(&mut my_mat, mean_vec);

    let cov_mat_small = get_covariant_mat_small(&my_mat);

    let svd_mat = na::linalg::SVD::new(cov_mat_small, true, true);
    let mut eigen_vals_pwr_n_hlf = svd_mat.clone().singular_values.map(|v| v.powf(-0.5));
//    println!("{}",eigen_vals_pwr_N_hlf);

    let eigenvector_v = svd_mat.v_t.unwrap().clone();

    let mut mat_holder = Vec::with_capacity(my_mat.nrows() * my_mat.ncols());
    for i in 0..eigenvector_v.ncols() {
//        println!("{} x {} vs {} x {}", my_mat.nrows(), my_mat.ncols(), eigenvectorV.column(i).nrows(), eigenvectorV.column(i).ncols());
        let mut this_vec = (&my_mat * eigenvector_v.column(i)).to_owned();
        this_vec = this_vec * eigen_vals_pwr_n_hlf[i];
//        println!("{} x {} vs {} x {}", this_vec.nrows(), this_vec.ncols(), eigen_vals_pwr_N_hlf.nrows(), eigen_vals_pwr_N_hlf.ncols());
//        println!("{}", Vec::from(this_vec).len());
//        println!("{:?}", this_vec);
        mat_holder.append(&mut Vec::from(this_vec.as_slice()));
    }

    let mat_holder_as_slice: &[f64] = &mat_holder;
    let mut eigenvector_u = DMatrix::from_row_slice(my_mat.nrows(), my_mat.ncols(), mat_holder_as_slice);

    normalize_u(&mut eigenvector_u);
    create_buffer(&eigenvector_u);

}



fn normalize_u(u: &mut DMatrix<f64>) {

    // keeps track of mins and maxs while we iterate through every value to get sum_of_sq
    let mut mins = Vec::with_capacity(u.ncols());
    let mut maxs = Vec::with_capacity(u.ncols());

    for i in 0..u.ncols() {
        let mut this_min = u[(0, i)];
        let mut this_max = u[(0, i)];

        let mut sum_of_sq = 0f64;

        for j in 0..u.nrows() {
            sum_of_sq += u[(j, i)];

            if this_min > u[(j, i)] {this_min = u[(j,i)]}
            else if this_max < u[(j,i)] {this_max = u[(j,i)]}
        }
        sum_of_sq = sum_of_sq.sqrt();
        let mut this_col = u.column_mut(i);
        this_col /= sum_of_sq;
        mins.push(this_min / sum_of_sq);
        maxs.push(this_max / sum_of_sq);
    }

    for i in 0..u.ncols() {
        for j in 0..u.nrows() {
            u[(j,i)] = 255f64 * ((u[(j,i)] - mins[i]) / (maxs[i] - mins[i]));
            u[(j,i)] = u[(j,i)].floor();
        }
    }

}

fn create_buffer(u: &DMatrix<f64>) {


    let mut luma_holder = Vec::with_capacity(u.nrows());

    for i in 0..u.nrows() {
        luma_holder.push(Luma([u[(i, 0)] as u8]));
        if i < 512 {
            println!("{}", u[(i, 0)]);
        }
    }

    for i in 0..luma_holder.len() {
//        println!("{}", luma_holder[i].channels()[0]);
    }


    let mut buf = image::ImageBuffer::new(64, 64);


    let mut count = 0;
    for i in 0..64 {
        for j in 0..64 {
            buf.put_pixel(j, i, luma_holder[count]);
            count += 1;
        }
    }



    let mut fout = File::create("gen.png").unwrap();


    image::ImageLuma8(buf).save("gen.png").unwrap();


}


fn get_covariant_mat_large(inp: DMatrix<f64>) -> DMatrix<f64> {
    let transposed: DMatrix<f64> = inp.transpose();
    inp * transposed
}

fn get_covariant_mat_small(inp: &DMatrix<f64>) -> DMatrix<f64> {
    let transposed: DMatrix<f64> = inp.transpose();
    transposed * inp
}

fn img_to_1d(filename: &str, is_grayscale: bool) -> Vec<f64> {
    let mut img = image::open(filename).unwrap();
    let (x,y) = img.dimensions();

    if (!is_grayscale) {
        img = img.grayscale();
    }


    let mut vec_1d = Vec::with_capacity((x * y) as usize);


    for i in 0..y {
        for j in 0..x {
            vec_1d.push(img.get_pixel(i,j).channels()[0] as f64);
        }
    }

    println!("{}", vec_1d.len());
    vec_1d
}

fn create_column_vecs_na() -> DMatrix<f64> {

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

    let row_slice: &[f64] = &col_vec;
    DMatrix::from_row_slice(im_size, num_ims, row_slice)
}

fn calculate_mean_vec_na(inp: &DMatrix<f64>) -> DVector<f64> {

    let mut mean_vec = Vec::with_capacity(inp.nrows());
    let num_images = inp.len() as f64;

    for i in 0..inp.nrows() {
        let mut running_sum = 0f64;
        for j in 0..inp.ncols() {
            running_sum += inp[(i,j)];
        }

        let mean_val: f64 = running_sum / num_images;
        mean_vec.push(mean_val.ceil());
    }

    let row_slice: &[f64] = &mean_vec;
    DVector::from_row_slice(mean_vec.len(), row_slice)

}

fn reduce_matrix_na(inp_mat: &mut DMatrix<f64>, mean_vec: DVector<f64>) {
    for i in 0..inp_mat.ncols() {
        let mut this_col = inp_mat.column_mut(i);
        this_col -= &mean_vec;
    }
}

























pub fn create_column_vecs() -> Vec<Vec<f64>> {

    let paths = read_dir("eigen_training").unwrap();

    let mut col_vec = Vec::new();

    for item in  paths {
        col_vec.push(img_to_1d(item.unwrap().path().display().to_string().as_str(), false));
    }

    col_vec
}

pub fn calculate_mean_vec(inp: Vec<Vec<f64>>) -> Vec<f64> {

    let mut mean_vec = Vec::with_capacity(inp[0].len());
    let num_images = inp.len() as f64;

    for i in 0..inp[0].len() {
        let mut running_sum = 0f64;
        for j in 0..inp.len() {
            running_sum += inp[j][i] as f64;
        }

        let mean_val: f64 = running_sum / num_images;
        mean_vec.push(mean_val.ceil());
    }

    mean_vec
}


pub fn reduce_matrix(inp_vec: &mut Vec<Vec<f64>>, mean_vec: Vec<f64>) {
    for i in 0..inp_vec[0].len() {
        for j in 0..inp_vec.len() {
            inp_vec[j][i] -= mean_vec[i];
        }
    }
}
