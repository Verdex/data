
use std::str::Chars;

use renounce::*;
use structuralize::data::*;


pub fn parse(input : &str) -> Result<Data, Box<str>> {

    Err("!".into())
}

// TODO keywords
// TODO punctuation
// TODO literals?
// TODO <>

pat!(parse_any<'a>: char => char = x => x);

fn parse_id(input : &mut Chars) -> Result<Data, ParseError> {
    // TODO unicode escape 
    pat!(parse_at: char => () = '@' => { () });

    fn parse_init_id_char(input : &mut Chars) -> Result<char, ParseError> {
        parser!(input => {
            any <= parse_any;
            where any.is_alphabetic() || any == '_';
            select any
        })
    }

    fn parse_rest_id_char(input : &mut Chars) -> Result<char, ParseError> {
        parser!(input => {
            any <= parse_any;
            where any.is_alphanumeric() || any == '_';
            select any
        })
    }

    parser!(input => {
        at <= ? parse_at;
        let at : Option<()> = at;
        init <= parse_init_id_char;
        rest <= * parse_rest_id_char;
        select {
            let mut rest = rest;
            rest.insert(0, init);
            let s : Box<str> = rest.into_iter().collect::<String>().into();
            Data::Cons { name: "id".into(), params: vec![Data::String(s)] }
        }
    })
}

static KEYWORDS : [&'static str; 120] =
    [   
        "as",
        "by",
        "do",
        "if",
        "in",
        "is",
        "on",
        "or",
        "add",
        "and",
        "for",
        "get",
        "int",
        "let",
        "new",
        "not",
        "out",
        "ref",
        "set",
        "try",
        "var",
        "args",
        "base",
        "bool",
        "byte",
        "case",
        "char",
        "else",
        "enum",
        "file",
        "from",
        "goto",
        "init",
        "into",
        "join",
        "lock",
        "long",
        "nint",
        "null",
        "this",
        "true",
        "uint",
        "void",
        "when",
        "with",
        "alias",
        "async",
        "await",
        "break",
        "catch",
        "class",
        "const",
        "event",
        "false",
        "fixed",
        "float",
        "group",
        "nuint",
        "sbyte",
        "short",
        "throw",
        "ulong",
        "using",
        "value",
        "where",
        "while",
        "yield",
        "double",
        "equals",
        "extern",
        "global",
        "nameof",
        "object",
        "params",
        "public",
        "record",
        "remove",
        "return",
        "scoped",
        "sealed",
        "select",
        "sizeof",
        "static",
        "string",
        "struct",
        "switch",
        "typeof",
        "unsafe",
        "ushort",
        "checked",
        "decimal",
        "default",
        "dynamic",
        "finally",
        "foreach",
        "managed",
        "notnull",
        "orderby",
        "partial",
        "private",
        "virtual",
        "abstract",
        "continue",
        "delegate",
        "explicit",
        "implicit",
        "internal",
        "operator",
        "override",
        "readonly",
        "required",
        "volatile",
        "ascending",
        "interface",
        "namespace",
        "protected",
        "unchecked",
        "unmanaged",
        "descending",
        "stackalloc",
    ];

#[cfg(test)]
mod test {
    use super::*;

    use structuralize::pattern::*;
    use structuralize::pattern::check::*;

    #[test]
    fn parse_id_should_parse() {
        let input = "@_SomeInput786";
        let mut input = input.chars();
        let output = parse_id(&mut input).unwrap();

        let p : Pattern = "id(\"_SomeInput786\")".parse().unwrap();
        let p = check_pattern(p).unwrap();
        let results = pattern_match(&p, &output);

        assert_eq!( results.len(), 1 );
    }

}