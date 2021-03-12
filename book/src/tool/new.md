# Adding New Actions


The following is an example `Action` implementation that does very little.

```rust
use mcfg::actions::Action;
use mcfg::error::Result;
use mcfg::shared::Environment;

#[derive(Debug)]
pub struct ExampleAction {
    env: Environment,
}

impl Action for ExampleAction {
    fn run(&self) -> Result<()> {
        println!("ListAction::run {:?}", self);
        Ok(())
    }
}
impl ExampleAction {
    pub fn new(env: Environment) -> Result<Box<dyn Action>> {
        Ok(Box::from(ExampleAction { env }))
    }
}
```
