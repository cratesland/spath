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

//! # SPath: Query expressions for semi-structured data

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod node;
pub use node::*;

mod path;
pub use path::*;

mod spath;
pub use spath::*;

pub mod spec;

mod value;
pub use value::*;

mod parser;

#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "toml")]
pub mod toml;

/// An error that can occur during parsing the SPath query.
#[derive(Debug)]
pub struct ParseError {
    source: String,
    range: std::ops::Range<usize>,
    message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use annotate_snippets::Level;
        use annotate_snippets::Renderer;
        use annotate_snippets::Snippet;

        let message = Level::Error.title("failed to parse SPath query").snippet(
            Snippet::source(self.source.as_str()).annotation(
                Level::Error
                    .span(self.range.clone())
                    .label(self.message.as_str()),
            ),
        );

        let render = Renderer::plain();
        write!(f, "{}", render.render(message))?;
        Ok(())
    }
}

impl std::error::Error for ParseError {}
