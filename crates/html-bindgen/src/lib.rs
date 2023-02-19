use std::{fs, path::Path};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, Error>;

pub fn generate_html(path: &Path) -> Result<()> {
    let dir = fs::read_dir(path)?;
    for file in dir {
        let s = fs::read_to_string(file?.path())?;
        let definitions = match weedle::parse(&s) {
            Ok(t) => t,
            e => Err(format!("{:?}", e))?,
        };
        for def in definitions {
            match def {
                weedle::Definition::Callback(_) => {}
                weedle::Definition::CallbackInterface(_) => {}
                weedle::Definition::Interface(interface) => {
                    let name = interface.identifier.0;
                    print!("{}", name);
                    if let Some(parent) = interface.inheritance {
                        println!(": {}", parent.identifier.0);
                    }
                }
                weedle::Definition::InterfaceMixin(_) => {}
                weedle::Definition::Namespace(_) => {}
                weedle::Definition::Dictionary(_) => {}
                weedle::Definition::PartialInterface(_) => {}
                weedle::Definition::PartialInterfaceMixin(_) => {}
                weedle::Definition::PartialDictionary(_) => {}
                weedle::Definition::PartialNamespace(_) => {}
                weedle::Definition::Enum(_) => {}
                weedle::Definition::Typedef(_) => {}
                weedle::Definition::IncludesStatement(_) => {}
                weedle::Definition::Implements(_) => {}
            }
        }
    }
    Ok(())
}
