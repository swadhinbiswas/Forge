use anyhow::{Context, Result};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use reqwest::blocking::Client;
use std::env;
use std::fs;
use std::io::{Cursor, Read};
use std::process::Command;

/// Applies a full update by downloading the payload, verifying signature (if provided),
/// replacing the current executable, and restarting.
pub fn apply_update(
    url: String,
    signature_hex: Option<String>,
    pub_key_hex: Option<String>,
) -> Result<()> {
    // 1. Download payload to a temp file
    let client = Client::builder()
        .user_agent("forge-framework-updater")
        .build()?;

    let mut response = client.get(&url).send()?.error_for_status()?;
    let mut payload = Vec::new();
    response.read_to_end(&mut payload)?;

    // 2. Verify signature if provided
    if let (Some(sig_hex), Some(pk_hex)) = (signature_hex, pub_key_hex) {
        verify_signature(&payload, &sig_hex, &pk_hex)?;
    }

    // 3. Save payload to temporary executable file
    let temp_file = tempfile::NamedTempFile::new()?;
    let temp_path = temp_file.path().to_path_buf();
    fs::write(&temp_path, &payload)?;

    // Set executable permissions (Unix)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&temp_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&temp_path, perms)?;
    }

    // 4. Securely replace the current executable
    self_replace::self_replace(&temp_path)?;

    // Ensure cleanup of the temp file
    drop(temp_file);

    // 5. Restart the application
    restart_application()?;

    Ok(())
}

/// Applies a delta update by downloading the binary diff patch, verifying signature,
/// applying the patch to the current executable, and restarting.
///
/// Delta updates are significantly smaller than full updates (typically 1-5MB vs 30-50MB).
/// The patch is a bsdiff binary diff between the old and new versions.
pub fn apply_delta_update(
    patch_url: String,
    expected_old_hash: Option<String>,
    expected_new_hash: Option<String>,
    signature_hex: Option<String>,
    pub_key_hex: Option<String>,
) -> Result<()> {
    // 1. Download the delta patch
    let client = Client::builder()
        .user_agent("forge-framework-updater")
        .build()?;

    let mut response = client.get(&patch_url).send()?.error_for_status()?;
    let mut patch_data = Vec::new();
    response.read_to_end(&mut patch_data)?;

    // 2. Verify signature if provided
    if let (Some(sig_hex), Some(pk_hex)) = (signature_hex, pub_key_hex) {
        verify_signature(&patch_data, &sig_hex, &pk_hex)?;
    }

    // 3. Read the current executable
    let current_exe = env::current_exe()?;
    let old_data = fs::read(&current_exe)?;

    // 4. Verify old hash if provided
    if let Some(ref hash) = expected_old_hash {
        let actual_hash = sha256_hex(&old_data);
        if &actual_hash != hash {
            anyhow::bail!(
                "Current executable hash mismatch. Expected: {}, Got: {}",
                hash,
                actual_hash
            );
        }
    }

    // 5. Apply the bsdiff patch
    let mut patch_reader = Cursor::new(&patch_data);
    let mut new_data = vec![0u8; old_data.len() * 2]; // Allocate generous buffer
    bsdiff::patch::patch(&old_data, &mut patch_reader, &mut new_data)
        .context("Failed to apply binary patch. The patch may be for a different version.")?;
    new_data.truncate(new_data.len()); // Trim to actual size

    // 6. Verify new hash if provided
    if let Some(ref hash) = expected_new_hash {
        let actual_hash = sha256_hex(&new_data);
        if &actual_hash != hash {
            anyhow::bail!(
                "Patched executable hash mismatch. Expected: {}, Got: {}",
                hash,
                actual_hash
            );
        }
    }

    // 7. Save patched executable to temp file
    let temp_file = tempfile::NamedTempFile::new()?;
    let temp_path = temp_file.path().to_path_buf();
    fs::write(&temp_path, &new_data)?;

    // Set executable permissions (Unix)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&temp_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&temp_path, perms)?;
    }

    // 8. Securely replace the current executable
    self_replace::self_replace(&temp_path)?;

    // Ensure cleanup of the temp file
    drop(temp_file);

    // 9. Restart the application
    restart_application()?;

    Ok(())
}

/// Verify an Ed25519 signature against the payload.
fn verify_signature(payload: &[u8], sig_hex: &str, pk_hex: &str) -> Result<()> {
    let sig_bytes = hex::decode(sig_hex).context("Invalid signature hex")?;
    let pk_bytes = hex::decode(pk_hex).context("Invalid public key hex")?;

    let public_key =
        VerifyingKey::try_from(pk_bytes.as_slice()).context("Invalid public key format")?;
    let signature = Signature::from_slice(&sig_bytes).context("Invalid signature format")?;

    public_key
        .verify(payload, &signature)
        .context("Signature verification failed")?;

    Ok(())
}

/// Compute SHA-256 hash of data as hex string.
fn sha256_hex(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Restart the current application.
fn restart_application() -> Result<()> {
    let current_exe = env::current_exe()?;
    let args: Vec<String> = env::args().collect();
    let _ = Command::new(current_exe).args(&args[1..]).spawn()?;
    Ok(())
}
