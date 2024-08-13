# `Defvar` makes defining environment variables easy

`Defvar` provides a macro for declaring environment variables.  It
also makes it easy to describe how to parse the value and provide a
default.

## Usage

```rust
use defvar::defvar;
use std::time::Duration;

// Defining simple variables is easy.
defvar! { GREETING: String = "Howdy" }

// The macro supports types other than String.  You can provide your
// own parsing logic.
defvar! { TIMES: usize = 1, or try t => t.parse() }

// Here is a more complicated example.
defvar! { DURATION: Duration = Duration::from_secs(1), or try d => d.parse().map(Duration::from_secs) }
```
