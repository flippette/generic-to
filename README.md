# `generic-to`

method-generic conversion traits.

# example

```rust
use generic_to::*;

// instead of this
let one = <u16 as Into<u32>>::into(1);

// you can do this
let one = 1_u16.to::<u32>();
```

with the `nightly` feature enabled, all of these
traits can also be used in `const` contexts.

```rust
#![feature(const_trait_impl)]

# #[cfg(feature = "nightly")]
# mod nightly {
use generic_to::*;

// this is available now
const ONE: u32 = 1_u16.to();
# }
```
