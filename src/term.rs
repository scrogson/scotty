#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
    Nil,
    True,
    False,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    Atom(Atom),
    Integer(i64),
}
