'use client';

import { useAuth } from '@/hooks/useAuth';
import { GraduationCap, MapPin, MessageSquare, Calendar, Users } from 'lucide-react';
import Link from 'next/link';

export default function CampusPage() {
  const { user, isAuthenticated } = useAuth();

  if (!isAuthenticated) {
    return (
      <div className="p-4 md:p-6 max-w-4xl mx-auto text-center py-16">
        <GraduationCap className="w-12 h-12 text-gray-300 mx-auto mb-3" />
        <h3 className="font-medium text-gray-900 text-lg">Connect with your campus</h3>
        <p className="text-sm text-gray-500 mt-1">Log in to access your campus community</p>
        <Link
          href="/login"
          className="inline-block mt-4 px-6 py-2 bg-black text-white text-sm font-medium rounded-lg hover:bg-gray-800"
        >
          Log In
        </Link>
      </div>
    );
  }

  return (
    <div className="p-4 md:p-6 max-w-4xl mx-auto">
      <h1 className="text-2xl font-bold mb-2">My Campus</h1>
      <p className="text-gray-500 mb-6">Stay connected with your college community</p>

      {/* Quick Actions */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-3 mb-8">
        <Link href="/discover" className="flex flex-col items-center p-4 bg-blue-50 rounded-xl hover:bg-blue-100 transition-colors">
          <MessageSquare className="w-6 h-6 text-blue-600 mb-2" />
          <span className="text-sm font-medium text-blue-900">Ask Campus</span>
        </Link>
        <Link href="/discover" className="flex flex-col items-center p-4 bg-purple-50 rounded-xl hover:bg-purple-100 transition-colors">
          <Users className="w-6 h-6 text-purple-600 mb-2" />
          <span className="text-sm font-medium text-purple-900">Browse Clubs</span>
        </Link>
        <Link href="/events" className="flex flex-col items-center p-4 bg-green-50 rounded-xl hover:bg-green-100 transition-colors">
          <Calendar className="w-6 h-6 text-green-600 mb-2" />
          <span className="text-sm font-medium text-green-900">Events</span>
        </Link>
        <Link href="/marketplace" className="flex flex-col items-center p-4 bg-orange-50 rounded-xl hover:bg-orange-100 transition-colors">
          <MapPin className="w-6 h-6 text-orange-600 mb-2" />
          <span className="text-sm font-medium text-orange-900">Marketplace</span>
        </Link>
      </div>

      {/* Trending Section */}
      <section className="mb-8">
        <h2 className="text-lg font-semibold mb-4">🔥 Trending on Campus</h2>
        <div className="bg-gray-50 rounded-xl p-8 text-center">
          <p className="text-sm text-gray-500">
            Verify your college to see trending posts from your campus
          </p>
          <button className="mt-3 px-4 py-2 bg-black text-white text-sm font-medium rounded-lg hover:bg-gray-800">
            Verify College
          </button>
        </div>
      </section>

      {/* Campus Communities */}
      <section className="mb-8">
        <h2 className="text-lg font-semibold mb-4">Campus Communities</h2>
        <div className="bg-gray-50 rounded-xl p-8 text-center">
          <Users className="w-8 h-8 text-gray-300 mx-auto mb-2" />
          <p className="text-sm text-gray-500">
            Join your campus to see community suggestions
          </p>
        </div>
      </section>

      {/* Upcoming Events */}
      <section>
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-lg font-semibold">Upcoming Events</h2>
          <Link href="/events" className="text-sm text-blue-600 hover:text-blue-800">
            View all
          </Link>
        </div>
        <div className="bg-gray-50 rounded-xl p-8 text-center">
          <Calendar className="w-8 h-8 text-gray-300 mx-auto mb-2" />
          <p className="text-sm text-gray-500">No upcoming events</p>
        </div>
      </section>
    </div>
  );
}
