#[derive(Debug)]
pub struct Document {
    node: html::dom::Node,
    stylesheet: css_v2_1::stylesheet::Stylesheet
}

pub fn new_document(node: html::dom::Node, stylesheet: css_v2_1::stylesheet::Stylesheet) -> Document {
    Document {  node, stylesheet}
}

impl Document {
    pub fn print_res(&self) {
        println!("HTML: \n {:#?} \n\n CSS: \n {:#?}", self.node, self.stylesheet)
    }
}
