use chumsky::{input::BorrowInput, prelude::*};
use crate::lex;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Expr<'src> {
    src: Option<Range<usize>>,
    kind: ExprKind<'src>
}

impl<'src> Expr<'src> {
    pub fn location(&self) -> Option<Range<usize>> {
        self.src.clone()
    }

    pub fn to_span(&self) -> Option<SimpleSpan> {
        self.location().map(|l|
            SimpleSpan::new(l.start, l.end))
    }

    pub fn kind(&self) -> &ExprKind<'src> {
        &self.kind
    }

    pub fn new(kind: ExprKind<'src>) -> Self {
        Self { src: None, kind }
    }

    pub fn with_src(kind: ExprKind<'src>, src: Range<usize>) -> Self {
        Self { src: Some(src), kind }
    }

    pub fn traverse(&self, each: &mut impl FnMut(&Expr<'src>)) {
        each(self);
        match &self.kind {
            ExprKind::Wrap(x) => { x.traverse(each) },

            ExprKind::Err       |
            ExprKind::Num(_)    |
            ExprKind::Ident(_)  |
            ExprKind::Str(_)    |
            ExprKind::EmptyList => { },

            ExprKind::Subscript { arr, idx, .. } => {
                arr.traverse(each);
                idx.traverse(each);
            }

            ExprKind::Assign { var, val } => { 
                var.traverse(each);
                val.traverse(each);
            },

            ExprKind::Amend { src, val } => {
                src.traverse(each);
                val.traverse(each);
            },

            ExprKind::EachElem { arr, .. } => { 
                arr.traverse(each);
            },

            ExprKind::Binary { left, right, .. } => { 
                left.traverse(each);
                right.traverse(each);
            },

            ExprKind::Unary { val, .. } => {
                val.traverse(each);
            },

            ExprKind::Call { func: _, args } => {
                for arg in args {
                    arg.traverse(each);
                }
            },

            ExprKind::SpreadUnary { arr, .. } => {
                arr.traverse(each);
            },
            
            ExprKind::ForEach { vars: _, arr, body } => {
                body.iter().for_each(|x| x.traverse(each));
                arr.traverse(each);
            }

            ExprKind::While { cond, body } => {
                cond.traverse(each);
                body.iter().for_each(|x| x.traverse(each));
            }

            ExprKind::Function { name: _, args: _, body } => {
                body.iter().for_each(|x| x.traverse(each));
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Lt,
    Gt,
    Eq,
    Min, // &
    Max, // | 
    Match,
    SpreadIdx,
    Split,
    Fuse,
    Like,
    Dict,
    Take,
    Drop,
    CommaJoin,
    TableJoin,
    Join,
    Limit,
    Window,
    In,
    Unless,
    Cross,
    Parse,
    Format,
}

fn tok_to_bin(tok: &lex::Token) -> Option<BinOp> {
    match tok {
        lex::Token::OpPlus  => { Some(BinOp::Add) },
        lex::Token::OpMinus => { Some(BinOp::Sub) },
        lex::Token::OpMul   => { Some(BinOp::Mul) },
        lex::Token::OpDiv   => { Some(BinOp::Div) },
        lex::Token::OpMod   => { Some(BinOp::Mod) },
        lex::Token::OpPow   => { Some(BinOp::Pow) },
        lex::Token::OpLt    => { Some(BinOp::Lt) },
        lex::Token::OpGt    => { Some(BinOp::Gt) },
        lex::Token::OpEq    => { Some(BinOp::Eq) },
        lex::Token::OpAnd   => { Some(BinOp::Min) },
        lex::Token::OpOr    => { Some(BinOp::Max) },
        lex::Token::OpTilde => { Some(BinOp::Match) },
        lex::Token::OpAt    => { Some(BinOp::SpreadIdx) },
        lex::Token::OpSplit => { Some(BinOp::Split) },
        lex::Token::OpFuse  => { Some(BinOp::Fuse) },
        lex::Token::OpLike  => { Some(BinOp::Like) },
        lex::Token::OpDict  => { Some(BinOp::Dict) },
        lex::Token::OpTake  => { Some(BinOp::Take) },
        lex::Token::OpDrop  => { Some(BinOp::Drop) },
        lex::Token::OpComma => { Some(BinOp::CommaJoin) },
        lex::Token::OpJoin => { Some(BinOp::TableJoin) },
        lex::Token::OpLimit => { Some(BinOp::Limit) },
        lex::Token::OpWindow => { Some(BinOp::Window) },
        lex::Token::OpIn => { Some(BinOp::In) },
        lex::Token::OpUnless => { Some(BinOp::Unless) },
        lex::Token::OpCross => { Some(BinOp::Cross) },
        lex::Token::OpParse => { Some(BinOp::Parse) },
        lex::Token::OpFormat => { Some(BinOp::Format) },
        
        _ => { None }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,

    Floor,
    Cos,
    Sin,
    Tan,
    Exp,
    Ln,
    Sqrt,
    Sum,
    Prod,
    Raze,
    Min,
    Max,
    Typeof,
    Count,
    First,
    Last,
    Range,
    Keys,
    List,
    Flip,
    Rows,
    Cols,
    Table,
    Mag,
    Heading,
    Unit,
}

fn tok_to_uny(tok: &lex::Token) -> Option<UnaryOp> {
    match tok {
        lex::Token::OpMinus => { Some(UnaryOp::Negate) },
        lex::Token::OpNot => { Some(UnaryOp::Not) },
        lex::Token::OpFloor => { Some(UnaryOp::Floor) },
        lex::Token::OpCos => { Some(UnaryOp::Cos) },
        lex::Token::OpSin => { Some(UnaryOp::Sin) },
        lex::Token::OpTan => { Some(UnaryOp::Tan) },
        lex::Token::OpExp => { Some(UnaryOp::Exp) },
        lex::Token::OpLn => { Some(UnaryOp::Ln) },
        lex::Token::OpSqrt => { Some(UnaryOp::Sqrt) },
        lex::Token::OpSum => { Some(UnaryOp::Sum) },
        lex::Token::OpProd => { Some(UnaryOp::Prod) },
        lex::Token::OpRaze => { Some(UnaryOp::Raze) },
        lex::Token::OpMin => { Some(UnaryOp::Min) },
        lex::Token::OpMax => { Some(UnaryOp::Max) },
        lex::Token::OpTypeof => { Some(UnaryOp::Typeof) },
        lex::Token::OpCount => { Some(UnaryOp::Count) },
        lex::Token::OpFirst => { Some(UnaryOp::First) },
        lex::Token::OpLast => { Some(UnaryOp::Last) },
        lex::Token::OpRange => { Some(UnaryOp::Range) },
        lex::Token::OpKeys => { Some(UnaryOp::Keys) },
        lex::Token::OpList => { Some(UnaryOp::List) },
        lex::Token::OpFlip => { Some(UnaryOp::Flip) },
        lex::Token::OpRows => { Some(UnaryOp::Rows) },
        lex::Token::OpCols => { Some(UnaryOp::Cols) },
        lex::Token::OpTable => { Some(UnaryOp::Table) },
        lex::Token::OpMag => { Some(UnaryOp::Mag) },
        lex::Token::OpHeading => { Some(UnaryOp::Heading) },
        lex::Token::OpUnit => { Some(UnaryOp::Unit) },

        _ => { None }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind<'src> {
    Err,
    Ident(&'src str),
    Num(f64),
    Str(String),

    Wrap(Box<Expr<'src>>),

    Subscript {
        arr: Box<Expr<'src>>,
        idx: Box<Expr<'src>>,
        was_dot_notation: bool,
    },

    /** only if var is lexpr */ 
    Assign {
        var: Box<Expr<'src>>,
        val: Box<Expr<'src>>
    },

    /** only if var is rexpr */ 
    Amend {
        src: Box<Expr<'src>>,
        val: Box<Expr<'src>>
    },

    EachElem { 
        arr: Box<Expr<'src>>, 
        was_dot_notation: bool 
    },

    /** CAN ALSO BE ARRAY INDEX OR FOREACH */
    Call {
        func: Box<Expr<'src>>,
        args: Vec<Expr<'src>>,
    },

    Binary { 
        left: Box<Expr<'src>>, 
        right: Box<Expr<'src>>, 
        kind: BinOp 
    },

    Unary {
        val: Box<Expr<'src>>,
        op: UnaryOp,
    },

    EmptyList,

    /** example: first@arr */
    SpreadUnary {
        arr: Box<Expr<'src>>,
        op: UnaryOp,
    },
    
    ForEach {
        vars: Vec<&'src str>,
        arr: Box<Expr<'src>>,
        body: Vec<Expr<'src>>,
    },

    While {
        cond: Box<Expr<'src>>,
        body: Vec<Expr<'src>>,
    },

    Function {
        name: &'src str,
        args: Vec<&'src str>,
        body: Vec<Expr<'src>>,
    }
}

impl<'src> ExprKind<'src> {
    pub fn as_each_elem(&self) -> Option<(&Box<Expr<'src>>, bool)> {
        match self {
            ExprKind::EachElem { arr, was_dot_notation } => {
                Some((arr, *was_dot_notation))
            },

            ExprKind::Call { func, args } => {
                if args.len() == 0 {
                    Some((func, false))
                } else {
                    None
                }
            }

            _ => None
        }
    }

    pub fn as_subscript(&self) -> Option<(&Expr<'src>, &Expr<'src>, bool)> {
        match self {
            ExprKind::Subscript { arr, idx, was_dot_notation } => {
                Some((arr, idx, *was_dot_notation))
            },

            ExprKind::Call { func, args } => {
                if args.len() == 1 {
                    Some((func, &args[0], false))
                } else {
                    None
                }
            }

            _ => None
        }
    }

    /** can this expression be assigned to? */
    pub fn is_lexpr(&self) -> bool {
        match self {
            // this is NOT a LEXPR, because  (foo)[1]:44  should ammend instead of assign!
            ExprKind::Wrap(_) => { false },

            ExprKind::Err |
            ExprKind::Num(_) |
            ExprKind::Str(_) => { false },

            ExprKind::Ident(_) => { true },

            ExprKind::Subscript { arr, .. } => {
                arr.kind.is_lexpr()
            }

            ExprKind::Assign { .. } => { false },

            ExprKind::Amend { .. } => { false },

            // this is weird, but it ammends in the official lil linterpreter
            ExprKind::EachElem { .. } => { false },

            ExprKind::Binary { .. } => { false },
            ExprKind::Unary { .. } => { false },

            ExprKind::EmptyList => { false },

            ExprKind::Call { func: _, args } => { args.len() == 1 },

            ExprKind::SpreadUnary { .. } => { false },
            
            ExprKind::ForEach { .. } => { false },

            ExprKind::While { .. } => { false },

            ExprKind::Function { .. } => { false }
        }
    }
}

fn mov<T>(inp: T) -> T {inp}

pub fn parse_expr<'src, I, M>(
    make_input: M,
) -> impl Parser<'src, I, Expr<'src>, extra::Err<Rich<'src, lex::Token<'src>>>>
where
    I: BorrowInput<'src, Token = lex::Token<'src>, Span = SimpleSpan>,
    // Because this function is generic over the input type, we need the caller to tell us how to create a new input,
    // `I`, from a nested token tree. This function serves that purpose.
    M: Fn(SimpleSpan, &'src [lex::Spanned<lex::Token<'src>>]) -> I + Clone + 'src,
{
    let white = select_ref! { lex::Token::Padding => () }
        .repeated()
        .ignored()
        .labelled("whitespace")
        .boxed();

    macro_rules! padded {
        ($inner:expr) => {
            white.clone()
            .ignore_then($inner)
            .then_ignore(white.clone())
        }
    }

    macro_rules! into_range {
        ($ctx:expr) => {
            mov::<SimpleSpan>($ctx.span()).into_range()
        }
    }

    macro_rules! with_src {
        ($of:expr) => {
            ($of).map_with(|x, ctx| Expr::with_src(x, into_range!(ctx)))
        }
    }

    macro_rules! simple {
        ($x:ident) => {
            select_ref! { lex::Token::$x => () }
        }
    }

    recursive::<_, _, extra::Err<Rich<'src, lex::Token<'src>>>, _, _>(|parse_expr| {
        let ident = select_ref! { lex::Token::Ident(s) => ExprKind::Ident(s) }
                .labelled("identifier");

        let ident_as_str = select_ref! { lex::Token::Ident(s) => ExprKind::Str(s.to_string()) }
                .labelled("identifier");

        let num = select_ref! { lex::Token::Num(n) => ExprKind::Num(*n) }
                .labelled("number");

        let atom = with_src!(padded!(choice((
            with_src!(ident.clone())
                .then(parse_expr.clone()
                    .repeated()
                    .collect::<Vec<_>>()
                    .delimited_by(padded!(simple!(SquareOpen)), padded!(simple!(SquareClose))))
                .map(|(ident, args)| ExprKind::Call { func: Box::new(ident), args }),

            ident.clone(),
            num.clone(),

            select_ref! { lex::Token::Str(s) => ExprKind::Str(s.clone()) }
                .labelled("string"),

            padded!(simple!(ParenOpen))
                .then(padded!(simple!(ParenClose)))
                .labelled("empty list")
                .map(|_| ExprKind::EmptyList),

            parse_expr.clone()
                .delimited_by(padded!(simple!(ParenOpen)),
                    padded!(simple!(ParenClose)))
                .map(|x| ExprKind::Wrap(Box::new(x))),

            simple!(KwEach)
                .ignore_then(padded!(select_ref! { lex::Token::Ident(s) => *s })
                    .repeated()
                    .collect::<Vec<_>>())
                .then_ignore(padded!(simple!(KwIn)))
                .then(parse_expr.clone())
                .then_ignore(padded!(simple!(KwDo)  // non standard feature: allow use of `do` to avoid ambiguity
                    .or_not()))
                .then(parse_expr.clone()
                    .repeated()
                    .collect::<Vec<_>>())
                .then_ignore(padded!(simple!(KwEnd)))
                .map(|((idents, arr), body)| ExprKind::ForEach {
                    vars: idents,
                    arr: Box::new(arr),
                    body,
                }),

            simple!(KwWhile)
                .ignore_then(parse_expr.clone())
                .then_ignore(padded!(simple!(KwDo)  // non standard feature: allow use of `do` to avoid ambiguity
                    .or_not()))
                .then(parse_expr.clone()
                    .repeated()
                    .collect::<Vec<_>>())
                .then_ignore(padded!(simple!(KwEnd)))
                .map(|(cond, body)| ExprKind::While {
                    cond: Box::new(cond),
                    body,
                }),

            simple!(KwOn)
                .ignore_then(padded!(select_ref! { lex::Token::Ident(s) => *s }))
                .then(padded!(select_ref! { lex::Token::Ident(s) => *s })
                    .repeated()
                    .collect::<Vec<_>>())
                .then_ignore(padded!(simple!(KwDo)))
                .then(parse_expr.clone()
                    .repeated()
                    .collect::<Vec<_>>())
                .then_ignore(padded!(simple!(KwEnd)))
                .map(|((name, args), body)| ExprKind::Function {
                    name,
                    args,
                    body,
                }),
        ))));

        let arr_access = atom.clone().foldl_with(padded!(parse_expr.clone()
                .or_not()
                .delimited_by(simple!(SquareOpen), simple!(SquareClose)))
                .map(|x| (x, false))
                .labelled("array-style access")
                .or(padded!(simple!(Dot))
                    .ignore_then(with_src!(num.clone()
                            .or(ident_as_str.clone()))
                        .map(|x| (Some(x), true)))
                    .labelled("field-style access"))
                .or(simple!(Dot)
                    .then(padded!(simple!(SquareOpen).ignored()
                            .or(simple!(Dot).then_ignore(padded!(ident)).ignored()))
                        .rewind() // important!
                           .or(simple!(Dot)))
                    .map(|_| (None, true))
                    .labelled("each"))
                .repeated(),
            |arr, (idx, was_dot_notation), e| {
                let kind = if let Some(idx) = idx { 
                    ExprKind::Subscript { arr: Box::new(arr), idx: Box::new(idx), was_dot_notation }
                } else {
                    ExprKind::EachElem { arr: Box::new(arr), was_dot_notation }
                };

                Expr::with_src(kind, into_range!(e))
            });

        let match_unary_op = padded!(any_ref::<_, extra::Err<Rich<'src, lex::Token<'src>>>>()
                .filter(|x| tok_to_uny(x).is_some())
                .map(|x| tok_to_uny(&x).unwrap()))
            .labelled("unary operation");

        let binary = arr_access.foldl_with(padded!(any_ref::<_, extra::Err<Rich<'src, lex::Token<'src>>>>()
                    .filter(|x| tok_to_bin(x).is_some())
                    .map(|x| tok_to_bin(&x).unwrap()))
                .labelled("binary operation")
                .then(parse_expr.clone())
                .repeated(),
            |left, (op, right): (BinOp, Expr<'src>), e| {
                Expr::with_src(ExprKind::Binary {
                    left: Box::new(left),
                    right: Box::new(right),
                    kind: op
                }, into_range!(e))
            });

        let unary = match_unary_op.clone()
            .repeated()
            .foldr_with(binary,
                |op, val, e| {
                    Expr::with_src(ExprKind::Unary {
                        val: Box::new(val),
                        op
                    }, into_range!(e))
                });

        let spread = match_unary_op.clone()
            .then_ignore(padded!(simple!(OpAt)))
            .repeated()
            .foldr_with(unary,
                |op, arr, e| {
                    Expr::with_src(ExprKind::SpreadUnary {
                        arr: Box::new(arr),
                        op,
                    }, into_range!(e))
                });

        let pre_assign = spread;
        let assign = with_src!(pre_assign.clone()
            .then_ignore(padded!(simple!(Colon)))
            .then(parse_expr.clone())
            .map(|(a, b)| {
                if a.kind.is_lexpr() {
                    ExprKind::Assign { var: Box::new(a), val: Box::new(b) } 
                } else {
                    ExprKind::Amend { src: Box::new(a), val: Box::new(b) }
                }
            })).or(pre_assign);

        assign.boxed()
    })
}
