use std::collections::HashMap;
use crate::dom;
use crate::dom::{comment, doctype, elem, Node, style};

pub fn parse_html(source: String) -> Node {
    let mut nodes = Parser { pos: 0, input: source }.parse_nodes();

    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        elem("html".to_string(), HashMap::new(), nodes)
    }
}

struct Parser {
    pos: usize,
    input: String
}

const STYLE_TAG: &'static str = "style";

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }

    fn consume_while<F>(&mut self, c: F) -> String
        where F: Fn(char) -> bool {
            let mut result = String::new();
            while !self.eof() && c(self.next_char()) {
                result.push(self.consume_char());
            }
        return result;
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false
        })
    }

    fn parse_node(&mut self) -> Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text()
        }
    }

    fn parse_element(&mut self) -> Node {
        assert_eq!(self.consume_char(), '<');

        match self.next_char() {
            '!' => self.parse_special_elem(),
            _ => self.parse_tag_attrs()
        }
    }

    fn parse_tag_attrs(&mut self) -> Node {
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert_eq!(self.consume_char(), '>');

        if tag_name == STYLE_TAG {
            return self.parse_style();
        }

        // Content
        let children = self.parse_nodes();

        // Close tag
        self.check_close_tag(&tag_name);
        // assert_eq!(self.consume_char(), '<');
        // assert_eq!(self.consume_char(), '/');
        // assert_eq!(self.parse_tag_name(), tag_name);
        // assert_eq!(self.consume_char(), '>');

        return elem(tag_name, attrs, children);
    }

    fn check_close_tag(&mut self, tag_name: &String) {
        assert_eq!(self.consume_char(), '<');
        assert_eq!(self.consume_char(), '/');
        assert_eq!(self.parse_tag_name(), *tag_name);
        assert_eq!(self.consume_char(), '>');
    }

    fn parse_text(&mut self) -> Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    fn parse_special_elem(&mut self) -> Node {
        assert_eq!(self.consume_char(), '!');

        match self.next_char() {
            'D' => self.parse_doctype(),
            'd' => self.parse_doctype(),
            '-' => self.parse_comment(),
            _ => dom::empty()
        }
    }

    fn parse_comment(&mut self) -> Node {
        // Start comment tag
        assert_eq!(self.consume_char(), '-');
        assert_eq!(self.consume_char(), '-');

        self.consume_whitespace();

        // content
        let mut com = String::new();

        loop {
            if self.starts_with("-->") {
                break
            }

            com.push(self.consume_char());
        }

        //Close tag
        assert_eq!(self.consume_char(), '-');
        assert_eq!(self.consume_char(), '-');
        assert_eq!(self.consume_char(), '>');

        return comment(com)
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert_eq!(self.consume_char(), '=');
        let value = self.parse_attr_value();
        return (name, value)
    }

    fn parse_attr_value(&mut self) -> String {
        let open = self.consume_char();
        assert!(open == '"' || open == '\'');
        let value = self.consume_while(|c| c != open);
        assert_eq!(self.consume_char(), open);
        return value
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attrs = HashMap::new();

        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (tag, value) = self.parse_attr();
            attrs.insert(tag, value);
        }

        return attrs
    }

    fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();

        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break
            }

            nodes.push(self.parse_node());
        }

        return nodes
    }

    fn parse_doctype(&mut self) -> Node {
        let tag = self.consume_while(|c| !char::is_whitespace(c));
        assert_eq!(tag.to_uppercase(), "DOCTYPE");
        self.consume_whitespace();
        let value = self.consume_while(|c| c != '>');
        assert_eq!(self.consume_char(), '>');
        return doctype(value);
    }

    fn parse_style(&mut self) -> Node {
        let mut source = String::from("");

        loop {
            self.consume_whitespace();

            if self.eof() || self.starts_with("</") {
                break
            }

            source.push(self.consume_char());
        }

        let stylesheet = css_v2_1::css::parse_css(source);

        self.check_close_tag(&STYLE_TAG.to_string());

        return style(stylesheet);
    }
}
