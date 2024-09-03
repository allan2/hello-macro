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
    let output = &input.sig.output;

    let expanded = if sig.asyncness.is_some() {
        #[cfg(feature = "async-std")]
        {
            // async-std
            quote! {
                #vis fn main() #output {
                    println!("{}", #msg);

                    async fn main_async() #output {
                        #block
                    }

                    async_std::task::block_on(main_async())
                }
            }
        }

        // this feature does not actually depend on tokio, but we feature-gate it because async-std macro depends on async-std
        #[cfg(feature = "tokio")]
        {
            let fn_name = &input.sig.ident;
            let new_fn_name = format_ident!("{fn_name}_inner");

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
        }

        #[cfg(all(not(feature = "async-std"), not(feature = "tokio")))]
        quote! {
            compile_error!("No async runtime feature enabled")
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
