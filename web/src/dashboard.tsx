import { AppSidebar } from "@/components/app-sidebar";
import { format } from "date-fns";
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
import { createUser, deleteUser, getUserPage, updateUser } from "./api/user";
import { useQuery } from "@tanstack/react-query";
import { toast } from "sonner";
import { useNavigate } from "react-router-dom";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
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
  FormLabel,
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "./components/ui/select";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "./components/ui/popover";
import { cn } from "./lib/utils";
import { CalendarIcon } from "lucide-react";
import { Calendar } from "@/components/ui/calendar";

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

  const create_Schema = z.object({
    name: z.string().min(1).max(16),
    gender: z.enum(["male", "female"]),
    account: z.string().max(16),
    password: z.string().min(6).max(16),
    phone: z.string().length(11, {
      message: "Phone number must be exactly 11 characters.",
    }),
    birthday: z.date(),
    enabled: z.boolean(),
  });

  const create_form = useForm<z.infer<typeof create_Schema>>({
    resolver: zodResolver(create_Schema),
    defaultValues: {
      name: "",
      gender: "male",
      account: "",
      password: "",
      phone: "",
      birthday: new Date(),
      enabled: false,
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

  async function onSubmitCreate(params: z.infer<typeof create_Schema>) {
    try {
      await createUser({
        name: params.name,
        account: params.account,
        password: params.password,
        gender: params.gender,
        mobile_phone: params.phone,
        birthday: format(params.birthday, "yyyy-MM-dd"),
        enabled: params.enabled,
      });

      toast.success("User created successfully");
      refetch();
    } catch (error) {
      toast.error(
        "Fail to create user: " +
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
          <div className="flex justify-between items-center w-full">
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
            <Dialog>
              <DialogTrigger asChild>
                <Button variant="outline">Add User</Button>
              </DialogTrigger>
              <DialogContent className="dark">
                <DialogHeader>
                  <DialogTitle>Add New User</DialogTitle>
                  <DialogDescription>
                    Fill in the details to create a new user account.
                  </DialogDescription>
                </DialogHeader>
                <Form {...create_form}>
                  <form onSubmit={create_form.handleSubmit(onSubmitCreate)}>
                    <div className="py-4">
                      <div className="flex w-full gap-4 mt-4">
                        <div>
                          <Label htmlFor="name">Name</Label>
                          <FormField
                            control={create_form.control}
                            name="name"
                            render={({ field }) => (
                              <FormItem>
                                <FormControl>
                                  <Input
                                    id="name"
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
                        <div>
                          <Label htmlFor="account">Account</Label>
                          <FormField
                            control={create_form.control}
                            name="account"
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
                      </div>
                      <div className="gap-4 w-full flex mt-4">
                        <div>
                          <Label htmlFor="password">Password</Label>
                          <FormField
                            control={create_form.control}
                            name="password"
                            render={({ field }) => (
                              <FormItem>
                                <FormControl>
                                  <Input
                                    id="password"
                                    {...field}
                                    type="password"
                                    required
                                  />
                                </FormControl>
                                <FormMessage />
                              </FormItem>
                            )}
                          />
                        </div>
                        <div>
                          <Label htmlFor="phone">Mobile Phone number</Label>
                          <FormField
                            control={create_form.control}
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
                      <div className="gap-4 w-full flex mt-4">
                        <FormField
                          control={create_form.control}
                          name="gender"
                          render={({ field }) => (
                            <FormItem>
                              <FormLabel>Gender</FormLabel>
                              <Select
                                onValueChange={field.onChange}
                                defaultValue={field.value}
                              >
                                <FormControl>
                                  <SelectTrigger>
                                    <SelectValue
                                      className="dark"
                                      placeholder="Select a gender"
                                    />
                                  </SelectTrigger>
                                </FormControl>
                                <SelectContent className="dark">
                                  <SelectItem value="male" className="dark">
                                    Male
                                  </SelectItem>
                                  <SelectItem value="female" className="dark">
                                    Female
                                  </SelectItem>
                                </SelectContent>
                              </Select>
                              <FormMessage />
                            </FormItem>
                          )}
                        />
                        <FormField
                          control={create_form.control}
                          name="enabled"
                          render={({ field }) => (
                            <FormItem>
                              <FormLabel>Status</FormLabel>
                              <Select
                                onValueChange={(value) =>
                                  field.onChange(value === "true")
                                }
                                defaultValue={field.value ? "true" : "false"}
                              >
                                <FormControl>
                                  <SelectTrigger>
                                    <SelectValue
                                      className="dark"
                                      placeholder="Select a status"
                                    />
                                  </SelectTrigger>
                                </FormControl>
                                <SelectContent className="dark">
                                  <SelectItem value="true" className="dark">
                                    Enabled
                                  </SelectItem>
                                  <SelectItem value="false" className="dark">
                                    Disabled
                                  </SelectItem>
                                </SelectContent>
                              </Select>
                              <FormMessage />
                            </FormItem>
                          )}
                        />
                        <FormField
                          control={create_form.control}
                          name="birthday"
                          render={({ field }) => (
                            <FormItem className="flex flex-col dark">
                              <FormLabel>Date of birth</FormLabel>
                              <Popover>
                                <PopoverTrigger asChild>
                                  <FormControl>
                                    <Button
                                      variant={"outline"}
                                      className={cn(
                                        "w-[240px] pl-3 text-left font-normal",
                                        !field.value && "text-muted-foreground"
                                      )}
                                    >
                                      {field.value ? (
                                        format(field.value, "PPP")
                                      ) : (
                                        <span>Pick a date</span>
                                      )}
                                      <CalendarIcon className="ml-auto h-4 w-4 opacity-50" />
                                    </Button>
                                  </FormControl>
                                </PopoverTrigger>
                                <PopoverContent
                                  className="w-auto p-0 dark"
                                  align="start"
                                >
                                  <Calendar
                                    mode="single"
                                    selected={field.value}
                                    onSelect={field.onChange}
                                    disabled={(date) =>
                                      date > new Date() ||
                                      date < new Date("1900-01-01")
                                    }
                                    captionLayout="dropdown"
                                  />
                                </PopoverContent>
                              </Popover>
                              <FormMessage />
                            </FormItem>
                          )}
                        />
                      </div>
                    </div>
                    <div className="mt-4 flex justify-end gap-2">
                      <DialogClose asChild>
                        <Button variant="outline">Cancel</Button>
                      </DialogClose>
                      <Button type="submit">Create User</Button>
                    </div>
                  </form>
                </Form>
              </DialogContent>
            </Dialog>
          </div>
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
