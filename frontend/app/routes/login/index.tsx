import { createFileRoute } from "@tanstack/react-router";
import { Auth } from "@/features/auth/components/auth";
import { Calendar1Icon } from "lucide-react";

export const Route = createFileRoute("/login/")({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <main className="min-h-screen">
      <div className="grid grid-cols-1 lg:grid-cols-[1fr_minmax(700px,_50%)] h-screen">
        <div className="hidden lg:block bg-gray-700 bg-center">
          <div className="w-full h-full  flex flex-col justify-between p-16">
            <div className="flex justify-left items-center gap-2">
              <Calendar1Icon size={36} />
              <h1 className="text-2xl text-shadow-sm font-semibold">
                TickTime
              </h1>
            </div>
            <div className="flex flex-col gap-2 pr-12">
              <h2 className="text-xl">
                “The best way to predict the future is to invent it yourself,
                starting today. Take action and make it happen.”
              </h2>
              <span className="text">Alan Kay</span>
            </div>
          </div>
        </div>
        <div className="flex flex-col items-center h-full justify-center gap-6">
          <Auth />
        </div>
      </div>
    </main>
  );
}
