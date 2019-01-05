use structopt::StructOpt;
use ::std::os::raw::c_uint;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "A: initial num")]
    input_a: c_uint,
    #[structopt(name = "B: initial num")]
    input_b: c_uint,
}


fn main() {
    let opt = Opt::from_args();
    let security_parameter: c_uint = 60;    
    stark_rs::execute_fsrs(opt.input_a, opt.input_b, security_parameter);    
}

