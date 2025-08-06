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
import { useEffect } from "react";
import { getToken } from "./api/http";
import { getUserPage } from "./api/user";
import { useQuery } from "@tanstack/react-query";
import { toast, Toaster } from "sonner";
import { useNavigate } from "react-router-dom";

export default function Page() {
  const navigate = useNavigate();

  useEffect(() => {
    if (getToken() === null) {
      navigate("/login");
    }
  }, [navigate]);

  const { data, isLoading, error } = useQuery({
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

  const userNames = data?.data?.items?.map((user) => user.name) || [];

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
              onItemSelect={(item, index) => console.log(item, index)}
              showGradients={true}
              enableArrowNavigation={true}
              displayScrollbar={true}
            />
          )}
        </div>
      </SidebarInset>
      <Toaster richColors />
    </SidebarProvider>
  );
}
