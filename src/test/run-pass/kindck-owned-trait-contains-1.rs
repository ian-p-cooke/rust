// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

trait repeat<A> { fn get(&self) -> A; }

impl<A:Copy> repeat<A> for @A {
    fn get(&self) -> A { copy **self }
}

fn repeater<A:Copy>(v: @A) -> @repeat:<A> {
    // Note: owned kind is not necessary as A appears in the trait type
    @v as @repeat:<A> // No
}

pub fn main() {
    let x = &3;
    let y = repeater(@x);
    assert_eq!(*x, *(y.get()));
}
