# TableTop Macro Language Examples

The best way to learn a new language is to look at some examples. Below is a list of some useful
commands you might use in your games.

## Running Examples

Make sure you have built the project first.

```bash
cargo run --release
```

Pass in any example to the program to see the output.

```
MACRO=$( cat examples/bad-heal-check.ttml ) ttml-parser -vv "$MACRO"
```

## Examples

 - [Attack Enemy Token with Saving Roll](attack-enemy-token-with-saving-roll.ttml) - Roll to attack
an enemy token, but allow the enemy to perform a saving throw. If the saving throw fails, roll
another d20 for damage.
 - [Bad Heal Check](bad-heal-check.ttml) - Heal a player on successful healing check, otherwise do
damage
 - [Roll Initiative](roll-initiative.ttml) - List players and roll a d20 for each one

## Contributing

Feel free to [submit a pull request](https://github.com/UnicornHeartClub/tabletop-macro-language/issues/new)
if you want to add a new example or think an existing one can be improved.
