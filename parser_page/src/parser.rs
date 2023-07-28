use std::fs::File;
use std::io::Read;
use crate::document::{new_document, Document};

pub struct ParserPage {
    sources_files_path: Vec<String>
}

pub fn new_parser(sources_files_path: Vec<String>) -> ParserPage {
    ParserPage { sources_files_path }
}

impl ParserPage {
    pub fn parse(&self) -> Document {
        let mut node = html::dom::empty();
        let mut stylesheet = css_v2_1::stylesheet::Stylesheet { rules: Vec::new() };

        for source in self.sources_files_path.iter() {
            let source_split: Vec<&str> = source.split('.').collect();
            let source_extension = source_split[1];

            match source_extension {
                "html" => {
                    let content = self.open_file(source);
                    node = html::html::parse_html(content)
                },
                "css" => {
                    let content = self.open_file(source);
                    stylesheet = css_v2_1::css::parse_css(content)
                },
                _ => panic!("File extension {} is not supported", source_extension)
            }
        }

        return new_document(node, stylesheet)
    }

    fn open_file(&self, source: &String) -> String {
        let mut f = File::open(source).expect("File not found");
        let mut content = String::new();
        f.read_to_string(&mut content).expect("Cant read file");
        return content
    }
}

