#![warn(missing_docs)]

//! An extension of [`Regex`] supporting [`Result`] in `replace` methods.
//!
//! The [`replace`], [`replace_all`] and [`replacen`] methods of [`Regex`] accept a function
//! returning the replacement for each matched substring, but they do not allow this
//! function to return a [`Result`]. This crate provides [`try_replace`], [`try_replace_all`],
//! and [`try_replacen`] which fill this gap.
//!
//! # Use
//! Include `use regex_try::RegexTry` to use the additional methods provided by this crate.
//!
//! ## Example
//!
//! ```edition2018
//! use regex::Regex;
//! use regex_try::RegexTry;
//!
//! pub fn main() -> Result<(), std::io::Error> {
//!   let re = Regex::new(r"load! (\w+);").unwrap();
//!   let template = std::fs::read_to_string("Cargo.toml")?;
//!   let result = re.try_replace_all(&template, |captures|
//!     // read_to_string returns a Result, and so it couldn't
//!     // be used with re.replace_all
//!     std::fs::read_to_string(&captures[1])
//!   )?;
//!   println!("{}", result);
//!   Ok(())
//! }
//! ```
//!
//! [`Result`]: https://doc.rust-lang.org/std/result/
//! [`Regex`]: https://docs.rs/regex/*/regex/struct.Regex.html
//! [`replace`]: https://docs.rs/regex/*/regex/struct.Regex.html#method.replace
//! [`replace_all`]: https://docs.rs/regex/*/regex/struct.Regex.html#method.replace_all
//! [`replacen`]: https://docs.rs/regex/*/regex/struct.Regex.html#method.replacen
//! [`try_replace`]: ./trait.RegexTry.html#tymethod.replace
//! [`try_replace_all`]: ./trait.RegexTry.html#tymethod.replace_all
//! [`try_replacen`]: ./trait.RegexTry.html#tymethod.replacen

use regex::Captures;
use regex::Regex;
use std::borrow::Cow;

/// Defines the additional methods for Regex.
///
/// The replacer is always a function of type `FnMut(&Captures) -> Result<String, E>`.
pub trait RegexTry<F, E> {
  /// See [`Regex::replacen`]
  ///
  /// [`Regex::replacen`]: https://docs.rs/regex/*/regex/struct.Regex.html#method.replacen
  fn try_replacen<'t>(&self, text: &'t str, limit: usize, rep: F) -> Result<Cow<'t, str>, E>;

  /// See [`Regex::replace`]
  ///
  /// [`Regex::replace`]: https://docs.rs/regex/*/regex/struct.Regex.html#method.replace
  fn try_replace<'t>(&self, text: &'t str, rep: F) -> Result<Cow<'t, str>, E>;

  /// See [`Regex::replace_all`]
  ///
  /// [`Regex::replace_all`]: https://docs.rs/regex/*/regex/struct.Regex.html#method.replace_all
  fn try_replace_all<'t>(&self, text: &'t str, rep: F) -> Result<Cow<'t, str>, E>;
}

impl<F, E> RegexTry<F, E> for Regex
where
  F: FnMut(&Captures) -> Result<String, E>,
{
  fn try_replacen<'t>(&self, text: &'t str, limit: usize, mut rep: F) -> Result<Cow<'t, str>, E> {
    let mut it = self.captures_iter(text).enumerate().peekable();
    if it.peek().is_none() {
      return Ok(Cow::Borrowed(text));
    }
    let mut new = String::with_capacity(text.len());
    let mut last_match = 0;
    for (i, cap) in it {
      if limit > 0 && i >= limit {
        break;
      }
      // unwrap on 0 is OK because captures only reports matches
      let m = cap.get(0).unwrap();
      new.push_str(&text[last_match..m.start()]);
      let replacement = rep(&cap)?;
      new.push_str(&replacement);
      last_match = m.end();
    }
    new.push_str(&text[last_match..]);
    Ok(Cow::Owned(new))
  }

  fn try_replace<'t>(&self, text: &'t str, rep: F) -> Result<Cow<'t, str>, E> {
    self.try_replacen(text, 1, rep)
  }

  fn try_replace_all<'t>(&self, text: &'t str, rep: F) -> Result<Cow<'t, str>, E> {
    self.try_replacen(text, 0, rep)
  }
}
