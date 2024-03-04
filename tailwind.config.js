const fs = require('fs');

/** @type {import('tailwindcss').Config} */
module.exports = {
  purge: [
    './**/*.html',
    './**/*.rs',
    './**/*.md'
  ],
  content: [],
  theme: {
    extend: {},
  },
  plugins: [
    require('tailwind-typewriter')({
        wordsets: {
            aboutme: {
              words: [fs.readFileSync('res/content/aboutme.md', 'utf8')],
              writeSpeed: 0.05,
              pauseBetween: 1,
              repeat: 0,
              eraseSpeed: 0,
              caretWidth: '3px'
            }
        }
    })
  ],
}
