use std::collections::HashMap;
use css_v2_1::stylesheet::Stylesheet;
use crate::dom::NodeType::{Comment, Doctype, Empty, Style, Text};

#[derive(Debug)]
pub struct Node {
    children: Vec<Node>,
    node_type: NodeType
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
    Doctype(String),
    Style(Stylesheet),
    Empty
}

#[derive(Debug)]
pub struct ElementData {
    tag_name: String,
    attr_map: AttrMap
}

pub type AttrMap = HashMap<String, String>;

pub fn text(data: String) -> Node {
    Node { children: Vec::new(), node_type: Text(data) }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attr_map: attrs
        })
    }
}

pub fn comment(com: String) -> Node { Node { children: Vec::new(), node_type: Comment(com) } }

pub fn doctype(doctype: String) -> Node { Node { children: Vec::new(), node_type: Doctype(doctype) } }

pub fn empty() -> Node { Node { children: Vec::new(), node_type: Empty } }

pub fn style(stylesheet: Stylesheet) -> Node {    Node{  children: Vec::new(), node_type:Style(stylesheet)}}
