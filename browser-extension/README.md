# Slack Token Extractor - Chrome Extension

A Chrome extension to easily extract Slack authentication tokens (xoxc and xoxd) for use with the Slack SDK.

## Features

- 🔐 Automatically extracts `SLACK_XOXC_TOKEN` and `SLACK_XOXD_COOKIE`
- 📋 One-click copy to clipboard
- ✅ Visual confirmation of successful extraction
- 🔄 Refresh tokens without reopening the extension

## Installation

### Method 1: Load Unpacked Extension (Development)

1. Open Chrome and navigate to `chrome://extensions/`
2. Enable "Developer mode" (toggle in top-right corner)
3. Click "Load unpacked"
4. Select the `browser-extension` directory from this repository

### Method 2: Create Icons (Optional)

The extension needs icon files. You can either:

**Option A: Use placeholder icons**
Create simple PNG files named `icon16.png`, `icon48.png`, and `icon128.png` in the extension directory.

**Option B: Convert the SVG**
If you have ImageMagick installed:

```bash
cd browser-extension
convert -background none -resize 16x16 icon.svg icon16.png
convert -background none -resize 48x48 icon.svg icon48.png
convert -background none -resize 128x128 icon.svg icon128.png
```

**Option C: Use online converter**
1. Go to https://cloudconvert.com/svg-to-png
2. Upload `icon.svg`
3. Convert to 16x16, 48x48, and 128x128 PNG files
4. Save as `icon16.png`, `icon48.png`, `icon128.png`

## Usage

1. **Open Slack in your browser** and log in to your workspace
2. **Click the extension icon** in your Chrome toolbar
3. **Copy the tokens** using the copy buttons
4. **Add to your `.env` file**:

```bash
SLACK_XOXC_TOKEN=xoxc-1234567890-...
SLACK_XOXD_COOKIE=xoxd-1234567890...
```

5. **Use with the Slack SDK**:

```rust
use slack_sdk::{SlackClient, AuthConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SlackClient::new(AuthConfig::from_env()?)?;

    // Your code here
    Ok(())
}
```

## How It Works

The extension:

1. **Extracts xoxd cookie**: Reads the `d` cookie from Slack's domain
2. **Extracts xoxc token**: Searches multiple locations:
   - `localStorage.localConfig_v2` (primary method)
   - `window.boot_data.api_token`
   - `window.TS.boot_data.api_token`
   - Redux DevTools state (if available)

## Troubleshooting

### ⚠️ "Could not find xoxc token"

**Solutions:**
1. Refresh the Slack page
2. Navigate to your workspace's main page
3. Open DevTools (F12) and go to Application → Local Storage → Check for `localConfig_v2`
4. If still not working, try the manual method (see Main README)

### ⚠️ "Please open this extension while on slack.com"

The extension only works when you're viewing a Slack page. Make sure:
- You're on `https://*.slack.com/`
- The page has fully loaded
- You're logged in

### ❌ "Could not find xoxd cookie"

This means you're not logged in to Slack. Log in to your workspace first.

## Security Considerations

⚠️ **Important Security Notes:**

1. **Tokens are sensitive**: These tokens have full user permissions in your Slack workspace
2. **Never share tokens**: Don't commit them to git or share publicly
3. **Store securely**: Keep tokens in `.env` files (add to `.gitignore`)
4. **Rotate regularly**: Tokens can be revoked by logging out of Slack
5. **Personal use only**: Only use on your own Slack workspaces
6. **Extension permissions**: The extension only requests necessary permissions:
   - `cookies`: To read the xoxd cookie
   - `activeTab`: To inject scripts on the current Slack tab
   - `host_permissions`: Limited to `*.slack.com`

## Privacy

- ✅ No data is sent to external servers
- ✅ Tokens are only displayed in the popup
- ✅ No tracking or analytics
- ✅ Open source - you can audit the code

## Manual Method (Alternative)

If you prefer not to use the extension, you can extract tokens manually:

1. Open Slack in your browser and log in
2. Open Developer Tools (F12)
3. Go to **Application** → **Cookies** → `https://[workspace].slack.com`
4. Find cookie `d` → Copy the value (this is `SLACK_XOXD_COOKIE`)
5. Go to **Network** tab, filter by XHR
6. Refresh the page
7. Click on any API request
8. Look for `Authorization: Bearer xoxc-...` header
9. Copy the `xoxc-...` token (this is `SLACK_XOXC_TOKEN`)

## Files

- `manifest.json` - Extension configuration
- `popup.html` - User interface
- `popup.js` - Token extraction and UI logic
- `content.js` - Content script for monitoring tokens
- `icon.svg` - Source icon file
- `icon*.png` - Extension icons (need to be generated)

## Development

To modify the extension:

1. Edit the files in `browser-extension/`
2. Go to `chrome://extensions/`
3. Click the refresh icon on the extension card
4. Test your changes

## License

Same as the main Slack SDK project (MIT License).

## Support

For issues or questions:
- Check the main README for authentication documentation
- Open an issue in the GitHub repository
- Verify you're using the latest version of Chrome
