#[macro_use]
extern crate proc_macro_hack;

proc_macro_expr_impl! {
    pub fn noprefix_hex_length_2_into_u8_impl(input: &str) -> String {
        format!("0x{}u8", input)
    }

    pub fn noprefix_hex_length_4_into_u16_impl(input: &str) -> String {
        format!("0x{}u16", input)
    }

    pub fn noprefix_hex_length_8_into_u32_impl(input: &str) -> String {
        format!("0x{}u32", input)
    }

    pub fn noprefix_hex_length_16_into_u64_impl(input: &str) -> String {
        format!("0x{}u64", input)
    }
}