'use client';

import { useState } from 'react';
import { useParams } from 'next/navigation';
import PostFeed from '@/components/posts/PostFeed';
import { Users, FileText, Info, Plus } from 'lucide-react';
import { CommunityDetail } from '@/types';

export default function CommunityPageClient() {
  const params = useParams();
  const slug = params.slug as string;
  const [activeTab, setActiveTab] = useState<'posts' | 'about' | 'members'>('posts');
  const [isJoined, setIsJoined] = useState(false);

  // Mock data for UI structure
  const community: CommunityDetail = {
    publicId: 'c1',
    name: 'Computer Science',
    slug: slug,
    description: 'The official community for CS students. Discuss courses, projects, and career advice.',
    iconUrl: null,
    bannerUrl: null,
    communityType: 'academic',
    visibility: 'public',
    joinPolicy: 'open',
    memberCount: 1240,
    isOfficial: true,
    rules: [
      { ruleNumber: 1, title: 'Be respectful', description: 'No harassment or bullying.' },
      { ruleNumber: 2, title: 'No cheating', description: 'Do not share assignment answers.' }
    ],
    myRole: null
  };

  return (
    <div className="max-w-4xl mx-auto space-y-4 md:space-y-6 md:p-6 pb-20 md:pb-6">
      {/* Banner & Header */}
      <div className="bg-white md:rounded-xl shadow-sm border-x border-b md:border-t border-slate-200 overflow-hidden">
        <div className="h-32 md:h-48 bg-gradient-to-r from-blue-500 to-purple-600 relative">
          <div className="absolute inset-0 bg-black/20" />
        </div>
        
        <div className="px-4 sm:px-6 pb-6 pt-4 relative flex flex-col md:flex-row md:items-end justify-between gap-4">
          <div className="flex-1">
            <h1 className="text-2xl md:text-3xl font-bold text-slate-900 flex items-center gap-2">
              {community.name}
              {community.isOfficial && (
                <span className="text-xs bg-blue-100 text-blue-700 px-2 py-0.5 rounded-full font-medium align-middle">
                  Official
                </span>
              )}
            </h1>
            <p className="text-slate-600 mt-1 max-w-2xl">{community.description}</p>
            <div className="flex items-center gap-4 mt-3 text-sm text-slate-500">
              <span className="font-medium text-slate-900">{community.memberCount.toLocaleString()}</span> members
              <span className="capitalize">{community.visibility} • {community.communityType}</span>
            </div>
          </div>
          
          <button 
            onClick={() => setIsJoined(!isJoined)}
            className={`px-6 py-2 rounded-full font-medium transition-colors md:w-auto w-full ${
              isJoined 
                ? 'border border-slate-300 text-slate-700 hover:bg-slate-50' 
                : 'bg-blue-600 text-white hover:bg-blue-700'
            }`}
          >
            {isJoined ? 'Joined' : 'Join Community'}
          </button>
        </div>

        {/* Tabs */}
        <div className="flex border-t border-slate-200 px-4 sm:px-6">
          {[
            { id: 'posts', label: 'Posts', icon: FileText },
            { id: 'about', label: 'About', icon: Info },
            { id: 'members', label: 'Members', icon: Users },
          ].map(tab => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id as any)}
              className={`flex items-center gap-2 py-4 px-4 text-sm font-medium border-b-2 transition-colors ${
                activeTab === tab.id 
                  ? 'border-blue-600 text-blue-600' 
                  : 'border-transparent text-slate-600 hover:text-slate-900'
              }`}
            >
              <tab.icon className="w-4 h-4 hidden sm:block" />
              {tab.label}
            </button>
          ))}
        </div>
      </div>

      {/* Content Area */}
      <div className="px-4 md:px-0">
        {activeTab === 'posts' && (
          <PostFeed communitySlug={slug} />
        )}

        {activeTab === 'about' && (
          <div className="bg-white rounded-xl shadow-sm border border-slate-200 p-6 space-y-6">
            <div>
              <h2 className="text-lg font-semibold text-slate-900 mb-2">About</h2>
              <p className="text-slate-600">{community.description}</p>
            </div>
            
            <div>
              <h2 className="text-lg font-semibold text-slate-900 mb-4">Rules</h2>
              <div className="space-y-4">
                {community.rules.map(rule => (
                  <div key={rule.ruleNumber} className="border-l-2 border-blue-500 pl-4 py-1">
                    <h3 className="font-medium text-slate-900">
                      {rule.ruleNumber}. {rule.title}
                    </h3>
                    {rule.description && (
                      <p className="text-sm text-slate-600 mt-1">{rule.description}</p>
                    )}
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}

        {activeTab === 'members' && (
          <div className="bg-white rounded-xl shadow-sm border border-slate-200 p-6">
            <h2 className="text-lg font-semibold text-slate-900 mb-4">Members ({community.memberCount})</h2>
            <div className="text-slate-500 text-center py-8">
              List of members would appear here.
            </div>
          </div>
        )}
      </div>

      {/* Floating Action Button (Mobile) */}
      <button className="md:hidden fixed bottom-20 right-4 w-14 h-14 bg-blue-600 text-white rounded-full shadow-lg flex items-center justify-center hover:bg-blue-700 transition-colors z-10">
        <Plus className="w-6 h-6" />
      </button>
    </div>
  );
}
