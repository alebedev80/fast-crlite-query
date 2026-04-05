use clubcard_crlite::{CRLiteClubcard, CRLiteKey, CRLiteStatus};
use clap::Parser;
use serde::Serialize;
use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::convert::TryInto;

#[derive(Parser, Debug)]
#[command(author, version, about = "Mass CRLite revocation checker")]
struct Args {
    #[arg(short, long)]
    db_dir: String,
    #[arg(short, long)]
    json: bool,
}

#[derive(Serialize)]
struct CheckResult {
    serial: String,
    issuer_hash: String,
    status: String,
    scts_provided: usize,
}

fn main() {
    let args = Args::parse();
    let db_path = Path::new(&args.db_dir);

    let mut filter_files: Vec<PathBuf> = fs::read_dir(db_path)
        .expect("Failed to read DB directory")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            let filename = path.file_name()?.to_str()?;
            if filename.contains(".filter") || filename.contains(".delta") {
                Some(path)
            } else { None }
        })
        .collect();

    filter_files.sort();

    let mut filters = Vec::new();
    for file in &filter_files {
        if let Ok(bytes) = fs::read(file) {
            if let Ok(f) = CRLiteClubcard::from_bytes(&bytes) {
                filters.push(f);
            }
        }
    }

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input = match line { Ok(l) => l, Err(_) => break };
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 2 { continue; }

        let issuer_vec = hex::decode(parts[0]).unwrap_or_default();
        let serial_vec = hex::decode(parts[1]).unwrap_or_default();
        if issuer_vec.len() != 32 { continue; }

        let issuer_hash: [u8; 32] = issuer_vec.try_into().unwrap();
        let key = CRLiteKey::new(&issuer_hash, &serial_vec);

        let mut scts = Vec::new();
        if parts.len() > 2 {
            for sct_str in parts[2..].iter() {
                let sub: Vec<&str> = sct_str.split(':').collect();
                if sub.len() == 2 {
                    if let (Ok(id_vec), Ok(ts)) = (hex::decode(sub[0]), sub[1].parse::<u64>()) {
                        if let Ok(id) = id_vec.try_into() { scts.push((id, ts)); }
                    }
                }
            }
        }

        let mut final_status_str = "NotRevoked";
        for filter in &filters {
            let sct_iter = scts.iter().map(|(id, ts)| (id, *ts));
            if let CRLiteStatus::Revoked = filter.contains(&key, sct_iter) {
                final_status_str = "Revoked";
                break;
            }
        }

        if args.json {
            let res = CheckResult {
                serial: parts[1].to_string(),
                issuer_hash: parts[0].to_string(),
                status: final_status_str.to_string(),
                scts_provided: scts.len(),
            };
            println!("{}", serde_json::to_string(&res).unwrap());
        } else {
            println!("{} | {}", final_status_str, parts[1]);
        }
    }
}
