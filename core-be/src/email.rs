use reqwest::Client;
use serde_json::json;
use tracing::error;

const RESEND_API_URL: &str = "https://api.resend.com/emails";

/// Send a transactional email via the Resend API.
async fn send_email(
    api_key: &str,
    from: &str,
    to: &str,
    subject: &str,
    html: &str,
) -> Result<(), String> {
    let client = Client::new();
    let body = json!({
        "from": from,
        "to": [to],
        "subject": subject,
        "html": html,
    });

    let res = client
        .post(RESEND_API_URL)
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Resend HTTP error: {}", e))?;

    if res.status().is_success() {
        Ok(())
    } else {
        let status = res.status();
        let body_text = res.text().await.unwrap_or_default();
        error!("Resend API error {}: {}", status, body_text);
        Err(format!("Resend API error {}: {}", status, body_text))
    }
}

/// Send the account email verification email.
pub async fn send_verification_email(
    api_key: &str,
    from_address: &str,
    to_email: &str,
    verification_url: &str,
) -> Result<(), String> {
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
  <title>Verify your Kagua account</title>
</head>
<body style="margin:0;padding:0;background:#070b10;font-family:'Inter',Arial,sans-serif;">
  <table width="100%" cellpadding="0" cellspacing="0" style="background:#070b10;padding:48px 16px;">
    <tr><td align="center">
      <table width="560" cellpadding="0" cellspacing="0" style="background:#0d1520;border:1px solid rgba(0,240,255,0.12);border-radius:12px;padding:48px 40px;">
        <tr><td align="center" style="padding-bottom:32px;">
          <h1 style="font-size:22px;font-weight:800;color:#00f0ff;letter-spacing:0.1em;margin:0;">KAGUA</h1>
          <p style="color:#788a9c;font-size:12px;letter-spacing:0.15em;margin:4px 0 0;">COMPLIANCE INTELLIGENCE ENGINE</p>
        </td></tr>
        <tr><td>
          <h2 style="color:#e2f1f8;font-size:20px;font-weight:600;margin:0 0 12px;">Confirm your email address</h2>
          <p style="color:#788a9c;font-size:15px;line-height:1.6;margin:0 0 28px;">
            Welcome to Kagua. Click the button below to verify your email and activate your account. This link expires in <strong style="color:#e2f1f8;">24 hours</strong>.
          </p>
          <a href="{verification_url}"
             style="display:inline-block;padding:14px 32px;background:linear-gradient(135deg,#00f0ff,#0090a8);color:#070b10;font-weight:700;font-size:14px;letter-spacing:0.05em;text-decoration:none;border-radius:6px;">
            VERIFY EMAIL
          </a>
          <p style="color:#495867;font-size:12px;margin:24px 0 0;">
            Or paste this link in your browser:<br/>
            <a href="{verification_url}" style="color:#00f0ff;word-break:break-all;">{verification_url}</a>
          </p>
        </td></tr>
        <tr><td style="padding-top:36px;border-top:1px solid rgba(0,240,255,0.08);margin-top:36px;">
          <p style="color:#495867;font-size:12px;margin:0;">
            If you didn't create a Kagua account, you can safely ignore this email.
          </p>
        </td></tr>
      </table>
    </td></tr>
  </table>
</body>
</html>"#,
        verification_url = verification_url
    );

    send_email(
        api_key,
        from_address,
        to_email,
        "Verify your Kagua account",
        &html,
    )
    .await
}

