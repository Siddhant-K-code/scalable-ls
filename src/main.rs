use clap::Parser;
use nc::getdents64;
use std::path::PathBuf;

/// Represents command-line arguments.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The path to the directory to list.
    path: PathBuf,

    /// Buffer size for reading directory entries, with a default value.
    #[clap(short, long, default_value_t = 5 * 1024 * 1024)]
    buf_size: usize,
}

/// Exit codes indicating the outcome of the program.
#[derive(Debug, PartialEq)]
enum ExitCode {
    Success = 0,
    Error = 1,
}

/// Lists the contents of a directory with a given buffer size.
///
/// # Arguments
///
/// * `path` - A `PathBuf` representing the directory to list.
/// * `buf_size` - The size of the buffer used for reading directory entries.
///
/// # Returns
///
/// * An `ExitCode` indicating success or failure.
fn run(path: PathBuf, buf_size: usize) -> ExitCode {
    if !path.is_dir() {
        eprintln!("{:?} is not a directory", path);
        return ExitCode::Error;
    }

    let fd = match unsafe { nc::openat(nc::AT_FDCWD, &path, nc::O_DIRECTORY, 0) } {
        Ok(fd) => fd,
        Err(_) => {
            eprintln!("Failed to open directory: {:?}", path);
            return ExitCode::Error;
        }
    };

    let mut buf: Vec<u8> = vec![0; buf_size];
    let mut out: Vec<String> = Vec::new();

    loop {
        let ret = unsafe { getdents64(fd, buf.as_mut_ptr() as usize, buf_size) };
        match ret {
            Ok(n) if n > 0 => {
                let mut bufp: usize = 0;
                while bufp < n as usize {
                    let dirent =
                        unsafe { &*(buf.as_ptr().add(bufp) as *const nc::linux_dirent64_t) };
                    bufp += dirent.d_reclen as usize;

                    if dirent.d_ino == 0 {
                        continue;
                    }

                    let name = extract_name_from_dirent(dirent);
                    if name != "." && name != ".." {
                        out.push(name);
                    }
                }
            }
            Ok(_) => break,
            Err(_) => {
                eprintln!("Error reading directory entries");
                return ExitCode::Error;
            }
        }
    }

    println!("{}", out.join("\n"));
    ExitCode::Success
}

/// Extracts the name of a file from a directory entry.
///
/// # Arguments
///
/// * `dirent` - A pointer to a `linux_dirent64_t` structure.
///
/// # Returns
///
/// * A `String` representing the name of the file.
fn extract_name_from_dirent(dirent: &nc::linux_dirent64_t) -> String {
    let name_vec: Vec<u8> = (0..nc::PATH_MAX)
        .map(|i| dirent.d_name[i as usize])
        .take_while(|&c| c != 0)
        .collect();

    String::from_utf8(name_vec).unwrap_or_else(|_| "".to_string())
}

fn main() {
    let args = Cli::parse();
    std::process::exit(run(args.path, args.buf_size) as i32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let path = PathBuf::from("./demo_files");
        let buf_size = 5 * 1024 * 1024;
        assert_eq!(run(path, buf_size), ExitCode::Success);
    }
}
