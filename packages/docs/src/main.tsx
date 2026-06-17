import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "@elmethis/core/tokens.css";
import "@elmethis/react/style.css";
import "./index.css";
import App from "./App.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
