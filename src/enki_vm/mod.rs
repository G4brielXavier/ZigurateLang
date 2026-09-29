use std::collections::HashMap;

use crate::core::utils::primi::SignalType;

pub mod compiler;
pub mod enki;













#[derive(Debug, Clone)]
pub struct UDA {
    pub name: String,
    pub signal: String,
    pub value: UdaDeclarationType
}














#[derive(Debug, Clone)]
pub struct LUNIG {
    pub id: String,
    pub data: HashMap<String, UdaDeclarationType>
}

















#[derive(Debug, Clone, PartialEq)]
pub struct UDA_EDIT {
    pub name: String,
    pub signal: SignalType,
    pub new_value: UdaDeclarationType
}








#[derive(Debug, Clone, PartialEq)]
pub struct LUNIG_EDIT {
    pub name: String,
    pub properties: Vec<String>,
    pub new_value: UdaDeclarationType
}










#[derive(Debug, Clone)]
pub struct ANKI {
    pub enki_instr: Vec<EnkiInstruction>
}









#[derive(Debug, Clone, PartialEq)]
pub enum UdaDeclarationType {
    String(String),
    Integer(i32),
    Floating(f32),
    Boolean(bool)
}











#[derive(Debug, Clone)]
pub enum EnkiInstruction {

    INST_SILIM(String), // silim()
    INST_SILIM_PROP(Vec<String>), // silim(item.prop)

    INST_UDA_DECL(UDA), // uda name = value
    
    INST_CUT_UDA(Vec<String>),
    INST_GAZ_LUNIG(Vec<String>),

    INST_UDA_EDIT(UDA_EDIT),
    INST_LUNIG_EDIT(LUNIG_EDIT),


    INST_LUNIG_DECL(LUNIG), // lunig "Name" as name
    
    INST_ANKI(ANKI),



}