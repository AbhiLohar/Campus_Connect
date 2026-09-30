'use client';

import { useState } from 'react';
import { Comment } from '@/types';
import { formatTimeAgo } from '@/lib/utils';
import { Shield, ChevronUp, ChevronDown, UserCircle, MessageSquare } from 'lucide-react';
import CommentForm from './CommentForm';

interface CommentThreadProps {
  comments: Comment[];
  postPublicId: string;
}

export default function CommentThread({ comments, postPublicId }: CommentThreadProps) {
  if (!comments || comments.length === 0) {
    return (
      <div className="text-center py-8 text-slate-500">
        <p>No comments yet. Be the first to share your thoughts!</p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {comments.map(comment => (
        <CommentItem 
          key={comment.publicId} 
          comment={comment} 
          postPublicId={postPublicId} 
        />
      ))}
    </div>
  );
}

function CommentItem({ comment, postPublicId }: { comment: Comment, postPublicId: string }) {
  const [isReplying, setIsReplying] = useState(false);
  const [isCollapsed, setIsCollapsed] = useState(false);

  // Depth caps out visually at 10 to avoid extreme squishing
  const visualDepth = Math.min(comment.depth, 10);

  return (
    <div className="flex gap-2 md:gap-3" style={{ paddingLeft: `${visualDepth > 0 ? 16 + (visualDepth * 8) : 0}px` }}>
      {/* Indentation line & avatar column */}
      <div className="flex flex-col items-center">
        {comment.author.isAnonymous ? (
          <div className="h-8 w-8 rounded-full bg-slate-800 text-white flex items-center justify-center flex-shrink-0">
            <Shield className="w-4 h-4" />
          </div>
        ) : (
          <div className="h-8 w-8 rounded-full bg-slate-200 text-slate-500 flex items-center justify-center flex-shrink-0">
            <UserCircle className="w-5 h-5" />
          </div>
        )}
        {!isCollapsed && visualDepth > 0 && (
          <div className="w-px h-full bg-slate-200 my-1 group-hover:bg-slate-300 transition-colors" />
        )}
      </div>

      <div className="flex-1 bg-white pt-1">
        <div className="flex items-center gap-2 text-xs md:text-sm mb-1 cursor-pointer" onClick={() => setIsCollapsed(!isCollapsed)}>
          <span className="font-semibold text-slate-900">
            {comment.author.isAnonymous ? 'Anonymous Student' : comment.author.displayName}
          </span>
          {comment.author.verifiedBadge && !comment.author.isAnonymous && (
            <span className="bg-slate-100 text-slate-600 px-1.5 py-0.5 rounded text-[10px] md:text-xs font-medium">
              {comment.author.verifiedBadge}
            </span>
          )}
          <span className="text-slate-500">•</span>
          <span className="text-slate-500">{formatTimeAgo(comment.createdAt)}</span>
        </div>

        {!isCollapsed && (
          <>
            <div className="text-slate-800 text-sm md:text-base mb-2 whitespace-pre-wrap">
              {comment.body}
            </div>

            <div className="flex items-center gap-4 text-xs font-medium text-slate-500">
              <div className="flex items-center gap-1 bg-slate-100 rounded-full px-1">
                <button className="p-1 hover:text-blue-600 hover:bg-slate-200 rounded-full transition-colors">
                  <ChevronUp className="w-4 h-4" />
                </button>
                <span className="text-slate-900 min-w-[1ch] text-center">{comment.voteScore}</span>
                <button className="p-1 hover:text-red-600 hover:bg-slate-200 rounded-full transition-colors">
                  <ChevronDown className="w-4 h-4" />
                </button>
              </div>

              <button 
                onClick={() => setIsReplying(!isReplying)}
                className="flex items-center gap-1.5 hover:text-slate-900 transition-colors px-2 py-1 rounded hover:bg-slate-50"
              >
                <MessageSquare className="w-3.5 h-3.5" />
                Reply
              </button>
            </div>

            {isReplying && (
              <div className="mt-4 mb-2">
                <CommentForm 
                  postPublicId={postPublicId}
                  parentCommentPublicId={comment.publicId}
                  onSuccess={() => setIsReplying(false)}
                  onCancel={() => setIsReplying(false)}
                />
              </div>
            )}
          </>
        )}
      </div>
    </div>
  );
}
