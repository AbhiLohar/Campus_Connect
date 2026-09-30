'use client';

import { useEffect, useState } from 'react';
import { Download, X } from 'lucide-react';
import { checkForUpdate, UpdateInfo } from '@/lib/updater';

export default function UpdateBanner() {
  const [update, setUpdate] = useState<UpdateInfo | null>(null);
  const [dismissed, setDismissed] = useState(false);

  useEffect(() => {
    // Check for updates 3 seconds after launch (don't block UI)
    const timer = setTimeout(async () => {
      try {
        const info = await checkForUpdate();
        if (info.available) {
          setUpdate(info);
        }
      } catch {
        // Silently fail — update check is non-critical
      }
    }, 3000);

    return () => clearTimeout(timer);
  }, []);

  if (!update || dismissed) return null;

  return (
    <div className="bg-blue-600 text-white px-4 py-3 flex items-center justify-between gap-3 animate-in slide-in-from-top">
      <div className="flex items-center gap-3 min-w-0">
        <Download className="w-5 h-5 flex-shrink-0" />
        <div className="min-w-0">
          <p className="text-sm font-medium">
            Update available — v{update.latestVersion}
          </p>
          {update.releaseName && (
            <p className="text-xs text-blue-100 truncate">
              {update.releaseName}
            </p>
          )}
        </div>
      </div>
      <div className="flex items-center gap-2 flex-shrink-0">
        <a
          href={update.downloadUrl || '#'}
          target="_blank"
          rel="noopener noreferrer"
          className="px-3 py-1.5 bg-white text-blue-600 text-sm font-semibold rounded-lg hover:bg-blue-50 transition-colors"
        >
          Update
        </a>
        <button
          onClick={() => setDismissed(true)}
          className="p-1 hover:bg-blue-500 rounded transition-colors"
          aria-label="Dismiss"
        >
          <X className="w-4 h-4" />
        </button>
      </div>
    </div>
  );
}
