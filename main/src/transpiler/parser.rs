use anyhow::{Context, Result};

pub fn parse_rust_into_ast(rust_source: &str) -> Result<syn::File> {
  syn::parse_str(rust_source).with_context(|| "Parse error")
}
