use pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "c-api-header.pest"]
pub struct ApiParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn define_no_val() {
        let parse = ApiParser::parse(Rule::p_define, "#define Foo\n");
        assert!(parse.is_ok());
    }

    #[test]
    fn define_no_space_no_val() {
        let parse = ApiParser::parse(Rule::p_define, "#defineFoo\n");
        assert!(parse.is_err());
    }

    #[test]
    fn extern_c_is_ok() {
        let parse = ApiParser::parse(Rule::extern_c, "extern \"C\" { \n");
        assert!(parse.is_ok());
    }

    #[test]
    fn define_foo_no_val_is_ok() {
        let parse = ApiParser::parse(Rule::p_define, "#define FOO\n");
        assert!(parse.is_ok());
    }

    #[test]
    fn define_foo_with_val_is_ok() {
        let parse = ApiParser::parse(Rule::p_define, "#define FOO ((123) + 4) \n");
        assert!(parse.is_ok());
    }

    #[test]
    fn c_style_comments_simple() {
        let parse = ApiParser::parse(Rule::comment, "/* foo /* bar */ baz \n");
        assert!(dbg!(parse).is_ok());
    }

    #[test]
    fn cpp_style_comments_simple() {
        let parse = ApiParser::parse(Rule::comment, "//* foo /* bar */ baz \n");
        assert!(parse.is_ok());
    }
}

fn main() {
    let parse = ApiParser::parse(Rule::comment, "/* cdjhf * */ bla");
    println!("comment = {:?}", parse);

    let parse = ApiParser::parse(Rule::comment, "// fds  cdjhf * */ bla\n foo");
    println!("comment = {:?}", parse);

    let parse = ApiParser::parse(Rule::ident, "cdjhf_0");
    println!("{:?}", parse);

    let parse = ApiParser::parse(Rule::extern_c, "extern \"C\" { \n hf_0");
    println!("{:?}", parse);

    let parse = ApiParser::parse(Rule::p_define, "#define Foo\n");
    println!("{:?}", parse);
}
