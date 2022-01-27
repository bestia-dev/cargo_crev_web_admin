// crev_file_mod.rs

use dev_bestia_string_utils::*;
use unwrap::unwrap;

// there are 2 types of "my" crev files in folders:
// 1. containing `my trust yaml` are in the one folder ~/.config/crev/proofs/github_com_.../.../trust/*.crev
// 2. containing `my review yaml` are in the one folder ~/.config/crev/proofs/github_com_.../.../reviews/*.crev

// there are 2 types of "others" crev files in folders:
// This folders have also the .git sub-folder that must be excluded for the dir traversing
// 3. containing `others trust yaml` are in many sub-folders ~/.cache/crev/remotes/git.../.../trust/*.crev
// 4. containing `others review yaml` are in many sub-folders ~/.cache/crev/remotes/git.../.../reviews/*.crev

// region: CrevFile

pub struct CrevFile {
    file_path: std::path::PathBuf,
    file_content: String,
}

impl CrevFile {
    pub fn read(file_path: &std::path::Path) -> CrevFile {
        // dbg!(&file_path);
        // read crev file
        let file_content = unwrap!(std::fs::read_to_string(file_path));
        //return
        CrevFile {
            file_path: file_path.to_owned(),
            file_content,
        }
    }
    /// iterator for proof strings including markers
    pub fn iter_proof_range(&self) -> IterProofRange {
        IterProofRange::from_str(&self.file_content)
    }

    pub fn get_proof_from_range(
        &self,
        proof_range: &std::ops::Range<usize>,
    ) -> super::crev_proof_mod::CrevProof {
        let proof_str = unwrap!(self.file_content.get(proof_range.clone()));
        super::crev_proof_mod::CrevProof::from_str(proof_str)
    }

    /// remove trust proofs with this url
    /// and saves file
    pub fn delete_url(&mut self, repo_url: &str) -> anyhow::Result<()> {
        let mut ranges_to_delete: Vec<std::ops::Range<usize>> = vec![];
        for proof_range in self.iter_proof_range() {
            let proof = self.get_proof_from_range(&proof_range);
            let trust_yaml = proof.into_trust_yaml();
            for id in trust_yaml.ids.iter() {
                if let Some(url) = &id.url {
                    if url.as_str() == repo_url {
                        ranges_to_delete.push(proof_range.clone());
                        break;
                    }
                }
            }
        }
        if !ranges_to_delete.is_empty() {
            // remove all ranges from the bottom up
            while let Some(range) = ranges_to_delete.pop() {
                dbg!("drain {:#?}", &range);
                self.file_content.drain(range);
            }
            if self.file_content.trim().is_empty() {
                // delete the file
                unwrap!(std::fs::remove_file(&self.file_path));
            } else {
                std::fs::write(&self.file_path, &self.file_content)?;
            }
        }
        Ok(())
    }
}

// endregion: CrevFile

// region: IterProofRange

/// IterProofRange is between including "----- BEGIN CREV PROOF -----" and "----- END CREV PROOF -----"
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone, Debug)]
pub struct IterProofRange<'a> {
    file_content: &'a str,
    pos_cursor: usize,
}

impl<'a> IterProofRange<'a> {
    /// constructor
    pub fn from_str(content: &'a str) -> IterProofRange {
        IterProofRange {
            file_content: content,
            pos_cursor: 0,
        }
    }
}

/// iterator for proofs returns Range
impl<'a> Iterator for IterProofRange<'a> {
    type Item = std::ops::Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let proof_range = find_range_including_delimiters(
            self.file_content,
            &mut self.pos_cursor,
            "----- BEGIN CREV PROOF -----",
            "----- END CREV PROOF -----",
        );
        match proof_range {
            Some(mut proof_range) => {
                // if there is some white space after the segment, include it in the range.
                let pos_2 = find_pos_before_delimiter(
                    self.file_content,
                    proof_range.end,
                    "----- BEGIN CREV PROOF -----",
                );
                proof_range.end = match pos_2 {
                    Some(pos_2) => pos_2,
                    None => self.file_content.len(),
                };
                self.pos_cursor = proof_range.end;
                return Some(proof_range);
            }
            None => return None,
        }
    }
}
// endregion: IterProofRange
