use crossterm::style::Color::*;
use std::path::PathBuf;
use structopt::StructOpt;
use termimad::rgb;
use termimad::MadSkin;

#[derive(Debug, StructOpt)]
#[structopt(name = "rust-mdr", about = "Terminal markdown reader")]
struct Cli {
    /// File to read
    // #[structopt(short, long, default_value = "README.md")]
    #[structopt(short, long)]
    file: String,

    /// Get file from Github repository
    #[structopt(short, long)]
    github: Option<String>,

    /// Get file from Bitbucket repository
    #[structopt(short, long)]
    bitbucket: Option<String>,

    /// Git branch to fetch file from
    #[structopt(short = "B", long = "branch")]
    branch: Option<String>,

    /// Show help message
    #[structopt(short, long)]
    help: bool,
}

fn render_content(content: &str) {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(rgb(255, 187, 0));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fgbg(Magenta, rgb(30, 30, 40));
    // skin.headers.align = Alignment::Left;
    // skin.paragraph.align = Alignment::Center;
    // skin.table.align = Alignment::Center;

    let markdown = format!("{}", &content);

    println!("\n");
    println!("{}", skin.term_text(&markdown));
    println!("\n");
}

fn get_file_content(file_name: PathBuf) -> String {
    let result = std::fs::read_to_string(file_name);

    return match result {
        Ok(content) => content,
        Err(error) => {
            panic!("Could not open file. Error: {}", error);
        }
    };
}

fn main() {
    let args = Cli::from_args();
    println!("{:?}", args);

    if args.github != None {
        println!("Github!");
        return;
    }

    if args.bitbucket != None {
        println!("Bitbucket!");
        return;
    }

    println!("Opening file name: {}", args.file);

    let file = PathBuf::from(args.file);
    let content = get_file_content(file);

    render_content(&content);
}
