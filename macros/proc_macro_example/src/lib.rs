extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

#[macro_use]
extern crate proc_macro_example_level2;

use proc_macro::TokenStream;

#[proc_macro_derive(MacroExample)]
pub fn impl_trait_example(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    // Build the trait implementation
    impl_example_macro(&ast)
}

fn impl_example_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl MacroExample for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(MacroExampleLevel1)]
pub fn impl_trait_example_level1(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    // Build the trait implementation
    impl_example_macro_level1(&ast)
}

fn impl_example_macro_level1(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl MacroExample for #name {
            fn hello_macro() {
                println!("Hello, Macro level 1! My name is {}", stringify!(#name));
                SampleLevel2::hello_macro_level2();
            }
        }
        #[derive(MacroExampleLevel2)]
        pub struct SampleLevel2;
        
        trait MacroExampleLevel2 {
            fn hello_macro_level2();
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
