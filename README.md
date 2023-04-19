# auth0m2m

A little command line tool to manage auth0 M2M tokens.

## Install 

Use `cargo install` with the `--git` flag to install this on your system.


## About 

Your m2m app should be configured to return a `Bearer` access token. 

```
Usage: auth0m2m <COMMAND>

Commands:
  new    Create a new credential set.
  fetch  Log yourself in to a configured application. Prints a token to stdout.
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Credentials are stored like this: 
`auth0m2m set myapp`

And after the command prompts are added here under `{HOME}`:
``` 
~/.auth0m2m/myapp/config.json
```

You call `login` to fetch your access token.

_NOTE_ that credentials are stored insecurely so be careful using this tool. It is ideally meant for debugging and running workflows in a local environment. 



