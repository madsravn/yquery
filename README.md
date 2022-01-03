# yquery

![example workflow](https://github.com/madsravn/yquery/actions/workflows/rust.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/d/yquery.svg)](https://crates.io/crates/yquery)
[![Documentation](https://docs.rs/yquery/badge.svg)](https://docs.rs/yquery/)

yquery is a program which finds elements in your yaml documents using selectors. Sort of like how jquery works for css.


## [API Documentation](https://docs.rs/yquery)


## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Common Issues](#common-issues)
- [Copyright and License](#copyright-and-license)


## Features

yquery is a command-line tool which currently accepts one file and a 'search query'. It will then search through the document looking for elements of the document which adhere to the search query. I am currently working on multi-file support so you can search directories as well.

### Examples

A 'search query' consists of three parts: A element name, a child specifier and id selection. It is given in the form `ElementA[childname=name].id`. You can have multiple occurrences of each part: `(ElementA|ElementB)[childname=name,childage=age].(id,name)`. If only the element name part is given, the output will the entire element found, as seen in the first example below. If the id selection part is specified, you will only receive those specifically, as seen in the second example below.

Example with no id specifier
```console
mads@Adria][~/projects/yquery]% yquery documents/verify_apache.yaml "service"
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


