// extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn define_trait(input: TokenStream) -> TokenStream {
    let type_name = syn::parse_macro_input!(input as syn::Ident);
    let trait_name = quote::format_ident!("{}Construct", type_name);
    let mut type_string = format!("{}", type_name);
    {
        let mut_ident_string = &mut type_string[0..1];
        mut_ident_string.make_ascii_lowercase();
    }
    let method_name = quote::format_ident!("{}", type_string);
    let result = quote::quote!(
        pub trait #trait_name {
            fn #method_name(self) -> #type_name;
        }
    );
    // eprintln!("{}", result);
    TokenStream::from(result)
}

struct ImplementTraitInput {
    result_type: syn::Ident,
    arg_list: syn::punctuated::Punctuated<Arg, syn::Token![,]>,
}

// syn::FnArg is needlessly complicated to work with
struct Arg {
    name: syn::Ident,
    ty: syn::Ident,
}

impl syn::parse::Parse for ImplementTraitInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let result_type = input.parse()?;
        let content;
        syn::parenthesized!(content in input);
        let arg_list = content.parse_terminated(Arg::parse)?;
        Ok(ImplementTraitInput {
            result_type,
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

#[proc_macro]
pub fn implement_trait(input: TokenStream) -> TokenStream {
    let ImplementTraitInput {
        result_type,
        arg_list,
    } = syn::parse_macro_input!(input);
    let num_args = arg_list.len();
    let trait_name = quote::format_ident!("{}Construct", result_type);
    let mut type_string = format!("{}", result_type);
    {
        let mut_ident_string = &mut type_string[0..1];
        mut_ident_string.make_ascii_lowercase();
    }
    let result_arity: usize = type_string[type_string.len() - 1..type_string.len()]
        .parse()
        .unwrap_or(1);
    let fun_name = quote::format_ident!("{}", type_string);
    let method_name = quote::format_ident!("{}", type_string);
    let arg_names: syn::punctuated::Punctuated<_, syn::Token![,]> = arg_list
        .iter()
        .map(|Arg { name, .. }| name.clone())
        .collect();
    let args_pattern = if num_args > 1 {
        quote::quote!((#arg_names))
    } else {
        quote::quote!(#arg_names)
    };
    let arg_types: syn::punctuated::Punctuated<_, syn::Token![,]> =
        arg_list.iter().map(|Arg { ty, .. }| ty.clone()).collect();
    let impl_type = if num_args > 1 {
        quote::quote!((#arg_types))
    } else {
        quote::quote!(#arg_types)
    };
    let mut implementation_args = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::new();
    arg_list
        .iter()
        .for_each(|Arg { name, ty }| match ty.to_string().as_str() {
            "Float" => {
                implementation_args.push(syn::parse_quote!(#name));
                if num_args == 1 {
                    for _ in 0..result_arity - num_args {
                        implementation_args.push(syn::parse_quote!(#name));
                    }
                }
            }
            "Float2" => {
                implementation_args.push(syn::parse_quote!(#name.x));
                implementation_args.push(syn::parse_quote!(#name.y));
            }
            "Float3" => {
                implementation_args.push(syn::parse_quote!(#name.x));
                implementation_args.push(syn::parse_quote!(#name.y));
                implementation_args.push(syn::parse_quote!(#name.z));
            }
            "Float4" => {
                implementation_args.push(syn::parse_quote!(#name.x));
                implementation_args.push(syn::parse_quote!(#name.y));
                implementation_args.push(syn::parse_quote!(#name.z));
                implementation_args.push(syn::parse_quote!(#name.w));
            }
            _ => {}
        });
    let result = quote::quote!(
        impl #trait_name for #impl_type {
            fn #method_name(self) -> #result_type {
                let #args_pattern = self;
                #fun_name(#implementation_args)
            }
        }
    );
    // eprintln!("{}", result);
    TokenStream::from(result)
}
