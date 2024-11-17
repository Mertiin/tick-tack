"use client";

import { login } from "@/api/auth/login";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Calendar1Icon } from "lucide-react";
import { useState } from "react";

export default function Home() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  const onLogin = async () => {
    const result = await login(email, password);

    console.log(result);
  };

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
        <div className="flex flex-col gap-2 w-[375px]">
          <Input
            placeholder="Email"
            onChange={(e) => {
              setEmail(e.target.value);
            }}
          />
          <Input
            type="password"
            placeholder="Password"
            onChange={(e) => {
              setPassword(e.target.value);
            }}
          />
          <Button onClick={onLogin}>Login</Button>
        </div>
      </div>
    </div>
  );
}
