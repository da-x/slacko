// Content script for Slack Token Extractor
// This script runs in the context of Slack pages

// Store token in session storage when detected
(function() {
  'use strict';

  // Monitor for token changes
  const observer = new MutationObserver(() => {
    try {
      // Try to extract token from various sources
      const localConfig = localStorage.getItem('localConfig_v2');
      if (localConfig) {
        const config = JSON.parse(localConfig);
        for (const key in config.teams) {
          if (config.teams[key].token) {
            sessionStorage.setItem('slack_xoxc_token', config.teams[key].token);
            break;
          }
        }
      }
    } catch (e) {
      // Silently fail
    }
  });

  // Start observing
  if (document.body) {
    observer.observe(document.body, {
      childList: true,
      subtree: true
    });
  }
})();
