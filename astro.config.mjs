import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import starlightLinksValidator from 'starlight-links-validator'
import remarkMermaid from '@southball/remark-mermaid';
import remarkIncludeCode from './src/plugin/remark-code-import';
import emoji from 'remark-emoji';

// https://astro.build/config
export default defineConfig({
  site: "https://ratatui-org.github.io", // change this when switching to ratatui.rs
  base: "/ratatui-website", // remove this when switching to ratatui.rs
  markdown: {
    remarkPlugins: [
      remarkIncludeCode,
      remarkMermaid,
      emoji,
    ],
  },
  integrations: [
    starlightLinksValidator(),
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
          label: 'Installation',
          link: '/installation/'
        },
        {
          label: 'Tutorial',
          items: [
            {
              label: 'Hello World',
              link: '/tutorial/hello-world/'
            },
            {
              label: 'Counter App',
              items: [
                {
                  label: 'Single Function',
                  link: '/tutorial/counter-app/single-function'
                },
                {
                  label: 'Multiple Functions',
                  link: '/tutorial/counter-app/multiple-functions'
                },
                {
                  label: 'Multiple Files',
                  items: [
                    { label: 'Introduction', link: '/tutorial/counter-app/multiple-files' },
                    { label: 'app.rs', link: '/tutorial/counter-app/multiple-files/app' },
                    { label: 'ui.rs', link: '/tutorial/counter-app/multiple-files/ui' },
                    { label: 'event.rs', link: '/tutorial/counter-app/multiple-files/event' },
                    { label: 'tui.rs', link: '/tutorial/counter-app/multiple-files/tui' },
                    { label: 'update.rs', link: '/tutorial/counter-app/multiple-files/update' },
                    { label: 'main.rs', link: '/tutorial/counter-app/multiple-files/main' }
                  ]
                }
              ]
            }],
        },
        {
          label: 'Concepts',
          items: [
            {
              label: 'Rendering',
              link: './concepts/rendering',
              items: [
                {
                  label: 'Under the hood',
                  link: './concepts/rendering-under-the-hood'
                }
              ]
            },
            {
              label: 'Layout',
              link: './concepts/layout'
            },
            {
              label: 'Application Patterns',
              items: [
                {
                  label: 'The Elm Architecture',
                  link: './concepts/application-patterns/the-elm-architecture'
                },
                {
                  label: 'Component Architecture',
                  link: './concepts/application-patterns/component-architecture'
                },
                {
                  label: 'Flux Architecture',
                  link: './concepts/application-patterns/flux-architecture'
                }
              ]
            },
            {
              label: 'Backends',
              items: [
                {
                  label: 'Comparison',
                  link: './concepts/backends/comparison'
                },
                {
                  label: 'Raw Mode',
                  link: './concepts/backends/raw-mode'
                },
                {
                  label: 'Alternate Screen',
                  link: './concepts/backends/alternate-screen'
                },
                {
                  label: 'Mouse Capture',
                  link: './concepts/backends/mouse-capture'
                }
              ]
            },
            {
              label: 'Event Handling',
              link: './concepts/event_handling'
            }
          ]
        }
      ],
      editLink: {
        baseUrl: 'https://github.com/ratatui-org/ratatui-website/edit/main/',
      },
    }),
  ],
});
