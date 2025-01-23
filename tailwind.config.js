const fs = require('fs');

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './res/**/*.{md,yaml}',
    './src/**/*.rs',
    './index.html',
  ],
  theme: {
    extend: {
      fontFamily: {
        'sans': ['Raleway', 'sans-serif'],
        'serif': ['Raleway', 'serif'],
        'mono': ['OCR A Std', 'monospace'],
      },
      scale: {
        '35': '0.35',
        '85': '0.85',
      }
    },
  },
}
