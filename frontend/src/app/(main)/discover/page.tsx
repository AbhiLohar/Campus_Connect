'use client';

import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { apiGet } from '@/lib/api';
import { Community } from '@/types';
import Link from 'next/link';
import { Users, Globe, GraduationCap, Search } from 'lucide-react';

const filters = ['All', 'College', 'Global', 'Clubs'] as const;

export default function DiscoverPage() {
  const [activeFilter, setActiveFilter] = useState<string>('All');
  const [searchQuery, setSearchQuery] = useState('');

  const { data: communities, isLoading } = useQuery({
    queryKey: ['communities', activeFilter],
    queryFn: () => apiGet<Community[]>('/communities'),
  });

  const filtered = communities?.filter((c) => {
    if (searchQuery && !c.name.toLowerCase().includes(searchQuery.toLowerCase())) return false;
    if (activeFilter === 'College') return c.communityType === 'college';
    if (activeFilter === 'Global') return c.communityType === 'global';
    if (activeFilter === 'Clubs') return c.communityType === 'club';
    return true;
  });

  return (
    <div className="p-4 md:p-6 max-w-6xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Discover Communities</h1>

      <div className="relative mb-6">
        <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
        <input
          type="search"
          placeholder="Search communities..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="w-full pl-10 pr-4 py-2.5 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-black"
        />
      </div>

      <div className="flex gap-2 mb-6 overflow-x-auto">
        {filters.map((filter) => (
          <button
            key={filter}
            onClick={() => setActiveFilter(filter)}
            className={`px-4 py-1.5 rounded-full text-sm font-medium whitespace-nowrap transition-colors ${
              activeFilter === filter
                ? 'bg-black text-white'
                : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
            }`}
          >
            {filter}
          </button>
        ))}
      </div>

      {isLoading ? (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {[1, 2, 3, 4, 5, 6].map((i) => (
            <div key={i} className="h-40 bg-gray-100 rounded-xl animate-pulse" />
          ))}
        </div>
      ) : filtered && filtered.length > 0 ? (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {filtered.map((community) => (
            <Link
              key={community.publicId}
              href={`/communities/${community.slug}`}
              className="p-5 bg-white border border-gray-100 rounded-xl hover:shadow-md transition-shadow"
            >
              <div className="flex items-start gap-3">
                <div className="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center shrink-0">
                  {community.communityType === 'global' ? (
                    <Globe className="w-5 h-5 text-blue-600" />
                  ) : community.communityType === 'club' ? (
                    <GraduationCap className="w-5 h-5 text-purple-600" />
                  ) : (
                    <Users className="w-5 h-5 text-blue-600" />
                  )}
                </div>
                <div className="min-w-0">
                  <h3 className="font-semibold text-sm truncate">{community.name}</h3>
                  <p className="text-xs text-gray-500 mt-0.5">{community.memberCount} members</p>
                </div>
              </div>
              <p className="text-sm text-gray-600 mt-3 line-clamp-2">{community.description}</p>
              <div className="mt-3 flex items-center gap-2">
                <span className="text-xs px-2 py-0.5 bg-gray-100 rounded-full text-gray-600">{community.communityType}</span>
                {community.isOfficial && (
                  <span className="text-xs px-2 py-0.5 bg-blue-50 text-blue-600 rounded-full">Official</span>
                )}
              </div>
            </Link>
          ))}
        </div>
      ) : (
        <div className="text-center py-16">
          <Users className="w-12 h-12 text-gray-300 mx-auto mb-3" />
          <h3 className="font-medium text-gray-900">No communities found</h3>
          <p className="text-sm text-gray-500 mt-1">Try adjusting your search or filters</p>
        </div>
      )}
    </div>
  );
}
