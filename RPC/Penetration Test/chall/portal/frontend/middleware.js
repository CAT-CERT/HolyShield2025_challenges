import { NextResponse } from 'next/server';

export function middleware(request) {

  if (request.nextUrl.pathname === '/admin') {
    const adminCookie = request.cookies.get('AdminCookie');
    
    if (!adminCookie || adminCookie.value !== 'HolyShield2025!') {
      return NextResponse.redirect(new URL('/403', request.url));
    }
  }
  
  return NextResponse.next();
}

export const config = {
  matcher: '/admin',
};

