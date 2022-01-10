module.exports = ({ file, options, env }) => {
  return {
    plugins: [
      require("postcss-import"),
      require("tailwindcss"),
      require("autoprefixer")
    ]
  };
};
