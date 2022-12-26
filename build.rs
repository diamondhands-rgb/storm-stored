// Storage daemon (stored): microservice frontend for different storage backends
// used in LNP/BP nodes.
//
// Written in 2022 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2022 by LNP/BP Standards Association, Switzerland.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::{env, fs};

use clap::IntoApp;
use clap_complete::generate_to;
use clap_complete::shells::*;

pub mod stored {
    include!("src/opts.rs");
}

fn main() -> Result<(), configure_me_codegen::Error> {
    if env::var("DOCS_RS").is_err() {
        let outdir = "./shell";
        fs::create_dir_all(outdir).expect("failed to create shell dir");
        let mut app = stored::Opts::command();
        let name = app.get_name().to_string();
        generate_to(Bash, &mut app, &name, outdir)?;
        generate_to(PowerShell, &mut app, &name, outdir)?;
        generate_to(Zsh, &mut app, &name, outdir)?;

        // configure_me_codegen::build_script_auto()
    }

    Ok(())
}