/// Send the password reset email.
pub async fn send_password_reset_email(
    api_key: &str,
    from_address: &str,
    to_email: &str,
    reset_url: &str,
) -> Result<(), String> {
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
  <title>Reset your Kagua password</title>
</head>
<body style="margin:0;padding:0;background:#070b10;font-family:'Inter',Arial,sans-serif;">
  <table width="100%" cellpadding="0" cellspacing="0" style="background:#070b10;padding:48px 16px;">
    <tr><td align="center">
      <table width="560" cellpadding="0" cellspacing="0" style="background:#0d1520;border:1px solid rgba(0,240,255,0.12);border-radius:12px;padding:48px 40px;">
        <tr><td align="center" style="padding-bottom:32px;">
          <h1 style="font-size:22px;font-weight:800;color:#00f0ff;letter-spacing:0.1em;margin:0;">KAGUA</h1>
          <p style="color:#788a9c;font-size:12px;letter-spacing:0.15em;margin:4px 0 0;">COMPLIANCE INTELLIGENCE ENGINE</p>
        </td></tr>
        <tr><td>
          <h2 style="color:#e2f1f8;font-size:20px;font-weight:600;margin:0 0 12px;">Reset your password</h2>
          <p style="color:#788a9c;font-size:15px;line-height:1.6;margin:0 0 28px;">
            We received a request to reset the password for your Kagua account. Click the button below to set a new password. This link expires in <strong style="color:#e2f1f8;">1 hour</strong>.
          </p>
          <a href="{reset_url}"
             style="display:inline-block;padding:14px 32px;background:linear-gradient(135deg,#ff2a70,#a0001f);color:#ffffff;font-weight:700;font-size:14px;letter-spacing:0.05em;text-decoration:none;border-radius:6px;">
            RESET PASSWORD
          </a>
          <p style="color:#495867;font-size:12px;margin:24px 0 0;">
            Or paste this link in your browser:<br/>
            <a href="{reset_url}" style="color:#ff2a70;word-break:break-all;">{reset_url}</a>
          </p>
        </td></tr>
        <tr><td style="padding-top:36px;border-top:1px solid rgba(0,240,255,0.08);margin-top:36px;">
          <p style="color:#495867;font-size:12px;margin:0;">
            If you didn't request a password reset, please ignore this email. Your password will remain unchanged.
          </p>
        </td></tr>
      </table>
    </td></tr>
  </table>
</body>
</html>"#,
        reset_url = reset_url
    );

    send_email(
        api_key,
        from_address,
        to_email,
        "Reset your Kagua password",
        &html,
    )
    .await
}

/// Send a security notification email after a successful password change.
pub async fn send_password_changed_email(
    api_key: &str,
    from_address: &str,
    to_email: &str,
) -> Result<(), String> {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
  <title>Your Kagua password was changed</title>
</head>
<body style="margin:0;padding:0;background:#070b10;font-family:'Inter',Arial,sans-serif;">
  <table width="100%" cellpadding="0" cellspacing="0" style="background:#070b10;padding:48px 16px;">
    <tr><td align="center">
      <table width="560" cellpadding="0" cellspacing="0" style="background:#0d1520;border:1px solid rgba(0,240,255,0.12);border-radius:12px;padding:48px 40px;">
        <tr><td align="center" style="padding-bottom:32px;">
          <h1 style="font-size:22px;font-weight:800;color:#00f0ff;letter-spacing:0.1em;margin:0;">KAGUA</h1>
          <p style="color:#788a9c;font-size:12px;letter-spacing:0.15em;margin:4px 0 0;">COMPLIANCE INTELLIGENCE ENGINE</p>
        </td></tr>
        <tr><td>
          <h2 style="color:#e2f1f8;font-size:20px;font-weight:600;margin:0 0 12px;">Your password has been changed</h2>
          <p style="color:#788a9c;font-size:15px;line-height:1.6;margin:0 0 0;">
            Your Kagua account password was successfully changed. If you made this change, no further action is needed.
          </p>
          <p style="color:#ff2a70;font-size:14px;font-weight:600;margin:20px 0 0;">
            ⚠ If you did NOT make this change, please contact us immediately and reset your password.
          </p>
        </td></tr>
        <tr><td style="padding-top:36px;border-top:1px solid rgba(0,240,255,0.08);margin-top:36px;">
          <p style="color:#495867;font-size:12px;margin:0;">This is an automated security notification from Kagua.</p>
        </td></tr>
      </table>
    </td></tr>
  </table>
</body>
</html>"#;

    send_email(
        api_key,
        from_address,
        to_email,
        "Your Kagua password was changed",
        html,
    )
    .await
}
