export interface AccessToken {
  user_id: string;
  email: string;
  exp: Date;
  iat: Date;
}

export function parseAccessToken(accessToken: string): AccessToken {
  const payload = parseJwt(accessToken);

  return {
    user_id: payload.sub as string,
    email: payload.email as string,
    exp: new Date((payload.exp as number) * 1000),
    iat: new Date((payload.iat as number) * 1000),
  };
}

export function parseJwt(token: string): Record<string, unknown> {
  const base64Url = token.split(".")[1];
  const base64 = base64Url.replace(/-/g, "+").replace(/_/g, "/");
  const jsonPayload = decodeURIComponent(
    atob(base64)
      .split("")
      .map(function (c) {
        return "%" + ("00" + c.charCodeAt(0).toString(16)).slice(-2);
      })
      .join("")
  );

  return JSON.parse(jsonPayload);
}
