extern crate clap;

use clap::{App, AppSettings, SubCommand};

use library::{Library, LibraryResult};

static LIBFILE_PATH: &'static str = "library.json";

fn main() -> LibraryResult<()> {
    let matches = App::new("Library")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version("1.0")
        .author("Cameron Phillips <cameron0505@gmail.com>")
        .about("Library stock management")
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialise this directory with a new library file"))
        .subcommand(
            SubCommand::with_name("list")
                .about("List all collected books"))
    .get_matches();


    if let Some(_matches) = matches.subcommand_matches("init") {
        Library::new().save(LIBFILE_PATH)?;
        println!("{} initialised", LIBFILE_PATH);
    } else if let Some(_matches) = matches.subcommand_matches("list") {
        let library = Library::from_file(LIBFILE_PATH)?;
        println!("{} loaded", LIBFILE_PATH);
        println!("Book listing: \n");
        for book in library.iter_books() {
            println!("{:?}", book);
        }
    }

    Ok(())
}
