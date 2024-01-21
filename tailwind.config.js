/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      keyframes: {
        typing: {
          from: {
            width: "0",
          },
          to: {
            width: "17ch",
          },
        },
        blink: {
          "0%": {
            opacity: "0",
          },
        },
      },
      animation: {
        typing: "typing 1.5s steps(34, end) forwards",
        blink: "blink 1s steps(2) infinite",
      },
    },
  },
  plugins: [],
};
