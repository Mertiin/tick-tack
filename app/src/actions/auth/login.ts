"use server";
import { z } from "zod";
import { cookies } from "next/headers";

const LoginResultSchema = z.object({
  token: z.string(),
  refresh_token: z.string(),
  status: z.string(),
});

const login = async (email: string, password: string) => {
  try {
    const result = await fetch("http://localhost:3001/api/auth/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ email, password }),
    }).then((res) => res.json());

    console.log(result);

    const parsedResult = LoginResultSchema.parse(result);
    console.log(parsedResult);
    const cookieStore = await cookies();

    cookieStore.set("refresh_token", parsedResult.token);

    return {
      accessToken: parsedResult.token,
    };
  } catch (e) {
    console.error(e);
    return {
      error: "Failed to login",
    };
  }
};

export { login };
