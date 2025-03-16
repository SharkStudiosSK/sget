use anyhow::{Context, Result};
use clap::Parser;
use futures_util::StreamExt;
use humansize::{format_size, BINARY};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;
use url::Url;

#[derive(Parser)]
#[clap(
    name = "sget",
    about = "A CLI tool to download files from the web",
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS")
)]
struct Cli {
    /// URL to download
    url: String,

    /// Output file (defaults to the filename from URL)
    #[clap(short, long)]
    output: Option<PathBuf>,

    /// Show verbose output
    #[clap(short, long)]
    verbose: bool,

    /// No progress bar, just download
    #[clap(short, long)]
    quiet: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Parse the URL
    let url = Url::parse(&cli.url)
        .with_context(|| format!("Failed to parse URL: {}", cli.url))?;
    
    // Determine the output filename
    let output_path = if let Some(path) = cli.output {
        path
    } else {
        // Extract filename from URL or use a default
        url.path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("downloaded_file")
            .into()
    };
    
    if cli.verbose {
        println!("Downloading from: {}", cli.url);
        println!("Saving to: {}", output_path.display());
    }
    
    // Create the HTTP client
    let client = Client::new();
    
    // Make the request
    let res = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("Failed to send request to {}", cli.url))?;
    
    // Check if the request was successful
    let status = res.status();
    if !status.is_success() {
        anyhow::bail!("HTTP request failed with status: {}", status);
    }
    
    // Get the total size to use in the progress bar
    let total_size = res.content_length().unwrap_or(0);
    
    if cli.verbose {
        if total_size > 0 {
            println!("File size: {}", format_size(total_size, BINARY));
        } else {
            println!("File size: unknown (server didn't provide Content-Length)");
        }
    }
    
    // Create the output file
    let mut file = File::create(&output_path)
        .with_context(|| format!("Failed to create output file: {}", output_path.display()))?;
    
    let start_time = Instant::now();
    
    if cli.quiet {
        // No progress bar mode
        let bytes = res.bytes().await?;
        file.write_all(&bytes)?;
        if cli.verbose {
            println!(
                "Downloaded {} in {:.2} seconds",
                format_size(bytes.len() as u64, BINARY),
                start_time.elapsed().as_secs_f64()
            );
        }
    } else {
        // Set up the progress bar
        let pb = if total_size == 0 {
            // For unknown size, use a spinner with custom template
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::with_template(
                    "{spinner:.green} [{elapsed_precise}] {msg}"
                )
                .unwrap()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            );
            // Initialize with zero downloaded
            pb.set_message("0 B downloaded (0 B/s)");
            pb.enable_steady_tick(std::time::Duration::from_millis(120));
            pb
        } else {
            let pb = ProgressBar::new(total_size);
            pb.set_style(
                ProgressStyle::with_template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})"
                )
                .unwrap()
                .with_key("eta", |state: &ProgressState, w: &mut dyn std::fmt::Write| {
                    write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
                })
                .progress_chars("#>-"),
            );
            pb
        };
        
        // Process the download stream
        let mut downloaded = 0;
        let mut stream = res.bytes_stream();
        let mut last_update = Instant::now();
        
        while let Some(item) = stream.next().await {
            let chunk = item.with_context(|| "Error while downloading")?;
            file.write_all(&chunk)
                .with_context(|| "Error while writing to file")?;
            
            downloaded += chunk.len() as u64;
            
            if total_size == 0 {
                // Update the spinner message at most 5 times per second to avoid flickering
                let now = Instant::now();
                if now.duration_since(last_update).as_millis() > 200 || downloaded < 8192 {
                    last_update = now;
                    let elapsed = start_time.elapsed().as_secs_f64();
                    let speed = if elapsed > 0.0 { downloaded as f64 / elapsed } else { 0.0 };
                    
                    pb.set_message(format!(
                        "{} downloaded ({}/s)",
                        format_size(downloaded, BINARY),
                        format_size(speed as u64, BINARY)
                    ));
                }
            } else {
                pb.set_position(downloaded);
            }
        }
        
        pb.finish_with_message(format!("Downloaded {} in {:.2}s", 
            format_size(downloaded, BINARY),
            start_time.elapsed().as_secs_f64())
        );
    }
    
    Ok(())
}
