mod logger;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]

struct Args {
    /// Output file path
    #[arg(short, long)]
    output: String,
}

//Function to detect OS automatically

fn detect_os() -> &'static str {
    if cfg!(target_os = "windows") {
        "Windows"
    } else if cfg!(target_os = "linux") {
        "Linux"
    } else if cfg!(target_os = "macos") {
        "MacOS"
    } else {
        "unknown"
    }
}

fn main() {
    println!("🔍 Welcome to MemCrux - Cross-Platform Memory Forensics Tool");
    //logger.rs test
    logger::log("Memory Crux Created");
    logger::log("Ready for Memory Acquisition.");

    //platform Detect test
    let args = Args::parse();
    let os = detect_os();
    logger::log(&format!("Detected Platform : {}",os));
    logger::log(&format!("Output path: {}", args.output));
    // Later: Call the correct memory dump module here based on platform
}
