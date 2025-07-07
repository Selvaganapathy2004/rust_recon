use clap::{Parser, Subcommand};
use reqwest::{self, header::HeaderMap};
use serde::Deserialize;
use std::collections::HashSet;
use trust_dns_resolver::TokioAsyncResolver;
use std::env;
use clap::CommandFactory;

/// Passive Recon Tool in Rust
#[derive(Parser)]
#[command(name = "RustRecon")]
#[command(version = "1.0")]
#[command(about = "Perform DNS, Header, Subdomain recon", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Perform DNS enumeration
    Dns {
        #[arg(short, long)]
        target: String,
    },
    /// Fetch HTTP headers
    Headers {
        #[arg(short, long)]
        url: String,
    },
    /// Subdomain enumeration via crt.sh
    Subdomains {
        #[arg(short, long)]
        domain: String,
    },
}

#[derive(Debug, Deserialize)]
struct CrtShEntry {
    name_value: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    if env::args().len() == 1 {
        // User ran the command with no args
        Cli::command().print_help()?;  // Show help message
        println!(); // Newline after help
        std::process::exit(1); // Exit with code 1
    }


    let cli = Cli::parse();

    match &cli.command {
        Commands::Dns { target } => {
            dns_lookup(target).await?;
        }
        Commands::Headers { url } => {
            fetch_headers(url).await?;
        }
        Commands::Subdomains { domain } => {
            subdomain_enum(domain).await?;
        }
    }

    Ok(())
}

async fn fetch_headers(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::get(url).await?;
    let header: &HeaderMap = res.headers();

    println!("\n--- HTTP HEADERS ---\n");
    for (key, value) in header {
        println!("{}: {:?}", key, value);
    }

    println!("\n--- STATUS CODE ---\n");
    println!("{}", res.status());
    Ok(())
}

async fn dns_lookup(domain: &str) -> Result<(), Box<dyn std::error::Error>> {
    let resolver = TokioAsyncResolver::tokio_from_system_conf()?;

    println!("\n--- A RECORDS ---\n");
    let response = resolver.lookup_ip(domain).await?;
    for ip in response.iter() {
        println!("IPv4 Addr: {}", ip);
    }

    println!("\n--- AAAA RECORDS ---\n");
    let v6_resolve = resolver.ipv6_lookup(domain).await?;
    for ip in v6_resolve.iter() {
        println!("IPv6 Addr: {}", ip);
    }

    println!("\n--- MX RECORDS ---\n");
    if let Ok(mx_resolve) = resolver.mx_lookup(domain).await {
        for ip in mx_resolve.iter() {
            println!("MX Addr: {}", ip);
        }
    } else {
        println!("No MX records found.");
    }

    println!("\n--- NS RECORDS ---\n");
    if let Ok(ns_resolve) = resolver.ns_lookup(domain).await {
        for ip in ns_resolve.iter() {
            println!("NS Addr: {}", ip);
        }
    } else {
        println!("No NS records found.");
    }

    println!("\n--- TXT RECORDS ---\n");
    if let Ok(txt_resolve) = resolver.txt_lookup(domain).await {
        for ip in txt_resolve.iter() {
            println!("TXT: {}", ip);
        }
    } else {
        println!("No TXT records found.");
    }

    Ok(())
}

async fn subdomain_enum(domain: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- Subdomain Enumeration via crt.sh ---\n");
    let crt_url = format!("https://crt.sh/?q=%25.{}&output=json", domain);
    let resp = reqwest::get(&crt_url).await?;

    if resp.status().is_success() {
        let json_data = resp.text().await?;
        let parsed: Result<Vec<CrtShEntry>, _> = serde_json::from_str(&json_data);

        match parsed {
            Ok(entries) => {
                let mut subdomains = HashSet::new();
                for entry in entries {
                    for line in entry.name_value.lines() {
                        if line.contains(domain) {
                            subdomains.insert(line.trim().to_string());
                        }
                    }
                }

                if subdomains.is_empty() {
                    println!("No subdomains found.");
                } else {
                    println!("Subdomains found:");
                    for sd in &subdomains {
                        println!("- {}", sd);
                    }
                }
            }
            Err(_) => {
                println!("Failed to parse JSON from crt.sh");
            }
        }
    } else {
        println!("Failed to fetch from crt.sh");
    }

    Ok(())
}
