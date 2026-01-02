// Extract tokens when popup opens
document.addEventListener('DOMContentLoaded', () => {
  extractTokens();

  // Setup copy buttons
  document.getElementById('copy-xoxc').addEventListener('click', () => {
    copyToClipboard('xoxc-token', 'copy-xoxc');
  });

  document.getElementById('copy-xoxd').addEventListener('click', () => {
    copyToClipboard('xoxd-cookie', 'copy-xoxd');
  });

  // Setup refresh button
  document.getElementById('refresh').addEventListener('click', () => {
    extractTokens();
  });
});

async function extractTokens() {
  try {
    // Get the active tab
    const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });

    // Check if we're on Slack
    if (!tab.url || !tab.url.includes('slack.com')) {
      showStatus('error', '⚠️ Please open this extension while on slack.com');
      return;
    }

    // Extract xoxd cookie
    const cookies = await chrome.cookies.getAll({ domain: '.slack.com' });
    const xoxdCookie = cookies.find(c => c.name === 'd');

    if (!xoxdCookie) {
      showStatus('error', '❌ Could not find xoxd cookie. Make sure you are logged in to Slack.');
      return;
    }

    // Inject script to extract xoxc token from localStorage or page
    const results = await chrome.scripting.executeScript({
      target: { tabId: tab.id },
      func: extractXoxcToken
    });

    const result = results[0]?.result;

    if (!result) {
      showStatus('warning', '⚠️ Found xoxd cookie but could not find xoxc token. Try refreshing the Slack page and opening the extension again.');
      document.getElementById('xoxd-cookie').value = xoxdCookie.value;
      showContent();
      return;
    }

    // Display xoxd cookie
    document.getElementById('xoxd-cookie').value = xoxdCookie.value;

    // Handle multiple teams
    if (result.teams) {
      // Multiple teams found - show selector
      showStatus('warning', `⚠️ Found ${result.teams.length} workspaces. Select one:`);

      const select = document.createElement('select');
      select.id = 'team-select';
      select.style.cssText = 'width: 100%; padding: 8px; margin-bottom: 12px; border: 1px solid #ccc; border-radius: 4px;';

      result.teams.forEach((team, index) => {
        const option = document.createElement('option');
        option.value = team.token;
        option.textContent = `${team.name} (${team.domain}.slack.com)`;
        select.appendChild(option);
      });

      select.addEventListener('change', () => {
        document.getElementById('xoxc-token').value = select.value;
      });

      // Insert select before the token group
      const tokenGroup = document.querySelector('.token-group');
      tokenGroup.parentNode.insertBefore(select, tokenGroup);

      // Set first token as default
      document.getElementById('xoxc-token').value = result.teams[0].token;
      showContent();
      return;
    }

    // Single token found
    document.getElementById('xoxc-token').value = result.token;
    showStatus('success', `✅ Token extracted for: ${result.team || 'Unknown workspace'}`);
    showContent();

  } catch (error) {
    console.error('Error extracting tokens:', error);
    showStatus('error', `❌ Error: ${error.message}`);
    showContent();
  }
}

// This function runs in the context of the Slack page
function extractXoxcToken() {
  try {
    // Method 1: Check for token in global variables (most reliable for current workspace)
    if (window.boot_data?.api_token) {
      return { token: window.boot_data.api_token, team: window.boot_data.team_name };
    }

    // Method 2: Try to find it in the page's Redux store
    if (window.TS?.boot_data?.api_token) {
      return { token: window.TS.boot_data.api_token, team: window.TS.boot_data.team_name };
    }

    // Method 3: For app.slack.com, try to get team ID from URL path
    // URL format: https://app.slack.com/client/TEAM_ID/...
    const urlMatch = window.location.pathname.match(/\/client\/([A-Z0-9]+)/);
    const urlTeamId = urlMatch ? urlMatch[1] : null;

    // Method 4: Check localStorage
    const localConfig = localStorage.getItem('localConfig_v2');
    if (localConfig) {
      const config = JSON.parse(localConfig);
      const teams = [];

      // Collect all teams
      for (const key in config.teams) {
        const team = config.teams[key];
        if (team.token) {
          teams.push({
            id: key,
            name: team.name,
            domain: team.domain,
            token: team.token,
            enterprise_id: team.enterprise_id
          });
        }
      }

      // If we have a team ID from URL, try to match it
      if (urlTeamId) {
        // Try matching team_id or enterprise_id
        const match = teams.find(t => t.id === urlTeamId || t.enterprise_id === urlTeamId);
        if (match) {
          return { token: match.token, team: match.name };
        }
      }

      // Return all teams so user can choose
      if (teams.length > 0) {
        return { teams: teams };
      }
    }

    // Method 5: Check for Redux DevTools state
    if (window.__REDUX_DEVTOOLS_EXTENSION__) {
      const state = window.__REDUX_DEVTOOLS_EXTENSION__.latestState;
      if (state?.user?.token) {
        return { token: state.user.token, team: 'Unknown' };
      }
    }

    return null;
  } catch (e) {
    console.error('Error extracting xoxc token:', e);
    return null;
  }
}

function showStatus(type, message) {
  const statusEl = document.getElementById('status');
  statusEl.className = `status ${type}`;
  statusEl.textContent = message;
}

function showContent() {
  document.getElementById('loading').style.display = 'none';
  document.getElementById('content').style.display = 'block';
}

async function copyToClipboard(inputId, buttonId) {
  const input = document.getElementById(inputId);
  const button = document.getElementById(buttonId);

  try {
    await navigator.clipboard.writeText(input.value);

    // Visual feedback
    const originalText = button.textContent;
    button.textContent = '✓ Copied!';
    button.classList.add('copied');

    setTimeout(() => {
      button.textContent = originalText;
      button.classList.remove('copied');
    }, 2000);
  } catch (error) {
    console.error('Failed to copy:', error);
    alert('Failed to copy to clipboard');
  }
}
