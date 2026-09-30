// Core types
export interface User {
  publicId: string;
  displayName: string;
  createdAt: string;
  bio?: string;
  avatarUrl?: string;
  status?: string;
}

export interface AuthResponse {
  accessToken: string;
  refreshToken: string;
  user: User;
}

export interface ApiError {
  error: { code: string; message: string };
}

export interface PaginatedResponse<T> {
  items: T[];
  nextCursor: string | null;
  hasMore: boolean;
}

// College types
export interface College {
  publicId: string;
  name: string;
  slug: string;
  shortName: string | null;
  city: string | null;
  state: string | null;
  country: string;
  logoUrl: string | null;
}

export interface CollegeDetail extends College {
  websiteUrl: string | null;
  domains: string[];
}

export interface Affiliation {
  publicId: string;
  college: College;
  program: string | null;
  department: string | null;
  yearStart: number | null;
  yearEnd: number | null;
  status: string;
  verificationLevel: number;
}

// Community types
export interface Community {
  publicId: string;
  name: string;
  slug: string;
  description: string;
  iconUrl: string | null;
  bannerUrl: string | null;
  communityType: string;
  visibility: string;
  joinPolicy: string;
  memberCount: number;
  isOfficial: boolean;
}

export interface CommunityDetail extends Community {
  rules: CommunityRule[];
  myRole: string | null;
}

export interface CommunityRule {
  ruleNumber: number;
  title: string;
  description: string | null;
}

// Post types  
export interface PostAuthor {
  displayName: string;
  userPublicId: string | null;
  isAnonymous: boolean;
  verifiedBadge: string | null;
}

export interface Post {
  publicId: string;
  communitySlug: string;
  title: string;
  body: string;
  postType: string;
  linkUrl: string | null;
  postingIdentityType: string;
  author: PostAuthor;
  voteScore: number;
  upvoteCount: number;
  downvoteCount: number;
  commentCount: number;
  isPinned: boolean;
  isLocked: boolean;
  myVote: number | null;
  isSaved: boolean;
  createdAt: string;
  editedAt: string | null;
}

export interface Comment {
  publicId: string;
  postPublicId: string;
  parentCommentPublicId: string | null;
  body: string;
  author: PostAuthor;
  depth: number;
  voteScore: number;
  createdAt: string;
  editedAt: string | null;
}

// Chat types
export interface Conversation {
  publicId: string;
  conversationType: string;
  name: string | null;
  lastMessage: string | null;
  updatedAt: string;
}

export interface Message {
  publicId: string;
  body: string;
  senderDisplayName: string;
  isAnonymous: boolean;
  createdAt: string;
}

// Event types
export interface Event {
  publicId: string;
  title: string;
  description: string;
  eventType: string;
  location: string | null;
  isOnline: boolean;
  startTime: string;
  endTime: string | null;
  attendeeCount: number;
  capacity: number | null;
}

// Notification type
export interface Notification {
  id: string;
  notificationType: string;
  title: string;
  body: string | null;
  isRead: boolean;
  createdAt: string;
}

// Marketplace types
export interface Listing {
  publicId: string;
  title: string;
  description: string;
  category: string;
  priceCents: number;
  currency: string;
  condition: string;
  locationArea: string | null;
  imageUrls: string[];
  status: string;
  createdAt: string;
}
