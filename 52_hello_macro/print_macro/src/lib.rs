use proc_macro::TokenStream;
use proc_macro::TokenTree;
use quote::quote;

#[proc_macro]
pub fn prt(input: TokenStream) -> TokenStream {
    impl_prt_macro(input)
}

fn impl_prt_macro(input: TokenStream) -> TokenStream {
    let mut token_iter = input.into_iter();
    let mut result: Vec<String> = vec![];
    while let Some(item) = token_iter.next() {
        result.push(match item {
            TokenTree::Ident(id) => format!("Ident {}", id.to_string()),
            TokenTree::Punct(punc) => format!("Punct {}", punc.to_string()),
            TokenTree::Group(group) => format!("Group {}", group.to_string()),
            TokenTree::Literal(literal) => format!("Literal {}", literal.to_string()),
        });
    }

    let gen = quote! {
        #(println!("{}", #result);)*
        println!("End of macro !");
    };
    println!("End of macro Definition !");
    gen.into()
}
