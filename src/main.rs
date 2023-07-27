use std::fs::File;
use std::io::{BufRead, Read};
use html::html::parse_html;
use css_v2_1::css::parse_css;

fn main(){
    // let html = "resources/test.html";
    //
    // let mut input = File::open(html).expect("Error find file");
    // let mut content = String::new();
    // input.read_to_string(&mut content).expect("Error write to string");
    //
    // let node = parse_html(content);
    // println!("{:#?}", node);

    let css = "resources/test.css";

    let mut input_css = File::open(css).expect("Error find file");
    let mut content_css = String::new();
    input_css.read_to_string(&mut content_css).expect("Error write to string");

    let stylesheet = parse_css(content_css);
    println!("{:#?}", stylesheet)
}
