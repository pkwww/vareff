use crate::lang_int::{EvalInt, LangInt};
use crate::lang_let::{EvalLet, LangLet};

mod lang_int;
mod lang_let;

fn main() {
    let a = EvalInt::int(1);
    println!("Hello, world!");
}

