/// Contains the absolute paths of the directories containing: the source code for all challenges;
/// and the integration tests.
#[derive(Debug)]
struct ProjectPaths {
    integration_tests: std::path::PathBuf,
    source_code: std::path::PathBuf,
}

impl ProjectPaths {
    /// Returns a new `ProjectPaths` object containing: the source code for all challenges;
    /// and the integration tests. This is based on the assumption that the calling code is
    /// running in the "deps" directory within a subdirectory of the "target" directory.
    fn new() -> Self {
        let integration_test_full_path = std::env::current_exe()
            .expect("Integration test failed to determine the path it was run from");

        let integration_test_dir = integration_test_full_path.parent().unwrap().parent().unwrap();
        let crate_top_level_dir = integration_test_dir.parent().unwrap().parent().unwrap();
        let input_file_dir = crate_top_level_dir.join("src/bin/");

        Self {
            integration_tests: integration_test_dir.to_owned(),
            source_code: input_file_dir.to_owned()
        }
    }
}

/// Runs the challenge whose name is passed in `name` and returns the resulting exit status,
/// standard error and standard output if the run is successful.
///
/// # Panics
///
/// Panics if the challenge does not run successfully.
pub fn run_challenge(name: &str) -> String {
    let project_paths = ProjectPaths::new();

    let output = std::process::Command::new(project_paths.integration_tests.join(name))
        .current_dir(project_paths.source_code)
        .output()
        .expect(&format!("Failed to run challenge {}", name));

    if !output.status.success() {
        println!("Challenge {} returned a non-zero exit status", name);
        println!("    Exit status: {}", output.status);
        println!("    Standard error: {}", String::from_utf8_lossy(&output.stderr));
        println!("    Standard output: {}", String::from_utf8_lossy(&output.stdout));
        panic!("    Terminating test");
    }

    String::from_utf8_lossy(&output.stdout).into_owned()
}
