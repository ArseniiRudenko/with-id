use proc_macro::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, DeriveInput, Data, DataStruct, Field, parse_macro_input};

fn get_by_attr(data_struct: &DataStruct) -> Option<&Field> {
    data_struct.fields.iter().find(|field| {
        field.attrs.iter().any(|attr| attr.path().is_ident("id"))
    })
}


fn get_by_name(data_struct: &DataStruct) -> Option<&Field> {
    data_struct.fields.iter().find(|field| {
        field.ident.as_ref().map_or(false, |ident| ident == "id")
    })
}

fn get_id_field(ast: &DeriveInput)->Result<&Field,syn::Error>{

    let id_field =
        match ast.data {
            Data::Struct(ref data_struct) => get_by_attr(data_struct),
            _ => Err(syn::Error::new(ast.span(),
                "WithId can only be derived for structs",
            ))?
        };


    // If no #[id] attribute is present, try to find a field named "id"
    let id_field =match id_field {
        Some(field) => Some(field),
        None => {
            match ast.data {
                Data::Struct(ref data_struct) => get_by_name(data_struct),
                _ => Err(syn::Error::new(
                    ast.span(),
                    "WithId can only be derived for structs",
                ))?
            }
        }
    };
    // If no "id" field is present, return an error
    return match id_field {
        Some(field) => Ok(field),
        None => {
            Err( syn::Error::new(
                ast.span(),
                "struct must have a field marked with #[id] attribute or named 'id'",
            ))?
        }
    };
}


#[proc_macro_derive(WithStrId, attributes(id))]
pub fn with_str_id_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    // Try to find the field marked with the #[id] attribute
    let id_field =  match get_id_field(&ast) {
        Ok(field) => field,
        Err(err) => return err.to_compile_error().into()
    };

    let id_field_name = id_field.ident.as_ref().unwrap();
    let id_field_ty = &id_field.ty;

    // Generate the implementation for the trait
    let gen = if let syn::Type::Path(type_path) = id_field_ty {
        if let Some(segment) = type_path.path.segments.first() {
            if segment.ident == "String" {
                // If the id field is already a string, generate a simple implementation
                quote! {
                    impl<'a> WithStrId<'a> for #name {
                        fn id(&'a self) -> &'a str {
                            &self.#id_field_name.as_str()
                        }
                    }
                }
            } else if segment.ident == "str"{
                quote! {
                    impl<'a> WithStrId<'a> for #name {
                        fn id(&'a self) -> &'a str {
                            &self.#id_field_name
                        }
                    }
                }
            }else  {
                // If the id field is not a string, generate an implementation that converts it to a string
                return syn::Error::new(id_field_ty.span(), "unexpected error: id field has to be String or &str").to_compile_error().into();
            }
        } else {
            // Unexpected error: id field has an empty path
            return syn::Error::new(id_field_ty.span(), "unexpected error: id field has an empty path").to_compile_error().into();
        }
    } else {
        // Unexpected error: id field is not a path type
        return syn::Error::new(id_field_ty.span(), "unexpected error: id field is not a path type").to_compile_error().into();
    };

    // Return the generated implementation
    gen.into()
}


#[proc_macro_derive(WithStringId, attributes(id))]
pub fn with_string_id_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    // Try to find the field marked with the #[id] attribute
    let id_field =  match get_id_field(&ast) {
        Ok(field) => field,
        Err(err) => return err.to_compile_error().into()
    };

    let id_field_name = id_field.ident.as_ref().unwrap();

    // Generate the implementation for the trait
    let gen =
        quote! {
                    impl WithStringId for #name {
                        fn id(&self) -> String {
                            self.#id_field_name.to_string()
                        }
                    }
        };
    // Return the generated implementation
    gen.into()
}


#[proc_macro_derive(WithId, attributes(id))]
pub fn with_id_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    // Try to find the field marked with the #[id] attribute
    let id_field =  match get_id_field(&ast) {
        Ok(field) => field,
        Err(err) => return err.to_compile_error().into()
    };

    let id_field_name = id_field.ident.as_ref().unwrap();
    let id_field_ty = &id_field.ty;

    // Generate the implementation for the trait
    let gen =
        quote! {
                    impl WithId<#id_field_ty> for #name {
                        fn id(&self) -> #id_field_ty {
                            self.#id_field_name.clone()
                        }
                    }
        };
    // Return the generated implementation
    gen.into()
}


#[proc_macro_derive(WithRefId, attributes(id))]
pub fn with_ref_id_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    // Try to find the field marked with the #[id] attribute
    let id_field =  match get_id_field(&ast) {
        Ok(field) => field,
        Err(err) => return err.to_compile_error().into()
    };

    let id_field_name = id_field.ident.as_ref().unwrap();
    let id_field_ty = &id_field.ty;

    // Generate the implementation for the trait
    let gen =
        quote! {
                    impl WithRefId<#id_field_ty> for #name {
                        fn id(&self) -> &#id_field_ty {
                            &self.#id_field_name
                        }
                    }
        };
    // Return the generated implementation
    gen.into()
}
