#![recursion_limit = "128"]
#![feature(proc_macro_diagnostic)]
#![feature(const_vec_new)]

extern crate proc_macro;
use self::proc_macro::TokenStream;

use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{braced, parse_macro_input, token, Expr, Field, Ident, Token, Type, Visibility};

struct StructTokens {
  table: Expr,
  visibility: Visibility,
  _struct_token: Token![struct],
  name: Ident,
  _brace_token: token::Brace,
  fields: Punctuated<Field, Token![,]>,
}

impl Parse for StructTokens {
  fn parse(input: ParseStream) -> Result<Self> {
    let content;
    Ok(StructTokens {
      table: input.parse()?,
      visibility: input.parse()?,
      _struct_token: input.parse()?,
      name: input.parse()?,
      _brace_token: braced!(content in input),
      fields: content.parse_terminated(Field::parse_named)?,
    })
  }
}

#[proc_macro]
pub fn requestable_object(input: TokenStream) -> TokenStream {
  let StructTokens {
    table,
    visibility,
    _struct_token,
    name,
    _brace_token,
    fields,
  } = parse_macro_input!(input as StructTokens);


  let mut fields_name: Vec<&Ident> = Vec::new();
  let mut fields_type: Vec<&Type> = Vec::new();
  for field in &fields {
    fields_name.push(field.ident.as_ref().unwrap());
    fields_type.push(&field.ty);
  }
  let fname_borrow = &fields_name;
  let fname_borrow2 = &fields_name;

  let expanded = quote! {
    #[derive(Default, Debug)]
    #visibility struct #name {
      #fields
    }

    juniper::graphql_object!(#name: Context |&self| {
      #(
        field #fname_borrow() -> &#fields_type {
            &self.#fname_borrow2
        }
      )*
    });

    #[allow(unused_assignments)]
    impl RequestableObject for #name {
      fn field_names() -> &'static [&'static str] {
          static NAMES: &'static [&'static str] = &[#( stringify!(#fname_borrow) ),*];
          NAMES
      }
      fn table() -> &'static str {
        #table
      }

      fn row(row: &mut Row) -> Box<Self> {
        let mut obj_row = #name::default();
        let mut index = 0;
        #(
          obj_row.#fname_borrow = row.get(index);
          index += 1;
        )*
        Box::new(obj_row)
      }
    }
  };

  TokenStream::from(expanded)
}

#[proc_macro]
pub fn objects_connection(input: TokenStream) -> TokenStream {
  let base_ident: Ident = syn::parse(input).unwrap();
  let connection = format!("Connection{}", base_ident);
  let connection_ident = syn::Ident::new(&connection, base_ident.span());

  let expanded = quote! {
    pub type #connection_ident = Connection<#base_ident>;

    juniper::graphql_object!(#connection_ident: Context |&self| {
      field nodes() -> &Vec<#base_ident> {
          &self.nodes
      }
    });
  };

  TokenStream::from(expanded)
}

// #[proc_macro]
// pub fn proceduralMacro(input: TokenStream) -> TokenStream {
//   let table: Vec<&str> = vec!["test"];

//   let expanded = quote! {
//       fn test1() -> &'static str {
//         #( #table )*
//         // #( #&table )* -> outputs nothing
//       }

//       fn test2() -> &'static str {
//         #( #table )*
//       }
//   };

//   TokenStream::from(expanded)
// }

// macro_rules! requestable_objects {
//     ($table:ident struct $name:ident { $($fname:ident : $ftype:ty),* } ) => {
//         #[derive(Default, Debug)]
//         pub struct $name {
//             $($fname : $ftype),*
//         }

//         juniper::graphql_object!($name: Context |&self| {
//           $(
//             field $fname() -> &$ftype {
//                 &self.$fname
//             }
//           )*
//         });

//         #[allow(unused_assignments)]
//         impl RequestableObject for $name {
//             fn field_names() -> &'static [&'static str] {
//                 static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
//                 NAMES
//             }
//             fn table() -> ETables {
//               ETables::$table
//             }

//             fn row(row: &mut Row) -> Box<Self> {
//               let mut obj_row = $name::default();
//               let mut index = 0;
//               $(
//                 obj_row.$fname = row.get(index);
//                 index += 1;
//               )*
//               Box::new(obj_row)
//             }
//         }
//     }
// }


// macro_rules! objects_connection {
//   ($conname:ident, $name:ident) => {
//     pub type $conname = Connection<$name>;

//     juniper::graphql_object!($conname: Context |&self| {
//       field nodes() -> &Vec<$name> {
//           &self.nodes
//       }
//     });
//   }
// }