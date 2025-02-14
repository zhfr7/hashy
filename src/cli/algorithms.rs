use lazy_static::lazy_static;

pub enum Specification {
    Single(&'static str),
    Family {
        name: &'static str,
        members: Vec<&'static str>,
    },
}

lazy_static! {
    pub static ref ALGORITHMS: Vec<Specification> = vec![
        Specification::Family {
            name: "MD",
            members: vec!["md2", "md4", "md5"]
        },
        Specification::Single("sha1"),
        Specification::Family {
            name: "SHA2",
            members: vec![
                "sha-224 (sha2-224)",
                "sha-256 (sha2, sha2-256)",
                "sha-384 (sha2-384)",
                "sha-512 (sha2-512)",
                "sha-512-224 (sha2-512-224)",
                "sha-512-256 (sha2-512-256)"
            ]
        },
        Specification::Family {
            name: "SHA3",
            members: vec![
                "sha3-224",
                "sha3-256",
                "sha3-384",
                "sha3-512",
                "shake128-n",
                "shake256-n"
            ]
        }
    ];
}
