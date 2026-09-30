'use client';

import { useState } from 'react';
import { Calendar, MapPin, Globe, Users, Filter } from 'lucide-react';

const filters = ['All', 'My College', 'Global', 'Online'] as const;

interface EventCard {
  id: string;
  title: string;
  description: string;
  eventType: string;
  location: string | null;
  isOnline: boolean;
  startTime: string;
  attendeeCount: number;
  capacity: number | null;
}

function formatEventDate(dateStr: string): string {
  const date = new Date(dateStr);
  const now = new Date();
  const diffDays = Math.floor((date.getTime() - now.getTime()) / (1000 * 60 * 60 * 24));

  if (diffDays === 0) return 'Today';
  if (diffDays === 1) return 'Tomorrow';
  if (diffDays < 7) return `In ${diffDays} days`;

  return date.toLocaleDateString('en-IN', {
    month: 'short',
    day: 'numeric',
    year: date.getFullYear() !== now.getFullYear() ? 'numeric' : undefined,
  });
}

export default function EventsPage() {
  const [activeFilter, setActiveFilter] = useState<string>('All');

  // Placeholder events for the UI
  const events: EventCard[] = [];

  return (
    <div className="p-4 md:p-6 max-w-4xl mx-auto">
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold">Events</h1>
        <button className="flex items-center gap-2 px-4 py-2 bg-black text-white text-sm font-medium rounded-lg hover:bg-gray-800 transition-colors">
          <Calendar className="w-4 h-4" />
          Create Event
        </button>
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

      {events.length > 0 ? (
        <div className="space-y-4">
          {events.map((event) => (
            <div
              key={event.id}
              className="p-5 bg-white border border-gray-100 rounded-xl hover:shadow-sm transition-shadow cursor-pointer"
            >
              <div className="flex items-start justify-between">
                <div>
                  <h3 className="font-semibold text-lg">{event.title}</h3>
                  <p className="text-sm text-gray-500 mt-1 line-clamp-2">{event.description}</p>
                </div>
                <span className="text-xs px-2.5 py-1 bg-blue-50 text-blue-600 rounded-full shrink-0 ml-3">
                  {event.eventType}
                </span>
              </div>
              <div className="flex items-center gap-4 mt-3 text-sm text-gray-500">
                <span className="flex items-center gap-1">
                  <Calendar className="w-4 h-4" />
                  {formatEventDate(event.startTime)}
                </span>
                <span className="flex items-center gap-1">
                  {event.isOnline ? (
                    <>
                      <Globe className="w-4 h-4" />
                      Online
                    </>
                  ) : (
                    <>
                      <MapPin className="w-4 h-4" />
                      {event.location || 'TBA'}
                    </>
                  )}
                </span>
                <span className="flex items-center gap-1">
                  <Users className="w-4 h-4" />
                  {event.attendeeCount}{event.capacity ? `/${event.capacity}` : ''} attending
                </span>
              </div>
              <div className="mt-3">
                <button className="px-4 py-1.5 text-sm font-medium border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors">
                  RSVP
                </button>
              </div>
            </div>
          ))}
        </div>
      ) : (
        <div className="text-center py-16">
          <Calendar className="w-12 h-12 text-gray-300 mx-auto mb-3" />
          <h3 className="font-medium text-gray-900">No upcoming events</h3>
          <p className="text-sm text-gray-500 mt-1">
            Events from your college and communities will appear here
          </p>
        </div>
      )}
    </div>
  );
}
