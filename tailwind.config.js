/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      keyframes: {
        "cat-say": {
          "0%": {
            transform: "translateY(0.5rem)",
            opacity: "0",
          },
          "25%, 70%": {
            transform: "translateY(0)",
            opacity: "1",
          },
          "100%": {
            transform: "translateY(-0.5rem)",
            opacity: "0",
          }
        },
      },
    },
  },
};
