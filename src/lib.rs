#[macro_use]
extern crate proc_macro_hack;

proc_macro_expr_impl! {
    pub fn noprefix_hex_u8(input: &str) -> String {
        format!("0x{}u8", input)
    }

    pub fn noprefix_hex_u16(input: &str) -> String {
        format!("0x{}u16", input)
    }

    pub fn noprefix_hex_u32(input: &str) -> String {
        format!("0x{}u32", input)
    }

    pub fn noprefix_hex_u64(input: &str) -> String {
        format!("0x{}u64", input)
    }
}