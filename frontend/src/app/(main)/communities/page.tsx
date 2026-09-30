'use client';

import { useQuery } from '@tanstack/react-query';
import { apiGet } from '@/lib/api';
import { Community } from '@/types';
import { useAuth } from '@/hooks/useAuth';
import Link from 'next/link';
import { Users, Plus, Shield } from 'lucide-react';

export default function MyCommunitiesPage() {
  const { isAuthenticated } = useAuth();

  const { data: communities, isLoading } = useQuery({
    queryKey: ['my-communities'],
    queryFn: () => apiGet<Community[]>('/communities/me'),
    enabled: isAuthenticated,
  });

  return (
    <div className="p-4 md:p-6 max-w-4xl mx-auto">
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold">My Communities</h1>
        <Link
          href="/discover"
          className="flex items-center gap-2 px-4 py-2 bg-black text-white text-sm font-medium rounded-lg hover:bg-gray-800 transition-colors"
        >
          <Plus className="w-4 h-4" />
          Join or Create
        </Link>
      </div>

      {isLoading ? (
        <div className="space-y-3">
          {[1, 2, 3].map((i) => (
            <div key={i} className="h-20 bg-gray-100 rounded-xl animate-pulse" />
          ))}
        </div>
      ) : communities && communities.length > 0 ? (
        <div className="space-y-3">
          {communities.map((community) => (
            <Link
              key={community.publicId}
              href={`/communities/${community.slug}`}
              className="flex items-center gap-4 p-4 bg-white border border-gray-100 rounded-xl hover:shadow-sm transition-shadow"
            >
              <div className="w-12 h-12 rounded-lg bg-blue-100 flex items-center justify-center shrink-0">
                <Users className="w-6 h-6 text-blue-600" />
              </div>
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2">
                  <h3 className="font-semibold truncate">{community.name}</h3>
                  {community.isOfficial && (
                    <Shield className="w-4 h-4 text-blue-500 shrink-0" />
                  )}
                </div>
                <p className="text-sm text-gray-500">
                  {community.memberCount} members · {community.communityType}
                </p>
              </div>
              <span className="text-xs px-2.5 py-1 bg-gray-100 rounded-full text-gray-600 shrink-0">
                {community.visibility}
              </span>
            </Link>
          ))}
        </div>
      ) : (
        <div className="text-center py-16">
          <Users className="w-12 h-12 text-gray-300 mx-auto mb-3" />
          <h3 className="font-medium text-gray-900">No communities yet</h3>
          <p className="text-sm text-gray-500 mt-1">
            Discover and join communities that interest you
          </p>
          <Link
            href="/discover"
            className="inline-block mt-4 px-6 py-2 bg-black text-white text-sm font-medium rounded-lg hover:bg-gray-800"
          >
            Browse Communities
          </Link>
        </div>
      )}
    </div>
  );
}
