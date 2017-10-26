# API

The official macro reference guide for the TableTop Macro Language.

## Table of Contents

1. [Language Basics](#language-basics)
2. [Commands](#commands)
3. [Passing Results](#passing-results)
4. [Reserved Variables](#reserved-variables)
5. [Roll](#roll)


## Language Basics

TableTop Macro Language is a simple macro language parser targeted for tabletop role-playing
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
| `!roll` `!r` | See [Roll](#roll) | Number | Roll dice |
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

Looking to use TTML in your project? Including the project is easy!

```bash
yarn add https://github.com/UnicornHeartClub/tabletop-macro-language
```

## Roll

Roll complicated or simple sets of dice.

`!roll, !r <dice>`

Rolls can be annotated using square brackets (e.g. `!roll 1d20[I strike thee!]`) and extended using
the flags below.

| Flag    | Description                 | Syntax                                    |
| ------- | --------------------------- | ------------------------------------------|
| `k`     | Keep certain dice.          | **dice**k**selector** (e.g. `r! 1d8kh1`)  |
