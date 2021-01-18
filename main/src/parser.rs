use anyhow::{Context, Result};

#[test]
fn test() -> Result<()> {
  let shader_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/shader.rs");
  let shader = std::fs::read_to_string(shader_path).unwrap();
  println!("{}", transpile_rust_to_msl(&shader)?);
  Ok(())
}

pub fn transpile_rust_to_msl(rust_source: &str) -> Result<String> {
  print_ast_into_msl(make_rust_ast_msl_compatible(parse_rust_into_ast(
    rust_source,
  )?))
}

fn parse_rust_into_ast(rust_source: &str) -> Result<syn::File> {
  syn::parse_str(rust_source).with_context(|| "Parse error")
}

fn make_rust_ast_msl_compatible(mut rust_ast: syn::File) -> syn::File {
  use syn::visit_mut::VisitMut;
  AstMutator.visit_file_mut(&mut rust_ast);
  rust_ast
}

fn print_ast_into_msl(file: syn::File) -> Result<String> {
  let functions = file
    .items
    .iter()
    .map(|item| {
      Ok(match item {
        syn::Item::Use(_) => None,
        syn::Item::Fn(fun) => Some((
          match &fun.sig.output {
            syn::ReturnType::Default => "void".to_string(),
            syn::ReturnType::Type(_, x) => cp(x),
          },
          cp(&fun.sig.ident),
          fun
            .sig
            .inputs
            .iter()
            .map(|param| {
              Ok(match param {
                syn::FnArg::Typed(syn::PatType { ty, pat, .. }) => {
                  format!("{} {}", cp(ty), cp(pat))
                }
                _ => anyhow::bail!("Unsupported argument type"),
              })
            })
            .collect::<Result<Vec<_>>>()?
            .join(", "),
          {
            fun
              .block
              .stmts
              .iter()
              .map(|statement| {
                Ok(match statement {
                  syn::Stmt::Expr(x) => format!("return {};", cp(x)),
                  syn::Stmt::Local(x) => format!(
                    "{} {} {};",
                    match &x.pat {
                      syn::Pat::Type(syn::PatType { ty, .. }) => {
                        cp(ty)
                      }
                      _ => "auto".to_string(),
                    },
                    match match &x.pat {
                      syn::Pat::Type(syn::PatType { pat, .. }) => pat,
                      pat => pat,
                    } {
                      syn::Pat::Ident(syn::PatIdent { ident, .. }) => cp(ident),
                      _ => anyhow::bail!("Unsupported assignment pattern"),
                    },
                    match &x.init {
                      Some((_, expression)) => format!("= {}", cp(expression)),
                      None => "".to_string(),
                    },
                  ),
                  _ => cp(statement),
                })
              })
              .collect::<Result<Vec<_>>>()?
              .join("\n  ")
          },
        )),
        _ => None,
      })
    })
    .collect::<Result<Vec<_>>>()?
    .into_iter()
    .filter_map(|x| x)
    .collect::<Vec<_>>();
  let function_declarations = functions
    .iter()
    .map(|(ret_type, name, params, _)| format!("{} {} ({});", ret_type, name, params));
  let function_definitions = functions.iter().map(|(ret_type, name, params, body)| {
    format!("{} {} ({}) {{\n  {}\n}}", ret_type, name, params, body)
  });

  Ok(
    function_declarations
      .chain(function_definitions)
      .collect::<Vec<_>>()
      .join("\n\n"),
  )
}

#[allow(dead_code)]
fn pp<T>(x: T) -> String
where
  T: std::fmt::Debug,
{
  format!("{:#?}", x)
}

fn cp<T>(x: &T) -> String
where
  T: quote::ToTokens,
{
  quote::quote!(#x).to_string()
}

struct AstMutator;

impl syn::visit_mut::VisitMut for AstMutator {
  fn visit_expr_mut(&mut self, node: &mut syn::Expr) {
    if let syn::Expr::MethodCall(expr) = node {
      let syn::ExprMethodCall {
        receiver,
        method,
        args,
        ..
      } = expr;
      *node = syn::parse_quote!(
        #method(#receiver, #args)
      );
      return;
    }

    // Delegate to the default impl to visit nested expressions.
    syn::visit_mut::visit_expr_mut(self, node);
  }
  fn visit_type_mut(&mut self, node: &mut syn::Type) {
    if let syn::Type::Path(path) = node {
      if let Some(ident) = path.path.get_ident() {
        let mut ident_string = ident.to_string();
        if ident_string.starts_with("Float")
          || ident_string.starts_with("Int")
          || ident_string.starts_with("Uint")
        {
          {
            let mut_ident_string = &mut ident_string[0..1];
            mut_ident_string.make_ascii_lowercase();
          }
          let new_type = syn::Ident::new(&ident_string, proc_macro2::Span::call_site());
          *node = syn::parse_quote!(#new_type);
        }
      }
    }
    // Delegate to the default impl to visit nested expressions.
    syn::visit_mut::visit_type_mut(self, node);
  }
}
