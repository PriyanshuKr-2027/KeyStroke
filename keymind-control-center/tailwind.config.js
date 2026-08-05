/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        claude: {
          50: '#FDF8F5',
          100: '#FAF0E8',
          200: '#F4DBCB',
          300: '#ECBA9E',
          400: '#E2936F',
          500: '#DA7756',
          600: '#C86544',
          700: '#A44F34',
          800: '#84412E',
          900: '#6E382A',
        }
      }
    },
  },
  plugins: [],
};
