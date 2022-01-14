//! String interner for Boa.
//!
//! The idea behind using a string interner is that in most of the code, strings such as
//! identifiers and literals are often repeated. This causes extra burden when comparing them and
//! storing them. A string interner stores a unique `usize` symbol for each string, making sure
//! that there are no duplicates. This makes it much easier to compare, since it's just comparing
//! to `usize`, and also it's easier to store, since instead of a heap-allocated string, you only
//! need to store a `usize`. This reduces memory consumption and improves performance in the
//! compiler.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/boa-dev/boa/main/assets/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/boa-dev/boa/main/assets/logo.svg"
)]
#![deny(
    clippy::all,
    unused_qualifications,
    unused_import_braces,
    unused_lifetimes,
    unreachable_pub,
    trivial_numeric_casts,
    // rustdoc,
    missing_debug_implementations,
    missing_copy_implementations,
    deprecated_in_future,
    meta_variable_misuse,
    non_ascii_idents,
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
)]
#![warn(clippy::perf, clippy::single_match_else, clippy::dbg_macro)]
#![allow(
    clippy::missing_inline_in_public_items,
    clippy::cognitive_complexity,
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::as_conversions,
    clippy::let_unit_value,
    rustdoc::missing_doc_code_examples
)]

#[cfg(test)]
mod tests;

use std::{fmt::Display, num::NonZeroUsize};

use gc::{unsafe_empty_trace, Finalize, Trace};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use string_interner::{backend::BucketBackend, StringInterner, Symbol};

/// Backend of the string interner.
type Backend = BucketBackend<Sym>;

/// The string interner for Boa.
///
/// This is a type alias that makes it easier to reference it in the code.
#[derive(Debug)]
pub struct Interner {
    inner: StringInterner<Backend>,
}

impl Interner {
    /// Creates a new StringInterner with the given initial capacity.
    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: StringInterner::with_capacity(cap),
        }
    }

    /// Returns the number of strings interned by the interner.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the string interner has no interned strings.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the symbol for the given string if any.
    ///
    /// Can be used to query if a string has already been interned without interning.
    pub fn get<T>(&self, string: T) -> Option<Sym>
    where
        T: AsRef<str>,
    {
        let string = string.as_ref();
        Self::get_static(string).or_else(|| self.inner.get(string))
    }

    /// Interns the given string.
    ///
    /// Returns a symbol for resolution into the original string.
    ///
    /// # Panics
    /// If the interner already interns the maximum number of strings possible by the chosen symbol type.
    pub fn get_or_intern<T>(&mut self, string: T) -> Sym
    where
        T: AsRef<str>,
    {
        let string = string.as_ref();
        Self::get_static(string).unwrap_or_else(|| self.inner.get_or_intern(string))
    }

    /// Interns the given `'static` string.
    ///
    /// Returns a symbol for resolution into the original string.
    ///
    /// # Note
    ///
    /// This is more efficient than [`StringInterner::get_or_intern`] since it might
    /// avoid some memory allocations if the backends supports this.
    ///
    /// # Panics
    ///
    /// If the interner already interns the maximum number of strings possible
    /// by the chosen symbol type.
    pub fn get_or_intern_static(&mut self, string: &'static str) -> Sym {
        Self::get_static(string).unwrap_or_else(|| self.inner.get_or_intern_static(string))
    }

    /// Shrink backend capacity to fit the interned strings exactly.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit()
    }

    /// Returns the string for the given symbol if any.
    #[inline]
    pub fn resolve(&self, symbol: Sym) -> Option<&str> {
        let index = symbol.as_raw().get();
        if index <= Self::STATIC_STRINGS.len() {
            Some(Self::STATIC_STRINGS[index - 1])
        } else {
            self.inner.resolve(symbol)
        }
    }

    /// Gets the symbol of the static string if one of them
    fn get_static(string: &str) -> Option<Sym> {
        Self::STATIC_STRINGS
            .into_iter()
            .enumerate()
            .find(|&(_i, s)| string == s)
            .map(|(i, _str)| {
                let raw = NonZeroUsize::new(i.wrapping_add(1)).expect("static array too big");
                Sym::from_raw(raw)
            })
    }
}

impl<T> FromIterator<T> for Interner
where
    T: AsRef<str>,
{
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            inner: StringInterner::from_iter(iter),
        }
    }
}

impl<T> Extend<T> for Interner
where
    T: AsRef<str>,
{
    #[inline]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.inner.extend(iter)
    }
}

impl<'a> IntoIterator for &'a Interner {
    type Item = (Sym, &'a str);
    type IntoIter = <&'a Backend as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl Default for Interner {
    fn default() -> Self {
        Self {
            inner: StringInterner::new(),
        }
    }
}

/// The string symbol type for Boa.
///
/// This symbol type is internally a `NonZeroUsize`, which makes it pointer-width in size and it's
/// optimized so that it can ocupy 1 pointer width even in an `Option` type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Finalize)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Sym {
    value: NonZeroUsize,
}

impl Sym {
    /// Padding for the symbol internal value.
    const PADDING: usize = Interner::STATIC_STRINGS.len() + 1;

    /// Creates a `Sym` from a raw `NonZeroUsize`.
    const fn from_raw(value: NonZeroUsize) -> Self {
        Self { value }
    }

    /// Retrieves the raw `NonZeroUsize` for this symbol.`
    const fn as_raw(self) -> NonZeroUsize {
        self.value
    }
}

impl Symbol for Sym {
    #[inline]
    fn try_from_usize(index: usize) -> Option<Self> {
        index
            .checked_add(Self::PADDING)
            .and_then(NonZeroUsize::new)
            .map(|value| Self { value })
    }

