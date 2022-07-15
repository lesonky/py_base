/* eslint-disable @typescript-eslint/no-var-requires */
/* eslint-disable global-require */
import { defineConfig } from 'windicss/helpers';
import colors from 'windicss/colors';
import plugin from 'windicss/plugin';

// https://cn.windicss.org/guide/configuration.html
export default defineConfig({
  darkMode: 'class', // or 'media'
  extract: {
    // accepts globs and file paths relative to project root
    include: ['index.html', 'src/**/*.{vue,html,jsx,tsx}'],
    exclude: ['node_modules/**/*', '.git/**/*'],
  },
  theme: {
    extend: {
      screens: {
        'xs': '480px',
        'sm': '576px',
        'md': '768px',
        'lg': '992px',
        'xl': '1200px',
        '2xl': '1600px',
      },
      colors: {
        gray: colors.coolGray,
        blue: colors.sky,
        red: colors.rose,
        pink: colors.fuchsia,
      },
    },
  },
  plugins: [
    plugin(({ addComponents }) => {
      const generalCard = {
        '.general-card': {
          'border': 'none',
          'borderRadius': '4px',
          '& > .arco-card-header': {
            height: 'auto',
            padding: '20px',
            border: 'none',
          },
          '& > .arco-card-body': {
            padding: '20px',
            paddingTop: '0',
          },
        },
      };
      addComponents(generalCard);
    }),
    require('windicss/plugin/filters'),
    require('windicss/plugin/forms'),
    require('windicss/plugin/aspect-ratio'),
    require('windicss/plugin/line-clamp'),
    require('windicss/plugin/typography')({
      modifiers: ['DEFAULT', 'sm', 'lg', 'red'],
    }),
  ],
});
