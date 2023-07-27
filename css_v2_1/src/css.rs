use crate::stylesheet::{ColorConst, Stylesheet};
use crate::stylesheet::Specificity;
use crate::stylesheet::Color;
use crate::stylesheet::Value;
use crate::stylesheet::Declaration;
use crate::stylesheet::Unit;
use crate::stylesheet::Selector;
use crate::stylesheet::Rule;
use crate::stylesheet::SimpleSelector;
use Some;
use crate::color_const;
use crate::stylesheet::Value::ColorConstValue;

impl Value {
    fn to_px(&self) -> f32 {
        match *self {
            Value::Length(f, Unit::Px) => f,
            _ => 0.0
        }
    }

    fn to_color(&self) -> Color {
        match self {
            Value::ColorConstValue(n, ColorConst::Green) => Color {
                r: 0, g: 255, b: 0, a: 255
            },
            Value::ColorConstValue(n, ColorConst::Blue ) => Color {
                r: 0, g: 0, b: 255, a: 255
            },
            Value::ColorConstValue(n, ColorConst::Red) => Color {
                r: 255, g: 0, b: 0, a: 255
            },
            _ => panic!("unknown color")
        }
    }
}

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let id_count = simple.id.iter().count();
        let class_lem = simple.class.len();
        let tag_name_count = simple.tag_name.iter().count();
        (id_count, class_lem, tag_name_count)
    }
}

pub fn parse_css(source: String) -> Stylesheet {
    let mut parser = Parser { pos: 0, input: source };
    Stylesheet { rules: parser.parse_rules() }
}

#[derive(Debug)]
pub struct Parser {
    pos: usize,
    input: String
}

impl Parser {

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector { tag_name: None, id: None, class: Vec::new() };

        // TODO: ADD VALIDATION
        while !self.eof() {
            match self.next_char() {
                '.' => {
                    self.consume_char();
                    selector.class.push(Some(self.parse_identifier()).unwrap());
                },
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier())
                },
                '*' => { self.consume_char(); },
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier())
                }
                _ => break
            }
        }
        return selector
    }

    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations()
        }
    }

    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();

        loop {
            self.consume_whitespace();
            if self.eof() { break }
            rules.push(self.parse_rule());
        }

        return rules
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut sels = Vec::new();

        loop {
            sels.push(Selector::Simple(self.parse_simple_selector()));

            self.consume_whitespace();

            match self.next_char() {
                ',' => { self.consume_char(); self.consume_whitespace() },
                '{' => break,
                c => panic!("Unexpected char {} in selector list", c)
            }
        }

        sels.sort_by(|a,b| b.specificity().cmp(&a.specificity()));

        return sels
    }
    fn parse_declarations(&mut self) -> Vec<Declaration> {
        assert_eq!(self.consume_char(), '{');
        let mut decs = Vec::new();

        loop {
            self.consume_whitespace();

            if self.next_char() == '}' {
                self.consume_char();
                break
            }

            decs.push(self.parse_declaration());
        }

        return decs
    }

    fn parse_declaration(&mut self) -> Declaration {
        let property_name = self.parse_identifier();
        self.consume_whitespace();
        assert_eq!(self.consume_char(), ':');
        self.consume_whitespace();
        let value = self.parse_value();
        self.consume_whitespace();
        assert_eq!(self.consume_char(), ';');

        return Declaration {
            name: property_name,
            value: value
        }
    }

    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '0'..='9' => self.parse_length(),
            '#' => self.parse_color(),
            _ => self.parse_keyword()
        }
    }

    fn parse_keyword(&mut self) -> Value {
        let identifier = self.parse_identifier();

        match &*identifier {
            color_const::GREEN |
            color_const::RED |
            color_const::BLUE => self.parse_color_const(identifier),
            _ => Value::Keyword(identifier)
        }
    }

    fn parse_unit(&mut self) -> Unit {
        match &*self.parse_identifier().to_ascii_lowercase() {
            "px" => Unit::Px,
            _ => panic!("Unsupported unit")
        }
    }

    fn parse_color_const(&mut self, color: String) -> Value {
        match &*color {
            color_const::RED => ColorConstValue(String::from(color_const::RED), ColorConst::Red),
            color_const::BLUE => ColorConstValue(String::from(color_const::BLUE), ColorConst::Blue),
            color_const::GREEN => ColorConstValue(String::from(color_const::GREEN), ColorConst::Green),
            _ => panic!("unsupported color const")
        }
    }

    fn parse_length(&mut self) -> Value {
        Value::Length(self.parse_float(), self.parse_unit())
    }

    fn parse_float(&mut self) -> f32 {
        let s = self.consume_while(|c| match c {
            '0'..= '9' | '.' => true,
            _ => false
        });
        s.parse().unwrap()
    }

    fn parse_color(&mut self) -> Value {
        assert_eq!(self.consume_char(), '#');

        Value::ColorValue(Color {
            r: self.parse_hex_pair(),
            g: self.parse_hex_pair(),
            b: self.parse_hex_pair(),
            a: 255
        })
    }

    fn parse_identifier(&mut self) -> String {
        self.consume_while(valid_identifier_char)
    }

    fn parse_hex_pair(&mut self) -> u8 {
        let s = &self.input[self.pos.. self.pos + 2];
        self.pos += 2;
        return u8::from_str_radix(s, 16).unwrap();
    }

    /// Consume and discard zero or more whitespace characters.
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    /// Consume characters until `test` returns false.
    fn consume_while<F>(&mut self, c: F) -> String
        where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && c(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    /// Return the current character, and advance self.pos to the next character.
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    /// Read the current character without consuming it.
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    /// Return true if all input is consumed.
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

fn valid_identifier_char(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
        _ => false
    }
}
