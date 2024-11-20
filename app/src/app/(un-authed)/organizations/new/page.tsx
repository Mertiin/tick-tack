"use client";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { z } from "zod";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
// import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import Cookies from "js-cookie";

const NewOrganizationSchema = z.object({
  name: z.string().min(3).max(50),
});

export default function NewOrganizationPage() {
  // const router = useRouter();
  const form = useForm<z.infer<typeof NewOrganizationSchema>>({
    resolver: zodResolver(NewOrganizationSchema),
    defaultValues: {
      name: "",
    },
  });

  const onSubmit = (values: z.infer<typeof NewOrganizationSchema>) => {
    const token = Cookies.get("access_token");

    fetch(process.env.NEXT_PUBLIC_API_URL + "/api/organizations", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(values),
    }).then((res) => {
      if (res.ok) {
        // router.push("/");
      }
    });
  };

  return (
    <div className="flex flex-col h-screen justify-center items-center">
      <div className="flex flex-col items-center gap-4 w-[350px]">
        <div className="text-center">
          <h1 className="text-2xl">Almost done</h1>
          <p className="text-sm text-primary/60">
            You just need to create a organization
          </p>
        </div>
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSubmit)}
            className="space-y-2 w-full"
          >
            <FormField
              control={form.control}
              name="name"
              render={({ field }) => (
                <FormItem>
                  <FormControl>
                    <Input placeholder="Org AB" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <Button type="submit" className="w-full">
              Create organization
            </Button>
          </form>
        </Form>
      </div>
    </div>
  );
}
