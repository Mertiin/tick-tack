import { getMe } from "@/actions/me";
import { MeProvider } from "@/contexts/me-context";
import { redirect } from "next/navigation";

export default async function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const me = await getMe();

  if (me.organizations.length === 0) {
    return redirect("/organizations/new");
  }

  return <MeProvider initialMe={me}>{children}</MeProvider>;
}
