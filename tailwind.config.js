/** @type {import('tailwindcss').Config} */
module.exports = {
  purge: [
    './**/*.html',
    './**/*.rs'
  ],
  content: [],
  theme: {
    extend: {
        keyframes: {
          typing: {
            "0%": {
              width: "0%",
              visibility: "hidden"
            },
            "100%": {
              width: "100%"
            }
          },
          blink: {
            "50%": {
              borderColor: "transparent"
            },
            "100%": {
              borderColor: "white"
            }  
          }
        },
        animation: {
          typing: "typing 4s steps(40) forwards, blink .7s"
        }
      },
  },
  plugins: [],
}
