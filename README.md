# TableTop Macro Language

A language and parser to add advanced functionality to tabletop role-playing games.

- [API](API.md) - A full reference on the TableTop Macro Language (TTML)
- [Documentation](DOCUMENTATION.md) - How the program works behind-the-scenes
- [Installation](#installation) - Use TTML for your own projects
- [Contributing](#contributing) - Help us make the language better

## Using

TableTop Macro Language is already integrated into [Power Virtual TableTop](https://www.poweredvtt.com).

For information on how to write macros, see the [API](API.md).

If you are looking to integrate TTML into your existing platform or game, TTML can be used as an
npm package. See [Installation](#installation).

# Installation

To use TableTop Macro Language in your project, simply add it with `npm` or `yarn`.

```bash
yarn add https://github.com/UnicornHeartClub/tabletop-macro-language
```

# Contributing

Want to help us improve the parser and language? We 💛 pull requests! Make sure you [disucss with us](https://github.com/UnicornHeartClub/tabletop-macro-language/issues/new) first
first but generally any new functionality should adhere to the tests.

## Development

Looking to get setup on your local machine? You will need
[emscripten](https://github.com/kripken/emscripten), [Rust](https://www.rust-lang.org/) and 
[Node.js](https://nodejs.org).

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

# License

[MIT](LICENSE) &copy; 2017 Unicorn Heart Club LLC
