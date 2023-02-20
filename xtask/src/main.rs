use std::fs;

use structopt::StructOpt;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, Error>;

/// Tooling for `yosh.is`
#[derive(StructOpt)]
enum Opt {
    /// Generate source code from static sources
    Generate,
}

fn main() -> Result<()> {
    match Opt::from_args() {
        Opt::Generate => {
            let idl_path = std::env::current_dir()?.join("resources/webidls");

            // generate IDL files
            let database = html_bindgen::parse(&idl_path)?;
            let path = std::env::current_dir()?.join("resources/nodes");
            fs::create_dir(&path)?;
            for def in database {
                let path = path.join(format!("{}.toml", def.tag_name));
                std::fs::write(path, def.to_string().as_bytes())?;
            }

            // generate bindings
            let s = html_bindgen::generate_html(&idl_path)?;

            // write
            let path = std::env::current_dir()?.join("crates/html-sys/src/lib.rs");
            std::fs::write(path, s.as_bytes())?;
            Ok(())
        }
    }
}
