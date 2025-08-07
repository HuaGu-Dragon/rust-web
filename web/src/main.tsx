import { StrictMode, lazy, Suspense } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import LiquidChrome from "./block/Backgrounds/LiquidChrome/LiquidChrome.tsx";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "./api/query";
import { Toaster } from "sonner";
import LoadingFallback from "./LoadingFallback.tsx";

// 使用懒加载导入组件
const App = lazy(() => import("./App.tsx"));
const Login = lazy(() => import("./Login.tsx"));
const Page = lazy(() => import("./dashboard.tsx"));

createRoot(document.getElementById("root")!).render(
  <div className="w-screen h-screen relative bg-white">
    <div
      className="absolute inset-0 z-0 pointer-events-auto bg-white"
      style={{ mixBlendMode: "multiply" }}
    >
      <LiquidChrome
        baseColor={[0.2, 0.1, 0.2]}
        speed={0.97}
        amplitude={0.48}
        interactive={true}
      />
    </div>

    <StrictMode>
      <QueryClientProvider client={queryClient}>
        <BrowserRouter>
          <Suspense fallback={<LoadingFallback />}>
            <Routes>
              <Route
                path="/"
                element={
                  <div className="absolute inset-0 z-10 pointer-events-none w-auto h-screen justify-center items-center flex">
                    <div className="relative pointer-events-auto z-10">
                      <App />
                    </div>
                  </div>
                }
              />
              <Route
                path="/login"
                element={
                  <div className="absolute inset-0 z-10 pointer-events-none w-auto h-screen justify-center items-center flex">
                    <div className="relative pointer-events-auto z-10">
                      <Login />
                    </div>
                  </div>
                }
              />
              <Route path="/dashboard" element={<Page />} />
            </Routes>
          </Suspense>
        </BrowserRouter>
      </QueryClientProvider>
      <Toaster richColors />
    </StrictMode>
  </div>
);
