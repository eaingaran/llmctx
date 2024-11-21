use clap::Parser;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use env_logger::Env;
use log::{debug, error, info};
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Comma-separated list of folders and files to exclude. (case insensitive)
    /// Example: 'target,node_modules,.git,cargo.toml' (default: '')
    #[arg(short, long, default_value = "")]
    exclude: String,

    /// Path to the output file (default: 'output.txt')
    #[arg(short, long, default_value = "output.txt")]
    output: PathBuf,

    /// Copy the output to the clipboard (default: false)
    #[arg(long, action)]
    clipboard: bool,

    /// Generate compact output (default: false)
    /// This will remove new lines and tab spaces
    #[arg(short, long, action)]
    compact: bool,

    /// Input directory to process (default: current directory)
    #[arg(default_value = ".")]
    input_dir: PathBuf,
}

fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let args = Args::parse();

    let base_path = args.input_dir.canonicalize().unwrap();

    info!("Processing directory: {}", base_path.display());

    let exclude_names: Vec<String> = args
        .exclude
        .split(',')
        .map(|name| name.to_lowercase())
        .collect();

    info!("Exclude names: {:?}", exclude_names);

    let mut output_content = String::new();
    if args.compact {
        output_content.push_str(&format!("The following is the source code of {}. Tabs (2 spaces and 4 spaces) and new lines have been removed from within a source file. So, each line either specifies the file path of the source file or the content of the source file. File path lines are of the format '--- file_path ---'. The source code content will end with '--- end of source code ---'.", base_path.file_name().unwrap_or_default().to_string_lossy().to_ascii_lowercase()));
    } else {
        output_content.push_str(&format!("The following is the source code of {}. The source code content will end with '--- end of source code ---'.", base_path.file_name().unwrap_or_default().to_string_lossy().to_ascii_lowercase()));
    }

    fs::remove_file(&args.output).err();

    for entry in WalkDir::new(&args.input_dir) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let file_path = entry.path();
            let file_path_str_normalized: String = file_path
                .to_str()
                .unwrap_or_default()
                .replace("\\", "/")
                .replace("./", "")
                .to_lowercase();
            let mut exclude_match = false;
            if !exclude_names.is_empty() {
                for exclude_name in &exclude_names {
                    if file_path_str_normalized.starts_with(exclude_name) {
                        exclude_match = true;
                        break;
                    }
                }
            }

            if !exclude_match {
                debug!("Including file: {}", file_path.display());
                match fs::read_to_string(file_path) {
                    Ok(contents) => {
                        output_content.push_str(&format!("\n--- {} ---\n", file_path.display()));
                        if args.compact {
                            output_content.push_str(&contents.replace("\n", "").replace("  ", ""));
                        } else {
                            output_content.push_str(&contents);
                        }
                    }
                    Err(err) => error!("Error reading file {}: {}", file_path.display(), err),
                }
            } else {
                debug!("Excluding file: {}", file_path.display());
            }
        }
    }
    output_content.push_str("\n--- end of source code ---");

    match fs::write(&args.output, output_content.clone()) {
        Ok(_) => info!("Context written to {}", args.output.display()),
        Err(err) => error!("Error writing to output file: {}", err),
    }

    if args.clipboard {
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents(output_content.to_owned()).unwrap();
        assert_eq!(ctx.get_contents().unwrap(), output_content);
    }
}
