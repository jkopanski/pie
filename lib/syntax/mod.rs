use std::fmt;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Name(String);

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	return write!(f, "{}", self.0);
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Atom(Name);

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	return write!(f, "'{}", self.0);
    }
}

#[derive(Clone, Debug)]
pub enum Expr<Ann = ()> {
    Atom { ann: Ann, ident: Name },
    Lam { ann: Ann, ident: Name, body: Box<Expr> },
    Arr { ann: Ann, domain: Box<Expr>, codomain: Vec<Box<Expr>> },
    App { ann: Ann, fun: Box<Expr>, arg: Vec<Box<Expr>> },
    Var { ann: Ann, ident: Name },
    Type { ann: Ann, ident: Name },
}

#[derive(Clone, Debug)]
pub enum Statement<Ann = ()> {
    Claim { ann: Ann, ident: Name, expr: Box<Expr> },
    Define { ann: Ann, ident: Name, body: Box<Expr> },
    Expression(Expr<Ann>),
}

#[derive(Clone, Debug)]
pub struct Source<Ann = ()> {
    pub source: String,
    pub statements: Vec<Statement<Ann>>,
    pub ann: Ann
}

// given that it is necessary to have claim for every definition, this
// might be easier to work with
#[derive(Clone, Debug)]
pub struct Definition<Ann = ()> {
    ident: Name,
    claim: (Ann, Box<Expr>),
    body: (Ann, Box<Expr>),
}
