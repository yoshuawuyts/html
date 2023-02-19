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
            // parse
            let path = std::env::current_dir()?.join("resources/webidls");
            let s = html_bindgen::generate_html(&path)?;

            // write
            let path = std::env::current_dir()?.join("crates/html-sys/src/lib.rs");
            std::fs::write(path, s.as_bytes())?;
            Ok(())
        }
    }
}
