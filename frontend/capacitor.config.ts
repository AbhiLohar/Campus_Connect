import type { CapacitorConfig } from '@capacitor/cli';

const config: CapacitorConfig = {
  appId: 'com.digitalcampus.app',
  appName: 'Digital Campus',
  webDir: 'out',
  server: {
    // For local testing: point to your PC's LAN IP (both phone & PC on same WiFi)
    // To find your IP: run `ipconfig` in terminal and use the IPv4 address
    // For production: change to your deployed URL or remove `url` entirely
    url: 'http://10.177.170.107:3000',
    cleartext: true,
  },
  plugins: {
    SplashScreen: {
      launchAutoHide: true,
      launchShowDuration: 1500,
      backgroundColor: '#000000',
      showSpinner: false,
    },
  },
};

export default config;
