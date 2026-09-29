use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{core::error::ZigurateError, tools::sita::Sita};

pub mod args;
pub mod commands;
pub mod matches;



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnkiProject {
    pub zigurate_ur: u16,
    pub name: String,
    pub desc: String,
    pub stage: String,
    pub main: String,
    pub rules: Vec<String>
}





pub fn get_enki_data(sita: &Sita, curr_path: &PathBuf) -> Result<EnkiProject, ZigurateError> {

    let enki_yaml_fil = sita.get_file_by_name("Enki.yaml", &curr_path)
        .map_err(|e| ZigurateError::IOError(e))?;


    let project_content_yaml = sita.read_file(enki_yaml_fil)
        .map_err(|e| ZigurateError::IOError(e))?;

    
    let enki_project = serde_yaml::from_str::<EnkiProject>(&project_content_yaml)
        .map_err(|e| ZigurateError::SerdeError(e))?;


    Ok(enki_project)

}
