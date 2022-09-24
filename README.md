# <h1 align="center"> exfac.rs </h1>

*Rust bindings & CLI to the ExFac API*

## CLI Usage

Run `cargo r -- --help` to get the top level help menu:

```
exfac 0.1.0 (9318c15 2022-09-24T00:04:11.379591Z)
1-click deploy infrastructure for blockchains.

USAGE:
    exfac [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -a, --api-key <API_KEY>    Your ExFac API key [default:
                               5580b8eb-0d8f-482e-936b-335f2ff6332d]
    -h, --help                 Print help information
    -u, --url <URL>            The URL pointing to the ExFac API [default:
                               http://localhost:8080]
    -V, --version              Print version information

SUBCOMMANDS:
    completions    Generate shell completions script. [aliases: com]
    help           Print this message or the help of the given subcommand(s)
    network        Commands about interacting with the various networks you
                       have spinned up
    user           Get information about your current session

Find more information in at: https://app.exfac.xyz
```
