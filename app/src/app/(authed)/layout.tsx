import { getMe } from "@/actions/me";
import { MeProvider } from "@/contexts/me-context";
import { redirect } from "next/navigation";
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/app-sidebar";

export default async function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const me = await getMe();

  if (me.organizations.length === 0) {
    return redirect("/organizations/new");
  }

  return (
    <SidebarProvider>
      <AppSidebar />
      <main className="min-h-screen">
        <SidebarTrigger />
        <MeProvider initialMe={me}>{children}</MeProvider>
      </main>
    </SidebarProvider>
  );
}
