use bitacross_cli::Cli;
use clap::Parser;

fn init() {
	let _ = env_logger::try_init();
}

#[test]
fn test_version() {
	init();

	let res = Cli::try_parse_from(vec!["placeholder_cli_path", "--version"]);
	let err = clap::Error::new(clap::error::ErrorKind::DisplayVersion);
	assert!(matches!(res, Err(err)));
}

#[test]
fn test_help() {
	init();

	let res = Cli::try_parse_from(vec!["placeholder_cli_path", "--help"]);
	let err = clap::Error::new(clap::error::ErrorKind::DisplayHelp);
	assert!(matches!(res, Err(err)));
}
