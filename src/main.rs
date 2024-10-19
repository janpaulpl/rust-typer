use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};
use std::{fs, io::{self, Write, Read}};
use reqwest;
use rand::Rng;
use serde::Deserialize;
use serde_json::Value;
use std::pin::Pin;
use clap::{Command, Arg};
use termion::terminal_size;

#[derive(Deserialize, Debug)]
struct GitHubContent {
    path: Option<String>,
    #[serde(rename = "type")]
    content_type: Option<String>,
}

async fn get_rust_files_from_github(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let api_url = format!("https://api.github.com/repos/rust-lang/rust/contents/{}", path);
    let client = reqwest::Client::new();

    let response = client
        .get(&api_url)
        .header("User-Agent", "rust_code_typer")
        .send()
        .await?;

    let json_text = response.text().await?;
    let json: Value = serde_json::from_str(&json_text)?;

    let mut rust_files = Vec::new();

    if json.is_array() {
        let contents: Vec<GitHubContent> = serde_json::from_value(json).unwrap_or_else(|e| {
            println!("Error deserializing GitHubContent: {}", e);
            vec![]
        });

        for content in contents {
            if let (Some(path), Some(content_type)) = (content.path, content.content_type) {
                if content_type == "file" {
                    rust_files.push(path);
                } else if content_type == "dir" {
                    let mut sub_rust_files = Pin::from(Box::new(get_rust_files_from_github(&path))).await?;
                    rust_files.append(&mut sub_rust_files);
                }
            }
        }
    }

    Ok(rust_files)
}

fn get_files_from_local_dir(path: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let sub_files = get_files_from_local_dir(path.to_str().unwrap())?;
            files.extend(sub_files);
        } else {
            files.push(path.to_str().unwrap().to_string());
        }
    }
    Ok(files)
}

async fn fetch_random_file(github: bool, local_path: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let files = if github {
        get_rust_files_from_github("").await?
    } else if let Some(path) = local_path {
        get_files_from_local_dir(path)?
    } else {
        return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "No valid source provided")));
    };

    if files.is_empty() {
        return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "No files found")));
    }

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..files.len());
    let selected_file = &files[random_index];

    println!("Selected file: {}", selected_file);

    // Check if the file can be read as UTF-8
    let mut file_content = String::new();
    let mut file = fs::File::open(selected_file)?;
    if file.read_to_string(&mut file_content).is_err() {
        // If the file cannot be read as UTF-8, skip this file
        return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "File is not valid UTF-8")));
    }

    Ok(file_content)
}

fn display_file_content(content: &str) -> io::Result<()> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    stdout.execute(Clear(ClearType::All))?;

    let (width, _height) = terminal_size().unwrap_or((80, 20)); // Use termion to get terminal size

    let mut chars = content.chars().peekable();

    while let Some(_) = chars.peek() {
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(event) = event::read()? {
                match event.code {
                    KeyCode::Char(_) => {
                        // Display 5 characters at a time when a key is pressed
                        let chunk: String = chars.by_ref().take(5).collect();
                        print!("{}", chunk);
                        stdout.flush()?;
                    }
                    KeyCode::Esc => break, // Exit on ESC key
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Rust Typer")
        .version("1.0")
        .author("janpaul.pl")
        .about("Displays a random file's content from GitHub or a local directory")
        .arg(
            Arg::new("local")
                .short('l')
                .long("local")
                .value_name("DIR")
                .help("Use a local directory instead of GitHub"), 
        )
        .get_matches();

    let local_path = matches.get_one::<String>("local").map(|s| s.as_str());
    let github = local_path.is_none();

    let content = fetch_random_file(github, local_path).await?;
    display_file_content(&content)?;

    Ok(())
}
