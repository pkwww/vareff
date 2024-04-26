
type Name = String;

enum Reader<T> {
    Ans(T),
    Query(Name, Box<dyn FnOnce(T) -> Reader<T>>),
    // Query(Name, Fn(T) -> Reader<T>),
}

pub struct VarEff;

impl VarEff {
    fn ans<T>(val: T) -> Reader<T> {
        Reader::Ans(val)
    }
    fn var<T>(var_name: Name) -> Reader<T> {
        Reader::Query(var_name, Box::new(|val| Reader::Ans(val)))
    }
    fn lift2<T: 'static>(bin_op: fn(T, T) -> T, r1: Reader<T>, r2: Reader<T>) -> Reader<T> {
        match (r1, r2) {
            (Reader::Ans(v1), Reader::Ans(v2)) => Reader::Ans(bin_op(v1, v2)),
            (Reader::Query(n1, f1), r2) => {
                Reader::Query(n1, Box::new(move |v1| VarEff::lift2(bin_op, f1(v1), r2)))
            },
            (r1, Reader::Query(n2, f2)) => {
                Reader::Query(n2, Box::new(move |v2| VarEff::lift2(bin_op, r1, f2(v2))))
            },
        }
    }
    fn lift<T>(f: fn(T) -> Reader<T>, r: Reader<T>) -> Reader<T> {
        match r {
            Reader::Ans(v) => f(v),
            Reader::Query(n, f1) => Reader::Query(n, Box::new(move |v| )),
        }
    }
}