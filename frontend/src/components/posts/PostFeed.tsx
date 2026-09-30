'use client';

import { useState } from 'react';
import { Post } from '@/types';
import { Flame, Clock, TrendingUp } from 'lucide-react';
// import PostCard from './PostCard'; // assuming it exists as requested

interface PostFeedProps {
  communitySlug?: string;
  feedType?: 'home' | 'trending';
}

export default function PostFeed({ communitySlug, feedType }: PostFeedProps) {
  const [sort, setSort] = useState<'hot' | 'new' | 'top'>('hot');
  const [posts] = useState<Post[]>([]);
  const [isLoading] = useState(false);

  return (
    <div className="space-y-4">
      <div className="flex gap-2">
        <button
          onClick={() => setSort('hot')}
          className={`flex items-center gap-1.5 px-4 py-1.5 rounded-full text-sm font-medium transition-colors ${
            sort === 'hot' ? 'bg-blue-100 text-blue-700' : 'bg-white text-slate-600 hover:bg-slate-50'
          }`}
        >
          <Flame className="w-4 h-4" />
          Hot
        </button>
        <button
          onClick={() => setSort('new')}
          className={`flex items-center gap-1.5 px-4 py-1.5 rounded-full text-sm font-medium transition-colors ${
            sort === 'new' ? 'bg-blue-100 text-blue-700' : 'bg-white text-slate-600 hover:bg-slate-50'
          }`}
        >
          <Clock className="w-4 h-4" />
          New
        </button>
        <button
          onClick={() => setSort('top')}
          className={`flex items-center gap-1.5 px-4 py-1.5 rounded-full text-sm font-medium transition-colors ${
            sort === 'top' ? 'bg-blue-100 text-blue-700' : 'bg-white text-slate-600 hover:bg-slate-50'
          }`}
        >
          <TrendingUp className="w-4 h-4" />
          Top
        </button>
      </div>

      <div className="space-y-4">
        {isLoading ? (
          Array.from({ length: 3 }).map((_, i) => (
            <div key={i} className="bg-white rounded-xl shadow-sm border border-slate-200 p-4 space-y-4 animate-pulse">
              <div className="flex items-center gap-3">
                <div className="w-8 h-8 rounded-full bg-slate-200" />
                <div className="space-y-2">
                  <div className="h-3 w-24 bg-slate-200 rounded" />
                  <div className="h-2 w-16 bg-slate-200 rounded" />
                </div>
              </div>
              <div className="space-y-2">
                <div className="h-4 w-3/4 bg-slate-200 rounded" />
                <div className="h-4 w-1/2 bg-slate-200 rounded" />
              </div>
              <div className="h-32 bg-slate-100 rounded-lg" />
            </div>
          ))
        ) : posts.length === 0 ? (
          <div className="bg-white rounded-xl shadow-sm border border-slate-200 p-12 text-center text-slate-500">
            <p className="text-lg font-medium text-slate-900 mb-2">No posts yet</p>
            <p>Be the first to share something!</p>
          </div>
        ) : (
          posts.map(post => (
            <div key={post.publicId} className="bg-white rounded-xl shadow-sm border border-slate-200 p-4">
              {/* Replace with actual PostCard */}
              <div className="font-medium">{post.title}</div>
            </div>
          ))
        )}
      </div>
    </div>
  );
}
