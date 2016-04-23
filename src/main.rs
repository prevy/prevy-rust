// External crates
extern crate ansi_term;
extern crate atty;
extern crate clap;
extern crate xdg_basedir;
extern crate yaml_rust;

// Project modules
mod context;
mod core;
mod display;
mod utils;

/// Run prevy.
fn main() {
    // Build the context
    let ctx = context::build_context();

    // Print the context if debug is enabled
    if ctx.config.debug {
        display::debug(&format!("{:#?}", ctx));
    }

    // Starting from here, we will work from workspace root
    context::workspace::cd_workspace_root(&ctx);
}
