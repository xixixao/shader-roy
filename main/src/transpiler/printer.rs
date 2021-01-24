use anyhow::Result;

pub fn print_ast_into_msl(file: syn::File) -> Result<String> {
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
                syn::FnArg::Typed(syn::PatType { ty, pat, .. }) => match &**pat {
                  syn::Pat::Ident(syn::PatIdent { ident, .. }) => {
                    format!("{} {}", cp(ty), ident)
                  }
                  _ => anyhow::bail!("Unsupported argument type"),
                },
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
