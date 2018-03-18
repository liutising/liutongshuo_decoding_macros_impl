#[macro_use]
extern crate proc_macro_hack;

//To u8
proc_macro_expr_impl! {
    pub fn noprefix_hex_length_1_into_u8_impl(input: &str) -> String {
        format!("0x{}u8", input)
    }

    pub fn noprefix_hex_length_2_into_u8_impl(input: &str) -> String {
        format!("0x{}u8", input)
    }

}

//To u16
proc_macro_expr_impl! {

    pub fn noprefix_hex_length_1_into_u16_impl(input: &str) -> String {
        format!("0x{}u16", input)
    }

    pub fn noprefix_hex_length_2_into_u16_impl(input: &str) -> String {
        format!("0x{}u16", input)
    }

    pub fn noprefix_hex_length_3_into_u16_impl(input: &str) -> String {
        format!("0x{}u16", input)
    }

    pub fn noprefix_hex_length_4_into_u16_impl(input: &str) -> String {
        format!("0x{}u16", input)
    }

}

//To u32
proc_macro_expr_impl! {

    pub fn noprefix_hex_length_1_into_u32_impl(input: &str) -> String {
        format!("0x{}u32", input)
    }

    pub fn noprefix_hex_length_2_into_u32_impl(input: &str) -> String {
        format!("0x{}u32", input)
    }

    pub fn noprefix_hex_length_3_into_u32_impl(input: &str) -> String {
        format!("0x{}u32", input)
    }

    pub fn noprefix_hex_length_4_into_u32_impl(input: &str) -> String {
        format!("0x{}u32", input)
    }

    pub fn noprefix_hex_length_5_into_u32_impl(input: &str) -> String {
        format!("0x{}u32", input)
    }

    pub fn noprefix_hex_length_6_into_u32_impl(input: &str) -> String {
        format!("0x{}u32", input)
    }

    pub fn noprefix_hex_length_7_into_u32_impl(input: &str) -> String {
        format!("0x{}u32", input)
    }

    pub fn noprefix_hex_length_8_into_u32_impl(input: &str) -> String {
        format!("0x{}u32", input)
    }
}

//To u64
proc_macro_expr_impl! {
    pub fn noprefix_hex_length_1_into_u64_impl(input: &str) -> String {
        format!("0x{}u64", input)
    }

    pub fn noprefix_hex_length_2_into_u64_impl(input: &str) -> String {
        format!("0x{}u64", input)
    }

    pub fn noprefix_hex_length_3_into_u64_impl(input: &str) -> String {
        format!("0x{}u64", input)
    }

    pub fn noprefix_hex_length_4_into_u64_impl(input: &str) -> String {
        format!("0x{}u64", input)
    }

    pub fn noprefix_hex_length_5_into_u64_impl(input: &str) -> String {
        format!("0x{}u64", input)
    }

    pub fn noprefix_hex_length_6_into_u64_impl(input: &str) -> String {
        format!("0x{}u64", input)
    }

    pub fn noprefix_hex_length_7_into_u64_impl(input: &str) -> String {
        format!("0x{}u64", input)
    }

    pub fn noprefix_hex_length_8_into_u64_impl(input: &str) -> String {
        format!("0x{}u64", input)
    }

    pub fn noprefix_hex_length_9_into_u64_impl(input: &str) -> String {
        format!("0x{}u64", input)
    }

    pub fn noprefix_hex_length_10_into_u64_impl(input: &str) -> String {
        format!("0x{}u64", input)
    }

    pub fn noprefix_hex_length_11_into_u64_impl(input: &str) -> String {
        format!("0x{}u64", input)
    }

    pub fn noprefix_hex_length_12_into_u64_impl(input: &str) -> String {
        format!("0x{}u64", input)
    }

    pub fn noprefix_hex_length_13_into_u64_impl(input: &str) -> String {
        format!("0x{}u64", input)
    }

    pub fn noprefix_hex_length_14_into_u64_impl(input: &str) -> String {
        format!("0x{}u64", input)
    }

    pub fn noprefix_hex_length_15_into_u64_impl(input: &str) -> String {
        format!("0x{}u64", input)
    }

    pub fn noprefix_hex_length_16_into_u64_impl(input: &str) -> String {
        format!("0x{}u64", input)
    }
}

//Length 4
proc_macro_expr_impl!{
    // xxxx -> (u8, u8)
    pub fn noprefix_hex_length_4_into_tuple_u8_u8_impl(input: &str) -> String {
        format!("(0x{}u8, 0x{}u8)", &input[0..2], &input[2..4])
    }

}

//Length 12
proc_macro_expr_impl!{
    // xxxxxxxxxxxx -> [u8;6]
    pub fn noprefix_hex_length_12_into_array_u8_6_impl(input: &str) -> String {
        format!("[0x{}u8, 0x{}u8, 0x{}u8, 0x{}u8, 0x{}u8, 0x{}u8]", 
            &input[0..2], &input[2..4], &input[4..6], &input[6..8], &input[8..10], &input[10..12])
    }
}
