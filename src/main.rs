extern crate ansi_term;
extern crate clap;
extern crate xdg_basedir;
extern crate yaml_rust;

mod context;
mod display;
mod errors;
mod workspace;

use clap::{App, Arg, ArgMatches};
use std::env;

/// Parse command line arguments.
fn parse_arguments<'a>() -> ArgMatches<'a> {
    App::new("prevy")
        .version("0.1.0")
        .author("Fabien Dubosson <fabien.dubosson@gmail.com>")
        .about("Manage your development workspaces with ease.")
        .arg(Arg::with_name("configuration_file")
                 .short("c")
                 .long("config")
                 .help("User configuration file")
                 .value_name("FILE")
                 .takes_value(true))
        .arg(Arg::with_name("workspace_filename")
                 .short("f")
                 .long("filename")
                 .help("Workspace filename")
                 .value_name("FILE")
                 .takes_value(true))
        .get_matches()
}

/// Run prevy.
fn main() {
    // Parse command line arguments
    let args = parse_arguments();

    // Parse the configuration
    let mut ctx = context::build_context(args);

    // Move to project root
    workspace::cd_workspace_root(&ctx);

    display::warn(&format!("{:?}", ctx));
}
