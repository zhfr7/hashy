use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{char, complete::digit1},
    combinator::{map, map_res},
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

use crate::algorithms::{
    md2::Md2,
    md4::Md4,
    md5::Md5,
    md6::Md6,
    sha1::Sha1,
    sha2::{Sha2, Sha2Variant},
    sha3::{Sha3, Sha3Variant, Shake, ShakeVariant},
    Algorithm,
};

fn num(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |v: &str| v.parse::<usize>()).parse(input)
}

fn md6(input: &str) -> IResult<&str, Box<dyn Algorithm>> {
    map(
        preceded(tag("md6-"), num),
        |output_length| -> Box<dyn Algorithm> { Box::new(Md6::new(output_length, None)) },
    )
    .parse(input)
}

fn sha2(input: &str) -> IResult<&str, Box<dyn Algorithm>> {
    map(
        preceded(
            (tag("sha"), alt((tag("2-"), tag("-")))),
            alt((
                map(tag("224"), |_| Sha2Variant::_224),
                map(tag("256"), |_| Sha2Variant::_256),
                map(tag("384"), |_| Sha2Variant::_384),
                map(tag("512"), |_| Sha2Variant::_512),
                map(tag("512-224"), |_| Sha2Variant::_512_224),
                map(tag("512-256"), |_| Sha2Variant::_512_256),
            )),
        ),
        |variant| -> Box<dyn Algorithm> { Box::new(Sha2::new(variant)) },
    )
    .parse(input)
}

fn sha3(input: &str) -> IResult<&str, Box<dyn Algorithm>> {
    map(
        preceded(
            tag("sha3-"),
            alt((
                map(tag("224"), |_| Sha3Variant::_224),
                map(tag("256"), |_| Sha3Variant::_256),
                map(tag("384"), |_| Sha3Variant::_384),
                map(tag("512"), |_| Sha3Variant::_512),
            )),
        ),
        |variant| -> Box<dyn Algorithm> { Box::new(Sha3::new(variant)) },
    )
    .parse(input)
}

fn shake(input: &str) -> IResult<&str, Box<dyn Algorithm>> {
    map_res(
        preceded(
            tag("shake"),
            separated_pair(
                alt((
                    map(tag("128"), |_| ShakeVariant::_128),
                    map(tag("256"), |_| ShakeVariant::_256),
                )),
                char('-'),
                num,
            ),
        ),
        |(variant, output_length)| {
            Shake::new(variant, output_length).map(|shake| Box::new(shake) as Box<dyn Algorithm>)
        },
    )
    .parse(input)
}

pub fn parse_algorithm(input: &str) -> IResult<&str, Box<dyn Algorithm>> {
    alt((
        map(tag("md2"), |_| -> Box<dyn Algorithm> { Box::new(Md2) }),
        map(tag("md4"), |_| -> Box<dyn Algorithm> { Box::new(Md4) }),
        map(tag("md5"), |_| -> Box<dyn Algorithm> { Box::new(Md5) }),
        md6,
        map(tag("sha1"), |_| -> Box<dyn Algorithm> { Box::new(Sha1) }),
        map(tag("sha1"), |_| -> Box<dyn Algorithm> { Box::new(Sha1) }),
        sha2,
        sha3,
        shake,
    ))
    .parse(input)
}
