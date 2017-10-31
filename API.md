# API

The official macro reference guide for the TableTop Macro Language.

# Table of Contents

1. [Language Basics](#language-basics)
2. [Commands](#commands)

# Language Basics

1. [Example Macro](#example)
2. [Passing Results](#passing-results)
3. [Prompts](#prompts)
4. [Reserved Variables](#reserved-variables)

## What's a Macro?

Macros allow users to define custom actions in tabletop role-playing games to issue chat commands,
dice rolls, automate attacks, calculate damages, roll initiatives, and more.

Macros can be complex and cumbersome to write. Inline with the goal's of Power VTT, we strive to
simplify so that users of all skill levels can add complex functionality to tabletop role-playing
games.

By open-sourcing the language and parser, we hope more users and non-users will be open to using our
macro language, whether inside of Power VTT or in other software.

### Example

For more examples, see [Examples](https://github.com/UnicornHeartClub/tabletop-macro-language/tree/master/examples).

The below example heals a token +1 each time a successful heal check is rolled otherwise it
subtracts 1 from the token's health.

```bash
#heal
!say "I cast a bad healing spell on myself" >> !roll 1d20 >= 15 ? !hp $me 1 : !hp $me -1
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
>>
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
arrow (`>>`), `?` denote a new statement to make but give you two options based on the outcome.

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

## Passing Results

Sometimes you want to pass the result of a command to the input of another command. You can easily
accomplish this in TTML with the `>>` operator.

By default, each command outputs an array of data. A previous command's output can be referenced
from the current command via `${n}` where `n` is a number >= 1 that references the index of the
result array.

Sounds complicated, but it's very easy. Take the below example to roll initiative:

```bash
#initiative
!roll 1d20+$me.dexterity >> $me.initiative = $1
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

# Commands

TTML provides comamnds to execute, modify, and automate tabletop role-playing scenarios.

| Command             | Usage            | Description               |
| ------------------- | ---------------- | ------------------------- |
| [Prompt](#prompt)   | `!prompt`, `!p`  | Prompt for input.         |
| [Roll](#roll)       | `!roll`, `!r`    | Roll dice.                |
| [Say](#say)         | `!say`, `!s`     | Send a message.           |
| [Whisper](#whisper) | `!whisper`, `!w` | Send a message privately. |

## Prompt

There are often times you would like to be able to select from a list of options when running a
macro. Or perhaps it would be easier to click on a token. Either way, **Prompts** let you define
when your macro might need additional input.

`!prompt, !p <prompt>`

### Syntax

`<prompt>` is written as follows:

| **Syntax**                      | **Usage**                                 |
| ------------------------------- | ----------------------------------------- |
| _(type)_ _(name)_ _([options])_ | `options "Choose attack type" $me.skills` |

## Roll

Roll complicated or simple sets of dice.

`!roll, !r <dice>`

### Syntax

`<dice>` is written as follows:

| **Syntax**                                         | **Usage**   |
| -------------------------------------------------- | ----------- |
| _(number)_**d**_(die)_[ _(flags)_ [ _(comment)_ ]] | `1d20`, `2d8k1`, `3d6 "Uncanny dodge!"`, `2d8ro2 "Custom attack"` |

#### Alternative Syntax

Instead of specifying `<dice>`, the roll command can also be given the below arguments for specific
types of rolls.

| **Syntax**                | **Usage**   |
| ------------------------- | ----------- |
| **adv**, **advantage**    | Roll advantage |
| **dis**, **disadvantage** | Roll disadvantage |


### Flags

Rolls can be extended using the flags below.

| **Flag** | **Description**                         | **Syntax**                                     |
| -------- | --------------------------------------- | ---------------------------------------------- |
| `e`      | Re-roll dice forever above a threshold. (e.g. Exploding Dice) | _(dice)_**e**_(threshold)_ (e.g. `r! 1d6e6`) |
| `k`      | Keep certain dice.                      | _(dice)_**k**_(selector)_ (e.g. `r! 1d8kh1`)   |
| `ro`     | Re-roll dice once below a threshold.    | _(dice)_**ro**_(threshold)_ (e.g. `r! 1d8ro2`) |
| `rr`     | Re-roll dice forever below a threshold. | _(dice)_**rr**_(threshold)_ (e.g. `r! 1d8rr2`) |

| **Selector**  | **Description**                | **Syntax**                                            |
| ------------- | ------------------------------ | ----------------------------------------------------- |
| `l#`          | Select the lowest `#` of dice  | _(dice)(operator)_**l**_(number)_ (e.g. `r! 4d8kl3`)  |
| `h#`          | Select the highest `#` of dice | _(dice)(operator)_**h**_(number)_ (e.g. `r! 3d6kh2`)  |

### Examples

Roll advantage.
```bash
!roll adv
```

Roll disadvantage.
```bash
!roll dis
```

Roll 3x d8's and keep only the top 2.
```bash
!roll 3d8kh2
```

Exploding dice.
```bash
!roll 3d8e8
```

Exploding dice with a comment.
```bash
!roll 3d8e8 "Going for gold"
```

## Say

_@todo_

## Whisper

_@todo_
