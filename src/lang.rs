pub trait LangInt {
    type Repr;
    fn int(n: i32) -> Self::Repr;
    fn add(r1: Self::Repr, r2: Self::Repr) -> Self::Repr;

    type Obs;
    fn observe(r: Self::Repr) -> Self::Obs;
}


pub type Name = &'static str;
pub trait LangLet: LangInt {
    fn var(var_name: Name) -> Self::Repr;
    fn let_(var: (Name, Self::Repr), body: Self::Repr) -> Self::Repr;
}