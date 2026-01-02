// Run this in the browser console on your Slack workspace to debug token extraction

console.log('=== Slack Token Debug ===');
console.log('Current URL:', window.location.href);
console.log('Current domain:', window.location.hostname.split('.')[0]);

console.log('\n--- Method 1: window.boot_data ---');
console.log('boot_data exists:', !!window.boot_data);
console.log('boot_data.api_token:', window.boot_data?.api_token?.substring(0, 30) + '...');
console.log('boot_data.team_id:', window.boot_data?.team_id);
console.log('boot_data.team_name:', window.boot_data?.team_name);

console.log('\n--- Method 2: window.TS ---');
console.log('TS exists:', !!window.TS);
console.log('TS.boot_data.api_token:', window.TS?.boot_data?.api_token?.substring(0, 30) + '...');
console.log('TS.boot_data.team_id:', window.TS?.boot_data?.team_id);

console.log('\n--- Method 3: localStorage ---');
const localConfig = localStorage.getItem('localConfig_v2');
if (localConfig) {
  const config = JSON.parse(localConfig);
  console.log('Teams in localStorage:');
  for (const key in config.teams) {
    const team = config.teams[key];
    console.log(`  - ${team.name || key}: domain=${team.domain}, token=${team.token?.substring(0, 30)}...`);
  }
} else {
  console.log('No localConfig_v2 found');
}

console.log('\n=== End Debug ===');
