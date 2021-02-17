# string-explode

This crate provides a procedural macro, `explode`, that expands a string literal to an array of its characters.

```
// Expands to: ['H', 'e', 'l', 'l', 'o']
let foo: [char; 5] = explode!("Hello")
```

This can be useful in a number of situations, such as when O(1) random
access to characters is desired.
