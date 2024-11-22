import { Cookies } from "@/lib/cookies";
import { redirect } from "@tanstack/react-router";
import { createMiddleware, createServerFn } from "@tanstack/start";
import { deleteCookie, getCookie, setCookie } from "vinxi/http";
import { z } from "zod";

const GetMeSchema = z.object({
  id: z.string(),
  email: z.string(),
  organizations: z.array(
    z.object({
      id: z.string(),
      name: z.string(),
    })
  ),
});

export type Me = z.infer<typeof GetMeSchema>;

const accessTokenResponseSchema = z.object({
  accessToken: z.string(),
});

async function getAccessToken(refreshToken: string) {
  return await fetch(process.env.API_URL + "/api/auth/access_token", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ refreshToken: refreshToken }),
  }).then(async (res) => {
    if (!res.ok) {
      throw new Error("Failed to get access token");
    }
    const json = await res.json();
    return accessTokenResponseSchema.parse(json);
  });
}

export const authMiddleware = createMiddleware()
  .server(async ({ next }) => {
    const refreshToken = getCookie(Cookies.REFRESH_TOKEN);
    if (!refreshToken) {
      deleteCookie(Cookies.ACCESS_TOKEN);
      throw redirect({
        to: "/login",
      });
    }

    let token: z.infer<typeof accessTokenResponseSchema>;
    try {
      token = await getAccessToken(refreshToken);
    } catch {
      deleteCookie(Cookies.ACCESS_TOKEN);
      throw redirect({
        to: "/login",
      });
    }
    setCookie(Cookies.ACCESS_TOKEN, token.accessToken);

    return next({});
  })
  .clientAfter(async ({ next }) => {
    return next({
      headers: { authorization: `Bearer xxx` },
    });
  });

const getMe = createServerFn({ method: "GET" })
  .middleware([authMiddleware])
  .handler(async () => {
    // const accessToken = cookieStore.get(Cookies.ACCESS_TOKEN)?.value;
    const accessToken = getCookie(Cookies.ACCESS_TOKEN);

    const me = await fetch(process.env.API_URL + "/api/users/me", {
      method: "Get",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${accessToken}`,
      },
    }).then(async (res): Promise<Me> => {
      if (res.ok) {
        const json = await res.json();

        return GetMeSchema.parse(json);
      }

      throw new Error("Failed to fetch me");
    });

    return me;
  });

export { getMe };
