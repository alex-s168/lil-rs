use ariadne::{sources, Color, Label, Report, ReportKind};
use chumsky::{input::BorrowInput, prelude::*};
use std::{env, fmt};
use parse::{Expr, ExprKind};
use std::ops::Range;

mod lex;
mod parse;

fn failure(
    msg: String,
    kind: ReportKind,
    label: (String, SimpleSpan),
    extra_labels: impl IntoIterator<Item = (String, SimpleSpan)>,
    src: &str,
) {
    let fname = "example";
    Report::build(kind, (fname, label.1.start..label.1.end))
        .with_message(&msg)
        .with_label(
            Label::new((fname, label.1.into_range()))
                .with_message(label.0),
        )
        .with_labels(extra_labels.into_iter().map(|label2| {
            Label::new((fname, label2.1.into_range()))
                .with_message(label2.0)
        }))
        .finish()
        .print(sources([(fname, src)]))
        .unwrap();
}

fn parse_failure(kind: ReportKind, err: &Rich<impl fmt::Display>, src: &str) {
    failure(
        err.reason().to_string(),
        kind,
        (
            err.found()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "end of input".to_string()),
            *err.span(),
        ),
        err.contexts()
            .map(|(l, s)| (format!("while parsing this {l}"), *s)),
        src,
    )
}

fn make_input<'src>(
    eoi: SimpleSpan,
    toks: &'src [lex::Spanned<lex::Token<'src>>],
) -> impl BorrowInput<'src, Token = lex::Token<'src>, Span = SimpleSpan> {
    toks.spanned(eoi)
}

fn combine_ranges(a: &Range<usize>, b: &Range<usize>) -> Range<usize> {
    Range {
        start: a.start.min(b.start),
        end: a.end.max(b.end)
    }
}

fn main() {
    let src = env::args().skip(1).next().unwrap();

    let tokens = lex::lexer()
        .parse(&src)
        .into_result()
        .unwrap_or_else(|errs| {
            parse_failure(ReportKind::Error, &errs[0], &src);
            std::process::exit(1);
        });

    /*
    for (tk, span) in tokens {
        let tk = tk.color(&src[span.start..span.end]);
        print!("{}", tk);
    }
    println!("");
    */
    
    let expr = parse::parse_expr(make_input)
        .parse(make_input((0..src.len()).into(), &tokens))
        .into_result()
        .unwrap_or_else(|errs| { 
            for err in &errs {
                parse_failure(ReportKind::Error, err, &src);
            }
            std::process::exit(1);
        });

    let style_warn = ReportKind::Custom("Style", Color::BrightMagenta);

    expr.traverse(&mut |node| {
        if let Some((arr, was_dot_notation)) = node.kind().as_each_elem() {
            if was_dot_notation && arr.kind().as_each_elem().is_some() {
                let range = combine_ranges(
                    &node.location().unwrap(),
                    &arr.location().unwrap());
                let range = SimpleSpan::new(range.start, range.end);

                failure(
                    "you should use the arr[] syntax for nested eachs instead".to_string(),
                    style_warn,
                    ("here".to_string(), range),
                    [],
                    &src);
            }
        }
    });

    println!("{:#?}", expr);
}
