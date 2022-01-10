module.exports = {
  mode: 'jit',
  purge: [
    './src/**/*.hbs',
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
