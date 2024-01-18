use std::fmt;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Name(pub String);

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
    Atom {
        ann: Ann,
        ident: Name,
    },
    Lam {
        ann: Ann,
        args: Vec<Box<Expr<Ann>>>,
        body: Box<Expr<Ann>>,
    },
    // Arr { ann: Ann, domain: Box<Expr<Ann>>, codomain: Vec<Box<Expr<Ann>>> },
    App {
        ann: Ann,
        fun: Box<Expr<Ann>>,
        args: Vec<Box<Expr<Ann>>>,
    },
    Var {
        ann: Ann,
        ident: Name,
    },
    TyVar {
        ann: Ann,
        ident: Name,
    },
}

#[derive(Clone, Debug)]
pub enum Statement<Ann = ()> {
    Claim {
        ann: Ann,
        ident: Name,
        expr: Box<Expr<Ann>>,
    },
    Define {
        ann: Ann,
        ident: Name,
        body: Box<Expr<Ann>>,
    },
    Expression(Expr<Ann>),
}

#[derive(Clone, Debug)]
pub struct Source<Ann = ()> {
    pub statements: Vec<Statement<Ann>>,
    pub ann: Ann,
}

// given that it is necessary to have claim for every definition, this
// might be easier to work with
#[derive(Clone, Debug)]
pub struct Definition<Ann = ()> {
    ident: Name,
    claim: (Ann, Box<Expr>),
    body: (Ann, Box<Expr>),
}
