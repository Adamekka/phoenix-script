pub fn get_arguments() -> clap::ArgMatches {
    clap::Command::new("ph")
        .about("Phoenix Script")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Adamekka")
        .subcommand(
            clap::Command::new("build")
                .about("Builds the project")
                .visible_alias("b")
                .arg(
                    clap::Arg::new("file")
                        .required(true)
                        .value_hint(clap::ValueHint::FilePath),
                ),
        )
        .get_matches()
}
