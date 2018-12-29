// extern crate cc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {    

    // cc::Build::new()
    //     .cpp(true)
    //     .flag("-std=c++11")
    //     .file("libSTARK/libstark/src/common/Algebra/LinearSpace.cpp")
    //     .file("libSTARK/libstark/src/common/Algebra/MultiVarPoly.cpp")
    //     .file("libSTARK/libstark/src/common/Algebra/ShiftedSubspacePolynomial.cpp")
    //     .file("libSTARK/libstark/src/common/Infrastructure/Infrastructure.cpp")
    //     .file("libSTARK/libstark/src/common/utils/ErrorHandling.cpp")
    //     .file("libSTARK/libstark/src/common/utils/specsPrint.cpp")
    //     .file("libSTARK/libstark/src/common/utils/Timing.cpp")
    //     .file("libSTARK/libstark/src/languages/Acsp/AcspWitnessChecker.cpp")
    //     .file("libSTARK/libstark/src/languages/Bair/BairWitnessChecker.cpp")
    //     .file("libSTARK/libstark/src/languages/Bair/ConstraintsSys.cpp")
    //     .file("libSTARK/libstark/src/protocols/Ali/common_details/common.cpp")
    //     .file("libSTARK/libstark/src/protocols/Ali/verifier_details/queriesGen.cpp")
    //     .file("libSTARK/libstark/src/protocols/Ali/verifier_details/queriesGen.cpp")
    //     .file("libSTARK/libstark/src/protocols/Ali/prover.cpp")
    //     .file("libSTARK/libstark/src/protocols/Ali/verifier.cpp")        
    //     // .file("libSTARK/libstark/src/protocols/common/CryptoCommitment/MerkleCommitment.cpp")
    //     .include("libSTARK/algebra/algebralib/headers")
    //     .include("libSTARK/algebra/FFT/src")
    //     .include("libSTARK/libstark/src")
    //     .compile("algebra");

    

    let bindings = bindgen::builder()        
        .header("libSTARK/libstark/src/languages/Bair/BairInstance.hpp") 
        .header("libSTARK/libstark/src/languages/Bair/BairWitness.hpp") 
        .header("libSTARK/libstark/src/protocols/protocol.hpp") 
        .header("libSTARK/libstark/src/languages/Bair/BairWitnessChecker.hpp")
        .trust_clang_mangling(false)
        .rustfmt_bindings(true)      
        .enable_cxx_namespaces()
        .clang_arg(r"-xc++")
        .clang_arg(r"-std=c++11")
        .clang_arg("-Isrc")
        .clang_arg("-IlibSTARK/algebra/algebralib/headers")    
        .clang_arg("-IlibSTARK/algebra/FFT/src")
        // .whitelist_function("libstark::BairInstance")          
        .whitelist_type("libstark::BairInstance")
        // .whitelist_function("libstark::BairWitness")  
        .whitelist_type("libstark::BairWitness")     
        .whitelist_function("libstark::Protocols::executeProtocol")
        .whitelist_function("libstark::BairWitnessChecker::verify") 
        // .whitelist_function("POW2") 
        // .whitelist_type("ConstraintSys")  
        // .whitelist_type("FieldElement")  
        // .whitelist_type("size_t")
        // .whitelist_type("unique_ptr")
        // .whitelist_type("pair")
        // .whitelist_type("Sequence")
        // .whitelist_type("map")        
        // .whitelist_type("vector")
        .derive_copy(false)
        .layout_tests(false)
        // .conservative_inline_namespaces()
        // .use_core()
        .opaque_type("std::basic_string_value_type")
        .opaque_type("std::__value_type_value_type")
        .opaque_type("std::__value_type___nc_value_type")
        .generate()
        .expect("Unable to generte bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        // .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");

    
    println!("cargo:rustc-link-search=native=libSTARK/bin/algebralib");
    println!("cargo:rustc-link-lib=static=algebralib");
    println!("cargo:rustc-link-search=native=libSTARK/bin/fft");
    println!("cargo:rustc-link-lib=static=FFT");
    println!("cargo:rustc-link-search=native=libSTARK/bin/gadgetlib");
    println!("cargo:rustc-link-lib=static=gadgetlib");
    println!("cargo:rustc-link-search=native=libSTARK/bin/libstark");
    println!("cargo:rustc-link-lib=static=stark");
}