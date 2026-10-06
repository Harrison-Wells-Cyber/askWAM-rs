use std::process::Command;

#[test]
fn executable_help_works_without_a_wam_account() {
    let output = Command::new(env!("CARGO_BIN_EXE_askwam"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let help = String::from_utf8(output.stdout).unwrap();
    for arg in [
        "--enum",
        "--scope",
        "--resource",
        "--claims-json",
        "--cae",
        "--hide",
        "--username",
        "--account-id",
        "--client-id",
        "--authority",
        "--timeout-seconds",
    ] {
        assert!(help.contains(arg));
    }
}

#[test]
fn executable_input_errors_use_stderr_and_exit_two_without_echoing_claims() {
    let output = Command::new(env!("CARGO_BIN_EXE_askwam"))
        .args(["--claims-json", "{sensitive_claim_challenge}"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let json: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(json["status"], "input_error");
    assert!(!String::from_utf8(output.stderr)
        .unwrap()
        .contains("sensitive_claim_challenge"));
}

#[cfg(not(windows))]
#[test]
fn nonwindows_execution_fails_explicitly() {
    let output = Command::new(env!("CARGO_BIN_EXE_askwam"))
        .arg("--enum")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(40));
    assert!(output.stdout.is_empty());
    let json: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(json["status"], "unsupported_platform");
}
