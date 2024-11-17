"use client";

import { login } from "@/actions/auth/login";
import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Calendar1Icon } from "lucide-react";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { useRouter } from "next/navigation";

const formSchema = z.object({
  email: z.string().email(),
  password: z.string().min(6).max(50),
});

export default function Home() {
  const router = useRouter();
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      email: "",
      password: "",
    },
  });

  async function onSubmit(values: z.infer<typeof formSchema>) {
    const result = await login(values.email, values.password);

    if ("error" in result) {
      alert(result.error);
      return;
    }

    sessionStorage.setItem("accessToken", result.accessToken);
    router.push("/");
  }

  return (
    <div className="grid grid-cols-1 lg:grid-cols-[1fr_minmax(700px,_auto)] h-screen">
      <div className="hidden lg:block bg-[url('/img/loginbg.png')] bg-center">
        <div className="w-full h-full  flex flex-col justify-between p-16">
          <div className="flex justify-left items-center gap-2">
            <Calendar1Icon size={36} />
            <h1 className="text-2xl text-shadow-sm font-semibold">Max-toe</h1>
          </div>
          <div className="flex flex-col gap-2 pr-12">
            <h2 className="text-xl text-shadow-sm">
              “The best way to predict the future is to invent it yourself,
              starting today. Take action and make it happen.”
            </h2>
            <span className="text drop-shadow-md">Alan Kay</span>
          </div>
        </div>
      </div>
      <div className="border-l border-slate-700 flex flex-col items-center h-full justify-center gap-6">
        <div className="flex flex-col items-center">
          <h1 className="text-2xl">Login</h1>
          <span className="text-sm text-primary/60">
            Enter your email and password to access your account.
          </span>
        </div>
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSubmit)}
            className="space-y-2 w-[350px]"
          >
            <FormField
              control={form.control}
              name="email"
              render={({ field }) => (
                <FormItem>
                  <FormControl>
                    <Input placeholder="example@example.com" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="password"
              render={({ field }) => (
                <FormItem>
                  <FormControl>
                    <Input placeholder="Password" type="password" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <Button type="submit" className="w-full">
              Login
            </Button>
          </form>
        </Form>
      </div>
    </div>
  );
}
