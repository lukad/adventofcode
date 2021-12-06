use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

#[derive(Debug, PartialEq, Eq, FromMeta)]
#[repr(u8)]
enum Part {
    One,
    Two,
}

#[derive(Debug, PartialEq, Eq, FromMeta)]
struct SolutionAttributes {
    pub year: i32,
    pub day: i32,
    pub part: Part,
}

#[proc_macro_attribute]
pub fn aoc(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let fun = parse_macro_input!(input as ItemFn);

    let SolutionAttributes { year, day, part } = match SolutionAttributes::from_list(&args) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    let part_one = (part as u8) == 0;
    let block = fun.block;
    let name = fun.sig.ident;

    let result = quote! {

        #[allow(non_camel_case_types)]
        pub struct #name;

        impl aoc::Solution for #name {
            fn year(&self) -> i32 {
                #year
            }

            fn day(&self) -> i32 {
                #day
            }

            fn part(&self) -> aoc::Part {
                if #part_one {
                    aoc::Part::One
                } else {
                    aoc::Part::Two
                }
            }

            fn solve(&self, input: &str) -> i32 {
                #block
            }
        }
    };

    result.into()
}
