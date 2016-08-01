use std::borrow::Cow;
use std::rc::Rc;
use tokenizer::Span;
use interner::RcStr;

#[derive(Clone, Debug, PartialEq)]
pub enum ParsedItem {
    Text(RcStr),
    CodeBlock(Vec<Expr>),
}

pub type UseAlias = Option<RcStr>;

#[derive(Clone, Debug, PartialEq)]
pub enum UseClause {
    QualifiedName(Path, UseAlias),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Path {
    pub is_absolute: bool,
    pub namespace: Option<RcStr>,
    /// mostly something like the trait or class name
    pub identifier: RcStr,
}

impl Path {
    pub fn identifier(absolute: bool, name: RcStr) -> Path {
        Path { namespace: None, identifier: name, is_absolute: absolute }
    }

    pub fn ns_identifier(absolute: bool, namespace: RcStr, name: RcStr) -> Path {
        Path { namespace: Some(namespace), identifier: name, is_absolute: absolute }
    }
}

/// binary operators
#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Concat,
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
    // equality
    Identical,
    NotIdentical,
    Eq,
    Neq,
    // relational
    Lt,
    Gt,
    Le,
    Ge,
    // bitwise
    BitwiseAnd,
    BitwiseInclOr,
    /// XOR
    BitwiseExclOr,
    /// spaceship operator, <=>
    Spaceship,
    Sl,
    Sr,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    Positive,
    Negative,
    Not,
    PreInc,
    PreDec,
    PostInc,
    PostDec,
    BitwiseNot,
    /// "@"" http://php.net/manual/en/language.operators.errorcontrol.php
    /// any error messages that might be generated by that expression will be ignored.
    SilenceErrors,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Visibility {
    None,
    Public,
    Private,
    Protected
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ClassModifier {
    Abstract = 1<<0,
    Final = 1<<1,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClassModifiers(u8);

impl ClassModifiers {
    pub fn none() -> ClassModifiers {
        ClassModifiers(0)
    }

    pub fn new(cms: &[ClassModifier]) -> ClassModifiers {
        let mut flag = 0;
        for modifier in cms {
            flag |= *modifier as u8;
        }
        ClassModifiers(flag)
    }

    #[inline]
    pub fn has(&self, m: ClassModifier) -> bool {
        return self.0 & (m as u8) != 0;
    }
}

/// the boolean indicates whether the underlying item is static or not
/// TODO: error validation of duplicate and invalid states in ::new
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MemberModifier {
    Public = 1<<0,
    Protected = 1<<1,
    Private = 1<<2,
    Static = 1<<3,
    Abstract = 1<<4,
    Final = 1<<5,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MemberModifiers(u8);

impl MemberModifiers {
    pub fn none() -> MemberModifiers {
        MemberModifiers(0)
    }

    pub fn new(ms: &[MemberModifier]) -> MemberModifiers {
        let mut flag = 0;
        for modifier in ms {
            flag |= *modifier as u8;
        }
        MemberModifiers(flag)
    }

    pub fn has(&self, m: MemberModifier) -> bool {
        return self.0 & (m as u8) != 0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr(pub Expr_, pub Span);

#[derive(Clone, Debug, PartialEq)]
pub struct Stmt(pub Stmt_, pub Span);

#[derive(Clone, Debug, PartialEq)]
pub struct Block(pub Vec<Stmt>);

impl Block {
    pub fn empty() -> Block {
        Block(vec![])
    }

    pub fn is_empty(&self) -> bool {
        return self.0.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr_ {
    /// indicates the path to e.g. a namespace or is a simple identifier (e.g. a runtime-constant)
    Path(Path),
    /// a builtin (internal) constant like true, false, null or any magic-constant
    Constant(Const),
    String(RcStr),
    BinaryString(Rc<Vec<u8>>),
    Int(i64),
    Double(f64),
    Array(Vec<(Option<Expr>, Expr)>),
    Variable(RcStr),
    /// something like $$test, where another expression contains the name of the variable to be fetched
    FetchVariable(Box<Expr>),
    Reference(Box<Expr>),
    Clone(Box<Expr>),
    Isset(Vec<Expr>),
    Empty(Box<Expr>),
    Exit(Option<Box<Expr>>),

    Include(IncludeTy, Box<Expr>),
    ArrayIdx(Box<Expr>, Vec<Option<Expr>>),
    ObjMember(Box<Expr>, Vec<Expr>),
    StaticMember(Box<Expr>, Vec<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    New(Box<Expr>, Vec<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
    BinaryOp(Op, Box<Expr>, Box<Expr>),
    InstanceOf(Box<Expr>, Box<Expr>),
    Cast(Ty, Box<Expr>),
    Yield(Option<Box<Expr>>),
    /// an anonymous function
    Function(FunctionDecl),

    // statements
    Assign(Box<Expr>, Box<Expr>),
    /// compound (binary) assign e.g. $test += 3; which is equal to $test = $test + 3; (Assign, BinaryOp)
    CompoundAssign(Box<Expr>, Op, Box<Expr>),
    AssignRef(Box<Expr>, Box<Expr>),
    List(Vec<(Option<Expr>, Expr)>),

    /// same as if, just will pass the return-value of either expression to the parent
    /// if .1 (then) is None, the value of .0 (condition) will be used
    /// TODO: this should be desugared into an `If` during post-processing
    TernaryIf(Box<Expr>, Option<Box<Expr>>, Box<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt_ {
    Block(Block),
    Decl(Decl),
    Use(Vec<UseClause>),
    /// An expression which is terminated by a semicolon
    Expr(Expr),
    Echo(Vec<Expr>),
    Return(Option<Box<Expr>>),
    Break(Option<Box<Expr>>),
    Continue(Option<Box<Expr>>),
    Unset(Vec<Expr>),

    /// If (condition=.0) { Block=.1 } else Else_Expr=.2
    If(Box<Expr>, Block, Block),
    While(Box<Expr>, Block),
    DoWhile(Block, Box<Expr>),
    /// For(initializer=.0; cond=.1; end_of_loop=.2) statement=.3
    For(Option<Box<Expr>>, Option<Box<Expr>>, Option<Box<Expr>>, Block),
    ForEach(Box<Expr>, Option<Box<Expr>>, Box<Expr>, Block),
    /// Try(TryBlock, CatchClauses, FinallyClause)
    Try(Block, Vec<CatchClause>, Option<Block>),
    Throw(Box<Expr>),

    /// switch (stmt=.0) [case item: body]+=.1
    /// All item-cases for a body will be included in the first-member Vec
    /// so basically we have a mapping from all-cases -> body in .1
    /// TODO: should be desugared into an if-statement
    Switch(Box<Expr>, Vec<SwitchCase>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ty {
    Array,
    Callable,
    Bool,
    Float,
    Int,
    Double,
    String,
    Object(Option<Path>),
}

/// a builtin (internal) constant, which can be resolved at parse-time
/// and is an essential part of the language (by design and not by declaration)
/// TODO: use Null, True, False insteadof Path::Identifier("null" ... etc.
#[derive(Clone, Debug, PartialEq)]
pub enum Const {
    Null,
    True,
    False,
    MagicClass,
    MagicTrait,
    MagicFunction,
    MagicMethod,
    MagicLine,
    MagicFile,
    MagicDir,
    MagicNamespace,
}

/// A type and flag describing whether it's nullable
#[derive(Clone, Debug, PartialEq)]
pub struct NullableTy(pub Ty, pub bool);

#[derive(Clone, Debug, PartialEq)]
pub enum IncludeTy {
    Include,
    IncludeOnce,
    Require,
    RequireOnce,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TraitUse {
    InsteadOf(Path, RcStr, Vec<Path>),
    As(Option<Path>, RcStr, MemberModifiers, Option<RcStr>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParamDefinition {
    pub name: RcStr,
    pub as_ref: bool,
    /// The type of the parameter
    pub ty: Option<Ty>,
    /// The default value for the parameter
    pub default: Option<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDecl {
    pub params: Vec<ParamDefinition>,
    pub body: Option<Block>,
    /// A list of variables to pass from the parent scope to the scope of this function
    /// So variables which are basically available shared into this function's scope
    /// the boolean indicates whether to bind by-reference (true)
    pub usev: Vec<(bool, RcStr)>,
    pub ret_ref: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClassDecl {
    pub cmod: ClassModifiers,
    pub name: RcStr,
    pub base_class: Option<Path>,
    /// The implemented interfaces of this class
    pub implements: Vec<Path>,
    pub members: Vec<Member>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Member {
    Constant(MemberModifiers, RcStr, Expr),
    Property(MemberModifiers, RcStr, Option<Expr>),
    Method(MemberModifiers, RcStr, FunctionDecl),
    TraitUse(Vec<Path>, Vec<TraitUse>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Decl {
    Namespace(Path),
    GlobalFunction(RcStr, FunctionDecl),
    Class(ClassDecl),
    Interface(RcStr, Vec<Path>, Vec<Member>),
    Trait(RcStr, Vec<Member>),
    StaticVars(Vec<(RcStr, Option<Expr>)>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CatchClause {
    pub ty: Path,
    pub var: RcStr,
    pub block: Block,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SwitchCase {
    pub conds: Vec<Expr>,
    pub default: bool,
    pub block: Block,
}
