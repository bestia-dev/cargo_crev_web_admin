// crev_proof_mod.rs

// Inside .crev files there are many `proofs` that look like this:
// ----- BEGIN CREV PROOF -----
// kind: trust
// version: -1
// date: "2022-01-26T14:52:33.727061823+01:00"
// from:
//   id-type: crev
//   id: 24YKeuThJDNFSlJyxcl5diSZcKcRbh-0zXM0YxTOFJw
//   url: "https://github.com/LucianoBestia/crev-proofs"
// ids:
//   - id-type: crev
//     id: GH_y9BmI567xqh3_5vSStNAAZ79mcxazK52avCzttps
//     url: "https://github.com/bitwave/crev-proofs"
// trust: low
// ----- SIGN CREV PROOF -----
// -hTyO6bR_G3AY0feYhMLzQxwrOLEMdOV5Mr-MmapfVISRa879bjh1E1H28c0DA1cz1A0-syXoODg0PdakVMiDA
// ----- END CREV PROOF -----

use dev_bestia_string_utils::*;
use unwrap::unwrap;

pub struct CrevProof<'a> {
    proof_str: &'a str,
}

impl<'a> CrevProof<'a> {
    pub fn from_str(proof_str: &'a str) -> CrevProof {
        // return
        CrevProof { proof_str }
    }
    pub fn into_trust_yaml(&self) -> super::trust_yaml_proofs_mod::TrustYaml {
        // This must not panic because it is internal to the previous range.
        let range_yaml = unwrap!(find_range_between_delimiters(
            &self.proof_str,
            &mut 0,
            "----- BEGIN CREV PROOF -----",
            "----- SIGN CREV PROOF -----",
        ));
        // if this panics it's a bug in the code and not an exception to handle
        let yaml_str = unwrap!(self.proof_str.get(range_yaml));
        super::trust_yaml_proofs_mod::TrustYaml::from_str(yaml_str)
    }
}
