use opencv::core::{Mat, Size};
use opencv::imgcodecs::{imread, imwrite, IMREAD_UNCHANGED};
use opencv::imgproc::{cvt_color, create_clahe, COLOR_BGR2Lab, COLOR_Lab2BGR};
use opencv::prelude::*;
use opencv::types::VectorOfMat;

use std::env;
use std::process;

fn main() -> opencv::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_image_path> <output_image_path>", args[0]);
        process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    
    let img = imread(input_path, IMREAD_UNCHANGED)?;

    if img.empty() {
        eprintln!("Error: Could not read the image at {}", input_path);
        process::exit(1);
    }

    
    let img_type = img.typ();
    let depth = opencv::core::CV_MAT_DEPTH(img_type);
    if depth != opencv::core::CV_16U && depth != opencv::core::CV_8U {
        eprintln!("Unsupported image depth for CLAHE. Converting to 16-bit unsigned integer.");
        
        let mut img_16u = Mat::default();
        img.convert_to(&mut img_16u, opencv::core::CV_16U, 65535.0, 0.0)?;
        process_image(&img_16u, output_path)?;
    } else {
        process_image(&img, output_path)?;
    }

    println!("CLAHE applied successfully. Image saved at {}", output_path);

    Ok(())
}

fn process_image(img: &Mat, output_path: &str) -> opencv::Result<()> {
    
    let mut lab_image = Mat::zeros(img.rows(), img.cols(), img.typ())?.to_mat()?;
    cvt_color(&img, &mut lab_image, COLOR_BGR2Lab, 0)?;

    
    let mut lab_planes = VectorOfMat::new();
    opencv::core::split(&lab_image, &mut lab_planes)?;

    
    let mut clahe = create_clahe(2.0, Size::new(8, 8))?;
    let l_channel = lab_planes.get(0)?;
    let mut clahe_l = Mat::zeros(l_channel.rows(), l_channel.cols(), l_channel.typ())?.to_mat()?;
    clahe.apply(&l_channel, &mut clahe_l)?;

    
    lab_planes.set(0, clahe_l)?;
    let mut merged_lab = Mat::zeros(lab_image.rows(), lab_image.cols(), lab_image.typ())?.to_mat()?;
    opencv::core::merge(&lab_planes, &mut merged_lab)?;

    
    let mut final_image = Mat::zeros(img.rows(), img.cols(), img.typ())?.to_mat()?;
    cvt_color(&merged_lab, &mut final_image, COLOR_Lab2BGR, 0)?;

    
    imwrite(output_path, &final_image, &opencv::core::Vector::new())?;

    Ok(())
}