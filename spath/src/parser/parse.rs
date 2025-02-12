// Copyright 2024 tison <wander4096@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::iter::Peekable;

use winnow::combinator::alt;
use winnow::combinator::delimited;
use winnow::combinator::opt;
use winnow::combinator::preceded;
use winnow::combinator::repeat;
use winnow::combinator::separated;
use winnow::combinator::separated_pair;
use winnow::Parser;

use crate::parser::error::Error;
use crate::parser::error::RefineError;
use crate::parser::input::text;
use crate::parser::input::Input;
use crate::parser::token::Token;
use crate::parser::token::TokenKind::*;
use crate::spec::function::FunctionRegistry;
use crate::spec::query::Query;
use crate::spec::query::QueryKind;
use crate::spec::segment::QuerySegment;
use crate::spec::segment::QuerySegmentKind;
use crate::spec::segment::Segment;
use crate::spec::selector::index::Index;
use crate::spec::selector::name::Name;
use crate::spec::selector::slice::Slice;
use crate::spec::selector::Selector;

pub fn parse_query_main<Registry>(input: &mut Input<Registry>) -> Result<Query, RefineError>
where
    Registry: FunctionRegistry,
{
    (parse_root_query, EOI)
        .map(|(query, _)| query)
        .parse_next(input)
}

fn parse_root_query<Registry>(input: &mut Input<Registry>) -> Result<Query, RefineError>
where
    Registry: FunctionRegistry,
{
    (text("$"), parse_path_segments)
        .map(|(_, segments)| Query {
            kind: QueryKind::Root,
            segments,
        })
        .parse_next(input)
}

fn parse_path_segments<Registry>(
    input: &mut Input<Registry>,
) -> Result<Vec<QuerySegment>, RefineError>
where
    Registry: FunctionRegistry,
{
    repeat(0.., parse_segment).parse_next(input)
}

fn parse_segment<Registry>(input: &mut Input<Registry>) -> Result<QuerySegment, RefineError>
where
    Registry: FunctionRegistry,
{
    alt((
        parse_descendant_segment.map(|segment| QuerySegment {
            kind: QuerySegmentKind::Descendant,
            segment,
        }),
        parse_child_segment.map(|segment| QuerySegment {
            kind: QuerySegmentKind::Child,
            segment,
        }),
    ))
    .parse_next(input)
}

fn parse_descendant_segment<Registry>(input: &mut Input<Registry>) -> Result<Segment, RefineError>
where
    Registry: FunctionRegistry,
{
    preceded(
        text(".."),
        alt((
            parse_wildcard_selector.map(|_| Segment::Wildcard),
            parse_child_long_hand,
            parse_dot_member_name.map(Segment::DotName),
        )),
    )
    .parse_next(input)
}

fn parse_child_segment<Registry>(input: &mut Input<Registry>) -> Result<Segment, RefineError>
where
    Registry: FunctionRegistry,
{
    alt((
        parse_dot_wildcard_shorthand,
        parse_dot_member_name_shorthand,
        parse_child_long_hand,
    ))
    .parse_next(input)
}

fn parse_child_long_hand<Registry>(input: &mut Input<Registry>) -> Result<Segment, RefineError>
where
    Registry: FunctionRegistry,
{
    delimited(
        LBracket,
        parse_multi_selector.map(Segment::LongHand),
        RBracket,
    )
    .parse_next(input)
}

fn parse_multi_selector<Registry>(input: &mut Input<Registry>) -> Result<Vec<Selector>, RefineError>
where
    Registry: FunctionRegistry,
{
    separated(1.., parse_selector, text(",")).parse_next(input)
}

fn parse_selector<Registry>(input: &mut Input<Registry>) -> Result<Selector, RefineError>
where
    Registry: FunctionRegistry,
{
    alt((
        parse_wildcard_selector,
        parse_name_selector,
        parse_array_slice_selector,
        parse_index_selector,
        // parse_filter_selector,
    ))
    .parse_next(input)
}

fn parse_index_selector<Registry>(input: &mut Input<Registry>) -> Result<Selector, RefineError>
where
    Registry: FunctionRegistry,
{
    parse_index.map(Selector::Index).parse_next(input)
}

fn parse_index<Registry>(input: &mut Input<Registry>) -> Result<Index, RefineError>
where
    Registry: FunctionRegistry,
{
    LiteralInteger
        .try_map(|i| parse_integer(i))
        .map(Index::new)
        .parse_next(input)
}

fn parse_array_slice_selector<Registry>(
    input: &mut Input<Registry>,
) -> Result<Selector, RefineError>
where
    Registry: FunctionRegistry,
{
    parse_array_slice
        .map(Selector::ArraySlice)
        .parse_next(input)
}

