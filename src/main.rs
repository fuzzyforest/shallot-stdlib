use anyhow::Result;
use shallot::*;

fn main() -> Result<()> {
    let mut environment: Environment<shallot_stdlib::Expression> = Environment::default();
    shallot::builtins::set_environment(&mut environment);
    shallot_stdlib::set_environment(&mut environment);
    run_repl(&mut environment)
}
