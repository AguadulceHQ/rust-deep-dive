// proc_macro comes with Rust so we don't need to add it to the dependencies
// it's the compiler API that allows to read/manipulate Rust from our code
use proc_macro::TokenStream;

use quote::quote;
use syn;

// called whenever a user specifies [derive(HelloDeriveMacro)] on a type
// it's possible because we have the annotation and specified the name HelloDeriveMacro that matches the trait name
#[proc_macro_derive(MacroDerive)]
// converts input from TokenStream to a data structure that we can interpret and peform ops on
// the returned TokenStream is added to the code of our users when they compile their crate and so they'll get the extra funcionality
pub fn hello_derive_macro(input: TokenStream) -> TokenStream {
    // construct a representation of Rust code as a syntax tree that we can manipulate
    let ast = syn::parse(input).unwrap(); // we want this to panic if syn::parse fails, in general we need to be more thorough with panic!/expect error messages

    // Build the trait implementation
    impl_hello_derive_macro(&ast)
}

// build the trait implementation responsible for transforming the syntax tree
// generally this is the part that changes
fn impl_hello_derive_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident; // this field is output of syn
                           // quote! macro allows us to set Rust code we want to return
                           // we call into() to convert that output into TokenStream
    let gen = quote! {
        impl MacroDerive for #name { // we want the impelmentation for the type the user annotated
            fn hello_derive_macro() {
                // stringify macro is built into Rust and takes an expression e.g. 1+2 and at compile time turns it into a string literale e.g. "1+2" without evaluating the expression
                println!("Hello from a derive macro. My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}
