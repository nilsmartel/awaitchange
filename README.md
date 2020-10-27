# awaitchange

Command line tool that does little, is easy to use and helps a lot!

## Usage

```
awaitchange 0.2.1

USAGE:
    awaitchange [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r <checkrate>            How many times per second files should get checked for updates [default: 2]
        --do <command>...     Command to be executed on filechange. If unset, awaitchange simply exits on filechange and
                              yields controll to the programm next in line [default: ]
        --watch <files>...    Files to be watched

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

        awaitchange --watch deployment.yaml service.yaml
    done
```


Since 2.0.0, this can be done a little easier:

```sh
awaitchange --watch deployment.yaml --do kubectl apply -f deployment.yaml
```

## TODO

- Allow command to be executed in parrallel
