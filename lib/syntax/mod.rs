use std::fmt;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Identifier(pub String);

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.0);
    }
}

// #[derive(Clone, Debug, Eq, Hash, PartialEq)]
// pub struct Atom(Identifier);

// impl fmt::Display for Atom {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         return write!(f, "'{}", self.0);
//     }
// }

#[derive(Clone, Debug)]
pub enum Expression<Ann = ()> {
    Atom(Atom<Ann>),
    Ref(Variable<Ann>),
    Ty(Type<Ann>),
    Abs(Lambda<Ann>),
    App(Apply<Ann>),
}

#[derive(Clone, Debug)]
pub struct Atom<Ann = ()> {
    pub ann: Ann,
    pub ident: Identifier,
}

#[derive(Clone, Debug)]
pub struct Variable<Ann = ()> {
    pub ann: Ann,
    pub ident: Identifier,
}

#[derive(Clone, Debug)]
pub struct Type<Ann = ()> {
    pub ann: Ann,
    pub ident: Identifier,
}

#[derive(Clone, Debug)]
pub struct Lambda<Ann = ()> {
    pub ann: Ann,
    pub args: Vec<Box<Expression<Ann>>>,
    pub body: Box<Expression<Ann>>,
}

#[derive(Clone, Debug)]
pub struct Apply<Ann = ()> {
    pub ann: Ann,
    pub fun: Box<Expression<Ann>>,
    pub args: Vec<Box<Expression<Ann>>>,
}

#[derive(Clone, Debug)]
pub enum Statement<Ann = ()> {
    Claim(Claim<Ann>),
    Def(Define<Ann>),
    Expr(Expression<Ann>),
}

#[derive(Clone, Debug)]
pub struct Claim<Ann = ()> {
    pub ann: Ann,
    pub ident: Identifier,
    pub expr: Box<Expression<Ann>>,
}

#[derive(Clone, Debug)]
pub struct Define<Ann = ()> {
    pub ann: Ann,
    pub ident: Identifier,
    pub body: Box<Expression<Ann>>,
}

#[derive(Clone, Debug)]
pub struct Source<Ann = ()> {
    pub ann: Ann,
    // pub source: String,
    pub statements: Vec<Statement<Ann>>,
}

// given that it is necessary to have claim for every definition, this
// might be easier to work with
#[derive(Clone, Debug)]
pub struct Definition<Ann = ()> {
    pub ident: Identifier,
    pub claim: (Ann, Box<Expression<Ann>>),
    pub body: (Ann, Box<Expression<Ann>>),
}
