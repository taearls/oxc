use std::cell::RefCell;

use oxc_allocator::Allocator;
use oxc_ast::{
    Comment,
    ast::{FunctionBody, Program},
};
use oxc_span::{GetSpan, SourceType, Span};
use rustc_hash::FxHashMap;

use crate::{formatter::FormatElement, generated::ast_nodes::AstNode, options::FormatOptions};

use super::Comments;

/// Context object storing data relevant when formatting an object.
#[derive(Clone)]
pub struct FormatContext<'ast> {
    options: FormatOptions,

    source_text: &'ast str,

    source_type: SourceType,

    comments: Comments<'ast>,

    cached_elements: FxHashMap<Span, FormatElement<'ast>>,

    /// Flag to track if we're currently formatting call arguments
    /// Used to prevent double indentation in arrow function parameters
    in_call_arguments: RefCell<bool>,

    allocator: &'ast Allocator,
}

impl std::fmt::Debug for FormatContext<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FormatContext")
            .field("options", &self.options)
            .field("source_text", &self.source_text)
            .field("source_type", &self.source_type)
            .field("comments", &self.comments)
            .field("cached_elements", &self.cached_elements)
            .finish()
    }
}

impl<'ast> FormatContext<'ast> {
    pub fn new(
        program: &'ast Program<'ast>,
        allocator: &'ast Allocator,
        options: FormatOptions,
    ) -> Self {
        Self {
            options,
            source_text: program.source_text,
            source_type: program.source_type,
            comments: Comments::new(program.source_text, &program.comments),
            allocator,
            cached_elements: FxHashMap::default(),
            in_call_arguments: RefCell::new(false),
        }
    }

    /// Returns the formatting options
    pub fn options(&self) -> &FormatOptions {
        &self.options
    }

    /// Returns a reference to the program's comments.
    pub fn comments(&self) -> &Comments<'ast> {
        &self.comments
    }

    /// Returns a reference to the program's comments.
    pub fn comments_mut(&mut self) -> &mut Comments<'ast> {
        &mut self.comments
    }

    /// Returns the formatting options
    pub fn source_text(&self) -> &'ast str {
        self.source_text
    }

    /// Returns the source type
    pub fn source_type(&self) -> SourceType {
        self.source_type
    }

    /// Returns the cached formatted element for the given key.
    pub(crate) fn get_cached_element<T: GetSpan>(&self, key: &T) -> Option<FormatElement<'ast>> {
        self.cached_elements.get(&key.span()).cloned()
    }

    /// Caches the formatted element for the given key.
    pub(crate) fn cache_element<T: GetSpan>(&mut self, key: &T, formatted: FormatElement<'ast>) {
        self.cached_elements.insert(key.span(), formatted);
    }

    pub(crate) fn increment_printed_count(&mut self) {
        self.comments.increment_printed_count();
    }

    pub fn allocator(&self) -> &'ast Allocator {
        self.allocator
    }

    /// Check if we're currently formatting call arguments
    pub(crate) fn is_in_call_arguments(&self) -> bool {
        *self.in_call_arguments.borrow()
    }

    /// Set the call arguments formatting flag
    pub(crate) fn set_in_call_arguments(&self, value: bool) {
        *self.in_call_arguments.borrow_mut() = value;
    }
}
