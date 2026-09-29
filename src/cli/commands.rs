use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum Commands {

    Run {
        path: Option<PathBuf>
    },

    Create {
        project_name: String
    },

    Export {

        #[arg(short, long)]
        json: bool,

    }

}