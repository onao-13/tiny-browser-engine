fn main(){
    let sources = Vec::from([
        String::from("resources/test.html"),
        String::from("resources/test.css")
    ]);
    let parser = parser_page::parser::new_parser(sources);
    let page = parser.parse();
    page.print_res();
}
