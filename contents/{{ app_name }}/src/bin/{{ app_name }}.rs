use async_trait::async_trait;
use clap::{App, AppSettings, Arg};
use clap::{crate_name, crate_version, crate_authors, crate_description};
use log::{debug, trace, info};

use {{ app_name }}::{self};

#[tokio::main]
fn main() {
    let matches = App::new(&crate_name!()[..])
        .version(&crate_version!()[..])
        .author(&crate_authors!()[..])
        .about(&crate_description!()[..])
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::with_name("verbosity")
            .short("v")
            .long("verbose")
            .multiple(true)
            .global(true)
            .help("Increases the level of verbosity"))
        .get_matches();

    loggerv::init_with_verbosity(matches.occurrences_of("verbosity")).unwrap();

    debug!("Debug Level");
    trace!("Trace Level");
    info!("Info Level");
}
