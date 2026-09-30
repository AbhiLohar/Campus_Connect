'use client';

import { useState } from 'react';
import { useAuth } from '@/hooks/useAuth';
import { useRouter } from 'next/navigation';
import { useForm } from 'react-hook-form';

type SettingsFormData = {
  displayName: string;
  bio: string;
  isPublicDefault: boolean;
  showCollege: boolean;
  allowMessages: boolean;
};

export default function SettingsPage() {
  const { user } = useAuth();
  const router = useRouter();
  const [isSaving, setIsSaving] = useState(false);
  
  const { register, handleSubmit } = useForm<SettingsFormData>({
    defaultValues: {
      displayName: user?.displayName || '',
      bio: user?.bio || '',
      isPublicDefault: true,
      showCollege: true,
      allowMessages: true,
    }
  });

  const onSubmit = async (data: SettingsFormData) => {
    setIsSaving(true);
    // TODO: Connect to API
    setTimeout(() => {
      setIsSaving(false);
    }, 1000);
  };

  if (!user) return <div className="p-4">Please log in.</div>;

  return (
    <div className="max-w-2xl mx-auto p-4 md:p-6 space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold text-slate-900">Settings</h1>
        <button
          onClick={handleSubmit(onSubmit)}
          disabled={isSaving}
          className="bg-blue-600 text-white px-4 py-2 rounded-lg font-medium hover:bg-blue-700 disabled:opacity-50"
        >
          {isSaving ? 'Saving...' : 'Save Changes'}
        </button>
      </div>

      <div className="bg-white rounded-xl shadow-sm border border-slate-200 overflow-hidden">
        {/* Profile Section */}
        <div className="p-6 border-b border-slate-200 space-y-4">
          <h2 className="text-lg font-semibold text-slate-900">Profile</h2>
          
          <div className="space-y-1">
            <label className="text-sm font-medium text-slate-700">Display Name</label>
            <input 
              {...register('displayName')}
              className="w-full border border-slate-300 rounded-lg p-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
          
          <div className="space-y-1">
            <label className="text-sm font-medium text-slate-700">Bio</label>
            <textarea 
              {...register('bio')}
              rows={3}
              className="w-full border border-slate-300 rounded-lg p-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
        </div>

        {/* Privacy Section */}
        <div className="p-6 border-b border-slate-200 space-y-4">
          <h2 className="text-lg font-semibold text-slate-900">Privacy</h2>
          
          <label className="flex items-center justify-between cursor-pointer">
            <div>
              <div className="font-medium text-slate-900">Default Posting Identity</div>
              <div className="text-sm text-slate-500">Post publicly by default</div>
            </div>
            <input type="checkbox" {...register('isPublicDefault')} className="w-5 h-5 accent-blue-600" />
          </label>
          
          <label className="flex items-center justify-between cursor-pointer">
            <div>
              <div className="font-medium text-slate-900">Show College</div>
              <div className="text-sm text-slate-500">Display college affiliation on profile</div>
            </div>
            <input type="checkbox" {...register('showCollege')} className="w-5 h-5 accent-blue-600" />
          </label>

          <label className="flex items-center justify-between cursor-pointer">
            <div>
              <div className="font-medium text-slate-900">Allow Messages</div>
              <div className="text-sm text-slate-500">Let other users send you direct messages</div>
            </div>
            <input type="checkbox" {...register('allowMessages')} className="w-5 h-5 accent-blue-600" />
          </label>
        </div>

        {/* Account Section */}
        <div className="p-6 space-y-6">
          <h2 className="text-lg font-semibold text-slate-900">Account</h2>
          
          <div className="space-y-4 border border-slate-200 rounded-lg p-4">
            <h3 className="font-medium text-slate-900">Change Password</h3>
            <div className="space-y-1">
              <label className="text-sm text-slate-700">Email</label>
              <input type="email" className="w-full border border-slate-300 rounded-lg p-2" placeholder="Your email address" />
            </div>
            <div className="space-y-1">
              <label className="text-sm text-slate-700">New Password</label>
              <input type="password" className="w-full border border-slate-300 rounded-lg p-2" />
            </div>
            <button className="bg-slate-900 text-white px-4 py-2 rounded-lg font-medium hover:bg-slate-800">
              Update Password
            </button>
          </div>

          <div className="pt-4">
            <button className="text-red-600 font-medium hover:text-red-700 flex items-center gap-2">
              Logout
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
