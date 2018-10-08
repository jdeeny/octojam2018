use nom::*;

use crate::{ Word };

use derive_more::{ From, Into };

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParsedDefinition{
    Tetra(Vec<WordOrLiteral>),
    OctoCall(WordOrLiteral),
    OctoAddr(WordOrLiteral)
}

#[derive(From, Into, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParsedDictEntry {
    key: Word,
    value: ParsedDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WordOrLiteral{
    W(Word),
    L(usize),
}

/*pub fn parse_tetra(source: &str) -> Result<Vec<ParsedDictEntry>, u32> {
    tetra_source(source)
}
*/

named!(pub tetra_source<&str, Vec<ParsedDictEntry>>,
        many0!(
            do_parse!(
                opt!(many0!(comment)) >>
                line: dict_entry >>
                opt!(many0!(comment)) >>
                (line)
            )
        )
);

named!(dict_entry<&str, ParsedDictEntry>,
    alt_complete!(dict_entry_octocall | dict_entry_octoaddress | dict_entry_tetra )
);

named!(dict_entry_tetra<&str, ParsedDictEntry>,
    ws!(
        do_parse!(
            tag!(":") >>
            name: word >>
            value: many1!(word_or_literal) >>
            tag!(";") >>
            ( ParsedDictEntry{ key: name, value: ParsedDefinition::Tetra(value) } )
        )
    )
);

named!(dict_entry_octocall<&str, ParsedDictEntry>,
    ws!(
        do_parse!(
            tag!(":") >>
            name: word >>
            tag_no_case!("octo") >>
            value: word_or_literal >>
            tag!(";") >>
            ( ParsedDictEntry{ key: name, value: ParsedDefinition::OctoCall(value) } )
        )
    )
);

named!(dict_entry_octoaddress<&str, ParsedDictEntry>,
    ws!(
        do_parse!(
            tag!(":") >>
            name: word >>
            tag_no_case!("addr") >>
            value: word_or_literal >>
            tag!(";") >>
            ( ParsedDictEntry{ key: name, value: ParsedDefinition::OctoAddr(value) } )
        )
    )
);


named!(word<&str, Word>,
        do_parse!( value: is_not_s!(" \r\n\t:;") >> (Word(String::from(value))) )
);


named!(word_or_literal<&str, WordOrLiteral>,
    alt_complete!(
        do_parse!( value: literal >> (WordOrLiteral::L(value)) ) |
        do_parse!( value: word >> (WordOrLiteral::W(value)) )
    )
);


fn from_hex(input: &str) -> Result<usize, std::num::ParseIntError> {
  usize::from_str_radix(input, 16)
}

fn from_dec(input: &str) -> Result<usize, std::num::ParseIntError> {
  usize::from_str_radix(input, 10)
}

fn from_bin(input: &str) -> Result<usize, std::num::ParseIntError> {
  usize::from_str_radix(input, 2)
}

named!(dec_literal<&str, usize>,
    map_res!(take_while!(is_dec_digit), from_dec)
);

named!(hex_literal<&str, usize>,
    do_parse!(
        tag_no_case!("0x") >>
        value: map_res!(take_while!(is_hex_digit), from_hex) >>
        (value)
    )
);

named!(bin_literal<&str, usize>,
    do_parse!(
        tag_no_case!("0b") >>
        value: map_res!(take_while!(is_bin_digit), from_bin) >>
        (value)
    )
);

named!(literal<&str, usize>,
    alt!( hex_literal | bin_literal | dec_literal )
);

fn is_hex_digit(c: char) -> bool {
  c.is_digit(16)
}
fn is_dec_digit(c: char) -> bool {
  c.is_digit(10)
}
fn is_bin_digit(c: char) -> bool {
  c.is_digit(2)
}


named!(comment<&str, ()>,
    ws!(
        value!((),
            delimited!(
                tag!("#"),
                opt!(
                    many_till!(take!(1), alt_complete!(tag!("\n") | eof!()))
                ),
                alt_complete!(tag!("\n") | eof!())
            )
        )
    )
);
