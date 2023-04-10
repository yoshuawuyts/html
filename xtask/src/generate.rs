use crate::Result;
use crate::{lookup_json_dir, lookup_json_file};
use html_bindgen::generate::{Module, ModuleMapping};
use html_bindgen::merge::MergedElement;
use html_bindgen::parse::Attribute;
use std::{env::current_dir, fs};

pub fn generate_sys() -> Result<()> {
    eprintln!("task: generate sys");
    let merged = lookup_json_dir::<MergedElement>(crate::MERGED_ELEMENTS_PATH)?;
    let manual = lookup_json_file::<Vec<Attribute>>(crate::MANUAL_PATH, "global_attributes")?;
    let modules = lookup_json_file::<Vec<Module>>(crate::MANUAL_PATH, "web_sys_modules")?;
    let nodes = html_bindgen::generate::sys::generate(merged, &manual, &modules)?;

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
    let merged = lookup_json_dir::<MergedElement>(crate::MERGED_ELEMENTS_PATH)?;
    let manual = lookup_json_file::<Vec<Attribute>>(crate::MANUAL_PATH, "global_attributes")?;
    let module_map = lookup_json_file::<Vec<ModuleMapping>>(crate::MANUAL_PATH, "mdn_modules")?;
    let nodes = html_bindgen::generate::html::generate(merged, &manual, module_map.as_slice())?;

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
