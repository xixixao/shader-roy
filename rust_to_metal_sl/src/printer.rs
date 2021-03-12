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

  fn process_with_context<F>(&mut self, context: Context, processor: F)
  where
    F: FnOnce(&mut Self) -> Result<()>,
  {
    self.with_context(context, |_self| {
      _self.process(processor);
    });
  }

  fn maybe_process_with_context<F>(&mut self, maybe_context: Option<Context>, processor: F)
  where
    F: FnOnce(&mut Self) -> Result<()>,
  {
    match maybe_context {
      Some(context) => self.with_context(context, |_self| {
        _self.process(processor);
      }),
      None => self.process(processor),
    }
  }

  fn with_context<F>(&mut self, context: Context, processor: F)
  where
    F: FnOnce(&mut Self),
  {
    let outer_context = std::mem::replace(&mut self.context, context);
    processor(self);
    self.context = outer_context;
  }

  fn append<S>(&mut self, string: S)
  where
    S: AsRef<str>,
  {
    self.output.push_str(string.as_ref());
  }

  fn appendln<S>(&mut self, string: S)
  where
    S: AsRef<str>,
  {
    self.append(string);
    self.output.push('\n');
  }

  fn add<S>(&mut self, string: S)
  where
    S: AsRef<str>,
  {
    self.output.push_str(&self.indent);
    self.append(string.as_ref());
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

enum Context {
  TopLevel,
  ItemFn,
  LetBinding(String),
  LetBindingResult(String),
  NormalStmt,
  ReturnStmt,
}

impl syn::visit::Visit<'_> for AstPrinter {
  fn visit_item_struct(&mut self, strct: &syn::ItemStruct) {
    if !matches!(self.mode, PrinterMode::Declarations) {
      return;
    }
    self.addln(format!("struct {} {{", strct.ident.to_string()));
    self.indent(|_self| {
      if let syn::Fields::Named(syn::FieldsNamed { named, .. }) = &strct.fields {
        for field in named.iter() {
          _self.visit_type(&field.ty);
          _self.append(format!(" {}", field.ident.as_ref().unwrap().to_string()));
          _self.appendln(";");
        }
      }
    });
    self.addln("};\n");
  }

  fn visit_item_const(&mut self, _: &syn::ItemConst) {}

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
          syn::FnArg::Typed(syn::PatType { ty, pat, attrs, .. }) => match &**pat {
            syn::Pat::Ident(syn::PatIdent { ident, .. }) => {
              attrs.iter().for_each(|attr| {
                if cp(&attr.path) == "address_space" {
                  if let Ok(syn::Meta::List(syn::MetaList { nested, .. })) = attr.parse_meta() {
                    if let Some(syn::NestedMeta::Meta(syn::Meta::Path(name))) = nested.first() {
                      _self.add(format!("{} ", cp(name)));
                    }
                  }
                }
              });
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
          _self.append(";\n");
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
              Context::NormalStmt
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
    if matches!(statement, syn::Stmt::Semi(_, _)) {
      self.append(";\n");
    }
  }

  fn visit_local(&mut self, local: &syn::Local) {
    self.process(|_self| {
      let ty = match &local.pat {
        syn::Pat::Type(syn::PatType { ty, .. }) => cp(ty),
        syn::Pat::Struct(syn::PatStruct { path, .. }) => cp(path),
        _ => "auto".to_string(),
      };
      let var_name = match match &local.pat {
        syn::Pat::Type(syn::PatType { pat, .. }) => pat,
        pat => pat,
      } {
        syn::Pat::Ident(syn::PatIdent { ident, .. }) => cp(ident),
        syn::Pat::Struct(_) => _self.current_var(),
        _ => anyhow::bail!("Unsupported assignment pattern"),
      };
      _self.add(format!("{} {}", ty, var_name));
      match &local.init {
        Some((_, expression)) => _self.with_context(Context::LetBinding(var_name), |_self| {
          _self.visit_expr(expression);
        }),
        None => _self.appendln(";"),
      }
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
        _self.appendln(";");
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
        syn::Expr::While(syn::ExprWhile { cond, body, .. }) => {
          _self.addln(format!("while ({})", cp(cond)));
          _self.visit_block(body);
        }
        syn::Expr::If(if_expression) => {
          let binding_context = if let Context::LetBinding(var_name) = &_self.context {
            Some(Context::LetBindingResult(var_name.clone()))
          } else {
            None
          };
          let syn::ExprIf {
            cond,
            then_branch,
            else_branch,
            ..
          } = if_expression;
          let is_in_binding = binding_context.is_some();
          if is_in_binding && is_if_simple_ternary(if_expression) {
            _self.append(format!(" = ({}) ? ", cp(cond)));
            _self.with_context(Context::NormalStmt, |_self| {
              _self.visit_stmt(then_branch.stmts.first().unwrap());
              _self.append(" : ");
              let else_clause = &*else_branch.as_ref().unwrap().1;
              match else_clause {
                syn::Expr::If(_) => _self.visit_expr(else_clause),
                syn::Expr::Block(syn::ExprBlock { block, .. }) => {
                  _self.visit_stmt(block.stmts.first().unwrap())
                }
                // Never happens
                _ => {}
              }
              _self.appendln(";");
            })
          } else {
            if is_in_binding {
              // close the variable declaration
              _self.appendln(";");
            }
            _self.maybe_process_with_context(binding_context, |_self| {
              _self.addln(format!("if ({})", cp(cond)));
              _self.visit_block(then_branch);
              if let Some((_, else_branch)) = else_branch {
                _self.addln("else");
                _self.visit_expr(&**else_branch);
              }
              Ok(())
            });
          }
        }
        syn::Expr::Block(syn::ExprBlock { block, .. }) => _self.visit_block(block),
        syn::Expr::Return(_) => _self.add(cp(expression)),
        // We're gonna assume that any other expression is a bare expression in C++
        _ => match &_self.context {
          Context::ReturnStmt => _self.addln(format!("return {};", cp(expression))),
          Context::LetBinding(_) => {
            _self.appendln(format!(" = {};", cp(expression)));
          }
          Context::LetBindingResult(var_name) => {
            let var_name = var_name.clone();
            _self.addln(format!("{} = {};", var_name, cp(expression)));
          }
          _ => _self.add(cp(expression)),
        },
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

fn is_else_clause_simple_ternary_clause(
  else_branch: &Option<(syn::token::Else, Box<syn::Expr>)>,
) -> bool {
  if let Some((_, expression)) = else_branch {
    match &**expression {
      syn::Expr::If(if_expression) => is_if_simple_ternary(if_expression),
      syn::Expr::Block(syn::ExprBlock { block, .. }) => is_block_simple_ternary_clause(block),
      // This never happens
      _ => false,
    }
  } else {
    true
  }
}

fn is_block_simple_ternary_clause(block: &syn::Block) -> bool {
  let syn::Block { stmts, .. } = block;
  if stmts.len() != 1 {
    return false;
  }
  return is_stmt_simple_expr(stmts.first().unwrap());
}

fn is_stmt_simple_expr(statement: &syn::Stmt) -> bool {
  match statement {
    syn::Stmt::Expr(expression) => is_expr_simple_expr(expression),
    _ => false,
  }
}

fn is_expr_simple_expr(expression: &syn::Expr) -> bool {
  match expression {
    syn::Expr::ForLoop(_) => false,
    syn::Expr::While(_) => false,
    syn::Expr::Block(_) => false,
    syn::Expr::Return(_) => false,
    syn::Expr::If(if_expression) => is_if_simple_ternary(if_expression),
    // We're gonna assume that any other expression is a bare expression in C++
    _ => true,
  }
}

fn is_if_simple_ternary(expression: &syn::ExprIf) -> bool {
  let syn::ExprIf {
    then_branch,
    else_branch,
    ..
  } = expression;
  is_block_simple_ternary_clause(then_branch) && is_else_clause_simple_ternary_clause(else_branch)
}
