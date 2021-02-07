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
  unique_id: u64,
}

impl AstPrinter {
  fn print(file: &syn::File, mode: PrinterMode) -> Result<String> {
    let mut printer = AstPrinter {
      error: None,
      output: String::new(),
      mode,
      context: Context::TopLevel,
      indent: String::new(),
      unique_id: 0,
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

  fn current_var(&self) -> String {
    format!("__var__{}", self.unique_id)
  }

  fn done_with_var(&mut self) {
    self.unique_id += 1;
  }
}

#[derive(Copy, Clone)]
enum Context {
  TopLevel,
  ItemFn,
  Stmt,
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
      _self.add(format!("{} {} (", ret_type, name));

      let inputs = &fun.sig.inputs;
      let num_inputs = inputs.len();
      let is_last = |i: usize| i == num_inputs - 1;
      for (i, param) in inputs.iter().enumerate() {
        match param {
          syn::FnArg::Typed(syn::PatType { ty, pat, .. }) => match &**pat {
            syn::Pat::Ident(syn::PatIdent { ident, .. }) => {
              _self.visit_type(ty);
              _self.add(format!(" {}", ident));
              if !is_last(i) {
                _self.add(", ");
              }
            }
            _ => anyhow::bail!("Unsupported argument type"),
          },
          _ => anyhow::bail!("Unsupported argument type"),
        }
      }
      _self.add(")");
      match _self.mode {
        PrinterMode::Declarations => {
          _self.output.push_str(";\n");
        }
        PrinterMode::Definitions => _self.visit_block(&*fun.block),
      }
      _self.output.push('\n');
      Ok(())
    });
  }

  fn visit_type(&mut self, ty: &syn::Type) {
    match ty {
      syn::Type::Reference(syn::TypeReference { elem, .. }) => {
        self.visit_type(elem);
        self.add("&");
      }
      _ => self.add(cp(ty)),
    }
  }

  fn visit_block(&mut self, block: &syn::Block) {
    self.addln("{");
    self.indent(|_self| {
      if matches!(_self.context, Context::ItemFn | Context::ReturnStmt) {
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
          syn::Pat::Struct(syn::PatStruct { path, .. }) => {
            cp(path)
          }
          _ => "auto".to_string(),
        },
        match match &local.pat {
          syn::Pat::Type(syn::PatType { pat, .. }) => pat,
          pat => pat,
        } {
          syn::Pat::Ident(syn::PatIdent { ident, .. }) => cp(ident),
          syn::Pat::Struct(_) => _self.current_var(),
          _ => anyhow::bail!("Unsupported assignment pattern"),
        },
        match &local.init {
          Some((_, expression)) => format!("= {}", cp(expression)),
          None => "".to_string(),
        },
      ));
      if let syn::Pat::Struct(syn::PatStruct { fields, .. }) = &local.pat {
        let var_name = _self.current_var();
        for field in fields {
          if let syn::Member::Named(name) = &field.member {
            if let syn::Pat::Ident(syn::PatIdent { ident, .. }) = &*field.pat {
              _self.add(format!("; auto {} = {}.{}", ident, var_name, name));
            } else {
              anyhow::bail!("Unsupported struct member in pattern");
            }
          } else {
            anyhow::bail!("Unsupported struct member in pattern");
          }
        }
        _self.done_with_var();
      }
      Ok(())
    });
  }

  fn visit_expr(&mut self, expression: &syn::Expr) {
    self.process(|_self| {
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
        syn::Expr::Return(_) => _self.add(cp(expression)),
        _ => {
          if matches!(_self.context, Context::ReturnStmt) {
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
