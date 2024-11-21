import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";
import { parseAccessToken } from "./lib/parse-jwt";

async function getAccessToken(refreshToken: string) {
  return await fetch(
    process.env.NEXT_PUBLIC_API_URL + "/api/auth/access_token",
    {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ refreshToken: refreshToken }),
    }
  ).then((res) => {
    if (!res.ok) {
      throw new Error("Failed to get access token");
    }
    return res.json();
  });
}

// This function can be marked `async` if using `await` inside
export async function middleware(request: NextRequest) {
  const response = NextResponse.next();

  if (!request.nextUrl.pathname.startsWith("/login")) {
    const refreshTokenCookie = request.cookies.get("refresh_token");
    if (!refreshTokenCookie) {
      return NextResponse.redirect(new URL("/login", request.url));
    } else {
      try {
        const accessTokenCookie = request.cookies.get("access_token");
        if (!accessTokenCookie) {
          const accessTokenResponse = await getAccessToken(
            refreshTokenCookie.value
          );

          try {
            if (accessTokenResponse.accessToken) {
              response.cookies.set(
                "access_token",
                accessTokenResponse.accessToken,
                {
                  secure: true,
                  sameSite: "strict",
                  path: "/",
                }
              );
            } else {
              throw new Error("Failed to get access token");
            }
          } catch {
            response.cookies.delete("refresh_token");
            return NextResponse.redirect(new URL("/login", request.url));
          }
        } else {
          const accessToken = parseAccessToken(accessTokenCookie.value);
          if (accessToken.exp < new Date()) {
            const accessTokenResponse = await getAccessToken(
              refreshTokenCookie.value
            );

            try {
              if (accessTokenResponse.accessToken) {
                response.cookies.set(
                  "access_token",
                  accessTokenResponse.accessToken,
                  {
                    secure: true,
                    sameSite: "strict",
                    path: "/",
                  }
                );
              } else {
                throw new Error("Failed to get access token");
              }
            } catch {
              response.cookies.delete("refresh_token");
              return NextResponse.redirect(new URL("/login", request.url));
            }
          }
        }
      } catch {
        response.cookies.delete("refresh_token");
        return NextResponse.redirect(new URL("/login", request.url));
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
