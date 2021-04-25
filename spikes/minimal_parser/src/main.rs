extern crate nom;

use crate::minizinc::model;

mod minizinc;

fn main() {
    let bytes = include_bytes!("../model.mzn");
    let model_string = String::from_utf8_lossy(bytes);
    print!("model:\n{}", &model_string);
    print!("parsed:\n{:#?}", model(&model_string));
}
