class ThemeStore {
  isDark = $state(true);

  constructor() {
    if (typeof window !== "undefined") {
      const saved = localStorage.getItem("moniqa_theme");
      if (saved) {
        this.isDark = saved === "dark";
      } else {
        this.isDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
      }
      this.apply();
    }
  }

  toggle() {
    this.isDark = !this.isDark;
    if (typeof window !== "undefined") {
      localStorage.setItem("moniqa_theme", this.isDark ? "dark" : "light");
      this.apply();
    }
  }

  apply() {
    if (typeof document !== "undefined") {
      const root = document.documentElement;
      if (this.isDark) {
        root.classList.add("dark");
        root.classList.remove("light");
        root.style.colorScheme = "dark";
      } else {
        root.classList.remove("dark");
        root.classList.add("light");
        root.style.colorScheme = "light";
      }
    }
  }
}

export const theme = new ThemeStore();
