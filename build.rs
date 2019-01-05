#![allow(non_snake_case)]

extern crate glob;
extern crate cc;
extern crate bindgen;

use glob::glob;
use std::env;
use std::path::PathBuf;

fn main() {    
    let stark = glob("./libSTARK/libstark/src/**/*.cpp").unwrap().map(|e| e.unwrap()).into_iter();
    let algebralib = glob("./libSTARK/algebra/algebralib/**/*.cpp").unwrap().map(|e| e.unwrap()).into_iter();
    let FFT = glob("./libSTARK/algebra/FFT/**/*.cpp").unwrap().map(|e| e.unwrap()).into_iter();    
    let fsrs = glob("./libSTARK/fsrs/**/*.cpp").unwrap().map(|e| e.unwrap()).into_iter();    
    
    cc::Build::new()
        .cpp(true)                
        .flag("-xc++")
        .flag("-std=c++11")
        .flag("-O3")
        .flag("-g")
        .flag("-Wall")
        .flag("-fmessage-length=0")        
        .flag("-openmp")
        .flag("-maes")
        .flag("-msse4")
        .flag("-mtune=native")
        .flag("-Isrc")   
        .flag("-Xpreprocessor")
        .flag("-pthread")   
        .flag("-lomp")              
        .static_flag(true)
        .warnings(false)
        .extra_warnings(false)
        .files(stark)        
        .include("libSTARK/algebra/algebralib/headers")
        .include("libSTARK/algebra/FFT/src")
        .include("libSTARK/libstark/src")    
        .compile("stark");
    
    cc::Build::new()
        .cpp(true)    
        .flag("-xc++")
        .flag("-std=c++11")
        .flag("-O3")
        .flag("-g")
        .flag("-Wall")
        .flag("-fmessage-length=0")        
        .flag("-openmp")
        .flag("-maes")
        .flag("-msse4")
        .flag("-mtune=native")
        .flag("-Isrc")   
        .flag("-Xpreprocessor")
        .flag("-pthread")   
        .flag("-lomp")  
        .static_flag(true)
        .warnings(false)
        .extra_warnings(false)
        .files(algebralib)                
        .include("libSTARK/algebra/algebralib/headers")
        .include("libSTARK/algebra/FFT/src")
        .include("libSTARK/libstark/src")    
        .compile("algebralib");

    cc::Build::new()
        .cpp(true)    
        .flag("-xc++")
        .flag("-std=c++11")
        .flag("-O3")
        .flag("-g")
        .flag("-Wall")
        .flag("-fmessage-length=0")        
        .flag("-openmp")
        .flag("-maes")
        .flag("-msse4")
        .flag("-mtune=native")
        .flag("-Isrc")   
        .flag("-Xpreprocessor")
        .flag("-pthread")   
        .flag("-lomp")  
        .flag("-mpclmul") 
        .static_flag(true)
        .warnings(false)
        .extra_warnings(false)
        .files(FFT)        
        .include("libSTARK/algebra/algebralib/headers")
        .include("libSTARK/algebra/FFT/src")
        .include("libSTARK/libstark/src")
        .compile("FFT");

    cc::Build::new()
        .cpp(true)        
        .flag("-xc++")
        .flag("-std=c++11")
        .flag("-O3")
        .flag("-g")
        .flag("-Wall")
        .flag("-fmessage-length=0")        
        .flag("-openmp")
        .flag("-maes")
        .flag("-msse4")
        .flag("-mtune=native")
        .flag("-Isrc")   
        .flag("-Xpreprocessor")
        .flag("-pthread")   
        .flag("-lomp")                      
        .static_flag(true)     
        .warnings(false)
        .extra_warnings(false) 
        .files(fsrs)        
        .include("libSTARK/algebra/algebralib/headers")
        .include("libSTARK/algebra/FFT/src")
        .include("libSTARK/libstark/src")
        .compile("fsrs");

    println!("cargo:rustc-link-lib=gomp");     

    let bindings = bindgen::builder()        
        .header("libSTARK/fsrs/Fsrs_wrapper.hpp")               
        .trust_clang_mangling(false)
        .rustfmt_bindings(true)      
        .enable_cxx_namespaces()
        .clang_arg(r"-xc++")
        .clang_arg(r"-std=c++11")
        .clang_arg("-Isrc")
        .clang_arg("-IlibSTARK/algebra/algebralib/headers")    
        .clang_arg("-IlibSTARK/algebra/FFT/src")
        .clang_arg("-IlibSTARK/libstark/src")
        .whitelist_function("execute")          
        .derive_copy(false)
        .layout_tests(false)     
        .generate()
        .expect("Unable to generte bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    bindings
        .write_to_file(out_path.join("bindings.rs"))        
        .expect("Couldn't write bindings!");    
    
}
