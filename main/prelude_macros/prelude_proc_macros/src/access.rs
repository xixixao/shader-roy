pub fn implement(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let max_dimensions = 4;
  let vec_type = syn::parse_macro_input!(input as syn::Ident);
  let vec_type_name = vec_type.to_string();
  let scalar_type_name = &vec_type_name[0..vec_type_name.len() - 1];
  let scalar_fn_name = voca_rs::case::lower_first(scalar_type_name);
  let dimension = vec_type_name[vec_type_name.len() - 1..]
    .parse::<usize>()
    .unwrap();
  let trait_name = quote::format_ident!("{}{}", "AccessFrom", dimension);

  let axes = "xyzw";

  let trait_method_declarations: Vec<syn::TraitItem> = (dimension..=max_dimensions)
    .flat_map(|dim| {
      unique_accessor_names(dim, &axes[0..dimension])
        .into_iter()
        .map(move |name| {
          let ret_type = quote::format_ident!("{}{}", scalar_type_name, dim);
          let fn_name = quote::format_ident!("{}", name);
          syn::parse_quote!(fn #fn_name(self) -> #ret_type;)
        })
    })
    .collect();

  let trait_method_definitions: Vec<syn::TraitItem> = (dimension..=max_dimensions)
    .flat_map(|dim| {
      unique_accessor_names(dim, &axes[0..dimension])
        .into_iter()
        .map(|name| {
          let ret_type = quote::format_ident!("{}{}", scalar_type_name, dim);
          let fn_name = quote::format_ident!("{}", name);
          let constuctor = quote::format_ident!("{}{}", scalar_fn_name, dim);
          let args: syn::punctuated::Punctuated<_, syn::Token![,]> = name
            .chars()
            .map(|ch| {
              let field = quote::format_ident!("{}", ch);
              let access: syn::Expr = syn::parse_quote!(self.#field);
              access
            })
            .collect();
          syn::parse_quote!(fn #fn_name(self) -> #ret_type {(#args).#constuctor()})
        })
        .collect::<Vec<_>>()
    })
    .collect();

  let result = quote::quote!(
      pub trait #trait_name {
        #(#trait_method_declarations)*
      }

      impl #trait_name for #vec_type {
        #(#trait_method_definitions)*
      }
  );
  eprintln!("{}", result);
  proc_macro::TokenStream::from(result)
}

fn unique_accessor_names(dims: usize, axes: &str) -> Vec<String> {
  let mut result = accessor_names(dims, axes);
  result.sort();
  result.dedup();
  result
}

fn accessor_names(dims: usize, axes: &str) -> Vec<String> {
  if axes.is_empty() || dims == 0 {
    return vec!["".to_string()];
  }

  if axes.len() > dims {
    return (0..axes.len())
      .flat_map(|i| accessor_names(dims, &format!("{}{}", &axes[0..i], &axes[i + 1..])))
      .collect();
  }

  accessor_names(dims - 1, axes)
    .into_iter()
    .flat_map(|suffix| axes.chars().map(move |ch| format!("{}{}", ch, suffix)))
    .collect()
}
