use proc_macro::TokenStream;
use quote::format_ident;
use syn::{Data, DeriveInput, Expr, ExprLit, Ident, Lit, Token, Variant, punctuated::Punctuated};

fn impl_compiler_error_trait(ast: DeriveInput) -> TokenStream {
    let ident = ast.ident;
    let variants: Vec<Variant> = match ast.data {
        Data::Enum(data) => data.variants.into_iter().filter_map(Some).collect(),
        _ => panic!("CompilerError can only be derived for enums"),
    };

    let idents: Vec<Ident> = variants
        .iter()
        .map(|variant| variant.ident.clone())
        .collect();
    let fields: Vec<Vec<Ident>> = variants
        .iter()
        .map(|variant| {
            let fields = &variant.fields;

            fields
                .members()
                .map(|member| match member {
                    syn::Member::Named(ident) => ident,
                    syn::Member::Unnamed(_) => {
                        panic!("`#[msg]` interpolations only accept named enum field members")
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let (fmt, args): (Vec<String>, Vec<Vec<Expr>>) = variants
        .iter()
        .flat_map(|variant| {
            variant
                .attrs
                .iter()
                .filter(|attr| attr.path().is_ident("msg"))
                .map(|attr| {
                    let args = attr
                        .parse_args_with(Punctuated::<Expr, Token![,]>::parse_terminated)
                        .unwrap()
                        .into_pairs()
                        .map(|pair| pair.value().clone())
                        .collect::<Vec<_>>();

                    if let (
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(str), ..
                        }),
                        args,
                    ) = (&args[0], &args[1..])
                    {
                        let args = args.to_vec();

                        (str.value(), args)
                    } else {
                        panic!("`#[msg]` attribute must contain a valid format literal")
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let codes: Vec<u32> = variants
        .iter()
        .flat_map(|variant| {
            variant
                .attrs
                .iter()
                .filter(|attr| attr.path().is_ident("code"))
                .map(|attr| {
                    let arg = attr.parse_args::<syn::LitInt>().unwrap();
                    let code: u32 = arg.base10_parse().unwrap();
                    code
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let severities: Vec<Ident> = variants
        .iter()
        .map(|variant| {
            if let Some(attr) = variant
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("severity"))
            {
                attr.parse_args::<syn::Ident>().unwrap()
            } else {
                format_ident!("Error")
            }
        })
        .collect();

    quote::quote! {
        impl CompilerError for #ident {
            fn msg(&self) -> String {

                match self {
                    #(#ident::#idents { #(#fields),* } => format!(#fmt #(#args),*)),*
                }
            }

            fn code(&self) -> String {
                match self {
                    #(#ident::#idents { .. } => format!("E{:04}", #codes)),*
                }
            }

            fn severity(&self) -> ::codespan_reporting::diagnostic::Severity {
                match self {
                    #(#ident::#idents { .. } => ::codespan_reporting::diagnostic::Severity::#severities),*
                }
            }
        }
    }
    .into()
}

#[proc_macro_derive(CompilerError, attributes(msg, code, severity))]
pub fn compiler_error_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    impl_compiler_error_trait(ast)
}
