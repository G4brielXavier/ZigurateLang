use std::path::PathBuf;
use std::println;
use std::str::from_utf8;

use colored::Colorize;

use crate::cli::commands::Commands;

use crate::tools::sita::Sita;

use crate::core::error::ZigurateError;

use crate::core::engine::tokenizer::Tokenizer;
use crate::core::engine::ast::AST;

use crate::enki_vm::compiler::enki_compiler;
use crate::enki_vm::enki::{self, EnkiVM};

use crate::cli::{
    EnkiProject,

    get_enki_data
};




pub fn match_command(cmd: &Commands) -> Result<(), ZigurateError> {

    let sita = Sita::gibil();

    match cmd {

        Commands::Run { path } => {

            let curr_path = sita.current_dir()
                .map_err(|e| ZigurateError::IOError(e))?;

            let path_cont = match path {
                Some(e) => Ok(e.clone()),
                None => {

                    let is_enki_project = sita.path_exists(&curr_path.join("Enki.yaml"))
                        .map_err(|e| ZigurateError::IOError(e))?;

                    if is_enki_project {
                        
                        let enki_data = get_enki_data(&sita, &curr_path)?;
                        let path_world = PathBuf::from(enki_data.main);

                        Ok(path_world)

                    } else {
                        Err(ZigurateError::ProjectNotFoundOrNotExist)
                    }

                }
            };

            match path_cont {

                Ok(main_path) => {

                    let content = sita.read_file(main_path)
                        .map_err(|e| ZigurateError::IOError(e))?;

                    let byte_content = content.as_bytes();


                    let enki_data = get_enki_data(&sita, &curr_path)?;

                    let ur_info = format!("UR {}", enki_data.zigurate_ur);
                    let title = format!("{} {}", "/|\\ ZIGURATE".bold(), ur_info.bold().italic());
                    let cmd = format!("enki run");

                    let msg_2 = format!("");

                    println!();
                    println!("{}", title);
                    println!("--- {} {} st.{}", "info:".bold().yellow(), enki_data.name.italic(), enki_data.stage.italic());
                    println!("--- {} {} {}", "cmd:".bold().yellow(), cmd.yellow().italic(), "!executed".bold().green());
                    println!("--- {} {}", "in:".bold().yellow(), curr_path.display().to_string().underline());
                    println!();
                    println!("{} {}", "Running EnkiVM...".bright_blue().bold(), msg_2);

                    let mut tokenizer = Tokenizer::open(byte_content);
                    let tokens = tokenizer.ignite();

                    // for i in tokens.iter() {
                    //     println!("{:?}: {}", i.primi, from_utf8(i.value).unwrap());
                    // }

                    let mut ast = AST::open(tokens);
                    let ast = ast.create()?;

                    // println!("{:?}", ast);

                    let sig_enki_byte = enki_compiler(ast)?;

                    let mut enki_vm = EnkiVM::on(sig_enki_byte);

                    // println!("{} {}", "Compiled!".bright_green().bold(), msg_2);
                    println!();

                    enki_vm.run();

                    // println!("{:?}", enki_vm.instructions);

                },
                Err(err) => {
                    return Err(err)
                }

            }

            
        }
    
        Commands::Create { project_name } => {

            let curr_path = sita.current_dir()
                .map_err(|e| ZigurateError::IOError(e))?; 

            let project_already_exists = sita.path_exists(&curr_path.join(project_name))
                .map_err(|e| ZigurateError::IOError(e))?;

            if !project_already_exists {

                let fld_project = sita.create_dir(project_name, &curr_path)
                    .map_err(|e| ZigurateError::IOError(e))?;

                let src_project = sita.create_dir("world", &fld_project)
                    .map_err(|e| ZigurateError::IOError(e))?;

                let fil_main = sita.create_file("main.zi", src_project)
                    .map_err(|e| ZigurateError::IOError(e))?;

                let _ = sita.edit_file(fil_main, "silim(\"Hello World!\")".to_string());

                let fil_enki = sita.create_file("Enki.yaml", &fld_project)
                    .map_err(|e| ZigurateError::IOError(e))?;

                let enki_content = format!(
                    "zigurate_ur: 1\nname: \"{name}\"\ndesc: \"\"\nstage: \"1\"\nmain: \"world/main.zi\"\n\nrules: []",
                    name = project_name
                );

                let _ = sita.edit_file(fil_enki, enki_content);

                let title = format!("{}", "/|\\ ZIGURATE".bold());
                let cmd = format!("enki create {}", project_name.yellow().italic());

                let msg_2 = format!("Project `{}` was created.", project_name.yellow().italic());

                println!();
                println!("{}", title);
                println!("--- {} {} {}", "cmd:".bold().yellow(), cmd.yellow().italic(), "!executed".bold().green());
                println!("--- {} {}", "in:".bold().yellow(), curr_path.display().to_string().underline());
                println!();
                println!("{} {}", "success:".bright_green().bold(), msg_2);
                println!();

            } else {

                let title = format!("{}", "/|\\ ZIGURATE".bold());
                let cmd = format!("enki create {}", project_name);

                let msg_2 = format!("Already exists a project called `{}`", project_name.bold());
                
                println!();
                println!("{}", title);
                println!("--- {} {} {}", "cmd:".bold().yellow(), cmd.yellow().italic(), "!not executed".bold().red());
                println!("--- {} {}", "in:".bold().yellow(), curr_path.display().to_string().underline());
                println!();
                println!("{} {}", "warning:".bright_yellow().bold(), msg_2);
                println!();

            }

            return Ok(())
        }

        Commands::Export { json } => {
            return Ok(())
        }

    }

    Ok(())
}