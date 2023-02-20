use std::{collections::HashSet, fs, path::Path};

mod generate;
mod parse;
mod types;

use generate::def_to_string;
use types::*;

pub type Database = HashSet<Definition>;

pub fn generate_html(path: &Path) -> types::Result<String> {
    let database = parse(path)?;

    let mut output = String::new();
    output.push_str(
        "/// An HTML Element
        pub trait HtmlElement: ::std::fmt::Display {}\n
        ",
    );
    for entry in database {
        output.push_str(&def_to_string(entry));
    }

    // missing types in the IDL files!
    output.push_str(
        "#[derive(Default, Debug, PartialEq, Clone)]
        pub struct FileList {}
        
        #[derive(Default, Debug, PartialEq, Clone)]
        pub struct EventTarget {}
        ",
    );
    Ok(output)
}

pub fn parse(path: &Path) -> types::Result<Database> {
    let iter = fs::read_dir(path)?
        .into_iter()
        .map(|path| fs::read_to_string(path?.path()));
    let database = parse::parse_webidl(iter)?;

    Ok(database)
}
