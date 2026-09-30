import Link from 'next/link';

export default function LandingPage() {
  return (
    <div className="min-h-screen bg-gradient-to-b from-blue-50 to-white flex flex-col items-center justify-center p-4">
      <div className="max-w-3xl w-full space-y-8 text-center">
        <h1 className="text-5xl font-bold tracking-tight text-gray-900 sm:text-6xl">
          Digital Campus
        </h1>
        <p className="text-xl text-gray-600">
          Your campus. Your community. Your identity — on your terms.
        </p>
        
        <div className="grid grid-cols-1 gap-6 sm:grid-cols-3 mt-12 mb-12">
          <div className="p-6 bg-white rounded-2xl shadow-sm border border-gray-100">
            <h3 className="font-semibold text-lg mb-2">Verified Identity</h3>
            <p className="text-gray-500 text-sm">Connect exclusively with verified students from your university.</p>
          </div>
          <div className="p-6 bg-white rounded-2xl shadow-sm border border-gray-100">
            <h3 className="font-semibold text-lg mb-2">Anonymous Freedom</h3>
            <p className="text-gray-500 text-sm">Post freely and safely using a verified anonymous identity.</p>
          </div>
          <div className="p-6 bg-white rounded-2xl shadow-sm border border-gray-100">
            <h3 className="font-semibold text-lg mb-2">Campus Communities</h3>
            <p className="text-gray-500 text-sm">Find your people, join clubs, and stay updated on campus life.</p>
          </div>
        </div>

        <div className="flex flex-col sm:flex-row items-center justify-center gap-4 mt-8">
          <Link 
            href="/register" 
            className="w-full sm:w-auto px-8 py-3 bg-black text-white rounded-full font-medium hover:bg-gray-800 transition-colors"
          >
            Get Started
          </Link>
          <Link 
            href="/login" 
            className="w-full sm:w-auto px-8 py-3 bg-white text-black border border-gray-200 rounded-full font-medium hover:bg-gray-50 transition-colors"
          >
            Sign In
          </Link>
        </div>
      </div>
    </div>
  );
}
