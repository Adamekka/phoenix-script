pub fn build(args: clap::ArgMatches) {
    // Get file to build
    let file: &String;
    if let Some(arg_match) = args.subcommand_matches("build") {
        file = arg_match
            .get_one::<String>("file")
            .expect("Failed to get file");
    } else {
        unreachable!("Subcommand is required");
    }

    println!("Building {}", file);
}
