mod adapter;
mod parser;
mod printer;

use adapter::*;
use parser::*;
use printer::*;

use anyhow::Result;

pub fn transpile_rust_to_msl(rust_source: &str) -> Result<String> {
  let rust_ast = parse_rust_into_ast(rust_source)?;
  let msl_compatible_ast = make_rust_ast_msl_compatible(rust_ast);
  print_ast_into_msl(msl_compatible_ast)
}
