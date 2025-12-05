extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

fn attr_name(a: &syn::Path) -> String {
    a.segments
        .iter()
        .map(|x| x.ident.to_string())
        .collect::<Vec<String>>()
        .join("::")
}

fn default_attr(f: &syn::Field) -> Option<proc_macro2::TokenStream> {
    for attr in &f.attrs {
        if attr_name(&attr.path()) == "default" {
            return Some(attr.meta.require_list().unwrap().tokens.clone());
        }
    }
    None
}

fn array_length_attr(f: &syn::Field) -> Option<proc_macro2::TokenStream> {
    for attr in &f.attrs {
        if attr_name(&attr.path()) == "array_length" {
            let args: proc_macro2::TokenStream = attr.parse_args().unwrap();
            return Some(args);
        }
    }
    None
}

enum Array {
    Nibble,
    OfType(proc_macro2::TokenStream),
}

fn array_type_attr(f: &syn::Field) -> Option<Array> {
    for attr in &f.attrs {
        if attr_name(&attr.path()) == "array_type" {
            let args: proc_macro2::TokenStream = attr.parse_args().unwrap();
            return if format!("{}", args) == "N1" {
                Some(Array::Nibble)
            } else {
                Some(Array::OfType(args))
            };
        }
    }
    None
}

fn record_type_attr(attrs: &[syn::Attribute]) -> Option<(u8, u8)> {
    for attr in attrs {
        if attr_name(&attr.path()) == "record_type" {
            if let Ok(args) = attr.parse_args::<syn::LitInt>() {
                if let Ok(val) = args.base10_parse::<u8>() {
                    // Single value means both typ and sub are the same
                    return Some((val, val));
                }
            }
            // Try parsing as tuple (typ, sub)
            if let Ok(meta_list) = attr.meta.require_list() {
                let tokens = meta_list.tokens.to_string();
                let parts: Vec<&str> = tokens.split(',').map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    if let (Ok(typ), Ok(sub)) = (parts[0].parse::<u8>(), parts[1].parse::<u8>()) {
                        return Some((typ, sub));
                    }
                }
            }
        }
    }
    None
}

