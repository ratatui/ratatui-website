import partytown from "@astrojs/partytown";
import starlight from "@astrojs/starlight";
import tailwind from "@astrojs/tailwind";
import { pluginCollapsibleSections } from "@expressive-code/plugin-collapsible-sections";
import { defineConfig } from "astro/config";
import emoji from "remark-emoji";
import remarkMermaid from "remark-mermaidjs";
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
    remarkPlugins: [remarkIncludeCode, emoji, remarkMermaid, remarkYoutube, remarkSvgBob],
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
      social: {
        github: "https://github.com/ratatui-org/ratatui",
        discord: "https://discord.gg/pMCEU9hNEj",
        matrix: "https://matrix.to/#/#ratatui:matrix.org",
        mastodon: "https://fosstodon.org/@ratatui_rs",
        "x.com": "https://twitter.com/ratatui_rs",
        linkedin: "https://www.linkedin.com/company/ratatui-rs",
      },
      sidebar: [
        // note that the need to order items and name the groups in the sidebar prevents
        // autogeneration except for the leaf level of the sidebar hierarchy.
        // See https://github.com/withastro/starlight/discussions/972
        // and https://github.com/withastro/starlight/issues/1223
        { label: "Introduction", link: "/introduction/" },
        {
          label: "Installation",
          collapsed: true,
          autogenerate: { directory: "installation" },
        },
        {
          label: "Tutorials",
          collapsed: false,
          items: [
            { label: "Tutorials", link: "/tutorials/" },
            { label: "Hello World", link: "/tutorials/hello-world/" },
            {
              label: "Counter App",
              collapsed: true,
              autogenerate: { directory: "tutorials/counter-app" },
            },
            {
              label: "JSON Editor",
              collapsed: true,
              autogenerate: { directory: "tutorials/json-editor" },
            },
            {
              label: "Async Counter App",
              collapsed: true,
              autogenerate: { directory: "tutorials/counter-async-app" },
            },
          ],
        },
        {
          label: "Concepts",
          collapsed: false,
          items: [
            { label: "Concepts", link: "/concepts/" },
            { label: "Widgets", link: "/concepts/widgets/" },
            { label: "Layout", link: "/concepts/layout/" },
            { label: "Event Handling", link: "/concepts/event-handling/" },
            { label: "Builder Lite Pattern", link: "/concepts/builder-lite-pattern/" },
            {
              label: "Rendering",
              collapsed: true,
              autogenerate: { directory: "concepts/rendering" },
            },
            {
              label: "Application Patterns",
              collapsed: true,
              autogenerate: { directory: "concepts/application-patterns" },
            },
            {
              label: "Backends",
              collapsed: true,
              autogenerate: { directory: "concepts/backends" },
            },
          ],
        },
        {
          label: "How To",
          collapsed: false,
          items: [
            { label: "How To", link: "/how-to/" },
            {
              label: "Layout UIs",
              collapsed: true,
              autogenerate: { directory: "how-to/layout" },
            },
            {
              label: "Render UIs",
              collapsed: true,
              autogenerate: { directory: "how-to/render" },
            },
            {
              label: "Use Widgets",
              collapsed: true,
              autogenerate: { directory: "how-to/widgets" },
            },
            {
              label: "Develop Applications",
              collapsed: true,
              autogenerate: { directory: "how-to/develop-apps" },
            },
          ],
        },
        { label: "FAQ", link: "/faq/" },
        {
          label: "Highlights",
          collapsed: true,
          autogenerate: { directory: "highlights" },
        },
        {
          label: "Showcase",
          collapsed: true,
          autogenerate: { directory: "showcase" },
        },
        {
          label: "Templates",
          collapsed: true,
          items: [
            { label: "Templates", link: "/templates/" },
            {
              label: "Component",
              collapsed: true,
              autogenerate: { directory: "templates/component" },
            },
          ],
        },
        { label: "References", link: "/references/" },
        {
          label: "Developer Guide",
          collapsed: true,
          autogenerate: { directory: "developer-guide" },
        },
      ],
      editLink: {
        baseUrl: "https://github.com/ratatui-org/ratatui-website/edit/main/",
      },
    }),
    tailwind({
      applyBaseStyles: false,
    }),
    partytown(),
  ],
  vite: {
    server: {
      watch: {
        ignored: ["**/target/**/*"],
      },
    },
  },
});
