import { AppSidebar } from "@/components/app-sidebar";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import { Separator } from "@/components/ui/separator";
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from "@/components/ui/sidebar";
import AnimatedList from "./block/Components/AnimatedList/AnimatedList";
import { useEffect, useMemo, useState } from "react";
import { getToken } from "./api/http";
import { deleteUser, getUserPage, updateUser } from "./api/user";
import { useQuery } from "@tanstack/react-query";
import { toast } from "sonner";
import { useNavigate } from "react-router-dom";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "./components/ui/dialog";
import { Button } from "./components/ui/button";
import { Input } from "./components/ui/input";
import { Label } from "./components/ui/label";
import z from "zod";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormMessage,
} from "./components/ui/form";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "./components/ui/alert-dialog";

export default function Page() {
  const [selectedItem, setSelectedItem] = useState<{
    name: string;
    index: number;
  } | null>(null);
  const navigate = useNavigate();

  useEffect(() => {
    if (getToken() === null) {
      navigate("/login");
    }
  }, [navigate]);

  const { data, isLoading, error, refetch } = useQuery({
    queryKey: ["users", 1, 100],
    queryFn: () => getUserPage({ page_size: 100 }),
  });

  useEffect(() => {
    if (error) {
      if (error.message?.includes("ExpiredSignature")) {
        localStorage.removeItem("__TOKEN__");
        toast.error("Session expired, please log in again.", {
          duration: 2000,
        });
        navigate("/login");
      }
    }
  }, [error, navigate]);

  const formSchema = z.object({
    name: z.string().max(16),
    phone: z.string().length(11, {
      message: "Phone number must be exactly 11 characters.",
    }),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: "",
      phone: "",
    },
  });

  const users = useMemo(() => data?.data?.items || [], [data]);
  const userNames = useMemo(() => users.map((user) => user.name), [users]);

  useEffect(() => {
    if (selectedItem) {
      form.reset({
        name: selectedItem.name,
        phone: users[selectedItem.index]?.mobilePhone || "",
      });
    }
  }, [selectedItem, users, form]);

  function handleItemSelect(item: string, index: number) {
    console.log(`Selected item: ${item} at index ${index}`);
    setSelectedItem({ name: item, index });
  }

  async function onSubmit(params: z.infer<typeof formSchema>) {
    if (!selectedItem) return;
    try {
      const user = users[selectedItem?.index];
      await updateUser({
        id: user.id,
        name: params.name,
        mobilePhone: params.phone,
      });

      setSelectedItem(null);
      toast.success("User updated successfully");
      refetch();
    } catch (error) {
      toast.error(
        "Fail to update user: " +
          (error instanceof Error ? error.message : "Unknown error")
      );
    }
  }

  async function handleDeleteUser() {
    if (!selectedItem) return;
    console.log("Deleting user:", selectedItem);

    try {
      const userId = users[selectedItem.index]?.id;

      await deleteUser(userId);

      setSelectedItem(null);

      toast.success("User deleted successfully");

      await refetch();
    } catch (error) {
      toast.error(
        "Failed to delete user: " +
          (error instanceof Error ? error.message : "Unknown error")
      );
    }
  }

  return (
    <SidebarProvider className="dark">
      <AppSidebar className="dark flex" />
      <SidebarInset>
        <header className="bg-background sticky top-0 flex h-16 shrink-0 items-center gap-2 border-b px-4 dark">
          <SidebarTrigger className="-ml-1 dark" />
          <Separator orientation="vertical" className="mr-2 h-4 dark" />
          <Breadcrumb>
            <BreadcrumbList>
              <BreadcrumbItem className="hidden md:block">
                <BreadcrumbLink href="#">
                  Building Your Application
                </BreadcrumbLink>
              </BreadcrumbItem>
              <BreadcrumbSeparator className="hidden md:block" />
              <BreadcrumbItem>
                <BreadcrumbPage>Data Fetching</BreadcrumbPage>
              </BreadcrumbItem>
            </BreadcrumbList>
          </Breadcrumb>
        </header>
        <div className="flex flex-1 flex-col gap-4 p-4 h-[calc(100vh-4rem)] overflow-hidden">
          {isLoading ? (
            <div className="flex items-center justify-center h-full">
              <div className="animate-spin h-10 w-10 border-4 border-primary border-t-transparent rounded-full"></div>
            </div>
          ) : error ? (
            <div className="text-red-500 text-center">
              <p>Error loading user data: {error.message}</p>
            </div>
          ) : (
            <AnimatedList
              className="w-full"
              items={userNames}
              onItemSelect={handleItemSelect}
              showGradients={true}
              enableArrowNavigation={true}
              displayScrollbar={true}
            />
          )}
        </div>
      </SidebarInset>
      <Dialog
        open={selectedItem !== null}
        onOpenChange={(open) => !open && setSelectedItem(null)}
      >
        <DialogContent className="dark">
          <DialogHeader>
            <DialogTitle>User Details</DialogTitle>
            <DialogDescription>
              Information about the selected user
            </DialogDescription>
          </DialogHeader>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)}>
              {selectedItem && (
                <div className="py-4">
                  <h3 className="text-lg font-medium">
                    Account: {users[selectedItem.index]?.account || "Unknown"}
                  </h3>
                  <p className="text-muted-foreground">
                    User ID: {users[selectedItem.index]?.id || "Unknown"}
                  </p>
                  <div className="mt-4 gap-6 flex flex-col">
                    <div className="gap-6">
                      <Label htmlFor="name">Name</Label>
                      <FormField
                        control={form.control}
                        name="name"
                        render={({ field }) => (
                          <FormItem>
                            <FormControl>
                              <Input
                                id="account"
                                {...field}
                                type="text"
                                required
                              />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                    </div>
                    <div className="gap-6">
                      <Label htmlFor="phone">Mobile Phone number</Label>
                      <FormField
                        control={form.control}
                        name="phone"
                        render={({ field }) => (
                          <FormItem>
                            <FormControl>
                              <Input
                                id="phone"
                                {...field}
                                type="text"
                                required
                              />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                    </div>
                  </div>
                  <div className="mt-4 flex justify-end gap-2">
                    <Button
                      variant="outline"
                      onClick={() => setSelectedItem(null)}
                    >
                      Close
                    </Button>
                    <Button type="submit">Update User</Button>
                    <AlertDialog>
                      <AlertDialogTrigger asChild>
                        <Button
                          variant="outline"
                          className="hover:dark:bg-red-400 dark:bg-red-500"
                        >
                          Delete
                        </Button>
                      </AlertDialogTrigger>
                      <AlertDialogContent>
                        <AlertDialogHeader>
                          <AlertDialogTitle>
                            Are you absolutely sure?
                          </AlertDialogTitle>
                          <AlertDialogDescription>
                            This action cannot be undone. This will permanently
                            delete your account and remove your data from our
                            servers.
                          </AlertDialogDescription>
                        </AlertDialogHeader>
                        <AlertDialogFooter>
                          <AlertDialogCancel>Cancel</AlertDialogCancel>
                          <AlertDialogAction onClick={handleDeleteUser}>
                            Continue
                          </AlertDialogAction>
                        </AlertDialogFooter>
                      </AlertDialogContent>
                    </AlertDialog>
                  </div>
                </div>
              )}
            </form>
          </Form>
        </DialogContent>
      </Dialog>
    </SidebarProvider>
  );
}
