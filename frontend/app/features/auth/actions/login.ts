import { z } from "zod";

import { createServerFn } from "@tanstack/start";
import { setCookie, getEvent } from "vinxi/http";
import { Cookies } from "@/lib/cookies";

const LoginRequestSchema = z.object({
  email: z.string(),
  password: z.string(),
});
type LoginRequest = z.infer<typeof LoginRequestSchema>;

const LoginResultSchema = z.object({
  accessToken: z.string(),
  refreshToken: z.string(),
  status: z.string(),
});

const RegisterResultSchema = z.object({
  id: z.string(),
  email: z.string(),
  accessToken: z.string(),
  refreshToken: z.string(),
});

type AuthErrorType = "unknown" | "invalid_credentials" | "user_exists";
class AuthError extends Error {
  constructor(message: string, type: AuthErrorType) {
    super(message);
    this.name = type;
    this.message = message;
  }
}

const login = createServerFn({ method: "POST" })
  .validator((data: LoginRequest) => {
    return LoginRequestSchema.parse(data);
  })
  .handler(async (ctx) => {
    const { email, password } = ctx.data;
    try {
      const parsedResult = await fetch(
        process.env.API_URL + "/api/auth/login",
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ email, password }),
        }
      ).then(async (res) => {
        if (res.ok) {
          const json = await res.json();

          return LoginResultSchema.parse(json);
        } else {
          if (res.status === 401) {
            throw new AuthError(
              "Invalid email or password.",
              "invalid_credentials"
            );
          } else {
            throw new AuthError("Failed to login", "unknown");
          }
        }
      });

      setCookie(getEvent(), Cookies.REFRESH_TOKEN, parsedResult.refreshToken, {
        httpOnly: true,
        sameSite: "strict",
        expires: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000), // 30 days
        partitioned: true,
        secure: true,
      });
      setCookie(getEvent(), Cookies.ACCESS_TOKEN, parsedResult.accessToken, {
        sameSite: "strict",
        secure: true,
      });
    } catch (e) {
      if (e instanceof AuthError) {
        throw e;
      }

      throw new Error("Failed to login");
    }
  });

const register = createServerFn({ method: "POST" })
  .validator((data: LoginRequest) => {
    console.log(data);
    return LoginRequestSchema.parse(data);
  })
  .handler(async (ctx) => {
    const { email, password } = ctx.data;
    try {
      const parsedResult = await fetch(
        process.env.NEXT_PUBLIC_API_URL + "/api/users",
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ email, password }),
        }
      ).then(async (res) => {
        if (res.ok) {
          const json = await res.json();

          return RegisterResultSchema.parse(json);
        } else {
          if (res.status === 409) {
            throw new AuthError("User already exists.", "user_exists");
          }
          throw new AuthError("Failed to register", "unknown");
        }
      });

      setCookie(getEvent(), Cookies.REFRESH_TOKEN, parsedResult.refreshToken, {
        httpOnly: true,
        sameSite: "strict",
        expires: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000), // 30 days
        partitioned: true,
        secure: true,
      });
      setCookie(getEvent(), Cookies.ACCESS_TOKEN, parsedResult.accessToken, {
        sameSite: "strict",
        secure: true,
      });
    } catch (e) {
      if (e instanceof AuthError) {
        throw e;
      }

      throw new Error("Failed to register");
    }
  });

export {
  login,
  register,
  type AuthErrorType,
  AuthError,
  LoginRequestSchema,
  LoginRequest,
};
