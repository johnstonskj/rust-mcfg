# Using Existing Actions

## Example Calling InstallAction

```rust
use mcfg::actions::InstallAction;
use mcfg::shared::Environment;

# fn wrapper() {
let env = Environment::default();

let action = InstallAction::install(
    env,
    Some("work-tools".to_string()),
    Some("productivity".to_string())).unwrap();

action.run().unwrap();
# }
```
