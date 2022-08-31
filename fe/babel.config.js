module.exports = {
  plugins: [
    '@vue/babel-plugin-jsx',
    [
      'import',
      {
        libraryName: '@icon-park/vue-next',
        libraryDirectory: 'es/icons',
        camel2DashComponentName: false,
      },
    ],
  ],
};
