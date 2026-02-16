use anyhow::Result;
use clap::Parser;
use projector_rust::config::{Config, Operation};
use projector_rust::opts::Opts;
use projector_rust::projector::Projector;
use serde::de::value;

fn main() -> Result<()> {
    let opts = Opts::parse();
    let config: Config = opts.try_into()?;

    let proj = Projector::from_config(config.config, config.pwd);

    match config.operation {
        Operation::Print(None) => {
            let value = proj.get_value_all();
            let value = serde_json::to_string(&value)?;
            println!("{}", value)
        }
        Operation::Print(Some(k)) => {
            let value = proj.get_value(k).map(|x| println!("{}", x));
            let value = serde_json::to_string(&value)?;
            println!("{}", value)

        }
        Operation::Add(k, v) => {
            proj.set_value(k, v);
            proj.save()
        }
        Operation::Remove(k) => {
            proj.remove_value(&k);

        }
    }
    println!("{:?}", config);
    Ok(())
}
