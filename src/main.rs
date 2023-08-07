use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0.0",
    author = "Dewa Sahu",
    about = "hugging face translator"
)]

struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Translate { path: String, save_path: String },
    Print { path: String },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Translate { path, save_path }) => {
            rust_hf_gpu_translator::translate(&path, &save_path);
        }
        Some(Commands::Print { path }) => {
            let res = rust_hf_gpu_translator::read_file(&path)?;
            println!("{}", res);
        }
        None => println!("No command found"),
    }
    Ok(())
}
