pub trait LangInt {
    type Repr;
    fn int(n: i32) -> Self::Repr;
    fn add(r1: Self::Repr, r2: Self::Repr) -> Self::Repr;

    type Obs;
    fn observe(r: Self::Repr) -> Self::Obs;
}

pub struct EvalInt;

pub type Dom = i32;
impl LangInt for EvalInt {
    type Repr = Dom;
    fn int(n: i32) -> Self::Repr {
        n
    }
    fn add(r1: Self::Repr, r2: Self::Repr) -> Self::Repr {
        let sum = r1 + r2;
        format!("{}\n", sum);
        sum
    }

    type Obs = ();
    fn observe(r: Self::Repr) -> Self::Obs {
        format!("{}\n", r);
    }
}