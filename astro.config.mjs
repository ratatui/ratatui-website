import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import starlightLinksValidator from "starlight-links-validator";
import remarkMermaid from "astro-diagram/remark-mermaid";
import remarkIncludeCode from "/src/plugin/remark-code-import";
import emoji from "remark-emoji";
import { remarkKroki } from "remark-kroki";
import tailwind from "@astrojs/tailwind";
import remarkYoutube from "remark-youtube";

// https://astro.build/config
export default defineConfig({
  // change this when switching to ratatui.rs
  site: "https://ratatui-org.github.io",
  // remove this when switching to ratatui.rs
  base: "/ratatui-website",
  markdown: {
    remarkPlugins: [
      remarkIncludeCode,
      emoji,
      [
        remarkKroki,
        {
          server: "https://kroki.io/",
          output: "inline-svg",
        },
      ],
      remarkYoutube,
    ],
  },
  integrations: [
    starlightLinksValidator(),
    starlight({
      title: "Ratatui",
      customCss: ["/src/tailwind.css"],
      logo: {
        src: "./src/assets/logo.png",
        replacesTitle: true,
      },
      favicon: "./src/assets/logo.png",
      social: {
        github: "https://github.com/ratatui-org/ratatui",
      },
      sidebar: [
        {
          label: "Introduction",
          link: "/introduction/",
        },
        {
          label: "Installation",
          link: "/installation/",
        },
        {
          label: "Tutorial",
          items: [
            {
              label: "Hello World",
              link: "/tutorial/hello-world/",
            },
            {
              label: "Counter App",
              items: [
                {
                  label: "Single Function",
                  link: "/tutorial/counter-app/single-function",
                },
                {
                  label: "Multiple Functions",
                  link: "/tutorial/counter-app/multiple-functions",
                },
                {
                  label: "Multiple Files",
                  items: [
                    {
                      label: "Introduction",
                      link: "/tutorial/counter-app/multiple-files",
                    },
                    {
                      label: "app.rs",
                      link: "/tutorial/counter-app/multiple-files/app",
                    },
                    {
                      label: "ui.rs",
                      link: "/tutorial/counter-app/multiple-files/ui",
                    },
                    {
                      label: "event.rs",
                      link: "/tutorial/counter-app/multiple-files/event",
                    },
                    {
                      label: "tui.rs",
                      link: "/tutorial/counter-app/multiple-files/tui",
                    },
                    {
                      label: "update.rs",
                      link: "/tutorial/counter-app/multiple-files/update",
                    },
                    {
                      label: "main.rs",
                      link: "/tutorial/counter-app/multiple-files/main",
                    },
                  ],
                },
              ],
            },
            {
              label: "JSON Editor",
              items: [
                {
                  label: "Introduction",
                  link: "/tutorial/json-editor",
                },
                {
                  label: "App.rs",
                  link: "/tutorial/json-editor/app",
                },
                {
                  label: "Main.rs",
                  link: "/tutorial/json-editor/main",
                },
                {
                  label: "Ui.rs",
                  items: [
                    { label: "UI", link: "/tutorial/json-editor/ui" },
                    { label: "Ui.rs - Main", link: "/tutorial/json-editor/ui-main" },
                    { label: "Ui.rs - Editing", link: "/tutorial/json-editor/ui-editing" },
                    { label: "Ui.rs - Exit", link: "/tutorial/json-editor/ui-exit" },
                  ],
                },
                {
                  label: "Conclusion",
                  link: "/tutorial/json-editor/closing-thoughts",
                },
              ],
            },
            {
              label: "Async Counter App",
              items: [
                {
                  label: "Introduction",
                  link: "/tutorial/counter-async-app",
                },
                {
                  label: "Async KeyEvents",
                  link: "/tutorial/counter-async-app/async-event-stream",
                },
                {
                  label: "Async Render",
                  link: "/tutorial/counter-async-app/full-async-events",
                },
                {
                  label: "Introducing Actions",
                  link: "/tutorial/counter-async-app/actions",
                },
                {
                  label: "Async Actions",
                  link: "/tutorial/counter-async-app/full-async-actions",
                },
                {
                  label: "Conclusion",
                  link: "/tutorial/counter-async-app/conclusion",
                },
              ],
            },
          ],
        },
        {
          label: "Concepts",
          items: [
            {
              label: "Layout",
              link: "/concepts/layout",
            },
            {
              label: "Event Handling",
              link: "/concepts/event-handling",
            },
            {
              label: "Rendering",
              items: [
                {
                  label: "Introduction",
                  link: "/concepts/rendering",
                },
                {
                  label: "Under the hood",
                  link: "/concepts/rendering-under-the-hood",
                },
              ],
            },
            {
              label: "Application Patterns",
              items: [
                {
                  label: "The Elm Architecture",
                  link: "/concepts/application-patterns/the-elm-architecture",
                },
                {
                  label: "Component Architecture",
                  link: "/concepts/application-patterns/component-architecture",
                },
                {
                  label: "Flux Architecture",
                  link: "/concepts/application-patterns/flux-architecture",
                },
              ],
            },
            {
              label: "Backends",
              items: [
                {
                  label: "Comparison",
                  link: "/concepts/backends/comparison",
                },
                {
                  label: "Raw Mode",
                  link: "/concepts/backends/raw-mode",
                },
                {
                  label: "Alternate Screen",
                  link: "/concepts/backends/alternate-screen",
                },
                {
                  label: "Mouse Capture",
                  link: "/concepts/backends/mouse-capture",
                },
              ],
            },
          ],
        },
        {
          label: "How To",
          items: [
            {
              label: "Layout UIs",
              items: [
                {
                  label: "Dynamic Layouts",
                  link: "/how-to/layout/dynamic",
                },
                {
                  label: "Center a Rect",
                  link: "/how-to/layout/center-a-rect",
                },
                {
                  label: "Collapse Borders",
                  link: "/how-to/layout/collapse-borders",
                },
              ],
            },
            {
              label: "Render UIs",
              items: [
                {
                  label: "Display Text",
                  link: "/how-to/render/display-text",
                },
                {
                  label: "Style Text",
                  link: "/how-to/render/style-text",
                },
                {
                  label: "Overwrite Regions",
                  link: "/how-to/render/overwrite-regions",
                },
              ],
            },
            {
              label: "Use Widgets",
              items: [
                {
                  label: "Paragraph",
                  link: "/how-to/widgets/paragraph",
                },
                {
                  label: "Block",
                  link: "/how-to/widgets/block",
                },
                {
                  label: "Custom",
                  link: "/how-to/widgets/custom",
                },
              ],
            },
            {
              label: "Develop Applications",
              items: [
                {
                  label: "CLI arguments",
                  link: "/how-to/develop-apps/cli-arguments",
                },
                {
                  label: "Configuration Directories",
                  link: "/how-to/develop-apps/config-directories",
                },
                {
                  label: "Logging with Tracing",
                  link: "/how-to/develop-apps/logging-with-tracing",
                },
                {
                  label: "Terminal and Event handler",
                  link: "/how-to/develop-apps/terminal-and-event-handler",
                },
                {
                  label: "Setup Panic Hooks",
                  link: "/how-to/develop-apps/setup-panic-hooks",
                },
                {
                  label: "Better Panic Hooks",
                  link: "/how-to/develop-apps/better-panic-hooks",
                },
                {
                  label: "Migrate from tui-rs",
                  link: "/how-to/develop-apps/migrate-from-tui-rs",
                },
              ],
            },
          ],
        },
        {
          label: "Showcase",
          link: "/showcase",
        },
        {
          label: "FAQ",
          link: "/faq",
        },
        {
          label: "Highlights",
          items: [
            {
              label: "v0.24",
              link: "/highlights/v024",
            },
            {
              label: "v0.23",
              link: "/highlights/v023",
            },
            {
              label: "v0.22",
              link: "/highlights/v022",
            },
            {
              label: "v0.21",
              link: "/highlights/v021",
            },
          ],
        },
        {
          label: "Features",
          link: "/features",
        },
        {
          label: "References",
          link: "/references",
        },
        {
          label: "Developer Guide",
          items: [
            {
              label: "Ratatui",
              link: "/developer-guide/ratatui",
            },
            {
              label: "Website",
              link: "/developer-guide/website",
            },
          ],
        },
      ],
      editLink: {
        baseUrl: "https://github.com/ratatui-org/ratatui-website/edit/main/",
      },
    }),
    tailwind({
      applyBaseStyles: false,
    }),
  ],
});
