pub fn make_rust_ast_msl_compatible(mut rust_ast: syn::File) -> syn::File {
  use syn::visit_mut::VisitMut;
  AstAdapter.visit_file_mut(&mut rust_ast);
  rust_ast
}

const MSL_TYPES: [&str; 3] = ["Float", "Int", "Uint"];

lazy_static::lazy_static! {
  static ref LOWER_CASED_TYPES: regex::RegexSet = regex::RegexSet::new(
    MSL_TYPES
      .iter()
      .map(|type_name| format!(r"^{}\d?$", type_name)),
  )
  .unwrap();
}

lazy_static::lazy_static! {
  static ref CONSTRUCTOR_METHODS: regex::RegexSet = regex::RegexSet::new(
    MSL_TYPES
      .iter()
      .map(|type_name| voca_rs::case::lower_first(type_name))
      .map(|type_name| format!(r"^{}\d$", type_name)),
  )
  .unwrap();
}

lazy_static::lazy_static! {
  static ref ACCESS_METHODS: regex::Regex = regex::Regex::new(r"^[xywz]{1,4}$").unwrap();
}

lazy_static::lazy_static! {
  static ref RENAMED_METHODS: std::collections::HashMap<&'static str, &'static str> =
    [
      ("clamped", "clamp"),
      ("magnitude", "length"),
      ("magnitude_squared", "length_squared"),
      ("face_forward", "faceforward"),
      ("normalized", "normalize"),
      ("reflected", "reflect"),
      ("refracted", "refract"),
      ("min", "fmin"),
      ("max", "fmax"),
    ].iter().cloned().collect();
}

lazy_static::lazy_static! {
  static ref METHODS_WITH_RECEIVER_LAST: regex::RegexSet = regex::RegexSet::new(
    ["mix", "smoothstep", "step"]
      .iter()
      .map(|name| format!(r"^{}$", name)),
  )
  .unwrap();
}

struct AstAdapter;

impl syn::visit_mut::VisitMut for AstAdapter {
  fn visit_expr_mut(&mut self, node: &mut syn::Expr) {
    if let syn::Expr::MethodCall(expr) = node {
      let syn::ExprMethodCall {
        receiver,
        args,
        method,
        ..
      } = expr;

      RENAMED_METHODS.iter().for_each(|(old, new)| {
        if method == old {
          *method = quote::format_ident!("{}", new);
        }
      });

      let method_name = method.to_string();

      *node = if ACCESS_METHODS.is_match(&method_name) {
        syn::parse_quote!(
          #receiver.#method
        )
      } else if CONSTRUCTOR_METHODS.is_match(&method_name) {
        if let syn::Expr::Tuple(syn::ExprTuple {
          elems: unwrapped_args,
          ..
        }) = &**receiver
        {
          syn::parse_quote!(
            #method(#unwrapped_args)
          )
        } else {
          syn::parse_quote!(
            #method(#receiver)
          )
        }
      } else if METHODS_WITH_RECEIVER_LAST.is_match(&method_name) {
        syn::parse_quote!(
          #method(#args, #receiver)
        )
      } else {
        let new_args = std::iter::once(&**receiver)
          .chain(args.iter())
          .collect::<syn::punctuated::Punctuated<&syn::Expr, syn::Token![,]>>();
        syn::parse_quote!(
          #method(#new_args)
        )
      }
    }

    // Delegate to the default impl to visit nested expressions.
    syn::visit_mut::visit_expr_mut(self, node);
  }
  fn visit_type_mut(&mut self, node: &mut syn::Type) {
    if let syn::Type::Path(path) = node {
      if let Some(ident) = path.path.get_ident() {
        let type_name = ident.to_string();
        if LOWER_CASED_TYPES.is_match(&type_name) {
          let new_name = voca_rs::case::lower_first(&type_name);
          let new_type = syn::Ident::new(&new_name, proc_macro2::Span::call_site());
          *node = syn::parse_quote!(#new_type);
        }
      }
    }
    // Delegate to the default impl to visit nested expressions.
    syn::visit_mut::visit_type_mut(self, node);
  }

  // Removes trailing commas, since C++ doesn't allow them
  fn visit_expr_call_mut(&mut self, node: &mut syn::ExprCall) {
    let syn::ExprCall { args, .. } = node;
    if args.trailing_punct() {
      let last = args.pop().unwrap().into_value();
      args.push(last);
    }
    syn::visit_mut::visit_expr_call_mut(self, node);
  }
}
