'use client';

import { useState } from 'react';
import { Notification } from '@/types';
import { Bell, Check, MessageCircle, Heart, UserPlus, AtSign } from 'lucide-react';
import { formatTimeAgo } from '@/lib/utils';

export default function NotificationsPage() {
  const [notifications] = useState<Notification[]>([]);

  const getIcon = (type: string) => {
    switch (type) {
      case 'comment_reply': return <MessageCircle className="w-5 h-5 text-blue-500" />;
      case 'post_vote': return <Heart className="w-5 h-5 text-red-500" />;
      case 'community_invite': return <UserPlus className="w-5 h-5 text-green-500" />;
      case 'mention': return <AtSign className="w-5 h-5 text-purple-500" />;
      default: return <Bell className="w-5 h-5 text-slate-500" />;
    }
  };

  return (
    <div className="max-w-2xl mx-auto p-4 md:p-6 space-y-4">
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold text-slate-900">Notifications</h1>
        <button className="text-sm font-medium text-blue-600 hover:text-blue-700 flex items-center gap-1 bg-blue-50 px-3 py-1.5 rounded-full transition-colors">
          <Check className="w-4 h-4" />
          Mark all as read
        </button>
      </div>

      <div className="bg-white rounded-xl shadow-sm border border-slate-200 overflow-hidden min-h-[50vh]">
        {notifications.length === 0 ? (
          <div className="flex flex-col items-center justify-center p-12 text-slate-500 h-full">
            <Bell className="w-12 h-12 text-slate-300 mb-4" />
            <p className="text-lg font-medium text-slate-900 mb-1">No notifications yet</p>
            <p className="text-sm">When you get notifications, they'll show up here.</p>
          </div>
        ) : (
          <div className="divide-y divide-slate-100">
            {notifications.map(notif => (
              <div 
                key={notif.id} 
                className={`p-4 hover:bg-slate-50 transition-colors flex gap-4 ${!notif.isRead ? 'bg-blue-50/50' : ''}`}
              >
                <div className="mt-1 flex-shrink-0">
                  {getIcon(notif.notificationType)}
                </div>
                <div className="flex-1">
                  <div className="flex items-start justify-between gap-2">
                    <p className="text-sm text-slate-900 font-medium">
                      {notif.title}
                    </p>
                    <span className="text-xs text-slate-500 whitespace-nowrap">
                      {formatTimeAgo(notif.createdAt)}
                    </span>
                  </div>
                  {notif.body && (
                    <p className="text-sm text-slate-600 mt-1 line-clamp-2">
                      {notif.body}
                    </p>
                  )}
                </div>
                {!notif.isRead && (
                  <div className="w-2 h-2 rounded-full bg-blue-600 mt-2 flex-shrink-0" />
                )}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
