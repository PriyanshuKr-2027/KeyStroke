/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        surface: "#FFFFFF",
        surfaceRaised: "#F5F5F5",
        borderSubtle: "#EBEBEB",
        textPrimary: "#111111",
        textSecondary: "#6B6B6B",
        textTertiary: "#AAAAAA",
        calloutBg: "#FAFAE8",
        toggleOn: "#22C55E",
        toggleOff: "#D1D5DB",
        destructive: "#EF4444",
        statusGreen: "#22C55E",
        statusAmber: "#F59E0B",
        statusRed: "#EF4444",
      },
      fontFamily: {
        sans: ["Inter", "-apple-system", "BlinkMacSystemFont", "sans-serif"],
        serif: ["Georgia", "serif"],
        mono: ["JetBrains Mono", "monospace"],
      },
    },
  },
  plugins: [],
};
