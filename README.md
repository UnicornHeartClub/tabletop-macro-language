# TableTop Macro Language

A language and parser to add advanced functionality to tabletop role-playing games.

## What's a Macro?

Macros allow users to define custom actions in tabletop role-playing games to issue chat commands,
dice rolls, automate attacks, calculate damages, roll initiatives, and more.

Macros can be complex and cumbersome to write. Inline with the goal's of Power VTT, we strive to
simplify so that users of all skill levels can add complex functionality to tabletop role-playing
games.

By open-sourcing the language and parser, we hope more users and non-users will be open to using our
macro language, whether inside of Power VTT or in other software.

## Using TTML

TableTop Macro Language is already integrated into [Power Virtual TableTop](https://www.poweredvtt.com).

If you are looking to integrate TTML into your existing platform or game, TTML can be used as an
npm package. See [Installing](#installing).

# Documentation

1. [API](API.md) - A full reference on the TableTop Macro Language (TTML)
2. [Documentation](DOCUMENTATION.md) - How the program works behind-the-scenes
3. [Development](#development)
4. [Contributing](#contributing)

# Development

You will need [emscripten](https://github.com/kripken/emscripten) and [Rust](https://www.rust-lang.org/) in
addition to [Node.js](https://nodejs.org).

```bash
# Install dependencies
yarn

# Test Rust
cargo test

# Compile wasm/javascript library
yarn compile

# Run example "index.html"
yarn serve
```

# Contributing

Want to help us improve the parser and language? We 💛 pull requests! Make sure you [disucss with us](https://github.com/UnicornHeartClub/tabletop-macro-language/issues/new) first
first but generally any new functionality should adhere to the tests.

```bash
# Run tests
cargo test
```
