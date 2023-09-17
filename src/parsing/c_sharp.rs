
use std::str::Chars;

use renounce::*;
use structuralize::data::*;


pub fn parse(input : &str) -> Result<Data, Box<str>> {

    Err("!".into())
}

macro_rules! opt {
    ($parser : ident => $optional : ident) => {
        fn $optional(input : &mut Chars) -> Result<Option<Data>, ParseError> {
            Ok(Some($parser(input)?))
        }
    };
}

fn parse_c_sharp(input : &mut Chars) -> Result<Data, ParseError> {
    opt!(parse_keyword => o_parse_keyword);
    opt!(parse_id => o_parse_id);
    fn parse_item(input : &mut Chars) -> Result<Option<Data>, ParseError> {
        alt!(input => o_parse_keyword
                    ; o_parse_id 
                    )
    }

    parser!(input => {
        items <= * parse_item;
        select Data::List(items.into_iter().filter_map(|x| x).collect())
    })
}

// TODO punctuation
// TODO literals?
// TODO <>

pat!(parse_any<'a>: char => char = x => x);

fn parse_word(input : &mut Chars) -> Result<Box<str>, ParseError> {
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
        init <= parse_init_id_char;
        rest <= * parse_rest_id_char;
        select {
            let mut rest = rest;
            rest.insert(0, init);
            rest.into_iter().collect::<String>().into()
        }
    })
}

fn parse_id(input : &mut Chars) -> Result<Data, ParseError> {
    // TODO unicode escape 
    pat!(parse_at: char => () = '@' => { () });

    parser!(input => {
        at <= ? parse_at;
        let at : Option<()> = at;
        word <= parse_word;
        select Data::Cons { name: "id".into(), params: vec![Data::String(word)] }
    })
}

fn parse_keyword(input : &mut Chars) -> Result<Data, ParseError> {
    parser!(input => {
        word <= parse_word;
        where KEYWORDS.iter().find(|x| ***x == *word).is_some();
        select Data::Cons { name: "keyword".into(), params: vec![Data::String(word)] }
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