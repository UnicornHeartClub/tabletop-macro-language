# Documentation

This serves as a reference to how the program operates under-the-hood.

:warning: **This is under active development.** :warning:

# Table of Contents

1. [Usage](#usage)
1. [API](#api)
1. [Output](#output)

# Usage

```javascript
import { init } from 'tabletop-macro-language'

try {
  const { setup, runMacro } = await init()
} catch (err) {
  console.error(err)
}
```

# API

## init: Promise<TTML>

Initializes the module and returns useful functions for you to use to run macros.

## setup;
 
## Output

The output of the program is useful for interpretting how your game or software will handle actions.
Output is serialized as standard JSON.

In the the first example of the [API](API.md), we ran a macro to heal a player whenever a heal check
was successful. The output of that same program might look like the below:

```json
{
  "executed": "2017-10-20T10:03:21Z",
  "execution_time": 20,
  "input": "!say \"I cast a bad healing spell on myself\" -> !roll 1d20 >= 15 ? !hp $me 1 : !hp $me -1",
  "messages": [{
    "from": "<some-token-id>",
    "message": "I cast a bad healing spell on myself",
    "timestamp": "2017-10-20T10:03:21Z",
    "to": []
  }],
  "rolls": [{
    "_id": "1c75766e-4210-413d-9446-8fb162c56bc2",
    "dice": [{
      "_id": "0d019459-200c-4d7e-8253-df3cc38ac687",
      "die": "d20",
      "max": 20,
      "min": 1,
      "sides": 20,
      "timestamp": "2017-10-20T10:03:21Z",
      "value": 12
    }],
    "modifiers": [],
    "raw_value": 12,
    "token": "<some-token-id>",
    "value": 12
  }],
  "tokens": {
    "<some-token-id>": {
      "health": -1,
      "rolls": [ "1c75766e-4210-413d-9446-8fb162c56bc2" ]
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

