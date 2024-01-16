use super::PhotoType;

/// Groups whether a photo has raw file, developed file or extra files linked to it.
#[derive(Debug)]
struct HashType {
    hash_raw: bool,
    hash_img: bool,
    hash_other: bool,
}

impl HashType {
    /// Function that initialises a `HashType` struct to all fields false.
    fn new() -> HashType {
        let ht = HashType {
            hash_raw: false,
            hash_img: false,
            hash_other: false,
        };

        ht
    }
}

/// This `struct` models a physical file in a directory with pictures.
///
/// # Description
///
/// A folder with pictures might contain the following types of files:
/// - _raw_ files with the extension used by the camera's manufacturer.
/// - _developed_ files, usually _.jpg_.
/// - _other_ files, such as descriptors created by 3rd party SW like Darktable.
pub struct PhotoFile {
    /// Name of the file (not including the extension).
    name: String,
    /// What files with the name _name_ have been found in a directory.
    types_found: HashType,
    raw_ext: String,
    img_ext: String,
    other_ext: String,
}

impl PhotoFile {
    /// Creates a **PhotoFile** with default values.
    ///
    /// # Description
    ///
    /// The function receives a _name_ that will identify the photo file. This name should not
    /// contain the extension. For example, for a picture _DSCF10992.RAF_ the name passed while
    /// calling this function shall be _DSCF10992_.
    ///
    /// The default choice for the extension of the linked files are:
    /// - _RAF_ for the raw files.
    /// - _JPG_ for the developed files.
    ///
    /// Those are used by Fujifilm cameras. If the photo files belong to other manufacturer,
    /// the extension sufixes will most likely be different.
    ///
    /// # Arguments
    ///
    /// - _name_: a string slice that indicates the name of the picture (with no extension).
    /// - _raw_ext_: a wrapped string slice. When None is passed, the default choice (_RAF_) is
    ///              applied.
    /// - _img_ext_: a wrapped string slice. When None is passed, the default choice (_JPG_) is
    ///              applied.
    /// - _other_ext_: a wrapped string slice. When None is passed, the default choice (_xmp_) is
    ///              applied.
    ///
    /// # Example of use
    ///
    /// Create an instance of a **PhotoFile** using the default values (Fujifilm):
    ///
    /// ```rust
    /// use photo_lib::PhotoFile;
    ///
    /// let image = PhotoFile::new("myname", None, None, None);
    /// ```
    ///
    /// Another example when using files from Nikon:
    ///
    /// ```rust
    /// use photo_lib::PhotoFile;
    ///
    /// let image = PhotoFile::new("myname", Some("dng"), Some("jpg"), None);
    /// ```
    pub fn new(
            name: &str,
            raw_ext: Option<&str>,
            img_ext: Option<&str>,
            other_ext: Option<&str>
    ) -> PhotoFile {
        let rext = if raw_ext.is_some() {
            String::from(raw_ext.unwrap())
        } else {
            String::from("RAF")
        };

        let iext = if img_ext.is_some() {
            String::from(img_ext.unwrap())
        } else {
            String::from("JPG")
        };

        let oext = if other_ext.is_some() {
            String::from(other_ext.unwrap())
        } else {
            String::from("xmp")
        };

        let pf = PhotoFile {
            name: String::from(name),
            types_found: HashType::new(),
            raw_ext: rext,
            img_ext: iext,
            other_ext: oext,
        };

        pf
    }

    ///
    pub fn hash_raw(&mut self, raw_exists: Option<bool>) -> bool {
        if let Some(re) = raw_exists {
            self.types_found.hash_raw = re;
        }

        self.types_found.hash_raw
    }

    ///
    pub fn hash_img(&mut self, img_exists: Option<bool>) -> bool {
        if let Some(ie) = img_exists {
            self.types_found.hash_img = ie;
        }

        self.types_found.hash_img
    }

    ///
    pub fn hash_other(&mut self, other_exists: Option<bool>) -> bool {
        if let Some(oe) = other_exists {
            self.types_found.hash_other = oe;
        }

        self.types_found.hash_other
    }

    pub fn is_developed(&self) -> bool {
        if self.types_found.hash_raw && self.types_found.hash_img {
            true
        } else {
            false
        }
    }

    /// Delete one of the associated files to a photo instance.
    ///
    /// # Description
    ///
    /// This function will delete the type of file indicated by _image_type_. If the
    /// file doesn't exist, it will return an `std::io::ErrorKind::NotFound` error.
    /// On success, it will return 0.
    ///
    /// # Example
    ///
    /// An example to delete the developed image file for a photo named "DSCF1022":
    ///
    /// ```rust
    /// use photo_lib::*;
    ///
    /// let mut image = PhotoFile::new("myname", Some("dng"), Some("jpg"), None);
    /// image.hash_img(Some(true));
    /// let result = image.clear(PhotoType::Img);
    /// match result {
    ///     Ok(..) => println!("The developed file associated to myname was deleted."),
    ///     Err(e) => println!(
    ///         "Error trying to delete the develooped file associated to myname: {:?}", e
    ///     ),
    /// }
    /// ```
    pub fn clear(&mut self, image_type: PhotoType) -> std::io::Result<u32> {
        let mut filepath = self.name.clone();

        let (extra, exists) = match image_type {
            PhotoType::Raw => (self.raw_ext.as_str(), self.types_found.hash_raw),
            PhotoType::Img => (self.img_ext.as_str(), self.types_found.hash_img),
            PhotoType::Other => (self.other_ext.as_str(), self.types_found.hash_other),
        };

        filepath.push_str(extra);

        if !exists {
            Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        } else {
            std::fs::remove_file(filepath)?;
            Ok(0)
        }
    }

    // pub fn move(&mut self) -> std::io::Result<()> {

    // }

}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn photo_file_new() {
        let mut pf = PhotoFile::new("test", None, None, None);
        assert_eq!(pf.name, "test");
        assert_eq!(pf.raw_ext, "RAF");
        assert_eq!(pf.img_ext, "JPG");
        pf = PhotoFile::new("test2", Some("dng"), Some("jpg"), None);
        assert_eq!(pf.raw_ext, "dng");
        assert_eq!(pf.img_ext, "jpg");
    }

    #[rstest]
    fn photo_file_hash_raw() {
        let mut pf = PhotoFile::new("test", None, None, None);
        assert_eq!(pf.hash_raw(None), false);
        pf.hash_raw(None);
        assert_eq!(pf.hash_raw(None), false);
        pf.hash_raw(Some(true));
        assert_eq!(pf.hash_raw(None), true);
    }

    #[rstest]
    fn photo_file_hash_img() {
        let mut pf = PhotoFile::new("test", None, None, None);
        assert_eq!(pf.hash_img(None), false);
        pf.hash_img(None);
        assert_eq!(pf.hash_img(None), false);
        pf.hash_img(Some(true));
        assert_eq!(pf.hash_img(None), true);
    }

    #[rstest]
    fn photo_file_hash_other() {
        let mut pf = PhotoFile::new("test", None, None, None);
        assert_eq!(pf.hash_other(None), false);
        pf.hash_other(None);
        assert_eq!(pf.hash_other(None), false);
        pf.hash_other(Some(true));
        assert_eq!(pf.hash_other(None), true);
    }

    #[rstest]
    fn photo_file_is_developed() {
        let mut pf = PhotoFile::new("test", None, None, None);
        assert_eq!(pf.is_developed(), false);
        pf.hash_raw(Some(true));
        pf.hash_img(Some(true));
        assert_eq!(pf.is_developed(), true);
    }
}
