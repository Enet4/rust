// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Check that we correctly prevent users from making trait objects
// from traits with associated consts.

#![feature(associated_consts)]

trait Bar {
    const X: usize;
}

fn make_bar<T:Bar>(t: &T) -> &Bar {
    //~^ ERROR E0038
    //~| NOTE the trait cannot contain associated consts like `X`
    //~| NOTE the trait `Bar` cannot be made into an object
    t
}

fn main() {
}
