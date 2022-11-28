# Steps for setup
## Parsing
If you need to parse input, you can get a parser generator with LALRPOP.

Add these lines to your `Cargo.toml`. The lexer line is [explained in this GitHub issue](https://github.com/lalrpop/lalrpop/issues/650#issuecomment-1032308454).

```toml
# In Cargo.toml
[build-dependencies]
lalrpop = "0.19.7"

[dependencies]
lalrpop-util = { version = "^0.19", features = ["lexer"] }
regex = "1"
```

Then add a `build.rs` next to your `Cargo.toml` with these lines:
```rust
// build.rs
use lalrpop;

fn main() {
  lalrpop::process_root().unwrap();
}
```

Then write your grammar in a `.lalrpop` file in your `src` directory.

Then in your rust file make sure to import the parsed things:
```rust
use lalrpop_util::{lalrpop_mod};

lalrpop_mod!(pub the_name_of_your_lalrpop_file);
```

See [calculator.lalrpop](../testbed/src/calculator.lalrpop) for example.

## Testing
You can add a single test function like:

```rust
#[test]
fn test_fn() {
  assert!(something == something_else);
}
```

You can add a suite of tests like:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

You can run cargo in watch mode:
```zsh
cargo watch -x test
```

### VS Code debugging
Make sure to open the folder for the project as the root of VS Code (e.g. `code testbed`). Then you can create a `.vscode/launch.json` file very quickly from the run and debug sidebar using LLVM, and it will automatically import all the cargo configurations.