'use client';

import { useState } from 'react';
import { Conversation, Message } from '@/types';
import { Search, Send, Plus, UserCircle } from 'lucide-react';
import { formatTimeAgo } from '@/lib/utils';

export default function ChatPage() {
  const [conversations] = useState<Conversation[]>([]);
  const [selectedConv, setSelectedConv] = useState<string | null>(null);
  const [messages] = useState<Message[]>([]);
  const [newMessage, setNewMessage] = useState('');

  return (
    <div className="flex h-[calc(100vh-64px)] bg-white overflow-hidden">
      {/* Conversation List Sidebar */}
      <div className={`w-full md:w-80 border-r border-slate-200 flex flex-col ${selectedConv ? 'hidden md:flex' : 'flex'}`}>
        <div className="p-4 border-b border-slate-200 flex items-center justify-between">
          <h1 className="text-xl font-bold text-slate-900">Messages</h1>
          <button className="p-2 text-blue-600 hover:bg-blue-50 rounded-full transition-colors">
            <Plus className="w-5 h-5" />
          </button>
        </div>
        
        <div className="p-2 border-b border-slate-200">
          <div className="relative">
            <Search className="absolute left-3 top-2.5 h-4 w-4 text-slate-400" />
            <input 
              type="text" 
              placeholder="Search conversations..." 
              className="w-full pl-9 pr-4 py-2 bg-slate-100 border-transparent rounded-lg focus:bg-white focus:border-blue-500 focus:ring-2 focus:ring-blue-200 text-sm"
            />
          </div>
        </div>

        <div className="flex-1 overflow-y-auto">
          {conversations.length === 0 ? (
            <div className="p-8 text-center text-slate-500">
              <p>No conversations yet.</p>
            </div>
          ) : (
            conversations.map(conv => (
              <button 
                key={conv.publicId}
                onClick={() => setSelectedConv(conv.publicId)}
                className={`w-full p-4 text-left border-b border-slate-100 flex gap-3 hover:bg-slate-50 transition-colors ${selectedConv === conv.publicId ? 'bg-slate-50' : ''}`}
              >
                <div className="h-12 w-12 rounded-full bg-slate-200 flex items-center justify-center flex-shrink-0">
                  <UserCircle className="w-8 h-8 text-slate-400" />
                </div>
                <div className="flex-1 min-w-0">
                  <div className="flex justify-between items-baseline mb-1">
                    <h3 className="font-medium text-slate-900 truncate">{conv.name || 'Anonymous User'}</h3>
                    <span className="text-xs text-slate-500 flex-shrink-0">{formatTimeAgo(conv.updatedAt)}</span>
                  </div>
                  <p className="text-sm text-slate-500 truncate">{conv.lastMessage}</p>
                </div>
              </button>
            ))
          )}
        </div>
      </div>

      {/* Message View Area */}
      <div className={`flex-1 flex flex-col bg-slate-50 ${!selectedConv ? 'hidden md:flex' : 'flex'}`}>
        {!selectedConv ? (
          <div className="flex-1 flex items-center justify-center text-slate-500 flex-col gap-4">
            <MessageSquare className="w-16 h-16 text-slate-300" />
            <p className="text-lg">Select a conversation or start a new one</p>
          </div>
        ) : (
          <>
            {/* Chat Header */}
            <div className="p-4 bg-white border-b border-slate-200 flex items-center gap-3">
              <button 
                onClick={() => setSelectedConv(null)}
                className="md:hidden p-2 -ml-2 text-slate-500"
              >
                ←
              </button>
              <div className="font-semibold text-slate-900">Chat Name</div>
            </div>

            {/* Messages Area */}
            <div className="flex-1 overflow-y-auto p-4 space-y-4">
              {messages.map(msg => (
                <div key={msg.publicId} className="flex flex-col max-w-[80%]">
                  <div className="bg-white border border-slate-200 rounded-2xl p-3 shadow-sm">
                    {msg.body}
                  </div>
                </div>
              ))}
            </div>

            {/* Input Area */}
            <div className="p-4 bg-white border-t border-slate-200">
              <div className="flex gap-2">
                <input 
                  type="text" 
                  value={newMessage}
                  onChange={(e) => setNewMessage(e.target.value)}
                  placeholder="Type a message..." 
                  className="flex-1 border border-slate-300 rounded-full px-4 py-2 focus:outline-none focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                />
                <button className="bg-blue-600 text-white p-2 rounded-full hover:bg-blue-700 transition-colors flex-shrink-0 flex items-center justify-center w-10 h-10">
                  <Send className="w-5 h-5 ml-1" />
                </button>
              </div>
            </div>
          </>
        )}
      </div>
    </div>
  );
}

// Ensure MessageSquare is imported for the empty state
import { MessageSquare } from 'lucide-react';