#[proc_macro_derive(
    STDFRecord,
    attributes(default, array_length, nibble_array_length, array_type, record_type)
)]
pub fn stdf_record(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let name = derive_input.ident;
    let generics = derive_input.generics;
    let record_struct = if let syn::Data::Struct(ref record_struct) = derive_input.data {
        record_struct
    } else {
        return TokenStream::from(quote! {});
    };
    let try_read_vars = record_struct.fields.iter().map(|ref x| {
        let name = x.ident.as_ref().unwrap();
        let ty = &x.ty;
        let missing = match default_attr(x) {
            Some(ts) => ts,
            None => quote! { return Err(byte::Error::Incomplete) },
        };
        match (array_length_attr(x), array_type_attr(x).as_ref()) {
            (Some(ref index_name), Some(Array::Nibble)) => quote! {
                let mut #name: Vec<N1> = vec![];
                let blen = (#index_name.0 / 2) + (#index_name.0 % 2);
                for i in 0..blen.into() {
                    let v = match bytes.read_with::<u8>(offset, endian) {
                        Ok(v) => v,
                        Err(byte::Error::Incomplete) => break,
                        Err(byte::Error::BadOffset(_)) => break,
                        Err(e) => return Err(e),
                    };
                    #name.push(N1::from(v & 0x0f));
                    if #name.len() < #index_name.0.into() {
                        #name.push(N1::from((v & 0xf0) >> 4));
                    }
                }
            },
            (Some(ref index_name), Some(Array::OfType(ref index_type))) => quote! {
                let mut #name: Vec<#index_type> = vec![];
                for i in 0..#index_name.0.into() {
                    let v = match bytes.read_with::<#index_type>(offset, endian) {
                        Ok(v) => v,
                        Err(byte::Error::Incomplete) => break,
                        Err(byte::Error::BadOffset(_)) => break,
                        Err(e) => return Err(e),
                    };
                    #name.push(v);
                }
            },
            (_, _) => quote! {
                let #name = match bytes.read_with::<#ty>(offset, endian) {
                    Ok(v) => v,
                    Err(byte::Error::Incomplete) => #missing,
                    Err(byte::Error::BadOffset(_)) => #missing,
                    Err(e) => return Err(e),
                };
            },
        }
    });
    let try_read_fields = record_struct.fields.iter().map(|ref x| {
        let name = x.ident.as_ref().unwrap();
        quote! {
            #name: #name
        }
    });
    let try_write_fields = record_struct.fields.iter().map(|ref x| {
        let name = x.ident.as_ref().unwrap();
        let ty = &x.ty;
        let missing = match default_attr(x) {
            Some(_) => quote! { {} },
            None => quote! { return Err(byte::Error::Incomplete) },
        };
        match (array_length_attr(x), array_type_attr(x).as_ref()) {
            (_, Some(Array::OfType(ref index_type))) => quote! {
                for v in self.#name {
                    match bytes.write_with::<#index_type>(offset, v.clone(), endian) {
                        Ok(_) => {}
                        Err(byte::Error::Incomplete) => #missing,
                        Err(byte::Error::BadOffset(_)) => #missing,
                        Err(e) => return Err(e),
                    }
                }
            },
            (Some(ref index_name), Some(Array::Nibble)) => quote! {
                {
                    let mut i = 0;
                    while i < self.#index_name.0.into() {
                        let mut byteval = self.#name[i].0 & 0xf;
                        i+=1;
                        if i < self.#index_name.0.into() {
                            byteval |= (self.#name[i].0 << 4);
                            i+=1;
                        }
                        match bytes.write_with::<u8>(offset, byteval, endian) {
                            Ok(_) => {}
                            Err(byte::Error::Incomplete) => #missing,
                            Err(byte::Error::BadOffset(_)) => #missing,
                            Err(e) => return Err(e),
                        }
                    }
                }
            },
            (_, _) => quote! {
                match bytes.write_with::<#ty>(offset, self.#name, endian) {
                    Ok(_) => {},
                    Err(byte::Error::Incomplete) => #missing,
                    Err(byte::Error::BadOffset(_)) => #missing,
                    Err(e) => return Err(e),
                }
            },
        }
    });
    let default_impl_generics: syn::Generics = parse_quote! { <'a> };
    let (record_impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    // TryRead needs to declare an 'a lifetime for the byte slice. The record type we're
    // implementing for might already declare an 'a lifetime if it contains variable-length field
    // types, which should use the same lifetime as the read buffer. However if it doesn't, an 'a
    // lifetime needs to be declared.
    let (impl_generics, record_ty_lifetimes) = if generics.lifetimes().count() == 0 {
        let (ig, _, _) = default_impl_generics.split_for_impl();
        (ig, default_impl_generics.lifetimes())
    } else {
        (record_impl_generics, generics.lifetimes())
    };
    let try_read = quote! {
        impl #impl_generics TryRead<#(#record_ty_lifetimes,)* ctx::Endian> for #name #ty_generics #where_clause {
            fn try_read(bytes: &'a [u8], endian: ctx::Endian) -> byte::Result<(Self, usize)> {
                let offset = &mut 0;
                #(#try_read_vars)*
                Ok((
                    #name {
                        #(#try_read_fields),*
                    },
                    *offset,
                ))
            }
        }
    };
    let try_write = quote! {
        impl #impl_generics TryWrite<ctx::Endian> for #name #ty_generics #where_clause {
            fn try_write(self, bytes: &mut [u8], endian: ctx::Endian) -> byte::Result<usize> {
                let offset = &mut 0;
                #(#try_write_fields);*
                Ok(*offset)
            }
        }
    };
    
    let binary_fields = record_struct.fields.iter().map(|ref x| {
        let name = x.ident.as_ref().unwrap();
        match (array_length_attr(x), array_type_attr(x).as_ref()) {
            (_, Some(Array::OfType(_))) => quote! {
                for v in &self.#name {
                    data.extend_from_slice(&v.binary(endian));
                }
            },
            (Some(_), Some(Array::Nibble)) => quote! {
                {
                    let mut i = 0;
                    while i < self.#name.len() {
                        let mut byteval = self.#name[i].0 & 0xf;
                        i += 1;
                        if i < self.#name.len() {
                            byteval |= (self.#name[i].0 << 4);
                            i += 1;
                        }
                        data.push(byteval);
                    }
                }
            },
            (_, _) => quote! {
                data.extend_from_slice(&self.#name.binary(endian));
            },
        }
    });
    
    let record_type_values = record_type_attr(&derive_input.attrs);
    let binary_impl = if let Some((rec_typ, rec_sub)) = record_type_values {
        quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                pub fn binary(&self, endian: ctx::Endian) -> Vec<u8> {
                    let mut data = Vec::new();
                    #(#binary_fields)*
                    
                    // Calculate REC_LEN (length of data only, not including the 4-byte header)
                    let rec_len = data.len() as u16;
                    
                    // Build complete record with header
                    let mut bytes = Vec::with_capacity(4 + data.len());
                    bytes.extend_from_slice(&rec_len.to_le_bytes());  // REC_LEN is always little-endian in header
                    if endian == ctx::Endian::Big {
                        bytes[0] = (rec_len >> 8) as u8;
                        bytes[1] = (rec_len & 0xff) as u8;
                    }
                    bytes.push(#rec_typ);  // REC_TYP
                    bytes.push(#rec_sub);  // REC_SUB
                    bytes.extend_from_slice(&data);
                    bytes
                }
            }
        }
    } else {
        quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                pub fn binary(&self, endian: ctx::Endian) -> Vec<u8> {
                    let mut bytes = Vec::new();
                    #(#binary_fields)*
                    bytes
                }
            }
        }
    };
    
    // Generate ascii() method for ATDF format
    let ascii_fields = record_struct.fields.iter().map(|ref x| {
        let name = x.ident.as_ref().unwrap();
        let ty = &x.ty;
        
        // Check if this is an array field
        if array_length_attr(x).is_some() {
            // For arrays, we'll format them specially or skip for now
            quote! {
                // Arrays in ATDF are complex, skip for now
                result.push_str("");
            }
        } else {
            // For scalar fields, convert based on type
            let ty_str = quote!(#ty).to_string();
            if ty_str.contains("Cn") {
                quote! {
                    result.push_str(&String::from_utf8_lossy(self.#name.0));
                }
            } else if ty_str.contains("Bn") {
                quote! {
                    // Binary data, skip or encode
                    result.push_str("");
                }
            } else if ty_str.contains("U4T") {
                quote! {
                    // Timestamp formatting
                    if self.#name.0 == 0 {
                        result.push_str("");
                    } else {
                        result.push_str(&format!("{}", self.#name));
                    }
                }
            } else if ty_str.contains("B1") {
                quote! {
                    result.push_str(&format!("{:08b}", self.#name.0));
                }
            } else if ty_str.contains("C1") {
                quote! {
                    if self.#name.0.is_ascii_graphic() || self.#name.0 == b' ' {
                        result.push(self.#name.0 as char);
                    } else {
                        result.push_str(&format!("{}", self.#name.0));
                    }
                }
            } else if ty_str.contains("I2") {
                quote! {
                    if self.#name.0 == std::i16::MIN {
                        result.push_str("");
                    } else {
                        result.push_str(&format!("{}", self.#name.0));
                    }
                }
            } else if ty_str.contains("I4") {
                quote! {
                    if self.#name.0 == std::i32::MIN {
                        result.push_str("");
                    } else {
                        result.push_str(&format!("{}", self.#name.0));
                    }
                }
            } else if ty_str.contains("I8") {
                quote! {
                    if self.#name.0 == std::i64::MIN {
                        result.push_str("");
                    } else {
                        result.push_str(&format!("{}", self.#name.0));
                    }
                }
            } else if ty_str.contains("I1") {
                quote! {
                    if self.#name.0 == std::i8::MIN {
                        result.push_str("");
                    } else {
                        result.push_str(&format!("{}", self.#name.0));
                    }
                }
            } else if ty_str.contains("R4") || ty_str.contains("R8") {
                quote! {
                    if self.#name.0.is_nan() {
                        result.push_str("");
                    } else {
                        result.push_str(&format!("{}", self.#name.0));
                    }
                }
            } else {
                // Generic numeric type
                quote! {
                    result.push_str(&format!("{}", self.#name.0));
                }
            }
        }
    });
    
    let record_name = name.to_string();
    let ascii_impl = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn ascii(&self) -> String {
                let mut result = format!("{}:", #record_name);
                #(
                    #ascii_fields
                    result.push('|');
                )*
                result
            }
        }
    };
    
    TokenStream::from(quote! {
        #try_read
        #try_write
        #binary_impl
        #ascii_impl
    })
}
