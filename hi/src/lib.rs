use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn, LitStr};

#[proc_macro_attribute]
pub fn say(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let msg = parse_macro_input!(args as LitStr).value();

    let attrs = &input.attrs;
    let block = &input.block;
    let sig = &input.sig;
    let vis = &input.vis;
    let fn_name = &input.sig.ident;
    let output = &input.sig.output;
    let is_async = sig.asyncness.is_some();
    let new_fn_name = format_ident!("{fn_name}_inner");

    let expanded = if is_async {
        quote! {
            // non-async wrapper function
            #vis fn #fn_name() #output {
                println!("{}", #msg);
                #new_fn_name()
            }

            // orig async function, but renamed
            #(#attrs)*
            #vis async fn #new_fn_name() #output {
                #block
            }
        }
    } else {
        // not using async, just inject the `say` message at the start
        quote! {
            #(#attrs)*
            #vis #sig {
                println!("{}", #msg);
                #block
            }
        }
    };

    expanded.into()
}
