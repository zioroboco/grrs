use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::error::Error;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("pattern").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    let mut file = NamedTempFile::new()?;
    writeln!(file, "one\ntwo\nthree\nfour\n")?;

    cmd.arg("t").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("two\nthree"));

    Ok(())
}
