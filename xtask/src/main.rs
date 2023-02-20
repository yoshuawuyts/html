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

            let path = std::env::current_dir()?.join("resources/html-standard/index.html");
            let spec = fs::read_to_string(path)?;
            html_bindgen::parse_spec(spec)?;

            std::process::exit(0);

            // generate IDL files
            let database = html_bindgen::parse(&idl_path)?;
            let path = std::env::current_dir()?.join("resources/nodes");
            fs::create_dir_all(&path)?;
            for def in database {
                let path = path.join(format!("{}.json", def.tag_name));

                let s = serde_json::to_string_pretty(&def)?;
                std::fs::write(path, s.to_string().as_bytes())?;
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
