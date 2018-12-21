// use build_info::Version;
use build_info::{get_version, Version};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

pub fn get_matches() -> ArgMatches<'static> {
    let version = get_version!();

    App::new("ckb")
        .author("Nervos Core Dev <dev@nervos.org>")
        .about("Nervos CKB - The Common Knowledge Base")
        .version(version.short().as_str())
        .long_version(version.long().as_str())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("CONFIG")
            .takes_value(true)
            .help("Specify the configuration file PATH. Tries ckb.json, nodes/default.json in working directory when omitted.")
        )
        .subcommand(run())
        .subcommand(miner())
        .subcommand(export())
        .subcommand(import())
        .subcommand(cli())
        .get_matches()
}

fn run() -> App<'static, 'static> {
    SubCommand::with_name("run").about("Running ckb node")
}

fn miner() -> App<'static, 'static> {
    SubCommand::with_name("miner").about("Running ckb miner")
}

fn arg_format() -> Arg<'static, 'static> {
    Arg::with_name("format")
        .short("f")
        .long("format")
        .value_name("FORMAT")
        .required(true)
        .takes_value(true)
        .help("Specify the format.")
}

fn export() -> App<'static, 'static> {
    SubCommand::with_name("export")
        .about("Export ckb data")
        .arg(arg_format())
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .value_name("PATH")
                .required(true)
                .index(1)
                .help("Specify the export target path."),
        )
}

fn import() -> App<'static, 'static> {
    SubCommand::with_name("import")
        .about("Import ckb data")
        .arg(arg_format())
        .arg(
            Arg::with_name("source")
                .short("s")
                .long("source")
                .value_name("PATH")
                .required(true)
                .index(1)
                .help("Specify the exported data path."),
        )
}

fn cli() -> App<'static, 'static> {
    SubCommand::with_name("cli")
        .about("Running ckb cli")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("type_hash")
                .about("Generate lock script type hash using the first system cell, which by default is always_success"),
        )
        .subcommand(SubCommand::with_name("keygen").about("Generate new key"))
}
