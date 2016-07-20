use interner::RcStr;

#[derive(Clone, Debug)]
pub struct TokenSpan(pub Token, pub Span);

#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    /// the lower byte position (inclusive)
    pub start: u32,
    /// the upper byte position (exclusive)
    pub end: u32,
    /// This allows tokens to set or unset the current doc_comment for an declaration
    /// which the parser this way can easily track
    pub doc_comment: Option<String>,
}

impl Span {
    #[inline]
    pub fn new() -> Span {
        Span {
            start: 0,
            end: 0,
            doc_comment: None
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SyntaxError {
    None,
    Unterminated(&'static str, Span),
    UnknownCharacter(Span),
}

impl SyntaxError {
    pub fn span(&self) -> Span {
        match *self {
            SyntaxError::None => unimplemented!(),
            SyntaxError::Unterminated(_, ref span) => span.clone(),
            SyntaxError::UnknownCharacter(ref span) => span.clone(),
        }
    }
}

#[allow(dead_code)] //TODO: remove some day
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    End,
    // very simple tokens
    SemiColon,
    Colon,
    Comma,
    Dot,
    SquareBracketOpen,
    SquareBracketClose,
    ParenthesesOpen,
    ParenthesesClose,
    BwOr,
    BwXor,
    Ampersand,
    Plus,
    Minus,
    Div,
    Mul,
    Equal,
    Mod,
    BoolNot,
    BwNot,
    Dollar,
    Lt,
    Gt,
    QuestionMark,
    Silence,
    CurlyBracesOpen,
    CurlyBracesClose,
    /// `
    Backquote,
    DoubleQuote,
    HereDocStart,
    HereDocEnd,
    // php tokens
    OpenTagWithEcho,
    OpenTag,
    /// this counts as implicit ';'
    CloseTag,
    Exit,
    Function,
    Const,
    Return,
    Yield,
    YieldFrom,
    Try,
    Catch,
    Finally,
    Throw,
    If,
    ElseIf,
    EndIf,
    Else,
    While,
    EndWhile,
    Do,
    For,
    Endfor,
    Foreach,
    EndForeach,
    Declare,
    EndDeclare,
    InstanceOf,
    As,
    Switch,
    EndSwitch,
    Case,
    Default,
    Break,
    Continue,
    Goto,
    Echo,
    Print,
    Class,
    Interface,
    Trait,
    Extends,
    Implements,
    /// T_OBJECT_OPERATOR
    ObjectOp,
    /// T_PAAMAYIM_NEKUDOTAYIM
    ScopeOp,
    NsSeparator,
    Ellipsis,
    Coalesce,
    New,
    Clone,
    Var,
    CastInt,
    CastDouble,
    CastString,
    CastArray,
    CastObject,
    CastBool,
    CastUnset,
    Eval,
    Include,
    IncludeOnce,
    Require,
    RequireOnce,
    Namespace,
    Use,
    Insteadof,
    Global,
    Isset,
    Empty,
    HaltCompiler,
    Static,
    Abstract,
    Final,
    Private,
    Protected,
    Public,
    Unset,
    DoubleArrow,
    List,
    Array,
    Callable,
    Increment,
    Decrement,
    IsIdentical,
    IsNotIdentical,
    IsEqual,
    IsNotEqual,
    SpaceShip,
    IsSmallerOrEqual,
    IsGreaterOrEqual,
    PlusEqual,
    MinusEqual,
    MulEqual,
    Pow,
    PowEqual,
    DivEqual,
    ConcatEqual,
    ModEqual,
    SlEqual,
    SrEqual,
    AndEqual,
    OrEqual,
    XorEqual,
    BoolOr,
    BoolAnd,
    LogicalOr,
    LogicalAnd,
    LogicalXor,
    Sl,
    Sr,
    Variable(RcStr),
    Int(i64),
    Double(f64),
    Comment(RcStr),
    /// likely an arbitrary identifier
    String(RcStr),
    /// like 'test', constant encapsed string
    ConstantEncapsedString(RcStr),
    InlineHtml(RcStr),
    // magic-tokens
    MagicClass,
    MagicTrait,
    MagicFunction,
    MagicMethod,
    MagicLine,
    MagicFile,
    MagicDir,
    MagicNamespace,
}