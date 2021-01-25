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
            let num_stmts = fun.block.stmts.len();
            let is_last = |i: usize| i == num_stmts - 1;
            fun
              .block
              .stmts
              .iter()
              .enumerate()
              .map(|(i, statement)| print_statement(statement, is_last(i)))
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

fn print_statement(statement: &syn::Stmt, is_last_in_fn: bool) -> Result<String> {
  Ok(match statement {
    syn::Stmt::Expr(x) if is_last_in_fn => format!("return {};", cp(x)),
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
    syn::Stmt::Expr(syn::Expr::ForLoop(syn::ExprForLoop {
      pat, expr, body, ..
    })) => match &**expr {
      syn::Expr::Range(syn::ExprRange {
        from: Some(from),
        to: Some(to),
        limits,
        ..
      }) => format!(
        "for (auto {pat} = {from}; {pat} <{limit} {to}; {pat}++) {body}",
        pat = cp(pat),
        from = cp(&*from),
        to = cp(&*to),
        limit = if matches!(limits, syn::RangeLimits::Closed(_)) {
          "="
        } else {
          ""
        },
        body = print_block(body)?
      ),
      _ => anyhow::bail!("Unsupported for loop expression"),
    },
    _ => cp(statement),
  })
}

fn print_block(block: &syn::Block) -> Result<String> {
  let syn::Block { stmts, .. } = block;
  Ok(format!(
    "{{{}}}",
    stmts
      .iter()
      .map(|stmt| print_statement(stmt, false))
      .collect::<Result<Vec<_>>>()?
      .join("\n")
  ))
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
