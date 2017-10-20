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

If you are looking to integrate TTML into your existing platform or game, TTML can be used as a
standalone binary that accepts string input and returns serialized output. See [Installing](#installing).

# Documentation

1. [API](#api)
2. [Language Basics](#language-basics)
3. [Commands](#commands)
4. [Passing Results](#passing-results)
5. [Reserved Variables](#reserved-variables)

## API

See [the official documentation](#) for the complete API.

## Language Basics

TableTop Macro Language takes is a simple macro language parser targeted for tabletop role-playing
games. For more examples, see [Examples](https://github.com/UnicornHeartClub/tabletop-macro-language/tree/master/examples).

The below example heals a token +1 each time a successful heal check is rolled otherwise it
subtracts 1 from the token's health.

```bash
#heal
!say "I cast a bad healing spell on myself" -> !roll 1d20 >= 15 ? !hp $me 1 : !hp $me -1
```

Let's break down that example:

```bash
#heal
```

All macros start with a `#` and some unique identifying name. Any code following can be
referenced and executed from the chat console or other macros.

```bash
!say "I cast a bad healing spell on myself"
```

There are many commands you can run in TTML, one of them is the `!say` command which outputs a
message to everyone.

```bash
->
```

An arrow denotes a next step in the process. Multiple commands can be chained together for more
complex equations and functionality.

```bash
!roll 1d20 >= 15
```

The `!roll` command is very similar to that of [Avrae](http://avrae.io/commands#dice). Here we are
rolling a single d20 die and checking if the output is greater than or equal to 15.

```bash
?
```

The `?` at the begining here denotes that we have a true/false statement to make. Similar to an
arrow (`->`), `?` denote a new statement to make but give you two options based on the outcome.

If the roll we just made is greater than or equal to 15, we will execute the next statement that is
between the `?` and before the `:`. Otherwise, if the roll is less than 15, we will execute only the
section beyond the `:`.

```bash
!hp $me 1
```
When we are successful, we are going to modify HP using the `!hp` command. We tell the command to use
our token using the reserved keyword `$me` and finally give the command a number (1) to add to our
health.

```bash
: !hp $me -1
```

In the case that the roll was below 15, we would run the same command but subtract one from our
health instead.

## Commands

TTML provides comamnds to execute, modify, and automate tabletop role-playing scenarios.

| Name | Arguments | Returns | Description |
| ---- | --------- | ------- | ----------- |
| `!roll` `!r` | See [Rolling Dice](#) | Number | Roll dice |
| `!say` | `<message>` | void | Send a message to everyone |
| `!whisper` | `<player> <message>` | void | Send a message to a particular player |

## Passing Results

Sometimes you want to pass the result of a command to the input of another command. You can easily
accomplish this in TTML with the `->` operator.

By default, each command outputs an array of data. A previous command's output can be referenced
from the current command via `${n}` where `n` is a number >= 1 that references the index of the
result array.

Sounds complicated, but it's very easy. Take the below example to roll initiative:

```bash
#initiative
!roll 1d20+$me.dexterity -> $me.initiative = $1
```

Here, we roll a d20 die and add our dexterity modifier to it. We then pass that result to the next
command and set our token's initiative equal to our roll result using `$1`.


## Reserved Variables

TTML defines a few reserved variables for you to use in your macros.

| Name | Description |
| ---- | ----------- |
| `$me` | A reference to your token, if available |
| `$players` | A list of all players, can be iterated over |
| `$selected` | A reference to the selected token, if available |
| `$tokens` | A list of all tokens, can be iterated over |

# Installing

Looking to use TTML in your project? Building the project is easy!  To get started, you will need
[Rust](https://www.rust-lang.org/en-US/). Once installed, run the following to run the TTML binary.

```bash
cargo run -- --help
```

## Output

The output of the program is useful for interpretting how your game or software will handle actions.
Output is serialized as standard JSON.

In the the first example, we ran a macro to heal a player whenever a heal check was successful. The
output of that same program might look like the below:

```json
{
  "executed": "2017-10-20T10:03:21Z",
  "execution_time": 20,
  "messages": [{
    "from": "<some-token-id>",
    "message": "I cast a bad healing spell on myself",
    "timestamp": "2017-10-20T10:03:21Z",
    "to": []
  }],
  "rolls": [{
    "_id": "06ebb5b0-b5b8-11e7-abc4-cec278b6b50a",
    "die": "d20",
    "modifiers": [],
    "raw_value": 12,
    "sides": 20,
    "timestamp": "2017-10-20T10:03:21Z",
    "token": "<some-token-id>",
    "total": 12
  }],
  "tokens": {
    "<some-token-id>": {
      "health": -1,
      "rolls": [ "06ebb5b0-b5b8-11e7-abc4-cec278b6b50a" ]
    }
  },
  "version": "0.1.0"
}
```

In the above output, our token's health is -1 which would seem wrong. The macro interpretter will
only output _differences_ in what it sees and runs. This means if you have a token in your game with
60 health and run this macro, your token does not necessarily have -1 health now.

Instead, you might take this output and run a difference on your token to calculate the new health,
which would be 59.

How your platform handles output is up to you.

# Contributing

Want to help us improve the parser and language? We 💛 pull requests! Make sure you [disucss with us](https://github.com/UnicornHeartClub/tabletop-macro-language/issues/new) first
first but generally any new functionality should adhere to the tests.

```bash
# Run tests
cargo test
```
