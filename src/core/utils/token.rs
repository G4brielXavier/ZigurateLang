use crate::core::utils::primi::Primi;


#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {

    pub primi: Primi,
    pub value: &'a [u8],

    pub st_ln: u16,
    pub st_col: u8,

    pub end_ln: u16,
    pub end_col: u8

}


