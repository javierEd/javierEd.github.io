use std::process::Command;

use chrono::{SecondsFormat, Utc};

fn main() {
    let git_rev_short = {
        let output = Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .unwrap();

        String::from_utf8(output.stdout).unwrap()
    };
    let build_datetime = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);

    println!("cargo:rustc-env=GIT_REV_SHORT={git_rev_short}");
    println!("cargo:rustc-env=BUILD_DATETIME={build_datetime}");
}
