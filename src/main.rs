use crate::lang::{LangInt};
use crate::eval_int::{EvalInt};
use crate::eval_env::{EvalEnv};
use crate::eval_eff::{VarEff, EvalEff};

mod lang;
mod eval_int;
mod eval_env;
mod eval_eff;

fn main() {
    let a = EvalInt::int(1);
    println!("Hello, world!");
}

