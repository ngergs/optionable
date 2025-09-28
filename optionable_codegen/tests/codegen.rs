use assert_cmd::assert::OutputAssertExt;
use assert_cmd::Command;
use std::path::PathBuf;
use std::{fs, io};
use tempfile::TempDir;

#[test]
fn simple() {
    test("example/simple/input", "example/simple/output", vec![]);
}

#[test]
fn no_convert() {
    test(
        "example/no_convert/input",
        "example/no_convert/output",
        vec!["--no-convert"],
    );
}

#[test]
fn extra_derives() {
    test(
        "example/extra_derives/input",
        "example/extra_derives/output",
        vec!["-d", "serde::Serialize", "-d", "serde::Deserialize"],
    );
}

/// Implementation of the core integration test for the codegen.
/// Checks that the codegen returns without errors and verifies that the
/// expected output in the output directory is generated (into a temp directory).
fn test(input_path: &str, expected_output_path: &str, extra_args: Vec<&str>) {
    let temp_dir = TempDir::new().unwrap();
    let mut args = vec![
        input_path.to_owned(),
        temp_dir.path().to_str().unwrap().to_owned(),
    ];
    let mut extra_args = extra_args.iter().map(|e| e.to_string()).collect::<Vec<_>>();
    args.append(&mut extra_args);
    let cmd = Command::cargo_bin("codegen").unwrap().args(args).unwrap();
    cmd.assert().success();
    assert!(dirs_contained(&expected_output_path.into(), &temp_dir.path().into()).unwrap());
}

/// Checks whether the file content of all files in the input directory are contained in the others directory.
fn dirs_contained(input: &PathBuf, other: &PathBuf) -> Result<bool, io::Error> {
    let files_input = fs::read_dir(input)?
        .map(|file| {
            let file = file?;
            let file_type = file.file_type()?;
            Ok::<_, io::Error>((file, file_type))
        })
        .collect::<Result<Vec<_>, _>>()?;
    files_input
        .iter()
        .filter(|&(_, file_type)| file_type.is_dir())
        .map(|(f, _)| dirs_contained(&input.join(f.file_name()), &other.join(f.file_name())))
        .collect::<Result<Vec<_>, _>>()?;
    let files_input = files_input
        .into_iter()
        .filter(|&(_, file_type)| file_type.is_file());
    for (file_input, _) in files_input {
        let data_input = fs::read(file_input.path())?;
        let data_other = fs::read(other.join(file_input.file_name()))?;
        if data_input != data_other {
            return Ok(false);
        }
    }
    Ok(true)
}
