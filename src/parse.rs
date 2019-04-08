use crate::body::Body;
use crate::scene::Scene;
use nom::{char, delimited, digit, map, tag, value, ws, IResult};
use std::str::FromStr;

fn uint32p(input: &str) -> IResult<&str, u32> {
    map!(input, digit, |str| FromStr::from_str(str).unwrap_or(0))
}

#[rustfmt::skip]
fn scene(input: &str) -> IResult<&str, Scene> {
    ws!(input, do_parse!(
        tag!("Scene") >>
        bounces: uint32p >> 
        bodies: many0!(delimited!(char!('('), body, char!(')'))) >> 
        (Scene::new(bodies, bounces))
    ))
}

// currently there is only one type of body the basicbody which we will just call Body
#[rustfmt::skip]
fn body(input: &str) -> IResult<&str, Box<dyn Body + Sync>> {
    // ws!(input, do_parse!(
    //     tag!(Body)
    //     value!
    // ));
    unimplemented!()
}
