use std::error::Error;

use crate::config::get_options;

pub fn get_sops_secrets(key: Vec<&str>) -> Result<String, Box<dyn Error>> {
    let path = &get_options().secrets.path;

    let secret_keys = key
        .iter()
        .map(|k| format!(r#"["{}"]"#, k))
        .collect::<String>();

    let output = match std::process::Command::new("sops")
        .args([
            "--decrypt",
            "--output-type",
            "json",
            "--extract",
            &secret_keys,
            &path,
        ])
        .output()
    {
        Ok(r) => r,
        Err(r) => return Err(Box::new(r)),
    };

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
