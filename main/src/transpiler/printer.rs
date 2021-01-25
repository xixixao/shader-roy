use anyhow::Result;

pub fn print_ast_into_msl(file: syn::File) -> Result<String> {
  Ok(format!(
    "{}{}",
    AstPrinter::print(&file, PrinterMode::Declarations)?,
    AstPrinter::print(&file, PrinterMode::Definitions)?
  ))
}

enum PrinterMode {
  Declarations,
  Definitions,
}

struct AstPrinter {
  error: Option<anyhow::Error>,
  output: String,
  mode: PrinterMode,
  context: Context,
  indent: String,
}

impl AstPrinter {
  fn print(file: &syn::File, mode: PrinterMode) -> Result<String> {
    let mut printer = AstPrinter {
      error: None,
      output: String::new(),
      mode,
      context: Context::TopLevel,
      indent: String::new(),
    };
    use syn::visit::Visit;
    printer.visit_file(file);
    if let Some(err) = printer.error {
      return Err(err);
    }
    Ok(printer.output)
  }

  fn process<F>(&mut self, processor: F)
  where
    F: FnOnce(&mut Self) -> Result<()>,
  {
    if self.error.is_some() {
      return;
    }

    if let Err(err) = processor(self) {
      self.error = Some(err);
    }
  }

  fn process_with_context<F>(&mut self, node: Context, processor: F)
  where
    F: FnOnce(&mut Self) -> Result<()>,
  {
    self.with_context(node, |_self| {
      _self.process(processor);
    });
  }

  fn with_context<F>(&mut self, node: Context, processor: F)
  where
    F: FnOnce(&mut Self),
  {
    let outer_context = self.context;
    self.context = node;
    processor(self);
    self.context = outer_context;
  }

  fn add<S>(&mut self, string: S)
  where
    S: AsRef<str>,
  {
    self.output.push_str(&self.indent);
    self.output.push_str(string.as_ref());
  }

  fn addln<S>(&mut self, string: S)
  where
    S: AsRef<str>,
  {
    self.add(string);
    self.output.push('\n');
  }

  fn indent<F>(&mut self, processor: F)
  where
    F: FnOnce(&mut Self),
  {
    self.indent.push_str("  ");
    processor(self);
    self.indent.pop();
    self.indent.pop();
  }
}

#[derive(Copy, Clone)]
enum Context {
  TopLevel,
  ItemFn,
  Stmt,
  Expr,
  ReturnStmt,
}

impl syn::visit::Visit<'_> for AstPrinter {
  fn visit_item_fn(&mut self, fun: &syn::ItemFn) {
    self.process_with_context(Context::ItemFn, |_self| {
      let ret_type = match &fun.sig.output {
        syn::ReturnType::Default => "void".to_string(),
        syn::ReturnType::Type(_, x) => cp(x),
      };
      let name = cp(&fun.sig.ident);
      let params = fun
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
        .join(", ");
      _self.add(format!("{} {} ({})", ret_type, name, params));
      match _self.mode {
        PrinterMode::Declarations => {
          _self.output.push_str(";\n");
        }
        PrinterMode::Definitions => syn::visit::visit_item_fn(_self, fun),
      }
      _self.output.push('\n');
      Ok(())
    });
  }

  fn visit_block(&mut self, block: &syn::Block) {
    self.addln("{");
    self.indent(|_self| {
      if matches!(_self.context, Context::ItemFn) {
        let num_stmts = block.stmts.len();
        let is_last = |i: usize| i == num_stmts - 1;
        block.stmts.iter().enumerate().for_each(|(i, statement)| {
          _self.with_context(
            if is_last(i) {
              Context::ReturnStmt
            } else {
              Context::Stmt
            },
            |_self| _self.visit_stmt(statement),
          )
        });
      } else {
        syn::visit::visit_block(_self, block);
      }
    });
    self.addln("}");
  }

  fn visit_stmt(&mut self, statement: &syn::Stmt) {
    syn::visit::visit_stmt(self, statement);
    if !matches!(statement, syn::Stmt::Expr(_)) {
      self.output.push_str(";\n");
    }
  }

  fn visit_local(&mut self, local: &syn::Local) {
    self.process(|_self| {
      _self.add(format!(
        "{} {} {}",
        match &local.pat {
          syn::Pat::Type(syn::PatType { ty, .. }) => {
            cp(ty)
          }
          _ => "auto".to_string(),
        },
        match match &local.pat {
          syn::Pat::Type(syn::PatType { pat, .. }) => pat,
          pat => pat,
        } {
          syn::Pat::Ident(syn::PatIdent { ident, .. }) => cp(ident),
          _ => anyhow::bail!("Unsupported assignment pattern"),
        },
        match &local.init {
          Some((_, expression)) => format!("= {}", cp(expression)),
          None => "".to_string(),
        },
      ));
      Ok(())
    });
  }

  fn visit_expr(&mut self, expression: &syn::Expr) {
    let context = self.context;
    self.process_with_context(Context::Expr, |_self| {
      match expression {
        syn::Expr::ForLoop(syn::ExprForLoop {
          pat, expr, body, ..
        }) => match &**expr {
          syn::Expr::Range(syn::ExprRange {
            from: Some(from),
            to: Some(to),
            limits,
            ..
          }) => {
            _self.addln(format!(
              "for (auto {pat} = {from}; {pat} <{limit} {to}; {pat}++)",
              pat = cp(pat),
              from = cp(&*from),
              to = cp(&*to),
              limit = if matches!(limits, syn::RangeLimits::Closed(_)) {
                "="
              } else {
                ""
              },
            ));
            _self.visit_block(body);
          }
          _ => anyhow::bail!("Unsupported for loop expression"),
        },
        syn::Expr::If(syn::ExprIf {
          cond,
          then_branch,
          else_branch,
          ..
        }) => {
          _self.addln(format!("if ({})", cp(cond)));
          _self.visit_block(then_branch);
          if let Some((_, else_branch)) = else_branch {
            _self.addln("else");
            _self.visit_expr(&**else_branch);
          }
        }
        syn::Expr::Block(syn::ExprBlock { block, .. }) => _self.visit_block(block),
        _ => {
          if matches!(context, Context::ReturnStmt) {
            _self.addln(format!("return {};", cp(expression)));
          } else {
            _self.add(cp(expression))
          }
        }
      }
      Ok(())
    });
  }
}

fn cp<T>(x: &T) -> String
where
  T: quote::ToTokens,
{
  quote::quote!(#x).to_string()
}
