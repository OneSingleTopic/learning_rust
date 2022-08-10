use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(NamedColor)]
pub fn named_color_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_named_color_derive(&ast)
}

fn impl_named_color_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut variant_names: Vec<syn::Ident> = vec![];

    if let syn::Data::Enum(enum_item) = &ast.data {
        for element in enum_item.variants.iter() {
            variant_names.push(element.ident.clone());
        }
    }
    let gen = quote! {
        impl NamedColor for #name {
            fn name(&self) -> String {
                match self{
                    #(#name::#variant_names => format!("{}", stringify!(#variant_names)),)*
                }
            }

        }
    };
    gen.into()
}
