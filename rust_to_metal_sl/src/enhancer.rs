// TODO:
// Walk the call graph
//   - first preprocess the AST into a Map from name to definition
//     fns = Map<String, syn::FunDef>
//   - another mutable Map stores the result - whether a function needs the input param or not
//     is_using_input = mut Map<String, ?bool>
//   - then start from pixel_color, find all function calls within it,
//     then visit them all from the fns Map
//   - if a function was visited already, it would have written the result, so we can early return
//   - if a function uses input, set is_using_input Map to true (and still visit all calls)
//   - if none of the function calls uses, set is_using_input to False
//   - finally make another pass over the AST, mutating every fn that needs input by adding it as argument
//   - and every call to such a function by adding the input as last argument
//   - for simplicity can use very unusual name to avoid name clashing

use std::collections::{HashMap, HashSet};

pub struct EnhanceConfig {
  pub entry_point_fn_name: String,
  pub constant_name: String,
  pub param_type: String,
}

pub fn convert_constant_to_param(rust_ast: syn::File, properties: &EnhanceConfig) -> syn::File {
  let fn_items = FnItemsCollector::name_to_fn_items(&rust_ast);
  let is_using = {
    let mut is_using = HashMap::<String, bool>::new();
    check_is_using(
      &properties.entry_point_fn_name,
      &fn_items,
      &mut is_using,
      &mut HashSet::new(),
      &properties,
    );
    // Entry point always needs to include the parameter to maintain API, regardless of its use
    is_using.insert(properties.entry_point_fn_name.to_owned(), true);
    is_using
  };
  FnItemsEnhancer::add_params(rust_ast, is_using, properties)
}

struct FnItemsCollector {
  fn_items: HashMap<String, syn::ItemFn>,
}

impl FnItemsCollector {
  fn name_to_fn_items(rust_ast: &syn::File) -> HashMap<String, syn::ItemFn> {
    use syn::visit::Visit;
    let mut collector = Self {
      fn_items: HashMap::new(),
    };
    collector.visit_file(rust_ast);
    collector.fn_items
  }
}

impl syn::visit::Visit<'_> for FnItemsCollector {
  fn visit_item_fn(&mut self, node: &syn::ItemFn) {
    self
      .fn_items
      .insert(node.sig.ident.to_string(), node.clone());
  }
}

fn check_is_using(
  fn_name: &str,
  fn_items: &HashMap<String, syn::ItemFn>,
  is_using: &mut HashMap<String, bool>,
  is_being_processed: &mut HashSet<String>,
  properties: &EnhanceConfig,
) {
  if is_using.get(fn_name).is_some() {
    return;
  }
  if is_being_processed.contains(fn_name) {
    // Prevents infinite loops for mutually recursive functions
    return;
  }
  is_being_processed.insert(fn_name.to_owned());
  if let Some(item_fn) = fn_items.get(fn_name) {
    // It's important to not short-circuit in any of this code
    #[allow(clippy::unnecessary_fold)]
    let is_any_called_fn_using = CalledFnsCollector::called_fns(item_fn)
      .iter()
      .map(|called_fn| {
        check_is_using(
          called_fn,
          fn_items,
          is_using,
          is_being_processed,
          properties,
        );
        *is_using.get(called_fn).unwrap_or(&false)
      })
      .fold(false, |so_far, is_called_using| so_far || is_called_using);
    let is_using_directly =
      ConstantDetector::contains_contant_access(item_fn, &properties.constant_name);
    is_using.insert(
      fn_name.to_owned(),
      is_any_called_fn_using || is_using_directly,
    );
  }
}

struct ConstantDetector {
  constant_name: String,
  result: bool,
}

impl ConstantDetector {
  fn contains_contant_access(item_fn: &syn::ItemFn, constant_name: &str) -> bool {
    let mut detector = Self {
      constant_name: constant_name.to_owned(),
      result: false,
    };
    use syn::visit::Visit;
    detector.visit_item_fn(item_fn);
    detector.result
  }
}

impl syn::visit::Visit<'_> for ConstantDetector {
  fn visit_expr_field(&mut self, field_access: &syn::ExprField) {
    if let syn::Expr::Path(syn::ExprPath { path, .. }) = &*field_access.base {
      if cp(path) == self.constant_name {
        self.result = true;
        return;
      }
    }
    syn::visit::visit_expr_field(self, field_access);
  }
}

struct CalledFnsCollector {
  called_fns: Vec<String>,
}

impl CalledFnsCollector {
  fn called_fns(item_fn: &syn::ItemFn) -> Vec<String> {
    use syn::visit::Visit;
    let mut collector = Self { called_fns: vec![] };
    collector.visit_item_fn(item_fn);
    collector.called_fns
  }
}

impl syn::visit::Visit<'_> for CalledFnsCollector {
  fn visit_expr_call(&mut self, call: &syn::ExprCall) {
    if let syn::Expr::Path(syn::ExprPath { path, .. }) = &*call.func {
      self.called_fns.push(cp(path))
    }
  }
}

struct FnItemsEnhancer<'a> {
  is_using: HashMap<String, bool>,
  properties: &'a EnhanceConfig,
}

impl<'a> FnItemsEnhancer<'a> {
  fn add_params(
    mut rust_ast: syn::File,
    is_using: HashMap<String, bool>,
    properties: &'a EnhanceConfig,
  ) -> syn::File {
    use syn::visit_mut::VisitMut;
    let mut enhancer = Self {
      is_using,
      properties,
    };
    enhancer.visit_file_mut(&mut rust_ast);
    rust_ast
  }

  fn is_using(&self, fn_name: &str) -> bool {
    *self.is_using.get(fn_name).unwrap_or(&false)
  }
}

impl syn::visit_mut::VisitMut for FnItemsEnhancer<'_> {
  fn visit_item_fn_mut(&mut self, item_fn: &mut syn::ItemFn) {
    let fn_name = item_fn.sig.ident.to_string();
    if self.is_using(&fn_name) {
      let param_type = quote::format_ident!("{}", self.properties.param_type);
      let constant_name = quote::format_ident!("{}", self.properties.constant_name);
      item_fn.sig.inputs.push(syn::parse_quote!(
        #[address_space(constant)] #constant_name: &#param_type
      ));
    }
    syn::visit_mut::visit_item_fn_mut(self, item_fn);
  }

  fn visit_expr_call_mut(&mut self, call: &mut syn::ExprCall) {
    if let syn::Expr::Path(syn::ExprPath { path, .. }) = &*call.func {
      if self.is_using(&cp(path)) {
        let constant_name = quote::format_ident!("{}", self.properties.constant_name);
        call.args.push(syn::parse_quote!(#constant_name));
      }
    }
    syn::visit_mut::visit_expr_call_mut(self, call);
  }
}

fn cp<T>(x: &T) -> String
where
  T: quote::ToTokens,
{
  quote::quote!(#x).to_string()
}
