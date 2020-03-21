extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;


#[proc_macro_attribute]
pub fn ship_component(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut found_struct = false;
    let mut struct_name: Option<String> = None;
    input.into_iter().map(|r| {
        match &r {
            &proc_macro::TokenTree::Ident(ref ident) if ident.to_string() == "struct" => {
                found_struct = true;
                r
            },
            &proc_macro::TokenTree::Ident(ref ident) if found_struct && struct_name == None => {
                struct_name = Some(ident.to_string());
                r
            },
            &proc_macro::TokenTree::Group(ref group) if group.delimiter() == proc_macro::Delimiter::Brace && found_struct == true => {
                let mut stream = proc_macro::TokenStream::new();
                let mut state: Vec::<proc_macro::TokenStream> = vec![
                    quote!(
                        pub armor: u32,
                        pub mass: u32,
                        pub slots: Option<u32>,
                    ).into()
                ];
                stream.extend(state);
                stream.extend(group.stream());
                state = match &struct_name {
                    Some(s) => vec![
                        quote!(
                            impl ShipComponent for #s {}
                        ).into()
                    ],
                    _ => panic!("Could not find struct name!")
                };

                proc_macro::TokenTree::Group(
                    proc_macro::Group::new(
                        proc_macro::Delimiter::Brace,
                        stream
                    )
                )
            },
            _ => r,
        }
    }).collect()
}