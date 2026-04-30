import { create } from "zustand";

interface UIState {
  sidebarCollapsed: boolean;
  colorScheme: "light" | "dark";
  toggleSidebar: () => void;
  toggleColorScheme: () => void;
  setSidebarCollapsed: (collapsed: boolean) => void;
}

export const useUIStore = create<UIState>()((set) => ({
  sidebarCollapsed: false,
  colorScheme: "light",

  toggleSidebar: () =>
    set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed })),

  toggleColorScheme: () =>
    set((state) => ({
      colorScheme: state.colorScheme === "light" ? "dark" : "light",
    })),

  setSidebarCollapsed: (collapsed) => set({ sidebarCollapsed: collapsed }),
}));
