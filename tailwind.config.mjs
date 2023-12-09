import starlightPlugin from "@astrojs/starlight-tailwind";
import colors from "tailwindcss/colors";

// Generated color palettes
const accent = { 200: '#b5cbe7', 600: '#2a6cba', 900: '#173355', 950: '#14253a' };
const gray = { 100: '#f1f7ff', 200: '#e4eeff', 300: '#b5c3d9', 400: '#738cb6', 500: '#41587f', 700: '#22385c', 800: '#112648', 900: '#0e1828' };


/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}"],
  theme: {
    extend: {
      colors: { accent, gray },
    },
  },
  plugins: [require("tailwindcss-animate"), starlightPlugin()],
};
