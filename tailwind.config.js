/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontSize: {
        'base': 'var(--base-font-size)',
        'xs': 'calc(var(--base-font-size) * 0.75)',
        'sm': 'calc(var(--base-font-size) * 0.875)',
        'lg': 'calc(var(--base-font-size) * 1.125)',
        'xl': 'calc(var(--base-font-size) * 1.25)',
        '2xl': 'calc(var(--base-font-size) * 1.5)',
        '3xl': 'calc(var(--base-font-size) * 1.875)',
        '4xl': 'calc(var(--base-font-size) * 2.25)',
      },
      typography: {
        DEFAULT: {
          css: {
            maxWidth: 'none',
            code: {
              color: 'var(--tw-prose-code)',
              backgroundColor: 'var(--tw-prose-pre-bg)',
              borderRadius: '0.25rem',
              paddingTop: '0.125rem',
              paddingRight: '0.25rem',
              paddingBottom: '0.125rem',
              paddingLeft: '0.25rem',
              fontWeight: '500',
            },
            'code::before': {
              content: '""',
            },
            'code::after': {
              content: '""',
            },
            pre: {
              color: 'var(--tw-prose-pre-code)',
              backgroundColor: 'var(--tw-prose-pre-bg)',
              borderRadius: '0.375rem',
              padding: '1rem',
            },
            'pre code': {
              backgroundColor: 'transparent',
              borderWidth: '0',
              borderRadius: '0',
              padding: '0',
              color: 'inherit',
              fontSize: 'inherit',
              fontWeight: 'inherit',
              lineHeight: 'inherit',
            },
            'blockquote p:first-of-type::before': {
              content: '""',
            },
            'blockquote p:last-of-type::after': {
              content: '""',
            },
          },
        },
      },
    },
  },
  plugins: [
    require("daisyui"),
    require('@tailwindcss/typography'),
  ],
  daisyui: {
    themes: [
      {
        mytheme: {
          "primary": "#3b82f6",
          "secondary": "#6366f1",
          "accent": "#2dd4bf",
          "neutral": "#1f2937",
          "base-100": "#f3f4f6",
          "info": "#0ea5e9",
          "success": "#10b981",
          "warning": "#f59e0b",
          "error": "#ef4444",
        },
      },
      "light",
      "dark",
      "cupcake",
      "bumblebee",
      "emerald",
      "corporate",
      "synthwave",
      "retro",
      "cyberpunk",
      "valentine",
      "halloween",
      "garden",
      "forest",
      "aqua",
      "lofi",
      "pastel",
      "fantasy",
      "black",
      "luxury",
      "dracula",
      "autumn",
      "business",
      "acid",
      "lemonade",
      "night",
      "coffee",
      "winter"
    ],
  },
} 