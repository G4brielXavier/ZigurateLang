
use std::{println, todo};

use crate::{core::{error::ZigurateError, utils::primi::SignalType}, enki_vm::{EnkiInstruction, UdaDeclarationType}};

use crate::enki_vm::compiler::extr_udatype_string;

use std::collections::HashMap;

pub type UdaData = HashMap<String, UdaDeclarationType>;
pub type LunigData = HashMap<String, HashMap<String, UdaDeclarationType>>;

pub struct EnkiVM {
    instructions: Vec<EnkiInstruction>,
    uda_hash: UdaData,
    lunig_hash: LunigData,
    ip: usize
}

impl EnkiVM {

    pub fn on(inst: Vec<EnkiInstruction>) -> Self {
        Self {
            instructions: inst,
            uda_hash: HashMap::new(), 
            lunig_hash: HashMap::new(),
            ip: 0
        }
    }
    
    pub fn run(&mut self) {

        while self.ip < self.instructions.len() {
            let instr = self.instructions[self.ip].clone();
            self.ip += 1;

            match instr {

                EnkiInstruction::INST_SILIM(val) => {
                    
                    let iden_signal = val.chars().next().unwrap();

                    if iden_signal == '!' {

                        let uda_key = val.as_str()[1..].to_string();

                        let uda_captured = self.uda_hash.get(&uda_key)
                            .unwrap_or_else(|| panic!("{}", ZigurateError::UdaNotFound(uda_key)));

                        let uda_value = extr_udatype_string(uda_captured);

                        println!("{}", uda_value);

                    } else {
                        println!("{}", val);
                    }

                },






                EnkiInstruction::INST_SILIM_PROP(val) => {

                    let k_start = val[0].to_string();
                    
                    if val.is_empty() {
                        let msg = format!("lunig called \"{}\" not found or not exists.", k_start);
                        panic!("{}", ZigurateError::LunigAlreadyExist(msg));
                    }

                    let rest = &val[1..];

                    // Verify if exist in Lunighash
                    let lunig_founded = self.lunig_hash.get(&k_start);

                    match lunig_founded {
                        Some(lunig)  => {

                            for (i, prop) in rest.iter().enumerate() {

                                if i >= rest.len() {
                                    break;
                                }

                                let data = lunig.get(prop);

                                match data {
                                    Some(v) => {
                                        
                                        match v {
                                            _ => {
                                                let val = extr_udatype_string(v);
                                                println!("{}", val);
                                                break
                                            }
                                        }

                                    },
                                    None => {
                                        let msg = format!("In lunig \"{}\", not found propertie \"{}\".", k_start, prop);
                                        panic!("{}", ZigurateError::LunigAlreadyExist(msg))
                                    }
                                }

                            }

                        },
                        None => {
                            let msg = format!("lunig called \"{}\" not found or not exists.", k_start);
                            panic!("{}", ZigurateError::LunigAlreadyExist(msg))
                        }
                    }

                },






                EnkiInstruction::INST_UDA_DECL(uda) => {

                    let uda_founded = self.uda_hash.get(&uda.name);

                    match uda_founded {
                        Some(_) => {
                            let msg = format!("An Uda called \"{}\" already exists.", uda.name);
                            panic!("{}", ZigurateError::LunigAlreadyExist(msg));
                        },
                        None => {
                            self.uda_hash.insert(uda.name, uda.value);
                        }
                    }

                }







                EnkiInstruction::INST_UDA_EDIT(edit_uda) => {

                    let uda_founded = self.uda_hash.get_mut(&edit_uda.name);

                    match uda_founded {
                        Some(uda) => {

                            match edit_uda.signal {

                                SignalType::NewValue => {
                                    *uda = edit_uda.new_value;
                                }

                                _ => {
                                    let msg = format!("Occurred an error to change \"{}\"", edit_uda.name);
                                    panic!("{}", ZigurateError::SyntaxError(msg));
                                }
                            }

                        },
                        None => {
                            let msg = format!("An uda called \"{}\" not exist or not found.", edit_uda.name);
                            panic!("{}", ZigurateError::UdaNotFound(msg));
                        }
                    }

                }






                EnkiInstruction::INST_LUNIG_EDIT(edit_lunig) => {

                    let lunig_founded = self.lunig_hash.get_mut(&edit_lunig.name);

                    match lunig_founded {

                        Some(lunig) => {

                            if edit_lunig.properties.len() == 2 {

                                let prop = &edit_lunig.properties[1];
                                let prop_found = lunig.get_mut(prop);


                                match prop_found {

                                    Some(value) => {

                                        *value = edit_lunig.new_value

                                    }
                                    None => {
                                        lunig.insert(prop.clone(), edit_lunig.new_value);
                                    }

                                }


                            
                            } else {
                                let msg = format!("A lunig can accept only one propertie. To use more, see about `tud` in documentation.");
                                panic!("{}", ZigurateError::LunigPropertieLimitExceeded(msg));
                            }

                        }

                        None => {
                            let msg = format!("A lunig called \"{}\" not exist or not found.", edit_lunig.name);
                            panic!("{}", ZigurateError::UdaNotFound(msg));
                        }

                    }

                }






                EnkiInstruction::INST_LUNIG_DECL(lunig) => {

                    if self.lunig_hash.contains_key(&lunig.id) {
                        let msg = format!("lunig called \"{}\" already exists.", lunig.id);
                        panic!("{}", ZigurateError::LunigAlreadyExist(msg))
                    }

                    self.lunig_hash.insert(lunig.id, lunig.data);

                }







                EnkiInstruction::INST_ANKI(anki) => {

                    let instruct = anki.enki_instr;
                    
                    if instruct.len() == 1 {

                        let next_ip = if self.ip > self.instructions.len() {
                            self.instructions.len()
                        } else {
                            self.ip as usize
                        };
                        
                        self.instructions.insert(next_ip, instruct[0].clone());
                        continue;

                    }
                    
                }






                EnkiInstruction::INST_CUT_UDA(data) => {

                    
                    if data.len() == 1 {
                        
                        let uda_comp_k = data[0].to_string();
                        let signal = uda_comp_k.chars().next().unwrap();

                        if signal == '!' {

                            let rest = uda_comp_k[1..].to_string();

                            self.uda_hash.remove(&rest);
                            continue;
                        
                        }

                    } else {

                        for uda_k in data.iter() {

                            let signal = uda_k.chars().next().unwrap();

                            if signal == '!' {

                                let rest = uda_k[1..].to_string();

                                self.uda_hash.remove(&rest);
                            
                            }

                        }

                    }

                }
            
            



                EnkiInstruction::INST_GAZ_LUNIG(data) => {

                    if data.len() == 1 {

                        let lunig_comp_k = data[0].to_string();
                        let signal = lunig_comp_k.chars().next().unwrap();

                        if signal == '!' {

                            let rest = lunig_comp_k[1..].to_string();

                            self.lunig_hash.remove(&rest);
                            continue;

                        }

                    }

                }

            }
        }

    }
}