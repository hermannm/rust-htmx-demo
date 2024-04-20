/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/app/**/*.rs"],
  theme: {
    fontFamily: {
      main: ["sans-serif"],
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
