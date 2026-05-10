# awaitchange

Command line tool that does little, is easy to use and helps a lot!

## Usage

```
awaitchange 0.5.0

USAGE:
    awaitchange [OPTIONS] [--] [files]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r <checkrate>          How many times per second files should get checked for updates [default: 2]
    -e, --exec <exec>...    Command to be executed on filechange. If unset, awaitchange simply exits on filechange and
                            yields controll to the programm next in line. The special character {} can be inserted and
                            will be replaced by the name of the file that has changed upon execution

ARGS:
    <files>...    Files to be watched
```

## Example

lets say you want to update your kubernetes objects, every time you changed your deployment or service config,
you'd use awaitchange like this:
```sh
    while [ true ]
    do
        clear       # Clear terminal window
        kubectl apply -f deployment.yaml
        kubectl apply -f service.yaml

        awaitchange deployment.yaml service.yaml
    done
```

There is a shorthand for this, just use the `--exec` flag an pass a script, that should be executed (in `sh`)

```sh
awaitchange deployment.yaml --exec kubectl apply -f {}
```
