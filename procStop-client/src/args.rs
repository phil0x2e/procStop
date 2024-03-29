use clap::{crate_authors, crate_version, App, Arg};
use shellexpand::tilde;

pub struct Args {
    pub config_path: String,
}

pub fn get_commandline_args() -> Args {
    let description = "Stop procrastinating now!\n
This is the client software for procStop. You'll also need to run the web software.
For more infos see https://github.com/phil0x2e/procStop";
    let default_conf = tilde("~/.config/procstop/config.toml").to_string();

    let matches = App::new("ProcStop-Client")
        .version(crate_version!())
        .author(crate_authors!())
        .about(description)
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("CONFIG")
                .help("Path to the toml config file.")
                .takes_value(true)
                .default_value(&default_conf),
        )
        .get_matches();
    Args {
        config_path: String::from(matches.value_of("config").unwrap()),
    }
}
