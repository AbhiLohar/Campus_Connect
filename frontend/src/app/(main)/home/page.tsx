'use client';

import { useState } from 'react';
import EmptyState from '@/components/common/EmptyState';
import { MessageSquare } from 'lucide-react';

const tabs = ['For You', 'Following', 'My College', 'Global', 'Trending'];

export default function HomePage() {
  const [activeTab, setActiveTab] = useState('For You');

  return (
    <div className="flex flex-col h-full">
      <div className="sticky top-0 z-20 bg-white/80 backdrop-blur-md border-b border-gray-200">
        <div className="px-4 py-3 hidden md:block">
          <h1 className="text-xl font-bold">Home</h1>
        </div>
        
        <div className="flex overflow-x-auto hide-scrollbar border-t border-gray-100 md:border-t-0">
          {tabs.map((tab) => (
            <button
              key={tab}
              onClick={() => setActiveTab(tab)}
              className={`whitespace-nowrap px-4 py-3 text-sm font-medium border-b-2 transition-colors ${
                activeTab === tab
                  ? 'border-black text-black'
                  : 'border-transparent text-gray-500 hover:text-gray-900 hover:border-gray-300'
              }`}
            >
              {tab}
            </button>
          ))}
        </div>
      </div>

      <div className="flex-1 p-4">
        <div className="max-w-2xl mx-auto mt-8">
          <EmptyState
            icon={<MessageSquare className="w-12 h-12 text-gray-300" />}
            title="Welcome to Digital Campus!"
            description={`Join communities to see posts in your ${activeTab} feed.`}
            actionLabel="Discover Communities"
            onAction={() => {}}
          />
        </div>
      </div>
    </div>
  );
}
