# str-to-array

This crate provides a procedural macro, `expand`, that expands a string literal to an array of its characters.

```
// Expands to: ['H', 'e', 'l', 'l', 'o']
let foo: [char; 5] = expand!("Hello")
```

This can be useful in a number of situations, such as when O(1) random
access to characters is desired.
