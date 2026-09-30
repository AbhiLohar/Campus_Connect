'use client';

import { useAuth } from '@/hooks/useAuth';

export type IdentityType = 'public' | 'anonymous';

interface IdentitySelectorProps {
  value: IdentityType;
  onChange: (value: IdentityType) => void;
}

export default function IdentitySelector({ value, onChange }: IdentitySelectorProps) {
  const { user } = useAuth();

  return (
    <div className="space-y-3">
      <label className="text-sm font-medium text-gray-700">Posting as</label>
      <div className="grid grid-cols-2 gap-3">
        <label
          className={`flex items-start p-3 border rounded-xl cursor-pointer transition-colors ${
            value === 'public'
              ? 'border-black bg-gray-50'
              : 'border-gray-200 hover:bg-gray-50'
          }`}
        >
          <div className="flex items-center h-5">
            <input
              type="radio"
              className="w-4 h-4 text-black border-gray-300 focus:ring-black"
              checked={value === 'public'}
              onChange={() => onChange('public')}
            />
          </div>
          <div className="ml-3">
            <span className="block text-sm font-medium text-gray-900">
              {user?.displayName || 'Public Identity'}
            </span>
            <span className="block text-xs text-green-600 mt-0.5">
              ● Verified Student
            </span>
          </div>
        </label>

        <label
          className={`flex items-start p-3 border rounded-xl cursor-pointer transition-colors ${
            value === 'anonymous'
              ? 'border-black bg-gray-50'
              : 'border-gray-200 hover:bg-gray-50'
          }`}
        >
          <div className="flex items-center h-5">
            <input
              type="radio"
              className="w-4 h-4 text-black border-gray-300 focus:ring-black"
              checked={value === 'anonymous'}
              onChange={() => onChange('anonymous')}
            />
          </div>
          <div className="ml-3">
            <span className="block text-sm font-medium text-gray-900">
              Anonymous
            </span>
            <span className="block text-xs text-gray-500 mt-0.5">
              Identity protected
            </span>
          </div>
        </label>
      </div>
    </div>
  );
}
