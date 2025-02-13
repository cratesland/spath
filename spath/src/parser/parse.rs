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
use winnow::error::FromExternalError;
use winnow::stream::Stream;
use winnow::Parser;

use crate::parser::error::Error;
use crate::parser::input::text;
use crate::parser::input::Input;
use crate::parser::token::Token;
use crate::parser::token::TokenKind::*;
use crate::spec::function::FunctionExpr;
use crate::spec::function::FunctionExprArg;
use crate::spec::function::FunctionRegistry;
use crate::spec::function::FunctionValidationError;
use crate::spec::function::SPathType;
use crate::spec::query::Query;
use crate::spec::query::QueryKind;
use crate::spec::segment::QuerySegment;
use crate::spec::segment::QuerySegmentKind;
use crate::spec::segment::Segment;
use crate::spec::selector::filter::BasicExpr;
use crate::spec::selector::filter::Comparable;
use crate::spec::selector::filter::ComparisonExpr;
use crate::spec::selector::filter::ComparisonOperator;
use crate::spec::selector::filter::ExistExpr;
use crate::spec::selector::filter::Filter;
use crate::spec::selector::filter::LogicalAndExpr;
use crate::spec::selector::filter::LogicalOrExpr;
use crate::spec::selector::filter::SingularQuery;
use crate::spec::selector::index::Index;
use crate::spec::selector::name::Name;
use crate::spec::selector::slice::Slice;
use crate::spec::selector::Selector;
use crate::Literal;

pub fn parse_query_main<Registry>(input: &mut Input<Registry>) -> Result<Query, Error>
where
    Registry: FunctionRegistry,
{
    (parse_root_query, EOI)
        .map(|(query, _)| query)
        .parse_next(input)
}

fn parse_root_query<Registry>(input: &mut Input<Registry>) -> Result<Query, Error>
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

fn parse_path_segments<Registry>(input: &mut Input<Registry>) -> Result<Vec<QuerySegment>, Error>
where
    Registry: FunctionRegistry,
{
    repeat(0.., parse_segment).parse_next(input)
}

fn parse_segment<Registry>(input: &mut Input<Registry>) -> Result<QuerySegment, Error>
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

fn parse_descendant_segment<Registry>(input: &mut Input<Registry>) -> Result<Segment, Error>
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

fn parse_child_segment<Registry>(input: &mut Input<Registry>) -> Result<Segment, Error>
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

fn parse_child_long_hand<Registry>(input: &mut Input<Registry>) -> Result<Segment, Error>
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

fn parse_multi_selector<Registry>(input: &mut Input<Registry>) -> Result<Vec<Selector>, Error>
where
    Registry: FunctionRegistry,
{
    separated(1.., parse_selector, text(",")).parse_next(input)
}

fn parse_selector<Registry>(input: &mut Input<Registry>) -> Result<Selector, Error>
where
    Registry: FunctionRegistry,
{
    alt((
        parse_wildcard_selector,
        parse_name_selector,
        parse_array_slice_selector,
        parse_index_selector,
        parse_filter_selector,
    ))
    .parse_next(input)
}

fn parse_index_selector<Registry>(input: &mut Input<Registry>) -> Result<Selector, Error>
where
    Registry: FunctionRegistry,
{
    parse_index.map(Selector::Index).parse_next(input)
}

fn parse_index<Registry>(input: &mut Input<Registry>) -> Result<Index, Error>
where
    Registry: FunctionRegistry,
{
    LiteralInteger
        .try_map(parse_integer)
        .map(Index::new)
        .parse_next(input)
}

fn parse_array_slice_selector<Registry>(input: &mut Input<Registry>) -> Result<Selector, Error>
where
    Registry: FunctionRegistry,
{
    parse_array_slice
        .map(Selector::ArraySlice)
        .parse_next(input)
}

fn parse_array_slice<Registry>(input: &mut Input<Registry>) -> Result<Slice, Error>
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

fn parse_name_selector<Registry>(input: &mut Input<Registry>) -> Result<Selector, Error>
where
    Registry: FunctionRegistry,
{
    LiteralString
        .try_map(parse_string)
        .map(|name| Selector::Name(Name::new(name)))
        .parse_next(input)
}

fn parse_wildcard_selector<Registry>(input: &mut Input<Registry>) -> Result<Selector, Error>
where
    Registry: FunctionRegistry,
{
    text("*").map(|_| Selector::Wildcard).parse_next(input)
}

fn parse_dot_wildcard_shorthand<Registry>(input: &mut Input<Registry>) -> Result<Segment, Error>
where
    Registry: FunctionRegistry,
{
    preceded(text("."), parse_wildcard_selector)
        .map(|_| Segment::Wildcard)
        .parse_next(input)
}

fn parse_dot_member_name_shorthand<Registry>(input: &mut Input<Registry>) -> Result<Segment, Error>
where
    Registry: FunctionRegistry,
{
    preceded(text("."), parse_dot_member_name)
        .map(Segment::DotName)
        .parse_next(input)
}

