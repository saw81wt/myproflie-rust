module.exports = {
  mode: 'jit',
  content: [
    './src/**/*.rs',
    './static/**/*.hbs',
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require('daisyui'),
  ],
  daisyui: {
    styled: true,
    themes: true,
    base: true,
    utils: true,
    logs: false,
    rtl: false,
  },
}
