use std::collections::HashMap;

use crate::lang_int::{self, EvalInt};

type Name = String;
pub trait LangLet: lang_int::LangInt {
    fn var(var_name: Name) -> Self::Repr;
    fn let_(var: (Name, Self::Repr), body: Self::Repr) -> Self::Repr;
}

type Dom = lang_int::Dom;
type Env = HashMap<Name, Dom>;

pub struct EvalLet;

impl EvalLet {
    fn ans(val: Dom) -> <EvalLet as lang_int::LangInt> :: Repr {
        Box::new(move |env| val)
    }

    fn lift2(bin_op: fn(Dom, Dom) -> Dom, 
            r1: <EvalLet as lang_int::LangInt> :: Repr,
            r2: <EvalLet as lang_int::LangInt> :: Repr) -> <EvalLet as lang_int::LangInt> :: Repr {
        Box::new(move |env| bin_op(r1(env), r2(env)))
    }

}

impl lang_int::LangInt for EvalLet {
    type Repr = Box<dyn Fn(&mut Env) -> Dom>;


    fn int(n: i32) -> Self::Repr {
        let ein = lang_int::EvalInt::int(n);
        EvalLet::ans(ein)
    }

    fn add(r1: Self::Repr, r2: Self::Repr) -> Self::Repr {
        EvalLet::lift2(EvalInt::add, r1, r2)
    }

    type Obs = ();
    fn observe(r: Self::Repr) -> Self::Obs {
        let mut init_env = HashMap::new();
        let val = r(&mut init_env);
        EvalInt::observe(val);
    }
}

impl LangLet for EvalLet {
    fn var(var_name: Name) -> Self::Repr {
        Box::new(move |env| {
            env.get(&var_name).unwrap().clone()
        })
    }

    fn let_(var: (Name, Self::Repr), body: Self::Repr) -> Self::Repr {
        Box::new(move |env| {
            let var_eval_val = var.1(env);
            // TODO: how to handle var shadowing?
            env.insert(var.0.clone(), var_eval_val).unwrap();
            body(env)
        })
    }
}
