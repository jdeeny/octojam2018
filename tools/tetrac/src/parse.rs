use nom::*;
use crate::{ Word, DefPiece } ;//, DictEntry, WordOrLiteral, Definition };

named!(pub tetra_source<&str, Vec<Word>>,
        many0!(
            complete!(
            do_parse!(
                many0!(is_a!(" \r\t\n")) >>
                many0!(comment) >>
                many0!(is_a!(" \r\t\n")) >>
                line: dict_entry >>
                many0!(is_a!(" \r\t\n")) >>
                many0!(comment) >>
                many0!(is_a!(" \r\t\n")) >>
                (line)
            )
        ))
);

named!(dict_entry<&str, Word>,
    ws!(
        alt_complete!(dict_entry_octocall | dict_entry_octoaddress | dict_entry_tetra )
    )
);

named!(dict_entry_tetra<&str, Word>,
    ws!(
        do_parse!(
            tag!(":") >>
            name: word >>
            value: many1!(word) >>
            tag!(";") >>
            ( Word { name: name, def: value.iter().map(|v| DefPiece::Tetra(v.clone())).collect() } )
        )
    )
);

named!(dict_entry_octocall<&str, Word>,
    ws!(
        do_parse!(
            tag!(":") >>
            name: word >>
            tag_no_case!("octo") >>
            value: word >>
            tag!(";") >>
            ( Word{ name: name, def: vec!(DefPiece::OctoCall(value)) } )
        )
    )
);

named!(dict_entry_octoaddress<&str, Word>,
    ws!(
        do_parse!(
            tag!(":") >>
            name: word >>
            tag_no_case!("addr") >>
            value: word >>
            tag!(";") >>
            ( Word{ name: name, def: vec!(DefPiece::OctoAddr(value)) } )
        )
    )
);


named!(word<&str, String>,
        complete!(do_parse!( value: is_not_s!(" \r\n\t:;") >> (String::from(value)) ))
);


/*named!(word_or_literal<&str, WordOrLiteral>,
    alt_complete!(
        do_parse!( value: literal >> (WordOrLiteral::L(value)) ) |
        do_parse!( value: word >> (WordOrLiteral::W(value)) )
    )
);*/


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
    alt_complete!( hex_literal | bin_literal | dec_literal )
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
    value!((),
        alt_complete!(
            terminated!(
                preceded!(tag!("#"), is_not!("\n")),
                alt_complete!(eof!() | tag!("\n"))
            ) |
            terminated!(tag!("#"),
                alt_complete!(eof!() | tag!("\n"))
            )

        )
    )
);
