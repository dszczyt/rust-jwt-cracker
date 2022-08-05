# Rust JWT Cracker

Fast JSON Web Token cracker using bruteforce technique, written in Rust.
Currently supports HS256.

# Building

## from source

Use

```
$ cargo build -r
```

# Running

Help file:

```
USAGE:
    rust-jwt-cracker [OPTIONS] --token <TOKEN>

OPTIONS:
        --alphabet <ALPHABET>        [default:
                                     abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789]
    -h, --help                       Print help information
    -l, --max-length <MAX_LENGTH>    [default: 6]
        --token <TOKEN>
    -V, --version                    Print version information
```

Example:

```
$ ./rust-jwt-cracker --token eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.y3kjst36zujMF4HssVk3Uqxf_3bzumNAvOB9N0_uRV4 --alphabet secrt123 -l 10
```

(this example takes ~31s on a Mac M1)
