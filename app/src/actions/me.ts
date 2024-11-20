"use server";

import { Cookies } from "@/lib/cookies";
import { cookies } from "next/headers";
import { redirect } from "next/navigation";
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

type Me = z.infer<typeof GetMeSchema>;

const getMe = async () => {
  const cookieStore = await cookies();
  const accessToken = cookieStore.get(Cookies.ACCESS_TOKEN)?.value;

  let result: Me;
  try {
    result = await fetch("http://localhost:3001/api/users/me", {
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
  } catch (e) {
    console.error(e);
    cookieStore.delete(Cookies.ACCESS_TOKEN);
    cookieStore.delete(Cookies.REFRESH_TOKEN);

    return redirect("/login");
  }

  return result;
};

export { getMe, type Me };
