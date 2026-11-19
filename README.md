# What and why

This macro makes it really easy to make simple CLI programs. It handles keeping your help and version functions up to date, and even generates zsh completions for you.

# Generated command syntax

We generate a CLI with the following basic rules:
- Runnable code, e.g. commands, are alphanumeric nouns with no flag syntax. ex: "ProgramName run"
- Short flags have one dash, ex: "ProgramName -h"
- Long flags have two dashes, ex: "ProgramName --help"
- All programs have 'help, version, completions' generated for you as default commands.
    - As such, these are reserved as function names in your program because the macro generates them.

We generate a main function for your program, which we use to parse args and trigger functions. Any code you want to be part of the program should be exposed by providing a command that maps to the function you want to run, which should accept a `&Vec<CLIFlagType>` to read in any flags the user passes when calling it. All flags are universal.

Argument-taking flags are represented by `CLIFlagType::ArgumentedFlag(flag, value)`; other flags use `CLIFlagType::UnargumentedFlag(flag)`.

# Usage

```rust
cli_builder! {
    commands = [
        CLICommand {
            short_keyword: Some("t"),
            keyword: "test",
            command: test_command,
            description: "Run a test",
        },
    ];
    flags = [];
}

fn test_command(_runtime_flags: &Vec<CLIFlagType>) {
    println!("This is a test");
}
```

When run without arguments, it will trigger the built-in default, which is 'help':

```text
$ cargo run --
------------------------ ProgramName Help ------------------------
Commands:
        help/h: prints help message (you are here)
        version/v: prints tool version
        completions: prints zsh completions for compinit, add to your shell by using something like znap's fpath command
        test/t: Run a test

$ cargo run -- t
This is a test
```

## With flags

```rust
cli_builder! {
    commands = [
        CLICommand {
            short_keyword: Some("t"),
            keyword: "test",
            command: test_command,
            description: "Run a test",
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

fn test_command(runtime_flags: &Vec<CLIFlagType>) {
    let flags = runtime_flags
        .iter()
        .map(|flag| match flag {
            CLIFlagType::UnargumentedFlag(flag) => {
                format!("--{}", flag.long_flag)
            }
            CLIFlagType::ArgumentedFlag(flag, value) => {
                format!("--{} {}", flag.long_flag, value)
            }
        })
        .collect::<Vec<_>>()
        .join(", ");

    println!("You passed the following flags: {}", flags);
}
```

Output:

```text
$ cargo run -- t --output output.txt
You passed the following flags: --output output.txt
```
