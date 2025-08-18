/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'amos-dark': '#0a0a0a',
        'amos-darker': '#050505',
        'amos-accent': '#00ff88',
        'amos-accent-dim': '#00cc66',
        'amos-neural': '#ff00ff',
        'amos-agent': '#00ffff',
        'amos-warning': '#ffaa00',
        'amos-error': '#ff0055',
      },
      animation: {
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'glow': 'glow 2s ease-in-out infinite alternate',
      },
      keyframes: {
        glow: {
          '0%': { boxShadow: '0 0 5px theme(colors.amos-accent), 0 0 10px theme(colors.amos-accent)' },
          '100%': { boxShadow: '0 0 10px theme(colors.amos-accent), 0 0 20px theme(colors.amos-accent)' },
        },
      },
    },
  },
  plugins: [],
}