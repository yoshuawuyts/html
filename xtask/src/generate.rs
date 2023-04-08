use crate::Result;
use crate::{lookup_file, lookup_nodes};
use html_bindgen::generate::Module;
use html_bindgen::parse::{Attribute, ParsedElement};
use std::{env::current_dir, fs};

pub fn generate_sys() -> Result<()> {
    eprintln!("task: generate sys");
    let parsed = lookup_nodes::<ParsedElement>(crate::PARSED_ELEMENTS_PATH)?;
    let manual = lookup_file::<Vec<Attribute>>(crate::MANUAL_PATH, "global_attributes")?;
    let modules = lookup_file::<Vec<Module>>(crate::MANUAL_PATH, "web_sys_modules")?;
    let nodes = html_bindgen::generate::sys::generate(parsed, &manual, &modules)?;

    let root_dir = current_dir()?.join(crate::HTML_SYS_CRATE_PATH);
    let _ = fs::remove_dir_all(&root_dir);
    for code in nodes {
        let dir = root_dir.join(&code.dir);
        fs::create_dir_all(&dir)?;

        let filename = dir.join(&code.filename);
        eprintln!(
            "writing: {}/{}/{}",
            crate::HTML_SYS_CRATE_PATH,
            code.dir,
            code.filename
        );
        std::fs::write(filename, code.code.as_bytes())?;
    }
    Ok(())
}

pub fn generate_html() -> Result<()> {
    eprintln!("task: generate html");
    let parsed = lookup_nodes::<ParsedElement>(crate::PARSED_ELEMENTS_PATH)?;
    let manual = lookup_file::<Vec<Attribute>>(crate::MANUAL_PATH, "global_attributes")?;
    let modules = lookup_file::<Vec<Module>>(crate::MANUAL_PATH, "web_sys_modules")?;
    let nodes = html_bindgen::generate::html::generate(parsed, &manual, modules.as_slice())?;

    let root_dir = current_dir()?.join(crate::HTML_CRATE_ELEMENTS_PATH);
    let _ = fs::remove_dir_all(&root_dir);
    for code in nodes {
        let dir = root_dir.join(&code.dir);
        fs::create_dir_all(&dir)?;

        let filename = dir.join(&code.filename);
        eprintln!(
            "writing: {}/{}/{}",
            crate::HTML_CRATE_ELEMENTS_PATH,
            code.dir,
            code.filename
        );
        std::fs::write(filename, code.code.as_bytes())?;
    }
    Ok(())
}
