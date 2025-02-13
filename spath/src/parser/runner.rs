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

use crate::parser::error::Error;
use crate::parser::input::Input;
use crate::parser::input::InputState;
use crate::parser::input::TokenSlice;
use crate::parser::parse::parse_query_main;
use crate::parser::token::Token;
use crate::parser::token::Tokenizer;
use crate::spec::function::FunctionRegistry;
use crate::spec::query::Query;
use crate::ParseError;
use crate::VariantValue;

pub fn run_tokenizer(source: &str) -> Result<Vec<Token>, Error> {
    Tokenizer::new(source).collect::<Result<_, _>>()
}

pub fn run_parser<T, Registry>(
    source: &str,
    registry: Registry,
) -> Result<(Query, Registry), ParseError>
where
    T: VariantValue,
    Registry: FunctionRegistry<Value = T>,
{
    let tokens = run_tokenizer(source).map_err(|err| err.into_parse_error(source))?;

    let input = TokenSlice::new(&tokens);
    let state = InputState::new(registry);
    let mut input = Input { input, state };

    let query = parse_query_main(&mut input).map_err(|err| err.into_parse_error(source))?;
    let registry = input.state.into_registry();
    Ok((query, registry))
}
