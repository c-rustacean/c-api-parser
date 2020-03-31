use pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::{env, fs};

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
    fn define_multiline() {
        let test_input = "#define FOO \
        typedef struct { \
            struct { \
                uint8_t foo; \
                uint32_t bar; \
            } subfield; \
            int8_t baz; \
        } outer;
        ";
        let parse = ApiParser::parse(Rule::p_define, test_input);
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

    #[test]
    fn typedef_simple() {
        let parse = ApiParser::parse(Rule::typedef, "typedef uint32_t myType;");
        assert!(dbg!(parse).is_ok());
    }

    #[test]
    fn typedef_union() {
        let parse = ApiParser::parse(
            Rule::typedef,
            "typedef union tag { uint32_t u32_var1; } myunion;",
        );
        // let parse = ApiParser::parse(Rule::rhs_typedef_complex, "union tag { uint32_t u32_var1; } myunion");
        assert!(dbg!(parse).is_ok());
    }

    #[test]
    fn typedef_struct() {
        let parse = ApiParser::parse(
            Rule::typedef,
            "typedef struct tag { uint32_t u32_var1; } mystruct;",
        );
        // let parse = ApiParser::parse(Rule::rhs_typedef_complex, "union tag { uint32_t u32_var1; } myunion");
        assert!(dbg!(parse).is_ok());
    }

    #[test]
    fn typedef_array_u8_numeric_index() {
        let test_input = "typedef uint8_t byte[4];";
        let parse = ApiParser::parse(Rule::typedef, test_input);
        assert!(dbg!(parse).is_ok());
    }

    #[test]
    fn typedef_array_u8_numeric_index_suf() {
        let test_input = "typedef uint8_t byte[4U];";
        let parse = ApiParser::parse(Rule::typedef, test_input);
        assert!(dbg!(parse).is_ok());
    }

    #[test]
    fn typedef_array_u8_numeric_index_suf_parens() {
        let test_input = "typedef uint8_t byte[(4U)];";
        let parse = ApiParser::parse(Rule::typedef, test_input);
        assert!(dbg!(parse).is_ok());
    }


    #[test]
    fn array_index_numeric() {
        let parse = ApiParser::parse(Rule::array_index, "34");
        assert!(dbg!(parse).is_ok());
    }

    #[test]
    fn array_index_define() {
        let parse = ApiParser::parse(Rule::array_index, "FOO");
        assert!(dbg!(parse).is_ok());
    }

    #[test]
    fn array_alpha_index() {
        let test_input = "[ VAL ]   ; ";
        let parse = ApiParser::parse(Rule::array_suffix, test_input);
        assert!(dbg!(parse).is_ok());
    }

    #[test]
    fn typedef_array_u8_define_index() {
        let test_input = "typedef uint8_t byte[VAL];";
        let parse = ApiParser::parse(Rule::typedef, test_input);
        assert!(dbg!(parse).is_ok());
    }

    #[test]
    fn typedef_struct_multifield_no_tag() {
        let parse = ApiParser::parse(
            Rule::typedef,
            "typedef struct { \n uint32_t u32_var1; \n uint8_t bla; \n float f; \n } \n mystruct ;",
        );
        assert!(dbg!(parse).is_ok());
    }

    #[test]
    fn typedef_struct_in_struct() {
        let test_input = "typedef struct {
            struct {
                uint8_t foo;
                uint32_t bar;
            } subfield;
            int8_t baz;
        } outer;";
        let parse = ApiParser::parse(Rule::typedef, test_input);
        assert!(dbg!(parse).is_ok());
    }

    #[test]
    fn simple_h() {
        let test_input = "
        #define FOO
        #define BAZ FOO
        #define VAL 3

        typedef uint8_t byte;

        typedef uint16_t array16_val[VAL];

        typedef struct {
            struct {
                uint8_t foo;
                uint32_t bar;
            } subfield;
            int8_t baz;
        } outer;

        typedef uint8_t arr_u8_3[3];";
        let parse = ApiParser::parse(Rule::file, test_input);
        assert!(dbg!(parse).is_ok());
    }

    #[test]
    fn filename() {
        let test_input = "foo\\bar.h";
        let parse = ApiParser::parse(Rule::filename, test_input);
        assert!(dbg!(parse).is_ok());
    }

    #[test]
    fn preproc_line() {
        let test_input = "#line 1 \"..\\\\..\\\\minimal_api_user.c\"";
        let parse = ApiParser::parse(Rule::p_line, test_input);
        assert!(dbg!(parse).is_ok());
    }

    #[test]
    fn pragma_line() {
        let test_input = "#pragma foo bar\n";
        let parse = ApiParser::parse(Rule::p_line, test_input);
        assert!(dbg!(parse).is_ok());
    }

    // TODO: Support #includes(?)
}

fn get_complex_type_names(input_h: &str) -> Vec<String> {

    let unparsed_file = fs::read_to_string(input_h).expect("Could not read input header");

    let h_file = ApiParser::parse(Rule::file, &unparsed_file)
        .expect("Could not parse input .h file")
        .next()
        .expect("Could not get the root pair of the input file")
        .into_inner();


    let mut complex_c_types: Vec<String> = Vec::new();

    for pair in h_file {
        // dbg!(&pair, "\n");
        match pair.as_rule() {
            Rule::typedef => {
                let mut rhs = pair.into_inner().next().unwrap().into_inner();

                let old_type_pair = rhs.next().unwrap();
                let type_name: &str = rhs.next().unwrap().as_str();
                println!("typedef '{}' of type '{}'\n", type_name, old_type_pair.as_str());

                if let Rule::complex_type_def = old_type_pair.as_rule() {
                    complex_c_types.push(type_name.to_string());
                }
            },
            _x => () /* println!("   Ignored rule {:?}\n", _x) */ ,
        };
    }

    dbg!(complex_c_types)

}

fn test_file(complex_types: Vec<String>, input_h: &str) -> String {

    // first part of file
    let mut c_test_contents = format!(
        r#"#include "{}"

        int main() {{
            struct {{
        "#, input_h);
    for t in &complex_types {
        c_test_contents.push_str(&(format!("   {0} {0}_m;\n", t)));
    }
    c_test_contents.push_str("} members_t;\n");

    c_test_contents
        .push_str("\n\n    printf(\"\\nTOTAL\t | %d\\n\", sizeof(members_t));\n\n");

    for t in complex_types {
        c_test_contents
            .push_str(&(format!("    printf(\"{0}\t | %d\\n\", sizeof({0});\n", t)));
    }
    c_test_contents.push_str("}");

    c_test_contents
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // dbg!(&args, &args[1]);
    let input_h = &args[1];
    let complex_c_types = get_complex_type_names(&input_h);
    let c_test_contents = test_file(complex_c_types, &input_h);

    // dbg!(c_test_contents);

    println!("{}", c_test_contents);

}
