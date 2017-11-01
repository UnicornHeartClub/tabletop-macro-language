# TableTop Macro Language

:warning: **This is under active development. The API is subject to change without notice.** :warning:

A language and parser to add advanced functionality to tabletop role-playing games.

- :notebook: [API](API.md) - A full reference on the TableTop Macro Language (TTML)
- :books: [Documentation](DOCUMENTATION.md) - How the program works behind-the-scenes
- :package: [Installation](#installation) - Use TTML for your own projects
- :star: [Contributing](#contributing) - Help us make the language better

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

Want to help us improve the parser and language? We 💛 pull requests! Make sure you [discuss with us](https://github.com/UnicornHeartClub/tabletop-macro-language/issues/new) first
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

## Roadmap

We have a lot to get done but are looking to get it done quick!

- [x] ~~Die/Roll Models~~
- [ ] JavaScript Library _(In Progress)_
- [ ] Macro Parser _(In Progress)_
  - [x] ~~Primitives~~
  - [x] ~~Commands~~
  - [x] ~~Pass Results~~
  - [ ] Roll Parser _(In Progress)_
  - [ ] Prompt Parser
  - [ ] Say/Whisper Parser
  - [ ] Set Equals
- [ ] Examples _(In Progress, Needs Updating)_
- [ ] Documentation _(In Progress)_
- [ ] Error Handling _(In Progress)_
- [ ] Static/Reserved Variables
- [ ] Macro Executor

# License

[MIT](LICENSE) &copy; 2017 Unicorn Heart Club LLC
