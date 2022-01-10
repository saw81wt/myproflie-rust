module.exports = {
  mode: 'jit',
  purge: {
    content: [
      './src/**/*.rs',
      './src/*.rs',
      './static/**/*.hbs',
    ],
    options: {
      safelist: [
        /data-theme$/,
      ]
    },
  },
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
