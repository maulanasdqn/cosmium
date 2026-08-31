# landing

Documentation site for Cosmium — [cosmium.zod.rs](https://cosmium.zod.rs).

Astro + Starlight, built to static HTML and served from a Cloudflare Worker with
static assets. No adapter and no server-side rendering; `wrangler.jsonc` has no
`main` field because there is no Worker script to run.

## Develop

```bash
npm install
npm run dev        # http://localhost:4321
npm run build      # -> dist/
npm run preview
```

## Deploy

```bash
npm run deploy     # astro build && wrangler deploy
```

The custom domain `cosmium.zod.rs` is bound through the `routes` entry in
`wrangler.jsonc`, so a deploy keeps the binding — there is no separate DNS step.

## Content

Pages live in `src/content/docs/`, grouped into the four sidebar sections
defined in `astro.config.mjs`: `start/`, `guides/`, `reference/`, and
`operations/`. The splash landing page is `index.mdx`.

Adding a page means creating the markdown file *and* adding its slug to the
`sidebar` array — Starlight does not auto-generate the nav here, so the order
stays deliberate.

Full-text search is built at deploy time by Pagefind over the generated HTML;
nothing to configure.

## Keeping it honest

The docs describe real flags, switches, and endpoints. When you change the CLI
parser, `profile_to_flags`, the HTTP routes, or the patch series, update the
matching reference page:

| Changed | Update |
| --- | --- |
| `presentation/cli/` | `reference/cli.md` |
| `presentation/http/` | `reference/http-api.md` |
| `domain/runtime/flags/` | `reference/switches.md` |
| `patches/series` | `reference/patches.md` |
| `profiles/schema.json` | `reference/profile-schema.md` |
| `.config/src/env.rs` | `reference/environment.md` |
