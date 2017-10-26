# Notes

Scratchpad for ideas and brainstorming

## Prompts

- When a macro is run, a user should be able to configure a prompt that allows certain options to be selected


From a technical standpoint, a prompt should behave as follows:

 - The input is parsed and executed up until the point of the prompt
 - The program returns the current state with an additional unique hash to specify which step in the process we are on
 - The original macro input is sent to the program with the unique step hash
 - The input is executed from the point of the hash and returns the final state of the program or repeats if necessary

Examples:

Choose an attack type then roll with the modifier

```
#attack
!prompt options "Choose attack type" $me.skills -> !roll 1d20 + $1
```

Roll a check then prompt to select a token

```
#check-attack
!roll 1d20 > 12 -> !prompt token "Choose target" -> !hp $1 -(!roll 1d20)
```

Actions in parentheses `()` will be executed inline, meaning that we will subtract the roll from our
target's health in the example above.

### System Agnostic

It should be documented that prompts are to be implemented in a cycle, where input is run, read, and
re-run based on the outputs of the program.
