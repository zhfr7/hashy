pub mod md2;
pub mod md4;
pub mod md5;
pub mod sha1;
pub mod sha2;
pub mod sha3;

mod keccak;
mod helpers;

pub enum Algorithm<'a>{
    Single(&'a str),
    Family {
        name: &'a str,
        members: Vec<&'a str>
    }
}

use self::Algorithm::*;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ALGORITHMS: Vec<Algorithm<'static>> = vec![
        Family {
            name: "MD",
            members: vec![ "md2", "md4", "md5" ]
        },
        Single("sha1"),
        Family {
            name: "SHA2",
            members: vec![ "sha-224", "sha-256 (sha2)", "sha-384", 
                "sha-512", "sha-512-224", "sha-512-256" ]
        },
        Family {
            name: "SHA3",
            members: vec![ "sha3-224", "sha3-256", "sha3-384",
                "sha3-512", "shake128-n", "shake256-n" ]
        }
    ];
}
