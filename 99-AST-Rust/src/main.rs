use std::{env, fs};

use eyre::{self, OptionExt};
use syn;

fn main() -> eyre::Result<()> {
    let filename = env::args().nth(1).ok_or_eyre("No filename was provided")?;
    let code = fs::read_to_string(filename)?;

    let code_ast = syn::parse_file(&code)?;

    println!("{:#?}", code_ast);

    Ok(())
}
