mod generated {
    cli_builder_macros::cli_builder! {
        default = run;
        commands = [
            CLICommand {
                short_keyword: Some("r"),
                keyword: "run",
                description: "runs the test command",
                command: run,
            },
        ];
        flags = [
            CLIFlag {
                short_flag: "o",
                long_flag: "output",
                description: "sets the output file",
                argument_name: Some("file"),
            },
        ];
    }

    fn run(_: &::std::vec::Vec<CLIFlagType>) {}
}

#[test]
fn cli_builder_expands_commands_flags_and_default() {
    let runtime = generated::Runtime::new();

    assert_eq!(runtime.commands.len(), 4);
    assert_eq!(runtime.commands[3].keyword, "run");
    assert_eq!(runtime.flags.len(), 1);
    assert_eq!(runtime.flags[0].argument_name, Some("file"));
    assert!(runtime.default_command.is_some());
}

#[test]
fn help_includes_commands_and_flags() {
    let help = generated::Runtime::new().gen_help();

    assert!(help.contains("run/r: runs the test command"));
    assert!(help.contains("-o, --output: sets the output file"));
    assert!(help.contains("argument: file"));
}

#[test]
fn completions_include_commands_and_argument_flags() {
    let completions = generated::Runtime::new().gen_completions();

    assert!(completions.contains("*:command:(("));
    assert!(completions.contains("run\\:runs\\ the\\ test\\ command"));
    assert!(completions.contains("r\\:runs\\ the\\ test\\ command"));
    assert!(completions.contains(
        "(-o --output){-o,--output}[sets\\ the\\ output\\ file]:file:"
    ));
    assert!(completions.contains("_arguments $args"));
}

#[test]
fn version_uses_package_metadata() {
    assert_eq!(
        generated::Runtime::gen_version(),
        format!(
            "{} version {} by {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_AUTHORS"),
        )
    );
}
