use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_macro_input;
use syn::ItemFn;
use syn::FnArg;
//use syn::ItemStruct;
//
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
    system_add(item, quote! { SystemUpdate }).into()
}

#[proc_macro_attribute]
pub fn start(_: TokenStream, item: TokenStream) -> TokenStream {
    system_add(item, quote! { SystemStart }).into()
}

#[proc_macro_attribute]
pub fn exit(_: TokenStream, item: TokenStream) -> TokenStream {
    system_add(item, quote! { SystemExit }).into()
}

fn system_add(item: TokenStream, system_type: TokenStream2) -> TokenStream {
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
            ::satademo::ecs::#system_type(|world: &mut ::satademo::ecs::World| {
                for (#(#vars),*) in world.query_mut::<(#(#types),*)>() {
                    #name(#(#vars),*);
                }
            })
        }
    }.into()
}

//#[proc_macro]
//pub fn function(input: TokenStream) -> TokenStream {
//    let count = parse_macro_input!(input as syn::LitInt)
//        .base10_parse::<usize>()
//        .unwrap();
//
//    let funcs = (0..count).map(|i| {
//        let name = quote::format_ident!("system{}", i);
//
//        quote! {
//            #[update]
//            fn #name(c: &mut Dada) { c.0 += 1 }
//        }
//    });
//
//    quote! {
//        #(#funcs)*
//    }
//        .into()
//}