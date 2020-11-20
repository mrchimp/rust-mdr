# Rust Markdown Reader

It's written in Rust and it reads Markdown.

## Project Status

It kind of works! Could probably be better. I've never made anything with Rust before.

## Why?

Readmes are good. They should be read. What if there was a really quick way to read them. A way not much slower than `cat` but with colours and stuff? That'd be neat.

This is a remake of [mdr](https://github.com/mrchimp/mdr) which I slapped together in Node. That was fine but slow to launch.

## Usage

```bash
# Open README.md from the current directory
mdr

# Open test.md
mdr -f test.md
mdr --file test.md

# Open README.md from this Github repo. Meta!
mdr -g mrchimp/rust-mdr
mdr --github mrchimp/rust-mdr

# Open README.md from a Bitbucket repo
mdr -b somebitbucket/repo
mdr --bitbucket somebitbucket/repo

# Open README.md from another branch
mdr -g mrchimp/rust-mdr -b other_branch
mdr -g mrchimp/rust-mdr --branch other_branch
```

## Todo

- Check for non-200 statuses when getting remote files.
- Async remote requests
- Optimise some stuff?
- Make code better?
- Maybe just use [minimad](https://docs.rs/minimad/0.6.7/minimad/) instead of [termimad](https://github.com/Canop/termimad)?
