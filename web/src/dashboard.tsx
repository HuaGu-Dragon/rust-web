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

export default function Page() {
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
          <AnimatedList
            className="w-full"
            items={Array.from({ length: 100 }).map(
              (_, index) => "string" + index
            )}
            onItemSelect={(item, index) => console.log(item, index)}
            showGradients={true}
            enableArrowNavigation={true}
            displayScrollbar={true}
          />
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
}
