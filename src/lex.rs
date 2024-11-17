use chumsky::prelude::*;
use colored::Colorize;

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'src> {
    Num(f64),
    Str(String),
    Ident(&'src str),
    Padding,

    KwEach,
    KwIn,
    KvWhile,
    KwEnd,
    KwOn,
    KwDo,
    KwIf,
    KwElse,
    KwElseIf,
    KwWhere,
    KwBy,
    KwOrderBy,
    KwAsc,
    KwDesc,
    KwSelect,
    KwExtract,
    KwUpdate,
    KwFrom,
    KwInsert,
    KwWith,
    KwInto,
    KwSend,
    KwLocal,

    OpMinus,
    OpNot,
    OpPlus,
    OpMul,
    OpDiv,
    OpMod,
    OpPow,
    OpLt,
    OpGt,
    OpEq,
    OpAnd,
    OpOr,
    OpTilde,
    OpAt,

    OpSplit,
    OpFuse,
    OpLike,
    OpDict,
    OpTake,
    OpDrop,
    OpComma,

    ParenOpen,
    ParenClose,
    SquareOpen,
    SquareClose,
    Dot,
    Colon,
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Token::Num(n) => { format!("{}", n) }
            Token::Str(str) => { str.to_string() }
            Token::Ident(str) => { str.to_string() }
            Token::Padding => { "".to_string() }

            Token::KwEach => { "each".to_string() }
            Token::KwIn => { "in".to_string() }
            Token::KvWhile => { "while".to_string() }
            Token::KwEnd => { "end".to_string() }
            Token::KwOn => { "on".to_string() }
            Token::KwDo => { "do".to_string() }
            Token::KwIf => { "if".to_string() }
            Token::KwElse => { "else".to_string() }
            Token::KwElseIf => { "elseif".to_string() }
            Token::KwWhere => { "where".to_string() }
            Token::KwBy => { "by".to_string() }
            Token::KwOrderBy => { "orderby".to_string() }
            Token::KwAsc => { "asc".to_string() }
            Token::KwDesc => { "desc".to_string() }
            Token::KwSelect => { "select".to_string() }
            Token::KwExtract => { "extract".to_string() }
            Token::KwUpdate => { "update".to_string() }
            Token::KwFrom => { "from".to_string() }
            Token::KwInsert => { "insert".to_string() }
            Token::KwWith => { "with".to_string() }
            Token::KwInto => { "into".to_string() }
            Token::KwSend => { "send".to_string() }
            Token::KwLocal => { "local".to_string() }

            Token::OpMinus => { "-".to_string() }
            Token::OpNot => { "!".to_string() }
            Token::OpPlus => { "+".to_string() }
            Token::OpMul => { "*".to_string() }
            Token::OpDiv => { "/".to_string() }
            Token::OpMod => { "%".to_string() }
            Token::OpPow => { "^".to_string() }
            Token::OpLt => { "<".to_string() }
            Token::OpGt => { ">".to_string() }
            Token::OpEq => { "=".to_string() }
            Token::OpAnd => { "&".to_string() }
            Token::OpOr => { "|".to_string() }
            Token::OpTilde => { "~".to_string() }
            Token::OpAt => { "@".to_string() }
            Token::OpComma => { ",".to_string() }

            Token::OpSplit => { "split".to_string() }
            Token::OpFuse => { "fuse".to_string() }
            Token::OpLike => { "like".to_string() }
            Token::OpDict => { "dict".to_string() }
            Token::OpTake => { "take".to_string() }
            Token::OpDrop => { "drop".to_string() }

            Token::ParenOpen => { "(".to_string() }
            Token::ParenClose => { ")".to_string() }
            Token::SquareOpen => { "[".to_string() }
            Token::SquareClose => { "]".to_string() }
            Token::Dot => { ".".to_string() }
            Token::Colon => { ":".to_string() }
        };
        write!(fmt, "{}", str)
    }
}

impl Token<'_> {
    pub fn is_sep(&self) -> bool {
        match self {
            Token::ParenOpen |
            Token::ParenClose |
            Token::SquareOpen |
            Token::SquareClose |
            Token::Dot |
            Token::Colon => { true }

            _ => { false }
        }
    }

    pub fn is_op(&self) -> bool {
        match self {
            Token::OpMinus |
            Token::OpNot   |
            Token::OpPlus  |
            Token::OpMul   |
            Token::OpDiv   |
            Token::OpMod   |
            Token::OpPow   |
            Token::OpLt    |
            Token::OpGt    |
            Token::OpEq    |
            Token::OpAnd   |
            Token::OpOr    |
            Token::OpTilde |
            Token::OpAt    |
            Token::OpComma => { true }

            Token::OpSplit |
            Token::OpFuse  |
            Token::OpLike  |
            Token::OpDict  |
            Token::OpTake  |
            Token::OpDrop  => { true }

            _ => { false }
        }
    }

    pub fn is_kw(&self) -> bool {
        match self {
            Token::KwEach |
            Token::KwIn |
            Token::KvWhile |
            Token::KwEnd |
            Token::KwOn |
            Token::KwDo |
            Token::KwIf |
            Token::KwElse |
            Token::KwElseIf |
            Token::KwWhere |
            Token::KwBy |
            Token::KwOrderBy |
            Token::KwAsc |
            Token::KwDesc |
            Token::KwSelect |
            Token::KwExtract |
            Token::KwUpdate |
            Token::KwFrom |
            Token::KwInsert |
            Token::KwWith |
            Token::KwInto |
            Token::KwSend |
            Token::KwLocal => { true }

            _ => { false }
        }
    }

    pub fn color(&self, src: &str) -> String {
        if self.is_kw() {
            src.bright_red().to_string()
        } else if self.is_op() {
            src.bright_blue().to_string()
        } else if self.is_sep() {
            src.white().to_string()
        } else {
            match self {
                Token::Num(_) => { src.bright_blue().to_string() }
                Token::Str(_) => { src.bright_green().to_string() }
                Token::Ident(_) => { src.to_string() }
                Token::Padding => { src.to_string() }
                _ => { panic!(""); }
            }
        } 
    }
}

