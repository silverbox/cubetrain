module.exports = {
  // presets: [
  //   [
  //     '@babel/preset-env',
  //     {
  //       'modules': 'false',
  //       'useBuiltIns': 'usage',
  //       'targets': '> 0.25%, not dead',
  //     }
  //   ]
  // ],
  // env: {
  //   test: {
  //     presets: [['@babel/preset-env']],
  //   },
  // }
  presets: [
    '@vue/cli-plugin-babel/preset'
  ],
  env: {
    test: {
      plugins: ["@babel/plugin-transform-modules-commonjs"]
    },
  }
}
