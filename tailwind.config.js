/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/app/**/*.rs"],
  theme: {
    fontFamily: {
      main: ["Roboto Mono", "Consolas", "Courier New", "sans-serif"],
    },
    extend: {
      colors: {
        /** From https://github.com/morhetz/gruvbox#palette */
        "gruvbox-fg": "#ebdbb2",
        "gruvbox-bg0": "#282828",
        "gruvbox-bg2": "#504945",
        "gruvbox-bg3": "#665c54",
        "gruvbox-gray": "#928374",
      },
    },
    screens: {
      xs: "480px",
      sm: "624px",
      md: "768px",
      "md-lg": "896px",
      lg: "1024px",
      xl: "1280px",
    },
  },
  plugins: [],
};
