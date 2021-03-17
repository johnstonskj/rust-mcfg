# Actions

An action is a unit of work executed by a command in the CLI. It is simply a rust type that implements the following 
trait.

```rust
pub trait Action: Debug {
    /// Run this action, this assumes all information was passed to the action during creation.
    fn run(&self) -> Result<()>;
}
```