    #[inline]
    fn to_usize(self) -> usize {
        self.value.get() - Self::PADDING
    }
}

// Safe because `Sym` implements `Copy`.
unsafe impl Trace for Sym {
    unsafe_empty_trace!();
}

/// Converts a given element to a string using an interner.
pub trait ToInternedString {
    /// Converts a given element to a string using an interner.
    fn to_interned_string(&self, interner: &Interner) -> String;
}

impl<T> ToInternedString for T
where
    T: Display,
{
    fn to_interned_string(&self, _interner: &Interner) -> String {
        format!("{}", self)
    }
}

macro_rules! yes {
    ($({$id:ident, $name:literal, $number:literal}),*) => {
        impl Interner {
            /// List of commonly used static strings.
            ///
            /// Make sure that any string added as a `Sym` constant is also added here.
            const STATIC_STRINGS: [&'static str; 93] = [
                $(
                    $name,
                )*
            ];
        }

        impl Sym {
            $(
                #[doc="Symbol for the `"]
                #[doc=$name]
                #[doc="` string."]
                pub const $id: Self = unsafe { Self::from_raw(NonZeroUsize::new_unchecked($number)) };
            )*
        }
    }
}

yes!(
    {EMPTY_STRING, "", 1},
    {ARGUMENTS, "arguments", 2},
    {AWAIT, "await", 3},
    {YIELD, "yield", 4},
    {EVAL, "eval", 5},
    {DEFAULT, "default", 6},
    {NULL, "null", 7},
    {REGEXP, "RegExp", 8},
    {GET, "get", 9},
    {SET, "set", 10},
    {MAIN, "<main>", 11},
    {RAW, "raw", 12},
    {LENGTH, "length", 13},
    {NAME, "name", 14},
    {PROTOTYPE, "prototype", 15},
    {NEXT, "next", 16},
    {CONSTRUCTOR, "constructor", 17},
    {MESSAGE, "message", 18},
    {WRITABLE, "writable", 19},
    {VALUE, "value", 20},
    {CONFIGURABLE, "configurable", 21},
    {ENUMERABLE, "enumerable", 22},
    {CONSTRUCT, "construct", 23},
    {TO_ISO_STRING, "toISOString", 24},
    {CALLEE, "callee", 25},
    {DONE, "done", 26},
    {JOIN, "join", 27},
    {RETURN, "return", 28},
    {APPLY, "apply", 29},
    {OWN_KEYS, "ownKeys", 30},
    {DELETE_PROPERTY, "deleteProperty", 31},
    {HAS, "has", 32},
    {DEFINE_PROPERTY, "defineProperty", 33},
    {GET_OWN_PROPERTY_DESCRIPTOR, "getOwnPropertyDescriptor", 34},
    {PREVENT_EXTENSIONS, "preventExtensions", 35},
    {IS_EXTENSIBLE, "isExtensible", 36},
    {SET_PROTOTYPE_OF, "setPrototypeOf", 37},
    {GET_PROTOTYPE_OF, "getPrototypeOf", 38},
    {BYTE_OFFSET, "byteOffset", 39},
    {BYTE_LENGTH, "byteLength", 40},
    {BUFFER, "buffer", 41},
    {BYTES_PER_ELEMENT, "BYTES_PER_ELEMENT", 42},
    {DESCRIPTION, "description", 43},
    {UNSCOPABLES, "unscopables", 44},
    {TO_STRING_TAG, "toStringTag", 45},
    {TO_PRIMITIVE, "toPrimitive", 46},
    {SPLIT, "split", 47},
    {SPECIES, "species", 48},
    {SEARCH, "search", 49},
    {REPLACE, "replace", 50},
    {MATCH_ALL, "matchAll", 51},
    {MATCH, "match", 52},
    {ITERATOR, "iterator", 53},
    {IS_CONCAT_SPREADABLE, "isConcatSpreadable", 54},
    {HAS_INSTANCE, "hasInstance", 55},
    {ASYNC_ITERATOR, "asyncIterator", 56},
    {FLAGS, "flags", 57},
    {ADD, "add", 58},
    {VALUES, "values", 59},
    {KEYS, "keys", 60},
    {SIZE, "size", 61},
    {LAST_INDEX, "lastIndex", 62},
    {INDEX, "index", 63},
    {GROUPS, "groups", 64},
    {UNICODE, "unicode", 65},
    {GLOBAL, "global", 66},
    {INPUT, "input", 67},
    {EXEC, "exec", 68},
    {STICKY, "sticky", 69},
    {DOT_ALL, "dotAll", 70},
    {MULTILINE, "multiline", 71},
    {IGNORE_CASE, "ignoreCase", 72},
    {SOURCE,"source", 73},
    {REVOKE, "revoke", 74},
    {PROXY, "proxy", 75},
    {ENTRIES, "entries", 76},
    {TO_JSON, "toJSON", 77},
    {EPSILON, "EPSILON", 78},
    {MAX_SAFE_INTEGER, "MAX_SAFE_INTEGER", 79},
    {MIN_SAFE_INTEGER, "MIN_SAFE_INTEGER",80},
    {MAX_VALUE, "MAX_VALUE", 81},
    {MIN_VALUE, "MIN_VALUE", 82},
    {NEGATIVE_INFINITY, "NEGATIVE_INFINITY", 83},
    {POSITIVE_INFINITY, "POSITIVE_INFINITY", 84},
    {NAN, "NaN", 85},
    {E, "E", 86},
    {LN10, "LN10", 87},
    {LN2, "LN2", 88},
    {LOG10E, "LOG10E", 89},
    {LOG2E, "LOG2E", 90},
    {PI, "PI", 91},
    {SQRT1_2, "SQRT1_2", 92},
    {SQRT2, "SQRT2", 93}
);
