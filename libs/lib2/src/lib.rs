

// The linking error does not happen if this line is commented
extern crate lib1;

// or this other line
use lib3_dylib::*;

// or if the dylib crate type is removed from lib3_dylib Cargo.toml
// or if lto is not set to either 'thin' or 'fat'

// NOTE: to test the various cases cargo clean should be run between them

use lib1::lib1_function;

pub fn lib2_function() {
    lib1_function();
}

