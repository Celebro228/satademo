use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::ItemFn;
use syn::FnArg;
use syn::ItemStruct;

//#[proc_macro_attribute]
//pub fn component(_: TokenStream, item: TokenStream) -> TokenStream {
//    let input = parse_macro_input!(item as ItemStruct);
//    let name = &input.ident;
//
//    quote! {
//        #input
//        impl ::satademo::ecs::Component for #name {
//            fn create_sparse(
//                &self,
//                h: &mut Vec<::satademo::ecs::Mutex<Box<dyn ::satademo::ecs::Sparse>>>
//            ) {
//                h.push(::satademo::ecs::Mutex::new(Box::new(
//                    ::satademo::ecs::SparseSet::<#name>::new()
//                )));
//            }
//        }
//        //::satademo::ecs::submit!(&'static #name);;
//    }
        //    .into()
        //}

#[proc_macro_attribute]
pub fn update(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let types: Vec<_> = input.sig.inputs.iter().map(|arg: &FnArg| {
        let FnArg::Typed(arg_type) = arg else {
            panic!("expected typed argument");
        };
        &arg_type.ty
    }).collect();
    let vars: Vec<_> = input.sig.inputs.iter().map(|arg: &FnArg| {
        let FnArg::Typed(arg_type) = arg else {
            panic!("expected typed argument");
        };
        &arg_type.pat
    }).collect();

    quote! {
        #input

        ::satademo::ecs::submit! {
            ::satademo::ecs::SystemUpdate(|world: &mut ::satademo::ecs::World| {
                for (#(#vars),*) in world.query_mut::<(#(#types),*)>() {
                    #name(#(#vars),*);
                }
            })
        }
    }.into()
}