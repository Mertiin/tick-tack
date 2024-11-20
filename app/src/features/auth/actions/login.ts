"use server";
import { z } from "zod";
import { cookies } from "next/headers";
import { Cookies } from "@/lib/cookies";
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
    super(message, {});
    this.name = type;
    this.message = message;
  }
}

const login = async (email: string, password: string) => {
  try {
    const parsedResult = await fetch(
      process.env.NEXT_PUBLIC_API_URL + "/api/auth/login",
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

    const cookieStore = await cookies();

    cookieStore.set(Cookies.REFRESH_TOKEN, parsedResult.refreshToken, {
      httpOnly: true,
      sameSite: "strict",
      expires: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000), // 30 days
      partitioned: true,
      secure: true,
    });

    cookieStore.set(Cookies.ACCESS_TOKEN, parsedResult.accessToken, {
      sameSite: "strict",
      secure: true,
    });
  } catch (e) {
    if (e instanceof AuthError) {
      throw e;
    }

    console.log(e);

    throw new Error("Failed to login");
  }
};

const register = async (email: string, password: string) => {
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

    const cookieStore = await cookies();

    cookieStore.set(Cookies.REFRESH_TOKEN, parsedResult.refreshToken, {
      httpOnly: true,
      sameSite: "strict",
      expires: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000), // 30 days
      partitioned: true,
      secure: true,
    });

    cookieStore.set(Cookies.ACCESS_TOKEN, parsedResult.accessToken, {
      sameSite: "strict",
      secure: true,
    });
  } catch (e) {
    if (e instanceof AuthError) {
      throw e;
    }

    throw new Error("Failed to register");
  }
};

export { login, register, type AuthErrorType, AuthError };
