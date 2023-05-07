mod args;
mod build;

fn main() {
    let args: clap::ArgMatches = args::get_arguments();

    match args.subcommand() {
        Some(("build", _)) => {
            build::build();
        }

        _ => {
            unreachable!("Subcommand is required")
        }
    }
}
