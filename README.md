<p align="center">
  <img src="ishmael.png">
</p>

## Overview

Ishmael is command line application made with Rust. It allows the Whales ecosystem to look repos in github and tries to dockerize them.

## Usage
Basically, you need to first install the binary:

```bash
$ cargo install
```

That will install the binary `ishmael`. Then you can just run:

```bash
$ ishmael
```
And see the magic happen. 

### WANTED
- [ ] Take argument to change `keyword` for search.

## Development

As you can see, there are Docker files to easy development. 

To start play around
```bash
$ docker-compose run --rm console cargo run
```
You will see Ishmael in action.

To run some tests
```bash
$ docker-compose run --rm console cargo test
```

Basically, the usual thing you do with `cargo` happens here too, we just use Docker to start development as quickly as possible without worrying about the dependencies and/or installation compatibility.

