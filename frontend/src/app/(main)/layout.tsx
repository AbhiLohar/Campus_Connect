'use client';

import { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { useAuth } from '@/hooks/useAuth';
import Sidebar from '@/components/layout/Sidebar';
import MobileNav from '@/components/layout/MobileNav';
import Navbar from '@/components/layout/Navbar';
import UpdateBanner from '@/components/common/UpdateBanner';

export default function MainLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const { isAuthenticated, isLoading } = useAuth();
  const router = useRouter();

  useEffect(() => {
    if (!isLoading && !isAuthenticated) {
      router.push('/login');
    }
  }, [isAuthenticated, isLoading, router]);

  if (isLoading) {
    return <div className="min-h-screen flex items-center justify-center">Loading...</div>;
  }

  if (!isAuthenticated) {
    return null; // Will redirect
  }

  return (
    <div className="flex flex-col h-screen bg-white">
      <UpdateBanner />
      <div className="flex flex-1 overflow-hidden">
        {/* Desktop Sidebar */}
        <div className="hidden md:flex w-64 flex-col fixed inset-y-0">
        <Sidebar />
      </div>

      {/* Main Content */}
      <div className="md:pl-64 flex flex-col flex-1 h-full">
        <div className="md:hidden">
          <Navbar />
        </div>
        
        <main className="flex-1 overflow-y-auto pb-16 md:pb-0">
          {children}
        </main>
        
        <div className="md:hidden">
          <MobileNav />
        </div>
      </div>
      </div>
    </div>
  );
}
