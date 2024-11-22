import { Cookies } from "@/lib/cookies";
import { createServerFn } from "@tanstack/start";
import { deleteCookie, getCookie } from "vinxi/http";

const logout = createServerFn({ method: "POST" }).handler(async () => {
  const refreshToken = getCookie(Cookies.REFRESH_TOKEN);
  if (!refreshToken) {
    return {};
  }

  await fetch(process.env.API_URL + "/api/auth/logout", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ refresh_token: refreshToken }),
  });

  deleteCookie(Cookies.ACCESS_TOKEN);
  deleteCookie(Cookies.REFRESH_TOKEN, {
    httpOnly: true,
    sameSite: "strict",
    partitioned: true,
    secure: true,
  });

  return {};
});

export { logout };
