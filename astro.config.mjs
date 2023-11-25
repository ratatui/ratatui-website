import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
  site: "https://ratatui-org.github.io", // change this when switching to ratatui.rs
  base: "/ratatui-website", // remove this when switching to ratatui.rs
  integrations: [
    starlight({
      title: 'Ratatui',
      social: {
        github: 'https://github.com/ratatui-org/ratatui',
      },
      sidebar: [
        {
          label: 'Introduction',
          link: '/introduction/'
        },
        {
          label: 'Tutorial',
          autogenerate: { directory: 'tutorial' },
        },
      ],
      editLink: {
        baseUrl: 'https://github.com/ratatui-org/ratatui-website/edit/main/',
      },
    }),
  ],
});
