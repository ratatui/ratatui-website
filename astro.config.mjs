import partytown from "@astrojs/partytown";
import starlight from "@astrojs/starlight";
import { pluginCollapsibleSections } from "@expressive-code/plugin-collapsible-sections";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "astro/config";
import rehypeMermaid from "rehype-mermaid";
import { remarkHeadingId } from "remark-custom-heading-id";
import emoji from "remark-emoji";
import remarkSvgBob from "remark-svgbob";
import remarkYoutube from "remark-youtube";
import starlightLinksValidator from "starlight-links-validator";
import { collapsibleFrames } from "/src/plugins/collapsible-frames";
import remarkIncludeCode from "/src/plugins/remark-code-import";

// https://astro.build/config
export default defineConfig({
  site: "https://ratatui.rs",
  image: {
    service: {
      entrypoint: "astro/assets/services/sharp",
      config: {
        limitInputPixels: false,
      },
    },
  },
  prefetch: {
    prefetchAll: true,
  },
  markdown: {
    remarkPlugins: [remarkIncludeCode, emoji, remarkYoutube, remarkSvgBob, remarkHeadingId],
    rehypePlugins: [rehypeMermaid],
  },
  integrations: [
    starlight({
      title: "Ratatui",
      customCss: ["/src/tailwind.css"],
      logo: {
        dark: "./src/assets/logo-dark.png",
        light: "./src/assets/logo-light.png",
        replacesTitle: true,
      },
      favicon: "/favicon-32.png",
      head: [
        {
          tag: "meta",
          attrs: {
            property: "og:image",
            content: "/ratatui-og.png",
          },
        },
      ],
      components: {
        Head: "./src/components/Head.astro",
        Header: "./src/components/Header.astro",
      },
      plugins: [
        starlightLinksValidator({
          errorOnRelativeLinks: false,
        }),
      ],
      expressiveCode: {
        plugins: [pluginCollapsibleSections(), collapsibleFrames()],
      },
      social: [
        { icon: "github", label: "GitHub", href: "https://github.com/ratatui/ratatui" },
        { icon: "discord", label: "Discord", href: "https://discord.gg/pMCEU9hNEj" },
        { icon: "matrix", label: "Matrix", href: "https://matrix.to/#/#ratatui:matrix.org" },
        { icon: "discourse", label: "Discourse", href: "https://forum.ratatui.rs" },
        { icon: "x.com", label: "X.com", href: "https://twitter.com/ratatui_rs" },
        { icon: "blueSky", label: "Bluesky", href: "https://bsky.app/profile/ratatui.rs" },
        { icon: "mastodon", label: "Mastodon", href: "https://fosstodon.org/@ratatui_rs" },
        {
          icon: "linkedin",
          label: "LinkedIn",
          href: "https://www.linkedin.com/company/ratatui-rs",
        },
      ],
      sidebar: [
        // note that the need to order items and name the groups in the sidebar prevents
        // autogeneration except for the leaf level of the sidebar hierarchy.
        // See https://github.com/withastro/starlight/discussions/972
        // and https://github.com/withastro/starlight/issues/1223
        {
          label: "Getting Started",
          collapsed: true,
          autogenerate: {
            directory: "installation",
          },
        },
        {
          label: "Tutorials",
          collapsed: false,
          items: [
            {
              label: "Tutorials",
              link: "/tutorials/",
            },
            {
              label: "Hello Ratatui",
              link: "/tutorials/hello-ratatui/",
            },
            {
              label: "Counter App",
              collapsed: true,
              autogenerate: {
                directory: "tutorials/counter-app",
              },
            },
            {
              label: "JSON Editor",
              collapsed: true,
              autogenerate: {
                directory: "tutorials/json-editor",
              },
            },
            {
              label: "Videos",
              link: "/tutorials/videos/",
            },
            // // This material is expected to be revised significantly.
            // // Until then, it might be best to hide to avoid confusing new users.
            // {
            //   label: "Async Counter App",
            //   collapsed: true,
            //   autogenerate: { directory: "tutorials/counter-async-app" },
            // },
          ],
        },
        {
          label: "Examples",
          collapsed: true,
          autogenerate: {
            directory: "examples",
          },
        },
        {
          label: "Concepts",
          collapsed: false,
          items: [
            {
              label: "Concepts",
              link: "/concepts/",
            },
            {
              label: "Widgets",
              link: "/concepts/widgets/",
            },
            {
              label: "Layout",
              link: "/concepts/layout/",
            },
            {
              label: "Event Handling",
              link: "/concepts/event-handling/",
            },
            {
              label: "Builder Lite Pattern",
              link: "/concepts/builder-lite-pattern/",
            },
            {
              label: "Rendering",
              collapsed: true,
              autogenerate: {
                directory: "concepts/rendering",
              },
            },
            {
              label: "Application Patterns",
              collapsed: true,
              autogenerate: {
                directory: "concepts/application-patterns",
              },
            },
            {
              label: "Backends",
              collapsed: true,
              autogenerate: {
                directory: "concepts/backends",
              },
            },
          ],
        },
        {
          label: "Recipes",
          collapsed: false,
          items: [
            {
              label: "Recipes",
              link: "/recipes/",
            },
            {
              label: "Layout",
              collapsed: true,
              autogenerate: {
                directory: "recipes/layout",
              },
            },
            {
              label: "Rendering",
              collapsed: true,
              autogenerate: {
                directory: "recipes/render",
              },
            },
            {
              label: "Widgets",
              collapsed: true,
              autogenerate: {
                directory: "recipes/widgets",
              },
            },
            {
              label: "Testing",
              collapsed: true,
              autogenerate: {
                directory: "recipes/testing",
              },
            },
            {
              label: "Applications",
              collapsed: true,
              autogenerate: {
                directory: "recipes/apps",
              },
            },
          ],
        },
        {
          label: "FAQ",
          link: "/faq/",
        },
        {
          label: "Highlights",
          collapsed: true,
          items: [
            {
              label: "Highlights",
              link: "/highlights/",
            },
            {
              label: "v0.30",
              link: "/highlights/v030/",
            },
            {
              label: "v0.29",
              link: "/highlights/v029/",
            },
            {
              label: "v0.28",
              link: "/highlights/v028/",
            },
            {
              label: "v0.27",
              link: "/highlights/v027/",
            },
            {
              label: "v0.26.3",
              link: "/highlights/v0263/",
            },
            {
              label: "v0.26.2",
              link: "/highlights/v0262/",
            },
            {
              label: "v0.26",
              link: "/highlights/v026/",
            },
            {
              label: "v0.25",
              link: "/highlights/v025/",
            },
            {
              label: "v0.24",
              link: "/highlights/v024/",
            },
            {
              label: "v0.23",
              link: "/highlights/v023/",
            },
            {
              label: "v0.22",
              link: "/highlights/v022/",
            },
            {
              label: "v0.21",
              link: "/highlights/v021/",
            },
          ],
        },
        {
          label: "Showcase",
          collapsed: true,
          autogenerate: {
            directory: "showcase",
          },
        },
        {
          label: "Templates",
          collapsed: true,
          items: [
            {
              label: "Templates",
              link: "/templates/",
            },
            {
              label: "Component",
              collapsed: true,
              autogenerate: {
                directory: "templates/component",
              },
            },
          ],
        },
        {
          label: "References",
          link: "/references/",
        },
        {
          label: "Developer Guide",
          collapsed: true,
          autogenerate: {
            directory: "developer-guide",
          },
        },
      ],
      editLink: {
        baseUrl: "https://github.com/ratatui/ratatui-website/edit/main/",
      },
    }),
    partytown(),
  ],
  redirects: {
    // lots of manual redirects because dynamic redirects don't work with starlight / cloudflare
    // See https://discord.com/channels/830184174198718474/1242301878994468916/1242301878994468916
    "/how-to": "/recipes",
    "/how-to/develop-apps/better-panic": "/recipes/apps/better-panic",
    "/how-to/develop-apps/cli-arguments": "/recipes/apps/cli-arguments",
    "/how-to/develop-apps/color-eyre/demo.gif": "/recipes/apps/color-eyre/demo.gif",
    "/how-to/develop-apps/color-eyre/error-full.gif": "/recipes/apps/color-eyre/error-full.gif",
    "/how-to/develop-apps/color-eyre/error-full.png": "/recipes/apps/color-eyre/error-full.png",
    "/how-to/develop-apps/color-eyre/error.gif": "/recipes/apps/color-eyre/error.gif",
    "/how-to/develop-apps/color-eyre/error.png": "/recipes/apps/color-eyre/error.png",
    "/how-to/develop-apps/color-eyre/panic-full.gif": "/recipes/apps/color-eyre/panic-full.gif",
    "/how-to/develop-apps/color-eyre/panic-full.png": "/recipes/apps/color-eyre/panic-full.png",
    "/how-to/develop-apps/color-eyre/panic.gif": "/recipes/apps/color-eyre/panic.gif",
    "/how-to/develop-apps/color-eyre/panic.png": "/recipes/apps/color-eyre/panic.png",
    "/how-to/develop-apps/color-eyre/quit.gif": "/recipes/apps/color-eyre/quit.gif",
    "/how-to/develop-apps/color-eyre/quit.png": "/recipes/apps/color-eyre/quit.png",
    "/how-to/develop-apps/color_eyre": "/recipes/apps/color-eyre",
    "/how-to/develop-apps/config-directories": "/recipes/apps/config-directories",
    "/how-to/develop-apps": "/recipes/apps",
    "/how-to/develop-apps/log-with-tracing": "/recipes/apps/log-with-tracing",
    "/how-to/develop-apps/migrate-from-tui-rs": "/recipes/apps/migrate-from-tui-rs",
    "/how-to/develop-apps/panic-hooks": "/recipes/apps/panic-hooks",
    "/how-to/develop-apps/terminal-and-event-handler": "/recipes/apps/terminal-and-event-handler",
    "/how-to/layout/center-a-rect": "/recipes/layout/center-a-widget",
    "/how-to/layout/collapse-borders": "/recipes/layout/collapse-borders",
    "/how-to/layout/dynamic": "/recipes/layout/dynamic",
    "/how-to/layout": "/recipes/layout",
    "/how-to/render/display-text": "/recipes/render/display-text",
    "/how-to/render": "/recipes/render",
    "/how-to/render/overwrite-regions": "/recipes/render/overwrite-regions",
    "/how-to/render/style-text": "/recipes/render/style-text",
    "/how-to/widgets/block": "/recipes/widgets/block",
    "/how-to/widgets/custom": "/recipes/widgets/custom",
    "/how-to/widgets": "/recipes/widgets",
    "/how-to/widgets/paragraph": "/recipes/widgets/paragraph",
    "/recipes/layout/center-a-rect": "/recipes/layout/center-a-widget",
  },
  vite: {
    server: {
      watch: {
        ignored: ["**/target/**/*"],
      },
    },
    plugins: [tailwindcss()],
  },
});
