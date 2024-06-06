use crate::eval_int::{self, EvalInt};
use crate::lang::{LangInt, LangLet};

type Name = &'static str;
type Cont<T> = Box<dyn FnOnce(T) -> Reader<T>>;

pub enum Reader<T: Copy> {
    Ans(T),
    Query(Name, Cont<T>),
    // Query(Name, Fn(T) -> Reader<T>),
}

pub struct VarEff;

impl VarEff {
    fn ans<T: Copy>(val: T) -> Reader<T> {
        Reader::Ans(val)
    }
    fn var<T: Copy>(var_name: Name) -> Reader<T> {
        Reader::Query(var_name, Box::new(|val| Reader::Ans(val)))
    }
    fn lift2<T: 'static + Copy>(bin_op: fn(T, T) -> T, r1: Reader<T>, r2: Reader<T>) -> Reader<T> {
        match (r1, r2) {
            (Reader::Ans(v1), Reader::Ans(v2)) => Reader::Ans(bin_op(v1, v2)),
            (Reader::Query(n1, k), r2) => {
                Reader::Query(n1, Box::new(move |v1| VarEff::lift2(bin_op, k(v1), r2)))
            },
            (r1, Reader::Query(n2, k)) => {
                Reader::Query(n2, Box::new(move |v2| VarEff::lift2(bin_op, r1, k(v2))))
            },
        }
    }
    fn lift<T: 'static + Copy>(k1: Cont<T>, r: Reader<T>) -> Reader<T> {
        match r {
            Reader::Ans(v) => k1(v),
            Reader::Query(n, k2) => Reader::Query(n, Box::new(move |v| VarEff::lift(k1, k2(v)))),
        }
    }
    fn handle_var<T: 'static + Copy>(ret: fn(T) -> Reader<T>, lookup: Box<dyn Fn(&str) -> Option<T>>, r: Reader<T>) -> Reader<T> {
        match r {
            Reader::Ans(v) => ret(v),
            Reader::Query(n, k) => {
                match lookup(&n) {
                    Some(v) => VarEff::handle_var(ret, lookup, k(v)),
                    None => Reader::Query(n, Box::new(move |v| VarEff::handle_var(ret, lookup, k(v)))),
                }
            },
        }
    }
    fn letv<T: 'static + Copy>(name: Name, val: T, r: Reader<T>) -> Reader<T> {
        // TODO: replace lkup by HashMap?
        let lkup = move |n: &str| match n == name {
            true => Some(val),
            false => None,
        };
        VarEff::handle_var(VarEff::ans, Box::new(lkup), r)
    }
    fn top_hand<T: Copy>(r: Reader<T>) -> T {
        match r {
            Reader::Ans(v) => v,
            Reader::Query(n, k) => panic!(""),
        }
    }
}

pub struct EvalEff;

type Dom = eval_int::Dom;
impl LangInt for EvalEff {
    type Repr = Reader<Dom>;

    fn int(n: i32) -> Self::Repr {
        let ein = eval_int::EvalInt::int(n);
        VarEff::ans(ein)
    }

    fn add(r1: Self::Repr, r2: Self::Repr) -> Self::Repr {
        VarEff::lift2(EvalInt::add, r1, r2)
    }

    type Obs = ();

    fn observe(r: Self::Repr) -> Self::Obs {
        EvalInt::observe(VarEff::top_hand(r))
    }
}

impl LangLet for EvalEff {
    fn var(var_name: Name) -> Self::Repr {
        VarEff::var(var_name)
    }

    fn let_(var: (Name, Self::Repr), body: Self::Repr) -> Self::Repr {
        let name = var.0;
        let val = var.1;
        VarEff::lift(Box::new(move |v| VarEff::letv(name, v, body)), val)
    }
}