use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn plugin_config(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let tokens = parse_macro_input!(tokens);
    match tokens {
        syn::Item::Struct(item_struct) => add_config_schema(item_struct),
        _ => panic!("Only structs are currently supported"),
    }
}

#[proc_macro_attribute]
pub fn plugin_impl(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let tokens = parse_macro_input!(tokens);
    match tokens {
        syn::Item::Impl(item_impl) => handle_impl(item_impl),
        _ => panic!("Only structs are currently supported"),
    }
}

fn add_config_schema(item_struct: syn::ItemStruct) -> TokenStream {
    let ident = item_struct.ident.clone();

    // TODO build schema
    // println!("{:#?}", item_struct);
    let _methods = vec![String::from("access")];

    let ret = quote! {
        #[derive(Clone, serde::Deserialize, serde::Serialize)]
        #[serde(default)]
        #item_struct

        use kong_rust_pdk::PluginConfig;

        impl PluginConfig for #ident {
            fn get_phases() -> Vec<String> {
                vec![String::from("access")]
            }
        }
    };
    ret.into()
}

fn handle_impl(item_impl: syn::ItemImpl) -> TokenStream {
    // println!("attr {:#?}", attr);

    // TODO build methods
    // println!("{:#?}", item_impl);

    let ret = quote! {
        use kong_rust_pdk::async_trait;

        #[async_trait]
        #item_impl

        use kong_rust_pdk::PluginSchema;

        // TODO Struct name
        impl PluginSchema for Config {
            fn get_schema() -> String {
                String::new()
            }
        }
    };
    ret.into()
}

// [
//   {
//     "Name": "go-log",
//     "Phases": ["log"],
//     "Version": "0.3",
//     "Priority": 2,
//     "Schema": {
//       "fields": [
//         {
//           "config": {
//             "type": "record",
//             "fields": [
//               { "path": { "type": "string" } },
//               { "reopen": { "type": "boolean" } }
//             ]
//           }
//         }
//       ],
//       "name": "go-log"
//     }
//   }
// ]
