import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";
import LiquidChrome from "./block/Backgrounds/LiquidChrome/LiquidChrome.tsx";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import Login from "./Login.tsx";

createRoot(document.getElementById("root")!).render(
  <div className="w-screen h-screen overflow-hidden relative">
    <div
      className="absolute inset-0 z-0 pointer-events-auto"
      style={{ mixBlendMode: "multiply" }}
    >
      <LiquidChrome
        baseColor={[0.2, 0.1, 0.2]}
        speed={0.97}
        amplitude={0.48}
        interactive={true}
      />
    </div>

    <div className="absolute inset-0 z-10 w-auto pointer-events-none flex justify-center items-center">
      <div className="relative pointer-events-auto w-auto z-10">
        <StrictMode>
          <BrowserRouter>
            <Routes>
              <Route path="/" element={<App />} />
              <Route path="/login" element={<Login />} />
            </Routes>
          </BrowserRouter>
        </StrictMode>
      </div>
    </div>
  </div>
);
