extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use pulldown_cmark::{Parser, Options, html};
use web_sys::{Request, console};

#[wasm_bindgen(module = "/src/lib.js")]
extern "C" {
    fn name() -> String;
}

#[wasm_bindgen(start)]
pub fn run() {
    extern crate console_error_panic_hook;
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn parse(req: Request) -> String {
    console::log_1(&format!("Requesting {} for {}", req.url(), name()).into());

    let markdown_input: &str = "Hello world, this is a ~~complicated~~ *very simple* example.";
    println!("Parsing the following Markdown string:\n{}", markdown_input);

    if req.url() == "https://crash.com/" {
        panic!("help me")
    }

    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options);

    // Write to String buffer.
    let mut html_output: String = String::with_capacity(markdown_input.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    // Check that the output is what we expected.
    let expected_html: &str = "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";
    assert_eq!(expected_html, &html_output);

    format!("\nHTML output:\n{}", &html_output)
}