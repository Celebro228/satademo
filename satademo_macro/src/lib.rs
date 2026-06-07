use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::ItemFn;
use syn::FnArg;
use syn::ItemStruct;

#[proc_macro_attribute]
pub fn component(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    quote! {
        #[derive(::satademo::ecs::CD)]
        #input
    }
    .into()
}

#[proc_macro_attribute]
pub fn update(_: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let name = &func.sig.ident;
    let types: Vec<_> = func.sig.inputs.iter().map(|arg: &FnArg| {
        let FnArg::Typed(arg_type) = arg else {
            panic!("expected typed argument");
        };
        &arg_type.ty
    }).collect();

    quote! {
        #func

        ::satademo::ecs::submit! {
            ::satademo::ecs::SystemAdd(|world: &::satademo::ecs::WD| {
                world.
                    system::<(#(#types),*)>()
                    .each(#name);
            })
        }
    }.into()
}