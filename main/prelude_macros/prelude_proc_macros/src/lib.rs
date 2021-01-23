// extern crate proc_macro;
use proc_macro::TokenStream;

struct DefineTraitInput {
    type_name: syn::Ident,
    num: syn::LitInt,
}

impl syn::parse::Parse for DefineTraitInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(DefineTraitInput {
            type_name: input.parse()?,
            num: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn define_trait(input: TokenStream) -> TokenStream {
    let DefineTraitInput { type_name, num } = syn::parse_macro_input!(input);
    let num_args: usize = num.base10_parse().unwrap();
    let name = quote::format_ident!("{}Construct{}", type_name, format!("{}", num));
    let mut type_string = format!("{}", type_name);
    {
        let mut_ident_string = &mut type_string[0..1];
        mut_ident_string.make_ascii_lowercase();
    }
    let method_name = quote::format_ident!("{}_{}", type_string, num_args);
    let arg_names = ["a", "b", "c"];

    let varargs: syn::punctuated::Punctuated<_, syn::Token![,]> = (0..num_args - 1)
        .map(|index| quote::format_ident!("T{}", arg_names[index]))
        .collect();

    let args: syn::punctuated::Punctuated<_, syn::Token![,]> = (0..num_args - 1)
        .map(|index| {
            let arg_name = quote::format_ident!("{}", arg_names[index]);
            let arg_type = quote::format_ident!("T{}", arg_names[index]);
            let arg: syn::FnArg = syn::parse_quote!(#arg_name: #arg_type);
            arg
        })
        .collect();
    let trait_name = if num_args > 1 {
        quote::quote!(#name<#varargs>)
    } else {
        quote::quote!(#name)
    };
    let result = quote::quote!(
        pub trait #trait_name {
            fn #method_name(self, #args) -> #type_name;
        }
    );
    // eprintln!(result);
    TokenStream::from(result)
}

struct ImplementTraitInput {
    result_type: syn::Ident,
    receiver_type: syn::Ident,
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
        let receiver_type = input.parse()?;
        let content;
        syn::parenthesized!(content in input);
        let arg_list = content.parse_terminated(Arg::parse)?;
        Ok(ImplementTraitInput {
            result_type,
            receiver_type,
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
        receiver_type,
        arg_list,
    } = syn::parse_macro_input!(input);
    let num_args = arg_list.len() + 1;
    let name = quote::format_ident!("{}Construct{}", result_type, format!("{}", num_args));
    let mut type_string = format!("{}", result_type);
    {
        let mut_ident_string = &mut type_string[0..1];
        mut_ident_string.make_ascii_lowercase();
    }
    let result_arity: usize = type_string[type_string.len() - 1..type_string.len()]
        .parse()
        .unwrap_or(1);
    let fun_name = quote::format_ident!("{}", type_string);
    let method_name = quote::format_ident!("{}_{}", type_string, num_args);
    let varargs: syn::punctuated::Punctuated<syn::Ident, syn::Token![,]> =
        arg_list.iter().map(|Arg { ty, .. }| ty.clone()).collect();
    let trait_name = if num_args > 1 {
        quote::quote!(#name<#varargs>)
    } else {
        quote::quote!(#name)
    };
    // let mut complete_arg_list = arg_list.clone();
    // complete_arg_list.insert(0, syn::parse_quote!(self: #receiver_type));
    let mut implementation_args = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::new();
    std::iter::once(&Arg {
        name: quote::format_ident!("self"),
        ty: receiver_type.clone(),
    })
    .chain(arg_list.iter())
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
    let quoted_arg_list: syn::punctuated::Punctuated<_, syn::Token![,]> = arg_list
        .iter()
        .map(|Arg { name, ty }| {
            let arg: syn::FnArg = syn::parse_quote!(#name: #ty);
            arg
        })
        .collect();
    let result = quote::quote!(
        impl #trait_name for #receiver_type {
            fn #method_name(self, #quoted_arg_list) -> #result_type {
                #fun_name(#implementation_args)
            }
        }
    );
    // eprintln!(result);
    TokenStream::from(result)
}
