// Copyright 2024 Felipe Torres González

mod photo_file;

pub use crate::photo_file::*;

/// Enum that indicates whether a file is a raw photo, a developed photo or metadata.
pub enum FileType {
    Raw,
    Img,
    Other,
}

