# rjo

[![Build Status](https://travis-ci.org/dskkato/rjo.svg?branch=master)](https://travis-ci.org/dskkato/rjo)
[![Build status](https://ci.appveyor.com/api/projects/status/9cu385votebtm92e/branch/master?svg=true)](https://ci.appveyor.com/project/dskkato/rjo)
[![Coverage Status](https://coveralls.io/repos/github/dskkato/rjo/badge.svg?branch=master)](https://coveralls.io/github/dskkato/rjo?branch=master)

A small utility to create JSON objects.

The origin of this package is [jpmens/jo](https://github.com/jpmens/jo), and was inspired by a Golang ported version , [skanehira/gjo](https://github.com/skanehira/gjo).

## Installation

Only installation from souce is supported. You need Rust 1.30 or higher. You can then use cargo to build everything.

Clone and specify local directory:

```sh
$ git clone https://github.com/dskkato/rjo.git
$ cd rjo
$ cargo install --path .
```

Or specify the repository directly:

```sh
$ cargo install --git https://github.com/dskkato/rjo.git
```

## Usage

Creating objects:
```
$ rjo -p name=jo n=17 parser=false
{
    "name": "jo",
    "n": 17,
    "parser": false
}
```

or, arrays:
```
$ seq 1 10 | rjo -a
[1,2,3,4,5,6,7,8,9,10]

$ rjo -p -a Rust 0 false
[
    "Rust",
    0,
    false
]
```

A more complex example:
```sh
$ rjo -p name=JP object=$(rjo fruit=Orange point=$(rjo x=10 y=20) number=17) sunday=false
{
    "name": "JP",
    "object": {
        "fruit": "Orange",
        "point": {
            "x": 10,
            "y": 20
        },
        "number": 17
    },
    "sunday": false
}
```

## See also
* [jo](https://github.com/jpmens/jo)
* [gjo](https://github.com/skanehira/gjo)

## License

MIT
