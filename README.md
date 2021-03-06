# Rust language support in Atom

[![apm](https://img.shields.io/apm/v/language-rust-redux.svg)](https://atom.io/packages/language-rust-redux) [![Travis CI](https://travis-ci.org/TheEnigmaBlade/atom-language-rust-redux.svg?branch=master)](https://travis-ci.org/TheEnigmaBlade/atom-language-rust-redux)

Adds syntax highlighting and snippets for [Rust](http://www.rust-lang.org/) files in [Atom](http://atom.io/).

Forked from the existing [atom-language-rust](https://github.com/zargony/atom-language-rust) due to lack of support.

## Install

Install the package `language-rust-redux` in Atom (Preferences->Packages) or Atom's package manager:

```bash
$ apm install language-rust-redux
```

## Key changes from fork

Previews taken with Firewatch syntax. More improvements to come.

- The latest syntax, such as the `?` operator
- Format macro syntax highlighting<br>
  ![](http://i.imgur.com/mUlh8P0.png)
- Markdown syntax highlighting in doc comments<br>
  ![](http://i.imgur.com/JDSoPSQ.png)
- Invalid syntax common in similar languages<br>
  ![](http://i.imgur.com/KsS24Di.png)<br>
  ![](http://i.imgur.com/0C3xdPv.png)
- Common mistake recognition<br>
  ![](http://i.imgur.com/kPhbuE7.png)
- Improved keyword context (`where` actually works, `unsafe` allowed in more places)
- Numerous fixes: lifetimes in associated type definitions, `fn` in function arguments, and nested block comments

## Bugs and suggestions

Because this is a fork, there may be bugs I haven't noticed from the original version. Please submit an issue or pull request to get them fixed.

If you have any suggestions for improvement, please submit an issue with a full description and example code for when your suggestion should apply.
