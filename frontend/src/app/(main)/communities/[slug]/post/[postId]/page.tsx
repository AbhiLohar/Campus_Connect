'use client';

import { useState } from 'react';
import { useParams } from 'next/navigation';
import { MessageSquare, Share2, Bookmark, Flag, ChevronUp, ChevronDown, UserCircle } from 'lucide-react';
import CommentThread from '@/components/comments/CommentThread';
import CommentForm from '@/components/comments/CommentForm';
import { formatTimeAgo } from '@/lib/utils';
import { Post, Comment } from '@/types';

export default function PostDetailClient() {
  const params = useParams();
  const slug = params.slug as string;
  const postId = params.postId as string;
  const [isSaved, setIsSaved] = useState(false);
  const [voteScore, setVoteScore] = useState(42);

  const post: Post = {
    publicId: postId,
    communitySlug: slug,
    title: 'How to prepare for Data Structures final?',
    body: 'I have the final exam next week and I am really struggling with graphs and dynamic programming. Does anyone have good resources or tips for studying these topics?',
    postType: 'text',
    linkUrl: null,
    postingIdentityType: 'public',
    author: {
      displayName: 'Alex Johnson',
      userPublicId: 'u1',
      isAnonymous: false,
      verifiedBadge: 'CS 25'
    },
    voteScore: 42,
    upvoteCount: 45,
    downvoteCount: 3,
    commentCount: 5,
    isPinned: false,
    isLocked: false,
    myVote: 1,
    isSaved: false,
    createdAt: new Date().toISOString(),
    editedAt: null
  };

  const mockComments: Comment[] = [];

  return (
    <div className="max-w-3xl mx-auto p-4 md:p-6 space-y-6 pb-24 md:pb-6">
      <div className="bg-white rounded-xl shadow-sm border border-slate-200 overflow-hidden">
        <div className="p-4 md:p-6 flex gap-4">
          <div className="hidden md:flex flex-col items-center gap-1 min-w-[40px]">
            <button className="p-1 text-blue-600 hover:bg-blue-50 rounded">
              <ChevronUp className="w-6 h-6" />
            </button>
            <span className="font-bold text-slate-900">{voteScore}</span>
            <button className="p-1 text-slate-400 hover:bg-slate-50 hover:text-red-500 rounded">
              <ChevronDown className="w-6 h-6" />
            </button>
          </div>

          <div className="flex-1">
            <div className="flex items-center gap-2 text-xs md:text-sm text-slate-500 mb-2">
              <span className="font-medium text-slate-900 hover:underline cursor-pointer">c/{post.communitySlug}</span>
              <span>•</span>
              <span>Posted by {post.author.displayName}</span>
              {post.author.verifiedBadge && (
                <span className="bg-slate-100 px-1.5 py-0.5 rounded text-xs">{post.author.verifiedBadge}</span>
              )}
              <span>•</span>
              <span>{formatTimeAgo(post.createdAt)}</span>
            </div>
            
            <h1 className="text-xl md:text-2xl font-bold text-slate-900 mb-4">
              {post.title}
            </h1>
            
            <div className="text-slate-800 whitespace-pre-wrap leading-relaxed mb-6">
              {post.body}
            </div>

            <div className="flex flex-wrap items-center gap-2 md:gap-4 text-sm font-medium text-slate-500">
              <div className="md:hidden flex items-center bg-slate-100 rounded-full">
                <button className="p-1.5 text-blue-600 hover:bg-slate-200 rounded-l-full">
                  <ChevronUp className="w-5 h-5" />
                </button>
                <span className="px-2 font-bold text-slate-900">{voteScore}</span>
                <button className="p-1.5 text-slate-500 hover:bg-slate-200 rounded-r-full">
                  <ChevronDown className="w-5 h-5" />
                </button>
              </div>

              <div className="flex items-center gap-1.5 px-3 py-1.5 hover:bg-slate-50 rounded-full transition-colors cursor-pointer">
                <MessageSquare className="w-4 h-4" />
                <span>{post.commentCount} Comments</span>
              </div>
              
              <button className="flex items-center gap-1.5 px-3 py-1.5 hover:bg-slate-50 rounded-full transition-colors">
                <Share2 className="w-4 h-4" />
                <span>Share</span>
              </button>
              
              <button 
                onClick={() => setIsSaved(!isSaved)}
                className={`flex items-center gap-1.5 px-3 py-1.5 rounded-full transition-colors ${
                  isSaved ? 'text-blue-600 bg-blue-50' : 'hover:bg-slate-50'
                }`}
              >
                <Bookmark className={`w-4 h-4 ${isSaved ? 'fill-current' : ''}`} />
                <span>{isSaved ? 'Saved' : 'Save'}</span>
              </button>
              
              <button className="flex items-center gap-1.5 px-3 py-1.5 hover:bg-slate-50 hover:text-red-500 rounded-full transition-colors ml-auto md:ml-0">
                <Flag className="w-4 h-4" />
                <span className="hidden sm:inline">Report</span>
              </button>
            </div>
          </div>
        </div>
      </div>

      <div className="bg-white rounded-xl shadow-sm border border-slate-200 p-4 md:p-6">
        <h2 className="text-lg font-bold text-slate-900 mb-6 flex items-center gap-2">
          Comments <span className="text-sm font-normal text-slate-500 bg-slate-100 px-2 py-0.5 rounded-full">{post.commentCount}</span>
        </h2>
        
        <div className="mb-8">
          <CommentForm postPublicId={post.publicId} />
        </div>

        <div className="mt-8">
          <CommentThread comments={mockComments} postPublicId={post.publicId} />
        </div>
      </div>
    </div>
  );
}
