extern crate libstark_rs;

use failure::Error;
use structopt::StructOpt;



#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "A: initial num")]
    input_a: i32,
    #[structopt(name = "B: initial num")]
    input_b: i32,
}

// #[link(name = "libSTARK")]
// extern {
//     pub fn buildBairInstance();
// }


fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let security_parameter = 60;
    Ok(println!("{}, {}", &opt.input_a, &opt.input_b))
}

