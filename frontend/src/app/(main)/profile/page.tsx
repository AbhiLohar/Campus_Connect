'use client';

import { useState } from 'react';
import { useAuth } from '@/hooks/useAuth';
import Link from 'next/link';
import { CheckCircle, Settings, FileText, MessageSquare, Bookmark } from 'lucide-react';

export default function ProfilePage() {
  const { user } = useAuth();
  const [activeTab, setActiveTab] = useState<'posts' | 'comments' | 'saved'>('posts');

  if (!user) {
    return <div className="p-4">Please log in to view your profile.</div>;
  }

  const initials = user.displayName ? user.displayName.substring(0, 2).toUpperCase() : 'U';

  return (
    <div className="max-w-4xl mx-auto p-4 md:p-6 space-y-6">
      {/* Profile Header */}
      <div className="bg-white rounded-xl shadow-sm border border-slate-200 p-6">
        <div className="flex flex-col md:flex-row items-center md:items-start gap-6">
          <div className="h-24 w-24 rounded-full bg-blue-600 text-white flex items-center justify-center text-3xl font-bold">
            {initials}
          </div>
          <div className="flex-1 text-center md:text-left space-y-2">
            <h1 className="text-2xl font-bold text-slate-900">{user.displayName}</h1>
            <p className="text-slate-600 max-w-md">{user.bio || 'No bio provided yet.'}</p>
            
            <div className="flex items-center justify-center md:justify-start gap-2 text-sm text-blue-600 bg-blue-50 px-3 py-1 rounded-full w-fit mx-auto md:mx-0">
              <CheckCircle className="w-4 h-4" />
              <span>Verified Student</span>
            </div>
            
            <div className="flex items-center justify-center md:justify-start gap-4 pt-4 text-sm text-slate-600">
              <div className="flex flex-col items-center md:items-start">
                <span className="font-bold text-slate-900">0</span>
                <span>Posts</span>
              </div>
              <div className="flex flex-col items-center md:items-start">
                <span className="font-bold text-slate-900">0</span>
                <span>Communities</span>
              </div>
              <div className="flex flex-col items-center md:items-start">
                <span className="font-bold text-slate-900">0</span>
                <span>Reputation</span>
              </div>
            </div>
          </div>
          
          <Link 
            href="/profile/settings"
            className="flex items-center gap-2 px-4 py-2 border border-slate-200 rounded-lg hover:bg-slate-50 transition-colors"
          >
            <Settings className="w-4 h-4" />
            <span>Edit Profile</span>
          </Link>
        </div>
      </div>

      {/* Tabs */}
      <div className="bg-white rounded-xl shadow-sm border border-slate-200">
        <div className="flex border-b border-slate-200">
          <button
            onClick={() => setActiveTab('posts')}
            className={`flex-1 flex items-center justify-center gap-2 py-4 text-sm font-medium border-b-2 transition-colors ${
              activeTab === 'posts' ? 'border-blue-600 text-blue-600' : 'border-transparent text-slate-600 hover:text-slate-900'
            }`}
          >
            <FileText className="w-4 h-4" />
            My Posts
          </button>
          <button
            onClick={() => setActiveTab('comments')}
            className={`flex-1 flex items-center justify-center gap-2 py-4 text-sm font-medium border-b-2 transition-colors ${
              activeTab === 'comments' ? 'border-blue-600 text-blue-600' : 'border-transparent text-slate-600 hover:text-slate-900'
            }`}
          >
            <MessageSquare className="w-4 h-4" />
            My Comments
          </button>
          <button
            onClick={() => setActiveTab('saved')}
            className={`flex-1 flex items-center justify-center gap-2 py-4 text-sm font-medium border-b-2 transition-colors ${
              activeTab === 'saved' ? 'border-blue-600 text-blue-600' : 'border-transparent text-slate-600 hover:text-slate-900'
            }`}
          >
            <Bookmark className="w-4 h-4" />
            Saved
          </button>
        </div>
        
        <div className="p-8 text-center text-slate-500">
          <p>No items to display yet.</p>
        </div>
      </div>
    </div>
  );
}