fn parse_dot_member_name<Registry>(input: &mut Input<Registry>) -> Result<String, Error>
where
    Registry: FunctionRegistry,
{
    alt((Identifier, TRUE, FALSE, NULL))
        .map(|ident| ident.text().to_string())
        .parse_next(input)
}

fn parse_filter_selector<Registry>(input: &mut Input<Registry>) -> Result<Selector, Error>
where
    Registry: FunctionRegistry,
{
    parse_filter.map(Selector::Filter).parse_next(input)
}

fn parse_filter<Registry>(input: &mut Input<Registry>) -> Result<Filter, Error>
where
    Registry: FunctionRegistry,
{
    preceded(text("?"), parse_logical_or_expr)
        .map(Filter)
        .parse_next(input)
}

fn parse_logical_or_expr<Registry>(input: &mut Input<Registry>) -> Result<LogicalOrExpr, Error>
where
    Registry: FunctionRegistry,
{
    separated(1.., parse_logical_and, text("||"))
        .map(LogicalOrExpr)
        .parse_next(input)
}

fn parse_logical_and<Registry>(input: &mut Input<Registry>) -> Result<LogicalAndExpr, Error>
where
    Registry: FunctionRegistry,
{
    separated(1.., parse_basic_expr, text("&&"))
        .map(LogicalAndExpr)
        .parse_next(input)
}

fn parse_basic_expr<Registry>(input: &mut Input<Registry>) -> Result<BasicExpr, Error>
where
    Registry: FunctionRegistry,
{
    alt((
        parse_not_parent_expr,
        parse_paren_expr,
        parse_comp_expr.map(BasicExpr::Relation),
        parse_not_exist_expr,
        parse_exist_expr,
        parse_not_func_expr,
        parse_func_expr,
    ))
    .parse_next(input)
}

fn parse_paren_expr<Registry>(input: &mut Input<Registry>) -> Result<BasicExpr, Error>
where
    Registry: FunctionRegistry,
{
    parse_paren_expr_inner
        .map(BasicExpr::Paren)
        .parse_next(input)
}

fn parse_not_parent_expr<Registry>(input: &mut Input<Registry>) -> Result<BasicExpr, Error>
where
    Registry: FunctionRegistry,
{
    preceded(text("!"), parse_paren_expr_inner)
        .map(BasicExpr::ParenNot)
        .parse_next(input)
}

fn parse_paren_expr_inner<Registry>(input: &mut Input<Registry>) -> Result<LogicalOrExpr, Error>
where
    Registry: FunctionRegistry,
{
    delimited(text("("), parse_logical_or_expr, text(")")).parse_next(input)
}

fn parse_exist_expr<Registry>(input: &mut Input<Registry>) -> Result<BasicExpr, Error>
where
    Registry: FunctionRegistry,
{
    parse_exist_expr_inner
        .map(BasicExpr::Exist)
        .parse_next(input)
}

fn parse_not_exist_expr<Registry>(input: &mut Input<Registry>) -> Result<BasicExpr, Error>
where
    Registry: FunctionRegistry,
{
    preceded(text("!"), parse_exist_expr_inner)
        .map(BasicExpr::NotExist)
        .parse_next(input)
}

fn parse_exist_expr_inner<Registry>(input: &mut Input<Registry>) -> Result<ExistExpr, Error>
where
    Registry: FunctionRegistry,
{
    parse_query.map(ExistExpr).parse_next(input)
}

fn parse_current_query<Registry>(input: &mut Input<Registry>) -> Result<Query, Error>
where
    Registry: FunctionRegistry,
{
    preceded(text("@"), parse_path_segments)
        .map(|segments| Query {
            kind: QueryKind::Current,
            segments,
        })
        .parse_next(input)
}

fn parse_query<Registry>(input: &mut Input<Registry>) -> Result<Query, Error>
where
    Registry: FunctionRegistry,
{
    alt((parse_root_query, parse_current_query)).parse_next(input)
}

fn parse_comp_expr<Registry>(input: &mut Input<Registry>) -> Result<ComparisonExpr, Error>
where
    Registry: FunctionRegistry,
{
    (
        parse_comparable,
        parse_comparison_operator,
        parse_comparable,
    )
        .map(|(left, op, right)| ComparisonExpr { left, op, right })
        .parse_next(input)
}

fn parse_comparable<Registry>(input: &mut Input<Registry>) -> Result<Comparable, Error>
where
    Registry: FunctionRegistry,
{
    alt((
        parse_literal_comparable,
        parse_singular_path_comparable,
        parse_function_expr_comparable,
    ))
    .parse_next(input)
}

fn parse_singular_path_comparable<Registry>(
    input: &mut Input<Registry>,
) -> Result<Comparable, Error>
where
    Registry: FunctionRegistry,
{
    parse_singular_path
        .map(Comparable::SingularQuery)
        .parse_next(input)
}

