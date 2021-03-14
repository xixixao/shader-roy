pub fn define_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (type_name, type_template) = type_name_and_template_input(input);
    let result_type_name = type_name_and_template_to_type(&type_name, &type_template);
    let trait_name = type_name_and_template_to_trait(&type_name, &type_template);
    let method_name = type_name_and_template_to_constructor_name(&type_name, &type_template);
    let ty: syn::Type = syn::parse_str(&result_type_name).unwrap();
    let result = quote::quote!(
        pub trait #trait_name {
            fn #method_name(self) -> #ty;
        }
    );
    // eprintln!("{}", result);
    proc_macro::TokenStream::from(result)
}

fn type_name_and_template_input(input: proc_macro::TokenStream) -> (String, String) {
    let mut it = input.into_iter();
    let name = it.next().unwrap().to_string();
    let template = it.next().unwrap().to_string();
    (name, template)
}

struct ImplementTraitInput {
    type_name: String,
    result_type_template: String,
    arg_list: syn::punctuated::Punctuated<Arg, syn::Token![,]>,
}

// syn::FnArg is needlessly complicated to work with
struct Arg {
    name: syn::Ident,
    ty: syn::Ident,
}

impl syn::parse::Parse for ImplementTraitInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let type_name: syn::Ident = input.parse()?;
        let result_type_template: syn::Ident = input.parse()?;
        let content;
        syn::parenthesized!(content in input);
        let arg_list = content.parse_terminated(Arg::parse)?;
        Ok(ImplementTraitInput {
            type_name: type_name.to_string(),
            result_type_template: result_type_template.to_string(),
            arg_list,
        })
    }
}

impl syn::parse::Parse for Arg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _: syn::Token![:] = input.parse()?;
        let ty = input.parse()?;
        Ok(Arg { name, ty })
    }
}

pub fn implement_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ImplementTraitInput {
        type_name,
        result_type_template,
        arg_list,
    } = syn::parse_macro_input!(input);
    let num_args = arg_list.len();
    let result_type_name = type_name_and_template_to_type(&type_name, &result_type_template);
    let trait_name = type_name_and_template_to_trait(&type_name, &result_type_template);
    let method_name = type_name_and_template_to_constructor_name(&type_name, &result_type_template);
    let result_arity = type_arity(&result_type_template);
    let arg_names: syn::punctuated::Punctuated<_, syn::Token![,]> = arg_list
        .iter()
        .map(|Arg { name, .. }| name.clone())
        .collect();
    let args_pattern = if num_args > 1 {
        quote::quote!((#arg_names))
    } else {
        quote::quote!(#arg_names)
    };
    let filled_arg_list: syn::punctuated::Punctuated<_, syn::Token![,]> = arg_list
        .iter()
        .map(|Arg { name, ty }| {
            let arg_type_template = ty.to_string();
            let concrete_type_name = type_name_and_template_to_type(&type_name, &arg_type_template);
            let arity = type_arity(&arg_type_template);
            let concrete_type: syn::Type = syn::parse_str(&concrete_type_name).unwrap();
            (name, concrete_type, arity)
        })
        .collect();
    let arg_types: syn::punctuated::Punctuated<_, syn::Token![,]> = filled_arg_list
        .iter()
        .map(|(_, ty, _)| ty.clone())
        .collect();
    let impl_type = if num_args > 1 {
        quote::quote!((#arg_types))
    } else {
        quote::quote!(#arg_types)
    };
    const FIELD_NAMES: [char; 4] = ['x', 'y', 'z', 'w'];
    let mut implementation_args =
        syn::punctuated::Punctuated::<syn::FieldValue, syn::Token![,]>::new();
    filled_arg_list.iter().for_each(|(name, _, arg_arity)| {
        let num_fields_from_this_arg = if num_args == 1 {
            result_arity
        } else {
            *arg_arity
        };
        for arg_field_name_char in FIELD_NAMES.iter().take(num_fields_from_this_arg) {
            let arg_field_name = quote::format_ident!("{}", arg_field_name_char);
            let field_name = quote::format_ident!("{}", FIELD_NAMES[implementation_args.len()]);
            implementation_args.push(if *arg_arity == 1 {
                syn::parse_quote!(#field_name: #name)
            } else {
                syn::parse_quote!(#field_name: #name.#arg_field_name)
            });
        }
    });
    let ty: syn::Type = syn::parse_str(&result_type_name).unwrap();
    let struct_name = quote::format_ident!("{}", result_type_template);
    let result = quote::quote!(
        impl #trait_name for #impl_type {
            fn #method_name(self) -> #ty {
                let #args_pattern = self;
                #struct_name {#implementation_args}
            }
        }
    );
    // eprintln!("{}", result);
    proc_macro::TokenStream::from(result)
}

fn type_name_and_template_to_type(name: &str, template: &str) -> String {
    let arity_suffix = type_arity_suffix(&template);
    if arity_suffix == "" {
        name.to_owned()
    } else {
        format!(
            "Vec{}{}",
            arity_suffix,
            if name == "f32" {
                "".to_owned()
            } else {
                format!("<{}>", name)
            }
        )
    }
}

fn type_name_and_template_to_trait(name: &str, template: &str) -> syn::Ident {
    quote::format_ident!("Construct{}{}", template, name)
}

fn type_name_and_template_to_constructor_name(name: &str, template: &str) -> syn::Ident {
    quote::format_ident!(
        "vec{}{}",
        type_arity_suffix(&template),
        if name == "f32" { "" } else { name }
    )
}

fn type_arity_suffix(type_name: &str) -> String {
    let arity = type_arity(type_name);
    if arity == 1 {
        "".to_owned()
    } else {
        arity.to_string()
    }
}

fn type_arity(type_name: &str) -> usize {
    type_name
        .chars()
        .last()
        .and_then(|ch| char::to_digit(ch, 10))
        .unwrap_or(1) as usize
}
