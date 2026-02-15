use crate::quack_check::config::Config;
use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

pub fn ensure_dir(p: &Path) -> Result<()> {
    std::fs::create_dir_all(p).with_context(|| format!("create_dir_all {}", p.display()))
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    format!("{:x}", h.finalize())
}

pub fn hash_file(cfg: &Config, path: &Path) -> Result<String> {
    let mut f = File::open(path).with_context(|| "open file")?;
    let meta = f.metadata().with_context(|| "metadata")?;
    let size = meta.len();

    match cfg.hashing.mode.as_str() {
        "full_sha256" => {
            let mut h = Sha256::new();
            let mut buf = vec![0u8; 1024 * 1024];
            loop {
                let n = f.read(&mut buf)?;
                if n == 0 {
                    break;
                }
                h.update(&buf[..n]);
            }
            Ok(format!("{:x}", h.finalize()))
        }
        "fast_2x16mb" => {
            let w = cfg.hashing.fast_window_bytes.min(size);
            let mut h = Sha256::new();

            if w > 0 {
                f.seek(SeekFrom::Start(0))?;
                let mut buf = vec![0u8; w as usize];
                f.read_exact(&mut buf)?;
                h.update(&buf);

                if size > w {
                    f.seek(SeekFrom::Start(size - w))?;
                    let mut buf2 = vec![0u8; w as usize];
                    f.read_exact(&mut buf2)?;
                    h.update(&buf2);
                }
            }

            h.update(size.to_le_bytes());
            Ok(format!("{:x}", h.finalize()))
        }
        _ => anyhow::bail!("unknown hashing.mode: {}", cfg.hashing.mode),
    }
}