fn parse_singular_path<Registry>(input: &mut Input<Registry>) -> Result<SingularQuery, Error>
where
    Registry: FunctionRegistry,
{
    parse_query
        .try_map(|query| query.try_into())
        .parse_next(input)
}

fn parse_func_expr<Registry>(input: &mut Input<Registry>) -> Result<BasicExpr, Error>
where
    Registry: FunctionRegistry,
{
    parse_func_expr_inner
        .map(BasicExpr::FuncExpr)
        .parse_next(input)
}

fn parse_not_func_expr<Registry>(input: &mut Input<Registry>) -> Result<BasicExpr, Error>
where
    Registry: FunctionRegistry,
{
    preceded(text("!"), parse_func_expr_inner)
        .map(BasicExpr::FuncNotExpr)
        .parse_next(input)
}

fn parse_func_expr_inner<Registry>(input: &mut Input<Registry>) -> Result<FunctionExpr, Error>
where
    Registry: FunctionRegistry,
{
    parse_function_expr
        .try_map(|expr| match expr.return_type {
            SPathType::Logical | SPathType::Nodes => Ok(expr),
            _ => Err(FunctionValidationError::IncorrectFunctionReturnType),
        })
        .parse_next(input)
}

fn parse_function_expr_comparable<Registry>(
    input: &mut Input<Registry>,
) -> Result<Comparable, Error>
where
    Registry: FunctionRegistry,
{
    parse_function_expr
        .try_map(|expr| match expr.return_type {
            SPathType::Value => Ok(Comparable::FunctionExpr(expr)),
            _ => Err(FunctionValidationError::IncorrectFunctionReturnType),
        })
        .parse_next(input)
}

fn parse_function_expr<Registry>(input: &mut Input<Registry>) -> Result<FunctionExpr, Error>
where
    Registry: FunctionRegistry,
{
    let start = input.checkpoint();

    let (name, args) = (
        Identifier,
        delimited(
            text("("),
            separated(0.., parse_function_argument, text(",")),
            text(")"),
        ),
    )
        .parse_next(input)?;

    let registry = input.state.registry();
    let name = name.text().to_string();
    let args: Vec<FunctionExprArg> = args;

    let function = match registry.get(name.as_str()) {
        Some(function) => function,
        None => {
            return Err(FunctionValidationError::Undefined { name }).map_err(|err| {
                input.reset(&start);
                Error::from_external_error(input, err)
            })
        }
    };

    function
        .validate(args.as_slice(), registry)
        .map_err(|err| {
            input.reset(&start);
            Error::from_external_error(input, err)
        })?;

    Ok(FunctionExpr {
        name,
        args,
        return_type: function.result_type(),
    })
}

fn parse_function_argument<Registry>(input: &mut Input<Registry>) -> Result<FunctionExprArg, Error>
where
    Registry: FunctionRegistry,
{
    alt((
        parse_literal.map(FunctionExprArg::Literal),
        parse_singular_path.map(FunctionExprArg::SingularQuery),
        parse_query.map(FunctionExprArg::FilterQuery),
        parse_function_expr.map(FunctionExprArg::FunctionExpr),
        parse_logical_or_expr.map(FunctionExprArg::LogicalExpr),
    ))
    .parse_next(input)
}

fn parse_literal_comparable<Registry>(input: &mut Input<Registry>) -> Result<Comparable, Error>
where
    Registry: FunctionRegistry,
{
    parse_literal.map(Comparable::Literal).parse_next(input)
}

fn parse_literal<Registry>(input: &mut Input<Registry>) -> Result<Literal, Error>
where
    Registry: FunctionRegistry,
{
    alt((
        LiteralString.try_map(parse_string).map(Literal::String),
        LiteralInteger.try_map(parse_integer).map(Literal::Int),
        LiteralFloat.try_map(parse_float).map(Literal::Float),
        TRUE.map(|_| Literal::Bool(true)),
        FALSE.map(|_| Literal::Bool(false)),
        NULL.map(|_| Literal::Null),
    ))
    .parse_next(input)
}

fn parse_comparison_operator<Registry>(
    input: &mut Input<Registry>,
) -> Result<ComparisonOperator, Error>
where
    Registry: FunctionRegistry,
{
    alt((
        text("==").map(|_| ComparisonOperator::EqualTo),
        text("!=").map(|_| ComparisonOperator::NotEqualTo),
        text("<=").map(|_| ComparisonOperator::LessThanEqualTo),
        text("<").map(|_| ComparisonOperator::LessThan),
        text(">=").map(|_| ComparisonOperator::GreaterThanEqualTo),
        text(">").map(|_| ComparisonOperator::GreaterThan),
    ))
    .parse_next(input)
}

fn parse_integer(token: &Token) -> Result<i64, Error> {
    let text = token.text();
    text.parse()
        .map_err(|err| Error::new(token.span, format!("{err}")))
}

fn parse_float(token: &Token) -> Result<f64, Error> {
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
