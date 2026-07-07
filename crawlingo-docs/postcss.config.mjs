// postcss.config.mjs
export default {
  plugins: {
    tailwindcss: {},
    "postcss-preset-env": {
      features: {
        "custom-properties": false,
      },
    },
  },
};