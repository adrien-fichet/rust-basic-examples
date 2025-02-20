use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn log_duration(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn); // parse the input as a syn function
    let ItemFn { attrs, vis, sig, block } = input; // destructure the parsed input
    let stmts = block.stmts; // extract the statements from the block
    let function_identifier = sig.ident.clone(); // store the function identifier

    // Construct the new function
    quote!(
        #(#attrs)* // reuse the original function attributes
        #vis #sig { // reuse the original function visibility + signature
            let __start = std::time::Instant::now();
            let __result = { #(#stmts)* }; // reuse the original function body and store its result
            println!("The function `{}` took {} µs", stringify!(#function_identifier), __start.elapsed().as_micros());
            __result
        }
    )
    .into()
}
