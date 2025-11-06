
# What and why
This macro makes it really easy to make simple CLI programs. It handles keeping your help and version functions up to date, and even generates zsh completions for you.

# Currently unsupported

 - Second level arguments

 For example, '--run command', where the --run option is given its own argument.


 - Composite arguments

 For example, 'tar -zxf', where z x and f are all part of the same argument but have different effects. We treat all single dash arguments as a shortened version of a longer dash argument.

# Usage:

    cli_builder! {
        [
            CLICommand {
                short_flag: "t",
                long_flag: "test",
                command: test_command,
                description: "Run a test"
            },
        ]
    }

    fn test_command() {
        println!("This is a test");
    }
