extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(MacroExampleLevel2)]
pub fn impl_trait_example_level2(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    // Build the trait implementation
    impl_example_macro_level2(&ast)
}

fn impl_example_macro_level2(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl MacroExampleLevel2 for #name {
            fn hello_macro_level2() {
                println!("Hello, Macro level 2! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
