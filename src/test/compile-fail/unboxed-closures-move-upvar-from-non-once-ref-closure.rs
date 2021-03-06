// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test that a by-ref `FnMut` closure gets an error when it tries to
// consume a value.

fn call<F>(f: F) where F : Fn() {
    f();
}

fn main() {
    let y = vec!(format!("World"));
    call(|| {
        y.into_iter();
        //~^ ERROR cannot move out of captured outer variable in an `Fn` closure
    });
}
