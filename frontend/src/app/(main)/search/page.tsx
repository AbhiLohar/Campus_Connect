'use client';

import { useState, useRef, useEffect } from 'react';
import { Search as SearchIcon, Users, FileText, Calendar, Hash } from 'lucide-react';

type Tab = 'posts' | 'communities' | 'users' | 'events';

export default function SearchPage() {
  const [query, setQuery] = useState('');
  const [activeTab, setActiveTab] = useState<Tab>('posts');
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    inputRef.current?.focus();
  }, []);

  const tabs = [
    { id: 'posts', label: 'Posts', icon: FileText },
    { id: 'communities', label: 'Communities', icon: Hash },
    { id: 'users', label: 'People', icon: Users },
    { id: 'events', label: 'Events', icon: Calendar },
  ] as const;

  return (
    <div className="max-w-4xl mx-auto p-4 md:p-6 space-y-6">
      {/* Search Bar */}
      <div className="relative">
        <SearchIcon className="absolute left-4 top-3.5 h-5 w-5 text-slate-400" />
        <input 
          ref={inputRef}
          type="text" 
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Search Digital Campus..." 
          className="w-full pl-12 pr-4 py-3 bg-white border border-slate-300 rounded-xl focus:border-blue-500 focus:ring-2 focus:ring-blue-200 text-lg shadow-sm transition-shadow"
        />
      </div>

      {/* Tabs */}
      <div className="bg-white rounded-xl shadow-sm border border-slate-200 overflow-hidden">
        <div className="flex overflow-x-auto border-b border-slate-200 hide-scrollbar">
          {tabs.map(tab => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className={`flex-1 flex items-center justify-center gap-2 py-4 px-6 text-sm font-medium border-b-2 transition-colors whitespace-nowrap ${
                activeTab === tab.id 
                  ? 'border-blue-600 text-blue-600 bg-blue-50/50' 
                  : 'border-transparent text-slate-600 hover:text-slate-900 hover:bg-slate-50'
              }`}
            >
              <tab.icon className="w-4 h-4" />
              {tab.label}
            </button>
          ))}
        </div>

        {/* Results Area */}
        <div className="p-8 min-h-[400px]">
          {!query ? (
            <div className="text-center text-slate-500 flex flex-col items-center justify-center h-full pt-12">
              <SearchIcon className="w-12 h-12 text-slate-300 mb-4" />
              <h3 className="text-lg font-medium text-slate-900 mb-2">Search Digital Campus</h3>
              <p>Type to search for posts, communities, people, or events.</p>
              
              <div className="mt-8 flex flex-wrap gap-2 justify-center max-w-lg">
                <span className="text-sm font-medium text-slate-500 w-full mb-2">Suggested Searches</span>
                {['cs101', 'housing', 'textbooks', 'study group', 'freshman'].map(term => (
                  <button 
                    key={term}
                    onClick={() => setQuery(term)}
                    className="px-4 py-2 bg-slate-100 hover:bg-slate-200 text-slate-700 rounded-full text-sm font-medium transition-colors"
                  >
                    {term}
                  </button>
                ))}
              </div>
            </div>
          ) : (
            <div className="text-center text-slate-500 py-12">
              <p>No {activeTab} found for "{query}".</p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