pub type Spanned<T> = (T, SimpleSpan);

pub fn lexer<'src>() -> 
    impl Parser<'src, &'src str, Vec<Spanned<Token<'src>>>, extra::Err<Rich<'src, char>>> 
{
    let num = just("+").to(1f64)
        .or(just("-").to(-1f64))
        .or_not()
        .then(text::int(10)
            .then(just('.')
                .then(text::int(10))
                .or_not())
            .to_slice()
            .map(|slice: &str| slice.parse::<f64>().unwrap()))
        .map(|(sign, num)| Token::Num(num * sign.unwrap_or(1f64)))
        .boxed();

    let str = 
        choice((
            just("\\\\").to('\\'),
            just("\\\"").to('"'),
            just("\\n").to('\n'),
            none_of(['"']),
        ))
        .repeated()
        .collect::<String>()
        .delimited_by(just('"'), just('"'))
        .map(|x| Token::Str(x))
        .boxed();

    let alpha = one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");

    let ident = alpha.or(just('_')).or(just('?'))
        .repeated()
        .at_least(1)
        .to_slice()
        .map(|x| Token::Ident(x))
        .boxed();

    choice([
            just('-').to(Token::OpMinus).boxed(),
            just('!').to(Token::OpNot).boxed(),
            just('+').to(Token::OpPlus).boxed(),
            just('*').to(Token::OpMul).boxed(),
            just('/').to(Token::OpDiv).boxed(),
            just('%').to(Token::OpMod).boxed(),
            just('^').to(Token::OpPow).boxed(),
            just('<').to(Token::OpLt).boxed(),
            just('>').to(Token::OpGt).boxed(),
            just('=').to(Token::OpEq).boxed(),
            just('&').to(Token::OpAnd).boxed(),
            just('|').to(Token::OpOr).boxed(),
            just('~').to(Token::OpTilde).boxed(),
            just('@').to(Token::OpAt).boxed(),
            just(',').to(Token::OpComma).boxed(),

            text::keyword("each").to(Token::KwEach).boxed(),
            text::keyword("in").to(Token::KwIn).boxed(),
            text::keyword("while").to(Token::KvWhile).boxed(),
            text::keyword("end").to(Token::KwEnd).boxed(),
            text::keyword("on").to(Token::KwOn).boxed(),
            text::keyword("do").to(Token::KwDo).boxed(),
            text::keyword("if").to(Token::KwIf).boxed(),
            text::keyword("else").to(Token::KwElse).boxed(),
            text::keyword("elseif").to(Token::KwElseIf).boxed(),
            text::keyword("where").to(Token::KwWhere).boxed(),
            text::keyword("by").to(Token::KwBy).boxed(),
            text::keyword("orderby").to(Token::KwOrderBy).boxed(),
            text::keyword("asc").to(Token::KwAsc).boxed(),
            text::keyword("desc").to(Token::KwDesc).boxed(),
            text::keyword("select").to(Token::KwSelect).boxed(),
            text::keyword("extract").to(Token::KwExtract).boxed(),
            text::keyword("update").to(Token::KwUpdate).boxed(),
            text::keyword("from").to(Token::KwFrom).boxed(),
            text::keyword("insert").to(Token::KwInsert).boxed(),
            text::keyword("with").to(Token::KwWith).boxed(),
            text::keyword("into").to(Token::KwInto).boxed(),
            text::keyword("send").to(Token::KwSend).boxed(),
            text::keyword("local").to(Token::KwLocal).boxed(),

            text::keyword("split").to(Token::OpSplit).boxed(),
            text::keyword("fuse").to(Token::OpFuse).boxed(),
            text::keyword("like").to(Token::OpLike).boxed(),
            text::keyword("dict").to(Token::OpDict).boxed(),
            text::keyword("take").to(Token::OpTake).boxed(),
            text::keyword("drop").to(Token::OpDrop).boxed(),

            just('(').to(Token::ParenOpen).boxed(),
            just(')').to(Token::ParenClose).boxed(),
            just('[').to(Token::SquareOpen).boxed(),
            just(']').to(Token::SquareClose).boxed(),
            just('.').to(Token::Dot).boxed(),
            just(':').to(Token::Colon).boxed(),

            num,
            str,
            ident,

            text::whitespace()
                .at_least(1)
                .to(Token::Padding).boxed(),
        ])
        .map_with(|t, e| (t, e.span()))
        .repeated()
        .collect()
        .then_ignore(end())
}
