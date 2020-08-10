# awaitchange

Command line tool that does little, is easy to use and helps a lot!

## Usage

```
awaitchange 0.2.0

USAGE:
    awaitchange [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r <checkrate>            How many times per second files should get checked for updates [default: 2]
        --do <command>...     Command to be executed on filechange. If unset, awaitchange simply exits on filechange and
                              yields controll to the programm next in line [default: ]
        --files <files>...    Files to be watched
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

        awaitchange --files deployment.yaml service.yaml
    done
```


Now, since 1.2.0, something like this can be done a little easier:

```sh
awaitchange --files deployment.yaml --do kubectl apply -f deployment.yaml
```
