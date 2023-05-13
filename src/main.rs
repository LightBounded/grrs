use anyhow::{Context, Ok, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read  file `{}`", &args.path.display()))?;

    print_matches(&content, &args.pattern, &mut std::io::stdout());

    return Ok(());
}

fn print_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    content.lines().for_each(|line| {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).unwrap();
        }
    });
}

#[test]
fn print_a_match() {
  let mut result = Vec::new();
  print_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
  assert_eq!(result, b"lorem ipsum\n");
}
