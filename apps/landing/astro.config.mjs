// @ts-check
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

export default defineConfig({
  site: "https://cosmium.zod.rs",
  integrations: [
    starlight({
      title: "Cosmium",
      description:
        "A patched Chromium variant plus a Rust orchestration layer for stealth scraping inside containers.",
      logo: {
        light: "./src/assets/logo-light.svg",
        dark: "./src/assets/logo-dark.svg",
        replacesTitle: true,
      },
      favicon: "/favicon.svg",
      customCss: ["./src/styles/custom.css"],
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/maulanasdqn/cosmium",
        },
      ],
      editLink: {
        baseUrl:
          "https://github.com/maulanasdqn/cosmium/edit/develop/apps/landing/",
      },
      lastUpdated: true,
      tableOfContents: { minHeadingLevel: 2, maxHeadingLevel: 3 },
      sidebar: [
        {
          label: "Start here",
          items: [
            { label: "What is Cosmium", slug: "start/what-is-cosmium" },
            { label: "How it works", slug: "start/how-it-works" },
            { label: "Installation", slug: "start/installation" },
            { label: "Quickstart", slug: "start/quickstart" },
          ],
        },
        {
          label: "Guides",
          items: [
            { label: "Fingerprint profiles", slug: "guides/profiles" },
            { label: "Scraping a page", slug: "guides/scraping" },
            { label: "Workflows", slug: "guides/workflows" },
            { label: "Proxy rotation", slug: "guides/proxies" },
            { label: "Testing stealth", slug: "guides/testing" },
            { label: "LLM profile authoring", slug: "guides/llm-authoring" },
            { label: "HTTP server mode", slug: "guides/server" },
          ],
        },
        {
          label: "Reference",
          items: [
            { label: "CLI", slug: "reference/cli" },
            { label: "HTTP API", slug: "reference/http-api" },
            { label: "Profile schema", slug: "reference/profile-schema" },
            { label: "Chromium switches", slug: "reference/switches" },
            { label: "Patch series", slug: "reference/patches" },
            { label: "Environment", slug: "reference/environment" },
          ],
        },
        {
          label: "Operations",
          items: [
            { label: "Building Chromium", slug: "operations/building" },
            { label: "Docker", slug: "operations/docker" },
            { label: "Embedding the engine", slug: "operations/embedding" },
          ],
        },
      ],
    }),
  ],
});
