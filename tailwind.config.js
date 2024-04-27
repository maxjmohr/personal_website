const fs = require('fs');

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ['./**/*.{html,md,rs,yaml}']
  },
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
            },
            langs: {
                words: ['ét un peu...', "e un po'...", 'y un poco...'],
                writeSpeed: 0.05,
                pauseBetween: 1,
                repeat: -1,
                eraseSpeed: 0.05,
                caretWidth: '3px'
              }
        }
    })
  ],
}
