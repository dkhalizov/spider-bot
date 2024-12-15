use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

#[proc_macro_derive(BotCallback)]
pub fn bot_callback_derive(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    let callback_impl = impl_trait(&ast);
    let display_impl = impl_display(&ast);

    quote! {
        #callback_impl
        #display_impl
    }.into()
}

fn impl_trait(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    let variants = match &ast.data {
        Data::Enum(enum_data) => &enum_data.variants,
        _ => panic!("BotCallback can only be derived for enums"),
    };

    let match_arms = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;

        let handler_name = format!("handle_{}", to_snake_case(&variant_ident.to_string()));
        let handler_ident = syn::Ident::new(&handler_name, proc_macro2::Span::call_site());

        match &variant.fields {
            Fields::Unit => {
                quote! {
                    Self::#variant_ident => {
                        self.#handler_ident(&bot, query).await
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let field_names: Vec<_> = fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        syn::Ident::new(&format!("field{}", i), proc_macro2::Span::call_site())
                    })
                    .collect();

                quote! {
                    Self::#variant_ident(#(#field_names),*) => {
                        self.#handler_ident(&bot, query, #(#field_names),*).await
                    }
                }
            }
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields
                    .named
                    .iter()
                    .map(|f| f.ident.as_ref().unwrap())
                    .collect();

                quote! {
                    Self::#variant_ident { #(#field_names),* } => {
                        self.#handler_ident(&bot, query, #(#field_names),*).await
                    }
                }
            }
        }
    });

    let parse_arms = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let variant_str = to_snake_case(&variant_ident.to_string());

        match &variant.fields {
            Fields::Unit => {
                quote! {
                    #variant_str => Ok(Self::#variant_ident)
                }
            }
            Fields::Unnamed(fields) => {
                let field_count = fields.unnamed.len();
                let value_start_index = variant_str.split('_').count();
                let indices: Vec<syn::Index> = (0..field_count)
                    .map(|i| syn::Index::from(i + value_start_index))
                    .collect();

                quote! {
        s if s == #variant_str || s.starts_with(concat!(#variant_str, "_")) => {
            let parts: Vec<&str> = s.split('_').collect();
            let prefix_parts: Vec<&str> = #variant_str.split('_').collect();

            if parts.len() != #field_count + prefix_parts.len() {
                return Err(BotError::ValidationError(
                    format!("Expected {} parameters for {}, got {}",
                        #field_count, #variant_str, parts.len() - prefix_parts.len())
                ));
            }

            Ok(Self::#variant_ident(
                #(parts[#indices].parse()
                    .map_err(|_| BotError::ValidationError(
                        format!("Invalid parameter at position {}", #indices - prefix_parts.len())
                    ))?,)*
            ))
        }
    }
            }
            Fields::Named(fields) => {
                let field_count = fields.named.len();
                let field_names = fields.named.iter()
                    .map(|f| f.ident.as_ref().unwrap());

                quote! {
                    s if s.starts_with(concat!(#variant_str, "_")) => {
                        let parts: Vec<&str> = s.splitn(#field_count + 1, '_').collect();
                        if parts.len() != #field_count + 1 {
                            return Err(BotError::ValidationError(format!("Invalid {} format", #variant_str)));
                        }
                        Ok(Self::#variant_ident {
                            #(#field_names: parts[1].parse().map_err(|_| BotError::ValidationError("Invalid parameter".to_string()))?,)*
                        })
                    }
                }
            }
        }
    });
    quote! {
            #[async_trait]
            impl CallbackCommand for #name {
                async fn callback(&self, bot: Arc<TarantulaBot>, query: CallbackQuery) -> BotResult<()> {
                    match self {
                        #(#match_arms)*
                    }
                }
            }

            impl std::str::FromStr for #name {
        type Err = BotError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                #(#parse_arms,)*
                _ => {
                    Err(BotError::ValidationError(
                        format!(
                            "Invalid callback data '{}'",
                            s)
                    ))
                }
            }
        }
    }
        }
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_uppercase() {
            if !result.is_empty() && chars.peek().map_or(false, |next| next.is_lowercase()) {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

fn impl_display(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    let variants = match &ast.data {
        Data::Enum(enum_data) => &enum_data.variants,
        _ => panic!("BotCallback can only be derived for enums"),
    };

    let display_arms = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let variant_str = to_snake_case(&variant_ident.to_string());

        match &variant.fields {
            Fields::Unit => {
                quote! {
                    Self::#variant_ident => write!(f, #variant_str)
                }
            }
            Fields::Unnamed(fields) => {
                let field_count = fields.unnamed.len();
                let field_names: Vec<_> = (0..field_count)
                    .map(|i| syn::Ident::new(&format!("field{}", i), proc_macro2::Span::call_site()))
                    .collect();

                let format_str = format!("{}{}", variant_str, "_{}".repeat(field_count));

                quote! {
                    Self::#variant_ident(#(#field_names),*) => write!(f, #format_str, #(#field_names),*)
                }
            }
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields
                    .named
                    .iter()
                    .map(|f| f.ident.as_ref().unwrap())
                    .collect();

                let format_str = format!("{}{}", variant_str, "_{}".repeat(field_names.len()));

                quote! {
                    Self::#variant_ident { #(#field_names),* } => write!(f, #format_str, #(#field_names),*)
                }
            }
        }
    });

    quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_arms),*
                }
            }
        }
    }
}
