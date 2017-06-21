#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
    Nil,
    True,
    False,
    UserDefined(String)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    Atom(Atom),
    Integer(i64),
}
