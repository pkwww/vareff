#![allow(unused_imports)]
use crate::lang::{LangInt, LangLet};
use crate::eval_int::{EvalInt};
use crate::eval_env::{EvalEnv};
use crate::eval_eff::{EvalEff};

mod lang;
mod eval_int;
mod eval_env;
mod eval_eff;


fn run<T: LangLet>() {
    let varx = T::var("x");
    let vary = T::var("y");
    let letz = T::let_(("z", T::add(varx, vary)), T::add(T::var("z"), T::int(1)));
    let lety = T::let_(("y", T::int(5)), letz);
    let letx = T::let_(("x", T::int(6)), lety);
    T::observe(letx);
}

fn main() {
    run::<EvalEnv>();
}

