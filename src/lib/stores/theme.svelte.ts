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
      if (this.isDark) {
        document.documentElement.classList.add("dark");
      } else {
        document.documentElement.classList.remove("dark");
      }
    }
  }
}

export const theme = new ThemeStore();
