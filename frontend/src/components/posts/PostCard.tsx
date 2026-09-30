'use client';
import { Post } from '@/types';
import { formatTimeAgo, truncateText } from '@/lib/utils';

interface PostCardProps {
  post: Post;
  onVote?: (publicId: string, value: number) => void;
}

export function PostCard({ post, onVote }: PostCardProps) {
  return (
    <div className="border rounded p-4 mb-4 shadow-sm bg-white">
      <div className="flex items-center gap-2 mb-2 text-sm text-gray-500">
        <span>{post.author.isAnonymous ? 'Anonymous Student' : post.author.displayName}</span>
        <span>•</span>
        <span>{post.communitySlug}</span>
        <span>•</span>
        <span>{formatTimeAgo(post.createdAt)}</span>
      </div>
      <h2 className="text-xl font-semibold mb-2">{post.title}</h2>
      <p className="text-gray-700 mb-4">{truncateText(post.body, 150)}</p>
      <div className="flex gap-4 text-sm text-gray-500">
        <button onClick={() => onVote?.(post.publicId, 1)}>▲ {post.upvoteCount}</button>
        <button onClick={() => onVote?.(post.publicId, -1)}>▼ {post.downvoteCount}</button>
        <span>💬 {post.commentCount} comments</span>
      </div>
    </div>
  );
}
