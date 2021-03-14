pub fn make_rust_ast_msl_compatible(mut rust_ast: syn::File) -> syn::File {
  use syn::visit_mut::VisitMut;
  AstAdapter.visit_file_mut(&mut rust_ast);
  rust_ast
}

const DEFAULT_GENERIC_TYPE_ARG: &str = "f32";
const GENERIC_TYPE_PREFIX: &str = "Vec";
const GENERIC_METHOD_PREFIX: &str = "vec";

lazy_static::lazy_static! {
  static ref RUST_TO_METAL_TYPES: std::collections::HashMap<&'static str, &'static str> = maplit::hashmap![
    "i8" => "char",
    "u8" => "uchar",
    "i16" => "short",
    "u16" => "ushort",
    "i32" => "int",
    "u32" => "uint",
    "i32" => "long",
    "i32" => "ulong",
    "f16" => "half",
    "f32" => "float",
  ];

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
      } else if let Some(arity_and_type_suffix) = method_name.strip_prefix(GENERIC_METHOD_PREFIX) {
        let type_name = &arity_and_type_suffix[1..];
        let new_type_name = RUST_TO_METAL_TYPES
          .get::<str>(if type_name == "" {
            DEFAULT_GENERIC_TYPE_ARG
          } else {
            type_name
          })
          .unwrap();
        let new_method = quote::format_ident!("{}{}", new_type_name, arity_and_type_suffix[0..1]);
        if let syn::Expr::Tuple(syn::ExprTuple {
          elems: unwrapped_args,
          ..
        }) = &**receiver
        {
          syn::parse_quote!(
            #new_method(#unwrapped_args)
          )
        } else {
          syn::parse_quote!(
            #new_method(#receiver)
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

    // For now we will erase type casts, as properly compiling them
    // would require a full expression printer
    if let syn::Expr::Cast(syn::ExprCast { expr, .. }) = node {
      *node = syn::parse_quote!(#expr);
    }

    // Delegate to the default impl to visit nested expressions.
    syn::visit_mut::visit_expr_mut(self, node);
  }

  // Convert scalar/vector types
  // `scalar` or ```Vec`d`<`scalar`>```
  fn visit_path_mut(&mut self, node: &mut syn::Path) {
    let (type_name, type_args) = {
      let mut path_segments_iter = node.segments.iter();
      let ty = path_segments_iter.next().unwrap();
      let arg_list = if let syn::PathArguments::AngleBracketed(args) = &ty.arguments {
        Some(quote::quote!(#args).to_string())
      } else {
        None
      };
      (ty.ident.to_string(), arg_list)
    };
    if let Some(new_type_name) = RUST_TO_METAL_TYPES.get::<str>(&type_name) {
      let new_type = quote::format_ident!("{}", new_type_name);
      *node = syn::parse_quote!(#new_type);
    }
    if let Some(arity_suffix) = type_name.strip_prefix(GENERIC_TYPE_PREFIX) {
      if let Some(new_type_name) =
        RUST_TO_METAL_TYPES.get::<str>(&type_args.as_deref().unwrap_or(DEFAULT_GENERIC_TYPE_ARG))
      {
        let new_type = quote::format_ident!("{}{}", new_type_name, arity_suffix);
        *node = syn::parse_quote!(#new_type);
      }
    }
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
