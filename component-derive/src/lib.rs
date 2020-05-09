extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

fn impl_component_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        #[typetag::serde]
        impl Component for #name {
            fn as_any(&self) -> &(dyn Any + 'static) { self }
            fn as_any_mut(&mut self) -> &mut (dyn Any + 'static) { self }
            fn ctype(&self) -> ComponentType { self.common.ctype }
            fn name(&self) -> &str { &self.common.name }
            fn mass(&self) -> u32 { self.common.mass }
            fn slots(&self) -> u8 { self.common.slots}
        }
    };
    gen.into()
}

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_component_derive(&ast)
}