fn parse_array_slice<Registry>(input: &mut Input<Registry>) -> Result<Slice, RefineError>
where
    Registry: FunctionRegistry,
{
    separated_pair(
        opt(LiteralInteger),
        text(":"),
        alt((
            separated_pair(opt(LiteralInteger), text(":"), opt(LiteralInteger)),
            opt(LiteralInteger).map(|i| (i, None)),
        )),
    )
    .try_map(|(start, (end, step))| {
        let start = start.map(|i| parse_integer(i)).transpose()?;
        let end = end.map(|i| parse_integer(i)).transpose()?;
        let step = step.map(|i| parse_integer(i)).transpose()?;
        Ok::<Slice, Error>(Slice::new(start, end, step))
    })
    .parse_next(input)
}

fn parse_name_selector<Registry>(input: &mut Input<Registry>) -> Result<Selector, RefineError>
where
    Registry: FunctionRegistry,
{
    LiteralString
        .try_map(|s| parse_string(s))
        .map(|name| Selector::Name(Name::new(name)))
        .parse_next(input)
}

fn parse_wildcard_selector<Registry>(input: &mut Input<Registry>) -> Result<Selector, RefineError>
where
    Registry: FunctionRegistry,
{
    text("*").map(|_| Selector::Wildcard).parse_next(input)
}

fn parse_dot_wildcard_shorthand<Registry>(
    input: &mut Input<Registry>,
) -> Result<Segment, RefineError>
where
    Registry: FunctionRegistry,
{
    preceded(text("."), parse_wildcard_selector)
        .map(|_| Segment::Wildcard)
        .parse_next(input)
}

fn parse_dot_member_name_shorthand<Registry>(
    input: &mut Input<Registry>,
) -> Result<Segment, RefineError>
where
    Registry: FunctionRegistry,
{
    preceded(text("."), parse_dot_member_name)
        .map(Segment::DotName)
        .parse_next(input)
}

fn parse_dot_member_name<Registry>(input: &mut Input<Registry>) -> Result<String, RefineError>
where
    Registry: FunctionRegistry,
{
    alt((Identifier, TRUE, FALSE, NULL))
        .map(|ident| ident.text().to_string())
        .parse_next(input)
}

fn parse_integer(token: &Token) -> Result<i64, Error> {
    let text = token.text();
    text.parse()
        .map_err(|err| Error::new(token.span, format!("{err}")))
}

fn parse_string(token: &Token) -> Result<String, Error> {
    let text = token.text();
    let mut chars = text.chars();

    let quote = chars.next().expect("quote char always exist");
    if chars.next_back() != Some(quote) {
        return Err(Error::new(token.span, "mismatched quote"));
    }

    let mut chars = chars.peekable();
    let mut output = String::new();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('b') => output.push('\u{0008}'),
                Some('f') => output.push('\u{000C}'),
                Some('n') => output.push('\n'),
                Some('r') => output.push('\r'),
                Some('t') => output.push('\t'),
                Some('\\') => output.push('\\'),
                Some('u') => output.push(
                    unescape_unicode(&mut chars)
                        .ok_or_else(|| Error::new(token.span, "invalid escape sequence"))?,
                ),
                Some('x') => output.push(
                    unescape_byte(&mut chars)
                        .ok_or_else(|| Error::new(token.span, "invalid escape sequence"))?,
                ),
                Some(c) if c.is_digit(8) => output.push(unescape_octal(c, &mut chars)),
                Some(c) if c == quote => output.push(quote),
                _ => return Err(Error::new(token.span, "invalid escape sequence")),
            };
        } else if c == quote {
            return Err(Error::new(token.span, "intermediately close quote"));
        } else {
            output.push(c);
        }
    }

    Ok(output)
}

fn unescape_unicode(chars: &mut Peekable<impl Iterator<Item = char>>) -> Option<char> {
    let mut code = 0;

    for c in chars.take(4) {
        code = code * 16 + c.to_digit(16)?;
    }

    char::from_u32(code)
}

fn unescape_byte(chars: &mut Peekable<impl Iterator<Item = char>>) -> Option<char> {
    let mut byte = 0;

    for c in chars.take(2) {
        byte = byte * 16 + c.to_digit(16)?;
    }

    char::from_u32(byte)
}

fn unescape_octal(c1: char, chars: &mut Peekable<impl Iterator<Item = char>>) -> char {
    let mut oct = c1.to_digit(8).unwrap();

    while let Some(c) = chars.peek() {
        if let Some(digit) = c.to_digit(8) {
            oct = oct * 8 + digit;
            chars.next();
        } else {
            break;
        }
    }

    char::from_u32(oct).unwrap()
}
