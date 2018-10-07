use nom::*;

use crate::{ Word };

use derive_more::{ From, Into };

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParsedDefinition{
    Tetra(Vec<Word>),
    OctoCall(Word),
    OctoAddr(Word)
}

#[derive(From, Into, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParsedDictEntry {
    key: Word,
    value: ParsedDefinition,
}


/*pub fn parse_tetra(source: &str) -> Result<Vec<ParsedDictEntry>, u32> {
    tetra_source(source)
}
*/

named!(pub(crate) tetra_source<&str, Vec<ParsedDictEntry>>,
    ws!(many0!(dict_entry))
);

named!(dict_entry<&str, ParsedDictEntry>,
    alt_complete!(dict_entry_tetra | dict_entry_octocall | dict_entry_octoaddress )
);

named!(dict_entry_tetra<&str, ParsedDictEntry>,
    do_parse!(
        ws!(tag!(":")) >>
        name: word >>
        value: ws!(many1!(word)) >>
        tag!(";") >>
        ( ParsedDictEntry{ key: name, value: ParsedDefinition::Tetra(value) } )
    )
);

named!(dict_entry_octocall<&str, ParsedDictEntry>,
    do_parse!(
        ws!(tag!(":")) >>
        name: word >>
        ws!(tag_no_case!("octo")) >>
        value: ws!(word) >>
        tag!(";") >>
        ( ParsedDictEntry{ key: name, value: ParsedDefinition::OctoCall(value) } )
    )
);

named!(dict_entry_octoaddress<&str, ParsedDictEntry>,
    do_parse!(
        ws!(tag!(":")) >>
        name: word >>
        ws!(tag_no_case!("data")) >>
        value: ws!(word) >>
        tag!(";") >>
        ( ParsedDictEntry{ key: name, value: ParsedDefinition::OctoAddr(value) } )
    )
);



named!(word<&str, Word>,
    do_parse!(
        value: ws!(is_not!(" \r\n\t")) >>
        (Word(String::from(value)))
    )
);


fn from_hex(input: &str) -> Result<usize, std::num::ParseIntError> {
  usize::from_str_radix(input, 16)
}

fn from_dec(input: &str) -> Result<usize, std::num::ParseIntError> {
  usize::from_str_radix(input, 16)
}

fn from_bin(input: &str) -> Result<usize, std::num::ParseIntError> {
  usize::from_str_radix(input, 10)
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
    alt!( hex_literal | dec_literal | bin_literal)
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
