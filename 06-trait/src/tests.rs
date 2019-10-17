#![allow(dead_code)]

//!
//! This exercice can be hard but that will show you how Rust allow us to do zero copy memory.
//!
//!
//! What's zero copy ?
//! Instead of allocating dynamic memory and copying data in to pass to later function calls,
//! you use a buffer given to you by reference or made on the stack,
//! relying on rust’s lifetimes and ownership semantics to guarantee that you’re looking at valid data.
//! Other high level languages sometimes encourage copying of data, which can be slow and waste memory.
//! Rust doesn’t discourage copying directly, but makes it much easier to reason about memory semantics and
//! thus write code that doesn’t perform any copying.
//!

///We want to parse this block of HTML:
///```html
///<parent-element>
///   <single-element attribute="value" />
/// </parent-element>
///```
///

type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

// TODO: Implement the Trait "Parser"
// Should contain one method"parse()"
// take in argument: input: &str
// return a ParseResult
// dont forget the lifetime tag, you can find help here: https://doc.rust-lang.org/beta/reference/trait-bounds.html#higher-ranked-trait-bounds


impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParseResult<Output>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self(input)
    }
}

//TODO: add the missing lifetime
// you can find help here: https://doc.rust-lang.org/beta/reference/trait-bounds.html#lifetime-bounds
fn map<P, F, A, B>(parser: P, map_fn: F) -> impl Parser<B>
where
    P: Parser<A>,
    F: Fn(A) -> B,
{
    move |input|
        parser.parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
}

fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2.parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })
    }
}

fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_left, right)| right)
}

fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}


fn identifier(input: &str) -> ParseResult<String> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if next.is_alphabetic() => matched.push(next),
        _ => return Err(input),
    }

    while let Some(next) = chars.next() {
        if next.is_alphanumeric() || next == '-' {
            matched.push(next);
        } else {
            break;
        }
    }

    let next_index = matched.len();
    Ok((&input[next_index..], matched))
}

#[test]
fn test_parser() {
    let tag_opener = right(match_literal("<"), identifier);

    assert_eq!(
        Ok(("/>", "my-first-element".to_string())),
        tag_opener.parse("<my-first-element/>")
    );

    assert_eq!(Err("oops"), tag_opener.parse("oops"));
    assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
}