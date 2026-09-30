'use client';

import { useState } from 'react';
import { Send, UserCircle, Shield } from 'lucide-react';
// import IdentitySelector from '@/components/IdentitySelector'; // assuming it exists as requested

interface CommentFormProps {
  postPublicId: string;
  parentCommentPublicId?: string;
  onSuccess?: () => void;
  onCancel?: () => void;
}

export default function CommentForm({ postPublicId, parentCommentPublicId, onSuccess, onCancel }: CommentFormProps) {
  const [content, setContent] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isAnonymous, setIsAnonymous] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!content.trim()) return;

    setIsSubmitting(true);
    // API call would go here
    setTimeout(() => {
      setIsSubmitting(false);
      setContent('');
      if (onSuccess) onSuccess();
    }, 1000);
  };

  return (
    <form onSubmit={handleSubmit} className="w-full">
      <div className="border border-slate-300 rounded-xl overflow-hidden bg-white focus-within:border-blue-500 focus-within:ring-1 focus-within:ring-blue-500 transition-shadow">
        <textarea
          value={content}
          onChange={(e) => setContent(e.target.value)}
          placeholder={parentCommentPublicId ? "Write a reply..." : "Write a comment..."}
          className="w-full min-h-[100px] p-4 resize-y focus:outline-none text-sm md:text-base text-slate-900 bg-transparent"
          disabled={isSubmitting}
        />
        
        <div className="flex items-center justify-between bg-slate-50 px-3 py-2 border-t border-slate-200">
          <div className="flex items-center gap-2">
            {/* Simple Anonymous Toggle (mocking IdentitySelector for now if not available) */}
            <button
              type="button"
              onClick={() => setIsAnonymous(!isAnonymous)}
              className={`flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium transition-colors ${
                isAnonymous 
                  ? 'bg-slate-800 text-white' 
                  : 'bg-white text-slate-700 border border-slate-300 hover:bg-slate-100'
              }`}
            >
              {isAnonymous ? <Shield className="w-3.5 h-3.5" /> : <UserCircle className="w-3.5 h-3.5" />}
              {isAnonymous ? 'Anonymous' : 'Public'}
            </button>
          </div>
          
          <div className="flex items-center gap-2">
            {onCancel && (
              <button
                type="button"
                onClick={onCancel}
                className="px-4 py-1.5 text-sm font-medium text-slate-600 hover:bg-slate-200 rounded-full transition-colors"
                disabled={isSubmitting}
              >
                Cancel
              </button>
            )}
            <button
              type="submit"
              disabled={!content.trim() || isSubmitting}
              className="flex items-center gap-1.5 bg-blue-600 text-white px-5 py-1.5 rounded-full text-sm font-medium hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
              <Send className="w-3.5 h-3.5" />
              {isSubmitting ? 'Posting...' : 'Post'}
            </button>
          </div>
        </div>
      </div>
    </form>
  );
}
