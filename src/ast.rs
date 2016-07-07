use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq)]
pub enum ParsedItem<'a> {
    Text(Cow<'a, str>),
    CodeBlock(Vec<Expr<'a>>),
}

pub type UseAlias<'a> = Option<Cow<'a, str>>;

//TODO: maybe use Path for this too?
#[derive(Clone, Debug, PartialEq)]
pub enum UseClause<'a> {
    QualifiedName(Vec<Cow<'a, str>>, UseAlias<'a>),
}

//TODO: use this in more places instead of Vec<Cow<..>>
#[derive(Clone, Debug, PartialEq)]
pub enum Path<'a> {
    Class(Cow<'a, str>),
    /// fragment.1 = The namespace
    /// fragment.2 = The class
    NamespacedClass(Cow<'a, str>, Cow<'a, str>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    // arith
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    // logical
    Or,
    And,
    // unary
    Not,
    // pre/post
    PreInc,
    PreDec,
    PostInc,
    PostDec,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Visibility {
    None,
    Public,
    Private,
    Protected
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClassModifier {
    None,
    Abstract,
    Final,
}

/// the boolean indicates whether the underlying item is static or not
#[derive(Clone, Debug, PartialEq)]
pub struct Modifiers(pub bool, pub Visibility, pub ClassModifier);

#[derive(Clone, Debug, PartialEq)]
pub enum Expr<'a> {
    None,
    True,
    False,
    Null,
    Identifier(Cow<'a, str>),
    NsIdentifier(Vec<Cow<'a, str>>),
    String(String),
    Int(i64),
    Array(Vec<(Box<Expr<'a>>, Box<Expr<'a>>)>),
    Variable(Cow<'a, str>),
    Reference(Box<Expr<'a>>),
    Block(Vec<Expr<'a>>),
    Use(Vec<UseClause<'a>>),
    Echo(Vec<Expr<'a>>),
    Return(Box<Expr<'a>>),
    Break(usize),
    Continue(usize),

    ArrayIdx(Box<Expr<'a>>, Vec<Expr<'a>>),
    ObjMember(Box<Expr<'a>>, Vec<Expr<'a>>),
    StaticMember(Box<Expr<'a>>, Vec<Expr<'a>>),
    Call(Box<Expr<'a>>, Vec<Expr<'a>>),
    New(Path<'a>, Vec<Expr<'a>>),
    UnaryOp(Op, Box<Expr<'a>>),
    BinaryOp(Op, Box<Expr<'a>>, Box<Expr<'a>>),
    Function(FunctionDecl<'a>),
    // statements
    Assign(Box<Expr<'a>>, Box<Expr<'a>>),
    AssignRef(Box<Expr<'a>>, Box<Expr<'a>>),
    /// If (condition=.0) { Block=.1 } else Else_Expr=.2
    If(Box<Expr<'a>>, Box<Expr<'a>>, Box<Expr<'a>>),
    While(Box<Expr<'a>>, Box<Expr<'a>>),
    DoWhile(Box<Expr<'a>>, Box<Expr<'a>>),
    ForEach(Box<Expr<'a>>, Box<Expr<'a>>, Box<Expr<'a>>, Box<Expr<'a>>),

    /// switch (stmt=.0) [case item: body]+=.1
    /// All item-cases for a body will be included in the first-member Vec
    /// so basically we have a mapping from all-cases -> body in .1
    /// TODO: should be desugared into an if-statement
    Switch(Box<Expr<'a>>, Vec<(Vec<Expr<'a>>, Expr<'a>)>),

    /// same as if, just will pass the return-value of either expression to the parent
    /// if .1 (then) is None, the value of .0 (condition) will be used
    /// TODO: this should be desugared into an `If` during post-processing
    TernaryIf(Box<Expr<'a>>, Box<Expr<'a>>, Box<Expr<'a>>),

    // These are not actual expressions, but will be stored as such, before any filtering happens
    Decl(Decl<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ty {
    Array,
    Callable,
    Bool,
    Float,
    Int,
    String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParamDefinition<'a> {
    pub name: Cow<'a, str>,
    pub as_ref: bool,
    /// The type of the parameter
    pub ty: Option<Ty>,
    /// The default value for the parameter
    pub default: Expr<'a>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDecl<'a> {
    pub params: Vec<ParamDefinition<'a>>,
    pub body: Vec<Expr<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClassDecl<'a> {
    pub name: Cow<'a, str>,
    pub base_class: Option<Path<'a>>,
    pub members: Vec<ClassMember<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClassMember<'a> {
    Property(Modifiers, Cow<'a, str>, Expr<'a>),
    Method(Modifiers, Cow<'a, str>, FunctionDecl<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Decl<'a> {
    Namespace(Vec<Cow<'a, str>>),
    GlobalFunction(Cow<'a, str>, FunctionDecl<'a>),
    Class(ClassDecl<'a>),
}
