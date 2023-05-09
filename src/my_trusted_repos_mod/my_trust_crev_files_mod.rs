// my_trust_crev_files_mod.rs

use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRUST_FOLDER: std::path::PathBuf =
        crate::utils_mod::get_data_dir().join("trust");
}

pub struct MyTrustCrevFiles {
    folder_path: std::path::PathBuf,
}

impl MyTrustCrevFiles {
    pub fn new() -> MyTrustCrevFiles {
        let folder_path = (*TRUST_FOLDER).as_path();
        MyTrustCrevFiles {
            folder_path: folder_path.to_owned(),
        }
    }
    /// iterator for all .crev files inside my /trust/ folder
    pub fn iter_my_trust_crev_file(&self) -> IterMyTrustCrevFile {
        IterMyTrustCrevFile::open(&self.folder_path)
    }
}

// region: IterMyTrustCrevFile

/// iterator over my trust crev files
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Debug)]
pub struct IterMyTrustCrevFile {
    read_dir: std::fs::ReadDir,
}

impl IterMyTrustCrevFile {
    /// constructor
    pub fn open(folder_path: &std::path::Path) -> IterMyTrustCrevFile {
        IterMyTrustCrevFile {
            read_dir: folder_path.read_dir().unwrap(),
        }
    }
}
/// iterator for proofs returns Range
impl Iterator for IterMyTrustCrevFile {
    type Item = super::crev_file_mod::CrevFile;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry_opt_res = self.read_dir.next();
            match entry_opt_res {
                Some(entry_res) => {
                    let entry = entry_res.unwrap();
                    if entry.file_name().to_string_lossy().ends_with(".crev") {
                        let crev_file =
                            super::crev_file_mod::CrevFile::read(entry.path().as_path());
                        return Some(crev_file);
                    }
                }
                None => return None,
            }
        }
    }
}
// endregion: IterMyTrustCrevFile
