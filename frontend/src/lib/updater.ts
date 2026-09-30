const GITHUB_OWNER = 'AbhiLohar';
const GITHUB_REPO = 'Campus_Connect';
const CURRENT_VERSION = '1.0.0';

export interface UpdateInfo {
  available: boolean;
  latestVersion: string;
  currentVersion: string;
  downloadUrl: string | null;
  releaseNotes: string | null;
  releaseName: string | null;
}

export async function checkForUpdate(): Promise<UpdateInfo> {
  const result: UpdateInfo = {
    available: false,
    latestVersion: CURRENT_VERSION,
    currentVersion: CURRENT_VERSION,
    downloadUrl: null,
    releaseNotes: null,
    releaseName: null,
  };

  try {
    const res = await fetch(
      `https://api.github.com/repos/${GITHUB_OWNER}/${GITHUB_REPO}/releases/latest`,
      {
        headers: { Accept: 'application/vnd.github.v3+json' },
        cache: 'no-store',
      }
    );

    if (!res.ok) return result;

    const release = await res.json();
    const latestTag = (release.tag_name || '').replace(/^v/, '');

    result.latestVersion = latestTag;
    result.releaseName = release.name || `v${latestTag}`;
    result.releaseNotes = release.body || null;

    // Find the APK asset
    const apkAsset = release.assets?.find(
      (a: { name: string }) => a.name.endsWith('.apk')
    );
    result.downloadUrl = apkAsset?.browser_download_url || release.html_url;

    // Compare versions: simple semantic comparison
    result.available = isNewerVersion(latestTag, CURRENT_VERSION);

    return result;
  } catch (err) {
    console.warn('Update check failed:', err);
    return result;
  }
}

function isNewerVersion(latest: string, current: string): boolean {
  const latestParts = latest.split('.').map(Number);
  const currentParts = current.split('.').map(Number);

  for (let i = 0; i < Math.max(latestParts.length, currentParts.length); i++) {
    const l = latestParts[i] || 0;
    const c = currentParts[i] || 0;
    if (l > c) return true;
    if (l < c) return false;
  }
  return false;
}

export function getAppVersion(): string {
  return CURRENT_VERSION;
}
