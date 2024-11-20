import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";

// This function can be marked `async` if using `await` inside
export async function middleware(request: NextRequest) {
  const response = NextResponse.next();

  if (!request.nextUrl.pathname.startsWith("/login")) {
    const refreshTokenCookie = request.cookies.get("refresh_token");
    if (!refreshTokenCookie) {
      return NextResponse.redirect(new URL("/login", request.url));
    } else {
      const cookie = request.cookies.get("access_token");
      if (!cookie) {
        try {
          const accessToken = await fetch(
            "http://localhost:3001/api/auth/access_token",
            {
              method: "POST",
              headers: {
                "Content-Type": "application/json",
              },
              body: JSON.stringify({ refreshToken: refreshTokenCookie.value }),
            }
          ).then((res) => {
            if (!res.ok) {
              throw new Error("Failed to get access token");
            }
            return res.json();
          });

          if (accessToken.accessToken) {
            response.cookies.set("access_token", accessToken.accessToken, {
              secure: true,
              sameSite: "strict",
              path: "/",
            });
          } else {
            throw new Error("Failed to get access token");
          }
        } catch {
          response.cookies.delete("refresh_token");
          return NextResponse.redirect(new URL("/login", request.url));
        }
      }
    }

    return response;
  }
}

export const config = {
  matcher: [
    /*
     * Match all request paths except for the ones starting with:
     * - api (API routes)
     * - _next/static (static files)
     * - _next/image (image optimization files)
     * - favicon.ico, sitemap.xml, robots.txt (metadata files)
     */
    {
      source:
        "/((?!api|_next/static|_next/image|favicon.ico|sitemap.xml|robots.txt).*)",
      missing: [
        { type: "header", key: "next-router-prefetch" },
        { type: "header", key: "purpose", value: "prefetch" },
      ],
    },

    {
      source:
        "/((?!api|_next/static|_next/image|favicon.ico|sitemap.xml|robots.txt).*)",
      has: [
        { type: "header", key: "next-router-prefetch" },
        { type: "header", key: "purpose", value: "prefetch" },
      ],
    },

    {
      source:
        "/((?!api|_next/static|_next/image|favicon.ico|sitemap.xml|robots.txt).*)",
      has: [{ type: "header", key: "x-present" }],
      missing: [{ type: "header", key: "x-missing", value: "prefetch" }],
    },
  ],
};
