use std::collections::HashMap;

use crate::eval_int::{self, EvalInt};
use crate::lang::{LangInt, Name, LangLet};

type Dom = eval_int::Dom;
type Env = HashMap<Name, Dom>;

pub struct EvalEnv;

impl EvalEnv {
    fn ans(val: Dom) -> <EvalEnv as LangInt> :: Repr {
        Box::new(move |_env| val)
    }

    fn lift2(bin_op: fn(Dom, Dom) -> Dom, 
            r1: <EvalEnv as LangInt> :: Repr,
            r2: <EvalEnv as LangInt> :: Repr) -> <EvalEnv as LangInt> :: Repr {
        Box::new(move |env| bin_op(r1(env), r2(env)))
    }

}

impl LangInt for EvalEnv {
    type Repr = Box<dyn Fn(&mut Env) -> Dom>;


    fn int(n: i32) -> Self::Repr {
        let ein = EvalInt::int(n);
        EvalEnv::ans(ein)
    }

    fn add(r1: Self::Repr, r2: Self::Repr) -> Self::Repr {
        EvalEnv::lift2(EvalInt::add, r1, r2)
    }

    type Obs = ();
    fn observe(r: Self::Repr) -> Self::Obs {
        let mut init_env = HashMap::new();
        let val = r(&mut init_env);
        EvalInt::observe(val);
    }
}

impl LangLet for EvalEnv {
    fn var(var_name: Name) -> Self::Repr {
        Box::new(move |env| {
            env.get(&var_name).unwrap().clone()
        })
    }

    fn let_(var: (Name, Self::Repr), body: Self::Repr) -> Self::Repr {
        Box::new(move |env| {
            let var_eval_val = var.1(env);
            // TODO: how to handle var shadowing?
            env.insert(var.0, var_eval_val);
            body(env)
        })
    }
}
