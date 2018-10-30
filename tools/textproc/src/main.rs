
use std::fs::File;
use nom::*;


fn main() {
    println!("Hello, world!");
    let mut header_dest = File::create("build/text_header.o8").unwrap();
    let mut data_dest = File::create("build/text_data.o8").unwrap();

    let mut s = String::new();

    let result = File::open("../../assets/text/strings.txt").unwrap().read_to_string(&mut s).unwrap();
    println!("{} chars", result);
    // load string file
    // parse


}

enum WordDef {
    Word(String, String),
    Choice(Vec<(f32, WordDef)>),
}


// Parse all the text items in the string
named!(pub text_list<&str, Vec<WordDef>>,
        many0!(
            complete!(
            do_parse!(
                many0!(is_a!(" \r\t\n")) >>
                many0!(comment) >>
                many0!(is_a!(" \r\t\n")) >>
                line: definition >>
                many0!(is_a!(" \r\t\n")) >>
                many0!(comment) >>
                many0!(is_a!(" \r\t\n")) >>
                (line)
            )
        ))
);



named!(definition<&str, WordDef>,
    ws!(
        alt_complete!(definition_word | defintion_choice )
    )
);


named!(definition_word<&str, WordDef::Word>,
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

named!(definition_choice<&str, WordDef::Choice>,
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
