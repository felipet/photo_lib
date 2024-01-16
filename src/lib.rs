// Copyright 2024 Felipe Torres González

mod photo_file;

pub use crate::photo_file::*;


pub enum PhotoType {
    Raw,
    Img,
    Other,
}

