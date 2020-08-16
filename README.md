# pygamer-panic-led

Set the panicking behavior to enable the d13 red led on the back of the pygamer. It steals the underlying d13 pin, sets it to function_a and then into_push_pull before enabling, so you COULD still use it for during normal program operation.

## Usage

```rust
#![no_std]
use pygamer as hal;

use pygamer_panic_led as _;

#[hal::entry]
fn main() {
    panic!("argument is ignored");
}
```
