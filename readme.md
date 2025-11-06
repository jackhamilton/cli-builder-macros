
# What and why
This macro makes it really easy to make simple CLI programs. It handles keeping your help and version functions up to date, and even generates zsh completions for you.

# Currently unsupported

 - Second level arguments

 For example, '--run command', where the --run option is given its own argument.


 - Composite arguments

 For example, 'tar -zxf', where z x and f are all part of the same argument but have different effects. We treat all single dash arguments as a shortened version of a longer dash argument.

# Usage:

```rust
cli_builder! {
    [
        // Optional (sets what runs if no arguments are given, defaults to the builtin help function):
        // default = help,
        CLICommand {
            short_flag: "t",
            long_flag: "test",
            command: test_command,
            description: "Run a test"
        },
        // Any number of additional commands
    ]
}

fn test_command() {
    println!("This is a test");
}
```

Output:

```zsh
> cargo run -- -h
Help:
         -h, --help: prints help message (you are here)
         -v, --version: prints tool version
         -zsh, --completions: prints zsh completions for sourcing, add to your shell via e.g. "znap fpath _program_name 'program_name --completions'" for znap
         -t, --test: Run a test

> cargo run -- -t
This is a test
```

A main function will be generated for you, and routing to all cli commands handled from it. It expects static functions at the top level of the file - nothing with &self or &mut self. If you need a runtime object, it is suggested you create that either within the function context or as a static with a pointer.
