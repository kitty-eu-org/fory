// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use super::util::{is_default_value_variant, is_skip_enum_variant};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DataEnum;

pub fn gen_actual_type_id() -> TokenStream {
    quote! {
       fory_core::serializer::enum_::actual_type_id(type_id, register_by_name, compatible)
    }
}

pub fn gen_field_fields_info(_data_enum: &DataEnum) -> TokenStream {
    quote! {
        Ok(Vec::new())
    }
}

pub fn gen_reserved_space() -> TokenStream {
    quote! {
       4
    }
}

pub fn gen_write(_data_enum: &DataEnum) -> TokenStream {
    quote! {
        fory_core::serializer::enum_::write::<Self>(self, context, write_ref_info, write_type_info)
    }
}

pub fn gen_write_data(data_enum: &DataEnum) -> TokenStream {
    let default_variant_value = data_enum
        .variants
        .iter()
        .position(|v| is_default_value_variant(v))
        .unwrap_or(0) as u32;
    let mut variant_idents = Vec::with_capacity(data_enum.variants.len());
    let mut variant_values = Vec::with_capacity(data_enum.variants.len());
    for (variant_value, variant) in data_enum.variants.iter().enumerate() {
        if is_skip_enum_variant(variant) {
            variant_values.push(default_variant_value);
        } else {
            variant_values.push(variant_value as u32);
        }
        variant_idents.push(&variant.ident);
    }

    quote! {
        Ok(match self {
            #(
                Self::#variant_idents => {
                    context.writer.write_varuint32(#variant_values);
                }
            )*
            _ => {
            }
        })
    }
}
pub fn gen_write_type_info() -> TokenStream {
    quote! {
        fory_core::serializer::enum_::write_type_info::<Self>(context)
    }
}

pub fn gen_read(_: &DataEnum) -> TokenStream {
    quote! {
        fory_core::serializer::enum_::read::<Self>(context, read_ref_info, read_type_info)
    }
}

pub fn gen_read_with_type_info(_: &DataEnum) -> TokenStream {
    quote! {
        fory_core::serializer::enum_::read::<Self>(context, read_ref_info, false)
    }
}

pub fn gen_read_data(data_enum: &DataEnum) -> TokenStream {
    let mut variant_values = Vec::with_capacity(data_enum.variants.len());
    let mut variant_idents = Vec::with_capacity(data_enum.variants.len());
    let default_variant_value = data_enum
        .variants
        .iter()
        .position(|v| is_default_value_variant(v))
        .unwrap_or(0) as u32;
    for (variant_value, variant) in data_enum.variants.iter().enumerate() {
        if !is_skip_enum_variant(variant) {
            variant_values.push(variant_value as u32);
        } else {
            variant_values.push(default_variant_value);
        }
        variant_idents.push(&variant.ident);
    }
    quote! {
        let ordinal = context.reader.read_varuint32()?;
        match ordinal {
           #(
               #variant_values => Ok(Self::#variant_idents),
           )*
           _ => return Err(fory_core::error::Error::unknown_enum("unknown enum value")),
        }
    }
}

pub fn gen_read_type_info() -> TokenStream {
    quote! {
        fory_core::serializer::enum_::read_type_info::<Self>(context)
    }
}
