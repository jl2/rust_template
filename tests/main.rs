// main.rs
//
// Copyright (c) 2022 Jeremiah LaRocco <jeremiah_larocco@fastmail.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

#[cfg(test)]

extern crate rust_template;

mod tests {

    #[test]
    fn first_test() {
        let thing = rust_template::module::do_something();
        assert_eq!(thing, "first_thing".to_string());
        println!("{:?}", thing);
    }

    #[test]
    fn second_test() {
        let thing = rust_template::module::do_something();
        assert_eq!(thing, "second_thing".to_string());
        println!("{:?}", thing);
    }

}
