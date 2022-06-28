# Simple dmenu
A macro to call dmenu from rust

## How to use

### Standard
Requires every argument to implement `.as_bytes()`.
Might also not need to pass any arguments at all
```
use simple_dmenu::dmenu;

// Example
let output = dmenu!("1", "2", "3");
```

### Stringify
Same as standard but passes every argument to `stringify!()`.
Again, might also not need to pass any arguments at all
```
use simple_dmenu::dmenu;

// Example
let output = dmenu!(stringify 1, 2, 3);
```

### Iter
Accepts an iterator.
Every value yielded by the iterator needs to implement `.as_bytes()`
```
use simple_dmenu::dmenu;

// Example
let mut a = vec!["1", "2", "3"];
a.push("4");

let output = dmenu!(iter a);
```

### Prompt
Only shows a prompt with the specified prompt text
```
use simple_dmenu::dmenu;

// Example
let username = dmenu!(prompt "What's your name?");
```

### Optional arguments
You can specify optional arguments to pass to dmenu by separating them with `; args`
```
use simple_dmenu::dmenu;

// Example
let output = dmenu!(stringify 1, 2, 3; args "-p", "Choose a number", "-l", "3");
```

### Arguments only
Empty call, only allows to specify arguments to pass on to dmenu
```
use simple_dmenu::dmenu;

// Example
let output = dmenu!(args
    "-p", "What's your name?",
    "--nb", "#FFFFFF",
    "--nf", "#000000"
);
```
