# yquery

[![Build status](https://img.shields.io/github/workflow/status/madsravn/yquery/Rust/master)](https://github.com/madsravn/yquery)
[![crates.io](https://img.shields.io/crates/v/yquery.svg)](https://crates.io/crates/yquery)

yquery is a program which finds elements in your yaml documents using selectors. Sort of like how jquery works for css.


## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Common Issues](#common-issues)
- [Copyright and License](#copyright-and-license)


## Features

yquery is a command-line tool which currently accepts one file and a 'search query'. It will then search through the document looking for elements of the document which adhere to the search query. I am currently working on multi-file support so you can search directories as well.

### Examples

A 'search query' consists of four parts: A parent name, an element name, a child specifier and id selection. It is given in the form `<Parent>ElementA[childname=name].id`. You can have multiple occurrences of each part: `(<Parent>ElementA|<Parent>ElementB)[childname=name,childage=age].(id,name)`. If only the element name part is given, the output will the entire element found, as seen in the first example below. If the id selection part is specified, you will only receive those specifically, as seen in the second example below.

Example with no id specifier
```console
[mads@Adria][~/projects/yquery]% yquery documents/verify_apache.yaml "service"
service: { name: httpd, state: started}
service: { name: httpd, state: restarted}
```


Example with id specifier set
```console
[mads@Adria][~/projects/yquery]% yquery documents/verify_apache.yaml "service.name"
httpd
httpd
```

## Installation

Currently yquery does not reside in any package manager that I know of. So in order to install it you need to build the tool yourself.

### Building
You can also build and install from source (requires the latest stable [Rust] compiler.)
```console
cargo install --git https://github.com/madsravn/yquery.git yquery
```

[rust]: https://www.rust-lang.org

## Common Issues

Some issues can arise from the grammar I made to parse the search query. If you are in doubt whether or not your query was understood correctly, run the program with a third parameter. Then it will output how it understands your search query.

## Copyright and License

This tool and its source is licensed under [GNU GPLv3](https://www.gnu.org/licenses/gpl-3.0.en.html)

