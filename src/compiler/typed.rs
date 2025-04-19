#[derive(Clone, Debug)]
pub struct Program {
    pub declarations: Vec<Decl>,
}

#[derive(Clone, Debug)]
pub enum Decl {
    Struct(StructDecl),
    Function(FnDecl),
    MutVal(MutValDecl),
    Val(ValDecl),
    Expr(Expr),
}

pub type Ident = String;

/// Parameter
#[derive(Clone, Debug)]
pub struct Param {
    pub name: Ident,
    // ty: Option<Type>  // todo! optional type parameters
}

/// Function signature
#[derive(Clone, Debug)]
pub struct FnSig {
    pub params: Vec<Param>,
}

#[derive(Clone, Debug)]
pub struct StructDecl {
    pub name: Ident,
    pub methods: Vec<FnDecl>,
}

#[derive(Clone, Debug)]
pub struct FnDecl {
    pub name: Ident,
    pub sig: FnSig,
    pub body: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct MutValDecl {
    pub name: Ident,
    pub initializer: Option<Expr>,
}

#[derive(Clone, Debug)]
pub struct ValDecl {
    pub name: Ident,
    pub initializer: Option<Expr>,
}

#[derive(Clone, Debug)]
pub struct Expr {
    pub kind: ExprKind,
}

#[derive(Clone, Debug)]
pub enum ExprKind {
    /// An array (e.g. `[a, b, c]`).
    Array(Vec<Expr>),
    /// A function call.
    ///
    /// The first field resolves to the function itself,
    /// and the second is a list of arguments.
    Call(Box<Expr>, Vec<Expr>),
    /// A binary operation (e.g., `a + b`, `a * b`).
    Binary(BinOp, Box<Expr>, Box<Expr>),
    /// A unary operation (e.g., `-x`, `!x`).
    Unary(UnOp, Box<Expr>),
    /// An `if` block, with an optional `else` block.
    ///
    /// `if (expr) { block } else { expr }`
    If(Box<Expr>, Box<Block>, Option<Box<Block>>),
    /// A while loop, with an optional label.
    /// label todo!
    ///
    /// `'label: while expr { block }`
    While(Box<Expr>, Box<Block>),
    /// A for loop to loop over elements of a data structure.
    /// The first field is the element
    ///
    /// `for (x in range(10)) {  }`
    For(Ident, Box<Expr>, Box<Block>),
    /// A print to standard output.
    Print(Option<Box<Expr>>),
    /// A closure
    ///
    /// val x = 1;
    /// val f = |x|(times) => { times * x }
    Closure(Closure),
    /// A block (`{ ... }`).
    Block(Box<Block>),
    /// A break (`break "value"`).
    Break(Option<Box<Expr>>),
    /// A `return` (`return "value"`).
    Ret(Option<Box<Expr>>),
    // A struct literal expression.
    //
    // E.g., `Point { x: 1, y: 2 }`.
    // Struct(Box<StructExpr>),
    // todo!
}

/// A block (`{ .. }`).
///
/// E.g., `{ .. }` as in `fn f() { .. }`.
#[derive(Clone, Debug)]
pub struct Block {
    pub stmts: Vec<Decl>,
}

#[derive(Clone, Debug)]
pub enum BinOp {
    /// The `+` operator (addition)
    Add,
    /// The `-` operator (subtraction)
    Sub,
    /// The `*` operator (multiplication)
    Mul,
    /// The `/` operator (division)
    Div,
    /// The `%` operator (modulus)
    Rem,
    /// The `&&` operator (logical and)
    And,
    /// The `||` operator (logical or)
    Or,
    /// The `==` operator (equality)
    Eq,
    /// The `<` operator (less than)
    Lt,
    /// The `<=` operator (less than or equal to)
    Le,
    /// The `!=` operator (not equal to)
    Ne,
    /// The `>=` operator (greater than or equal to)
    Ge,
    /// The `>` operator (greater than)
    Gt,
}

/// Unary operator.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnOp {
    /// The `!` operator for logical inversion
    Not,
    /// The `-` operator for negation
    Neg,
}

#[derive(Clone, Debug)]
pub struct Closure {
    pub capture_args: Vec<Ident>,
    pub fn_sig: Box<FnSig>,
    pub body: Box<Expr>,
}
