# API

The official macro reference guide for the TableTop Macro Language.

## What's a Macro?

Macros allow users to define custom actions in tabletop role-playing games to issue chat commands,
dice rolls, automate attacks, calculate damages, roll initiatives, and more.

Macros can be complex and cumbersome to write. Inline with the goal's of Power VTT, we strive to
simplify so that users of all skill levels can add complex functionality to tabletop role-playing
games.

By open-sourcing the language and parser, we hope more users and non-users will be open to using our
macro language, whether inside of Power VTT or in other software.

# Table of Contents

1. [Example](#example)
1. [Results](#results)
1. [Variables](#reserved-variables)
1. [Tokens](#tokens)
1. [Types](#types)
1. [Commands](#commands)

# Example

For more examples, see [Examples](https://github.com/UnicornHeartClub/tabletop-macro-language/tree/master/examples).

The below example heals a token +1 each time a successful heal check is rolled otherwise it
subtracts 1 from the token's health.

```bash
#heal
!say "I cast a bad healing spell on myself" !roll 1d20 >= 15 ? @me.hp + 1 : @me.hp - 1
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
!roll 1d20 >= 15
```

Here we define our next step. Notice how there was nothing for us to do but simply write it.
The `!roll` command is very similar to that of [Avrae](http://avrae.io/commands#dice). Here we are
rolling a single d20 die and checking if the output is greater than or equal to 15.

```bash
?
```

The `?` at the begining here denotes that we have a true/false statement to make. Based on the outcome
of the true/false statement we setup, we can define two different results to run.

If the roll we just made is greater than or equal to 15, we will execute the next statement that is
between the `?` and before the `:`. Otherwise, if the roll is less than 15, we will execute only the
section beyond the `:`.

```bash
@me.hp + 1
```
When we are successful, we are going to modify HP by [updating our token attribute](#assign-token).
We tell the command to use our token using the [reserved token variable](#reserved-token-variables) `@me` and add 
one (1) to our `hp` by using `@me.hp + 1`.

```bash
@me.hp - 1
```

In the case that the roll was below 15, we would run the same command but subtract one from our
health instead.

# Results

Results from commands run in TTML can be "Saved" or "Ignored".

| Syntax | Description |
| ------ | ----------- |
| `>>`   | Save the result. Can be referenced by calling `$#` |
| -      | Ignore the result of the command (default) |

### Save vs. Ignore

Sometimes you want to save the result of a command to the input of another
command. You can easily accomplish this in TTML with the `>>` operator. Saving
a result means you can then reference that result from any consecutive command
by referencing `$#` (where `#` is a number that represents the index of the
saved result we can to fetch.)

Sounds complicated, but it's very easy. Take the below example to roll initiative:

```bash
#initiative
!roll 1d20+@me.dexterity >> @me.initiative = $1
```

Here, we roll a d20 die and add our dexterity modifier to it. We then pass that
result to the next command and set our token's initiative equal to our roll
result using `$1`. We use `$1` because we want to use the result of the _first_
command we ran.

Let's look at a slightly more complicated example. In the below example, we roll a 1d8
to determine how many d10s we should roll which we then use to determine how many d20s to roll.

```bash
#initiative
!roll 1d8 >> !roll $1d10 >> !roll $2d20
```

In the above code, we save each result and use it in the next command by reference `$1`, `$2` where
`$1` is the result of `!roll 1d8` and `$2` is the result of `!roll $1d10`.

In this example scenario, we might see something like this happen:

| Step | Result |
| ---- | ------ |
| `!roll 1d8` | 6 |
| `!roll 6d10` | 27 |
| `!roll 27d20` | 122 |

# Variables

Variables can be assigned and referenced using a `$` followed by any
alphanumeric sequence.

When assigning custom variable names, names cannot be strict numbers as the
results of commands are referenced using numbers.

## Assign Variable

Example:

```bash
$dex = @me.dexterity
```

## Use Variable

Example:

```bash
!roll 1d20+$dex
```

# Tokens

Similar to variables, tokens can be assigned and referenced using the `@`
operator followed by any alphanumeric sequence. Like variables, token attributes can be
created and updated on-the-fly but tokens can not be directly assigned.

## Assign Token

Tokens cannot be directly assigned by rather attributes of the token can be updated.

Example:

```bash
# Not allowed
@me = 1

# Allowed
@me.strength_mod = 5
```

## Reference Token

Example:

```bash
!roll 1d20+@me.dexterity
```

## Reserved Token Variables

If you use Power Virtual TableTop, there are already several reserved tokens for you
to use.

These tokens can be accessed (assuming they are available) from any macro within Power VTT.

| Name | Description |
| ---- | ----------- |
| `@me` | A reference to your token, if available |
| `@selected` | A reference to the selected token, if available |

# Types

Types all you to define different data for your macros. The following types are supported.

## String

```bash
"Double-quoted string"

'Single-quoted string'

OneWord
```

## Number

```bash
1

1000

-52
```

## Float

```bash
7.0

-1.55

593.2020
```

## Option

Options are used in the [!prompt](#prompt) command.

```bash
[ option, option2, option3 ]

[ value:label, "Just a label", @token.attribute, $score ]
```

## Token

```bash
@me

@test.attribute

@npc->some_macro
```

## Variable

```bash
$score = @me.hp + 100

$message = "Go forth!"
```

## VariableReserved

These are the results saved from previous macro steps.

```bash
$1

$2
```

# Commands

TTML provides comamnds to execute, modify, and automate tabletop role-playing scenarios.

| Command             | Usage            | Description               |
| ------------------- | ---------------- | ------------------------- |
| [Input](#input)     | `!input`, `!i <message>`   | Prompt for user input.    |
| [Prompt](#prompt)   | `!prompt`, `!p <message> <options>`  | Prompt a list of options. |
| [Roll](#roll)       | `!roll`, `!r <command>`    | Roll dice.                |
| [Say](#say)         | `!say`, `!s <from> <message>`     | Send a message.           |
| [Target](#target)   | `!target`, `!t <message>`  | Prompt to select a token. |
| [Whisper](#whisper) | `!whisper`, `!w <to> <message>` | Send a message privately. |

## Input

The **!input** command allows you to stop execution of the program and prompt the user for
additional input.

### Syntax

```bash
!input, !i <message>
```

| **Argument** | **Type** | **Description**                           |
| ------------ | -------- | ----------------------------------------- |
| _message_    | [String](#string)   | The message to display to the user        |

### Examples

```bash
!input "Choose an attack multiplier"

!input 'Enter an amount' >> !say 'I throw' $1 'gold pieces into the air'

!i 'Enter a new alias' >> @me.name = $1
```

## Prompt

The **!prompt** command allows you to select from a list of options. Similar to [Input](#input),
this will stop the execution of the program and wait for the user to select an option.

### Syntax

```bash
!prompt, !p <message> [<option>]
```

| **Argument** | **Type** | **Description**                           |
| ------------ | -------- | ----------------------------------------- |
| _message_    | [String](#string)   | The message to display to the user        |
| _option_     | [Option](#option)   | An option to present |


### Examples

```bash
!prompt "Choose a mod" [@me.dexterity_mod, @me.strength_mod]

!prompt 'Choose a number' [42, 16.3, "No, I don't want to"]

!p "Choose an option" [ value:label, 'Can mix and match option types', $hp ]
```

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
| `gt`     | Keep die greater than a threshold.      | _(dice)_**gt**_(threshold)_ (e.g. `r! 3d8gt3`) |
| `gte`    | Keep die greater than or equal to a threshold. | _(dice)_**gte**_(threshold)_ (e.g. `r! 3d8gte6`) |
| `lt`     | Keep die less than a threshold.         | _(dice)_**lt**_(threshold)_ (e.g. `r! 2d8lt5`) |
| `lte`    | Keep die less than or equal to a threshold. | _(dice)_**lte**_(threshold)_ (e.g. `r! 2d8lte4`) |
| `kl#`    | Keep the lowest `#` of dice             | _(dice)_**kl**_(number)_ (e.g. `r! 2d20kl1`)   |
| `kh#`    | Keep the highest `#` of dice            | _(dice)_**kh**_(number)_ (e.g. `r! 4d8kh2`)   |
| `max`    | Set the dice maximum.                   | _(dice)_**max**_(threshold)_ (e.g. `r! 1d8max16`)|
| `min`    | Set the dice minimum.                   | _(dice)_**min**_(threshold)_ (e.g. `r! 1d8min3`)|
| `ro`     | Re-roll dice once below a threshold.    | _(dice)_**ro**_(threshold)_ (e.g. `r! 1d8ro2`) |
| `rr`     | Re-roll dice forever below a threshold. | _(dice)_**rr**_(threshold)_ (e.g. `r! 1d8rr2`) |

### Examples

Roll advantage.
```bash
!roll adv
!r advantage
!r 2d20kh1
```

Roll disadvantage.
```bash
!roll dis
!r disadvantage
!r 2d20kl1
```

Roll overriding minimum
```bash
!roll 1d20min2
```

Roll 3x d8's and keep only the top 2.
```bash
!roll 3d8kh2
```

Roll and keep any die above 3
```bash
!roll 3d8gt3
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

Say a message to the room.

`!say "<message>" <as>`

### Examples

Say a message to the room.

```bash
!say "Hello, everyone!"
```

Impersonate another token

```bash
!say 'Hello, everyone!' @npc1
```

## Whisper

_@todo_
