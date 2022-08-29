// Copyright 2020 Tsang Hao Fung. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod config;
mod converter;
mod svg;

use config::*;
use converter::*;
// pub use svg::*;
// pub use module::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert(img: &str) -> String {
    let config = Config::from_args();
    let result = convert_image_to_svg(config,img);
    match result {
        Ok(svg) => {
            println!("Conversion successful.");
            let svg_str = format!("{}", svg);
            svg_str
        },
        Err(msg) => {
            panic!("Conversion failed with error message: {}", msg);
        }
    }
}