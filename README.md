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

Power VTT Macro Language is already integrated into [Power Virtual TableTop](https://www.poweredvtt.com).

If you are looking to integrate TTML into your existing platform or game, TTML can be used as a
standalone binary that accepts string input and returns serialized output. See [Installing](#installing)

# Documentation

1. [API](#api)
2. [Language Basics](#language-basics)
3. [Variables](#variables)

## API

See [the official documentation](#) for the complete API.

## Language Basics

TableTop Macro Language takes a lot of inspiration from simple scripting languages such as Lua but
is highly targeted for tabletop role-playing games.

The below example heals a token +1 each time a successful heal check is rolled.

```lua
@token = '<some-token-id>'
@check = 'heal'

if (@check === 'heal')
  if (roll_d20(1) > 15)
    health_up(@token, 1)
```

Let's break down that example:

```lua
@token = '<some-token-id>'
@check = 'heal'
```

Each line of a TTML program is executed inline. Lines that begin with an `@` symbol are considered
[Variables](#variables) and can be referenced later in the program.

```lua
if (@check === 'heal')
```
We check if the variable `@check` is equal to `heal` (designated by `===`). The variables here are all
very custom to this program and have no significant meaning in TTML. Here `@check` and `heal` are
both constants we defined but in a real-world program `heal` might be dynamicly generated.

```lua
if (roll_d20(1) > 15)
```

If we passed the heal check, roll a single d20 and if it's greater than 15, continue on. Here,
`roll_d20` is a function defined in the TTML [API](#api).

```
health_up(@token, 1)
```

Finally, increase the health of our token by 1.

## Variables

Variables let you define custom pieces of information in your programs before sending them to the
TTML parser. An example of a variable might be a Token ID or spell name.

# Installing

Looking to use TTML in your project? Building the project is easy!
To get started, you will need [Rust](https://www.rust-lang.org/en-US/).

```bash
# Build the binary
cargo build

# Use the binary
./target/release/build/ttml <input>
```

# Contributing

Want to help us improve the parser and language? We 💛 pull requests! Make sure you [disucss with us](https://github.com/UnicornHeartClub/tabletop-macro-language/issues/new) fir
first but generally any new functionality should adhere to the tests.

```bash
# Run tests
cargo test
```
