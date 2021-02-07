mod access;
mod construct;

#[proc_macro]
pub fn implement_accessors(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    access::implement(input)
}

#[proc_macro]
pub fn define_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    construct::define_trait(input)
}

#[proc_macro]
pub fn implement_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    construct::implement_trait(input)
}
