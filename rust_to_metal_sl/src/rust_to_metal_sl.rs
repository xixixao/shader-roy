mod adapter;
mod enhancer;
mod parser;
mod printer;

use anyhow::Result;

pub use enhancer::EnhanceConfig;

/// Returns Metal Shader Language source code.
pub fn transpile(rust_source: &str, config: &EnhanceConfig) -> Result<String> {
  let rust_ast = parser::parse_rust_into_ast(rust_source)?;
  let rust_ast_enhanced = enhancer::convert_constant_to_param(rust_ast, config);
  let msl_compatible_ast = adapter::make_rust_ast_msl_compatible(rust_ast_enhanced);
  printer::print_ast_into_msl(msl_compatible_ast)
}
