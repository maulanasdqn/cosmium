---
title: Workflows
description: The seven workflow step types, their JSON shape, and how to drive them from Rust.
---

A workflow is an ordered list of steps executed against a live page after
navigation: click, type, scroll, wait, extract, run script, follow links. It is
what turns a single page fetch into a scrape.

:::caution[Reachability]
The workflow engine lives in the domain and every step type is implemented, but
the **CLI and HTTP surfaces expose only a subset**. `--extract` and `--script`
(and their `extract` / `script` JSON equivalents) are compiled into a two-step
workflow for you. To use `click`, `input`, `scroll`, `delay`, or `follow_urls`
you currently need to [embed the engine](/operations/embedding/) and construct
`Vec<WorkflowStep>` yourself. There is no `--workflow file.json` flag yet.
:::

## Step types

Steps serialize with a `type` tag in snake_case.

### `delay`

```json
{ "type": "delay", "duration_ms": 1500 }
```

### `script`

Runs JavaScript in the page and stores the result under `name`.

```json
{
  "type": "script",
  "name": "hrefs",
  "code": "JSON.stringify([...document.querySelectorAll('a')].map(a => a.href))",
  "timeout_seconds": 30
}
```

`timeout_seconds` defaults to `30`.

### `click`

```json
{ "type": "click", "selector": ".load-more" }
```

### `input`

Types text into a field.

```json
{ "type": "input", "selector": "#search", "text": "wireless keyboard" }
```

### `scroll`

```json
{ "type": "scroll", "times": 5 }
{ "type": "scroll", "infinite": true, "selector": ".feed" }
```

`times` defaults to `1`, `infinite` to `false`, and `selector` is optional —
without it the window scrolls, with it the matched element does.

### `extract`

The full form the CLI flag cannot express:

```json
{
  "type": "extract",
  "name": "prices",
  "selector": ".price",
  "attribute": null,
  "limit": 10
}
```

- `name` — the key the values land under, independent of the selector
- `attribute` — pull an attribute instead of text content, e.g. `"href"`
- `limit` — cap the number of matches; `0` means no cap

### `follow_urls`

The pagination and detail-page primitive: collect links matching a selector,
then run a nested workflow on each.

```json
{
  "type": "follow_urls",
  "name": "products",
  "selector": ".product-card a",
  "attribute": "href",
  "limit": 20,
  "workflow": [
    { "type": "delay", "duration_ms": 800 },
    { "type": "extract", "name": "title", "selector": "h1" },
    { "type": "extract", "name": "price", "selector": ".price" }
  ]
}
```

Nested workflows may themselves contain `follow_urls`, so a listing → detail →
sub-detail crawl is expressible, though each level multiplies page loads.

## A complete example

Search, wait for results, scroll to load more, then walk each result:

```json
[
  { "type": "input", "selector": "#q", "text": "mechanical keyboard" },
  { "type": "click", "selector": "button[type=submit]" },
  { "type": "delay", "duration_ms": 2000 },
  { "type": "scroll", "times": 3 },
  {
    "type": "follow_urls",
    "name": "items",
    "selector": ".result a.title",
    "attribute": "href",
    "limit": 10,
    "workflow": [
      { "type": "extract", "name": "title", "selector": "h1" },
      { "type": "extract", "name": "specs", "selector": ".spec-row", "limit": 50 }
    ]
  }
]
```

## Driving it from Rust

```rust
use cosmium_engine::domain::scraping::workflow::WorkflowStep;

let workflow: Vec<WorkflowStep> = serde_json::from_str(include_str!("workflow.json"))?;
```

`WorkflowStep` derives both `Serialize` and `Deserialize`, so you can keep
workflows as JSON files next to your code and load them at runtime, or build
them programmatically. Pass the vector as `ScrapePageInput::workflow`. The
[embedding guide](/operations/embedding/) has the full call.

## Actions are human-shaped

`click`, `input`, and `scroll` do not fire synthetic DOM events at an element.
They drive a virtual cursor and keyboard through CDP:

- **`click`** resolves the element's bounding box, picks a randomized point
  inside it rather than the exact center, and moves the cursor there along a
  path before pressing. The cursor position persists across steps, so a
  sequence of clicks traces a plausible route rather than teleporting.
- **`input`** types character by character with variable delays, clearing the
  field first.
- **`scroll`** moves in smooth increments from the current cursor position
  instead of jumping `scrollTop`.

This matters because behavioral scoring is one of the vectors browser patches
cannot address. It does not make your automation indistinguishable from a human
— pacing and intent are still yours to get right — but it removes the trivially
detectable "every click landed on the exact pixel center, instantly" signature.

## Practical notes

- **Extraction results are strings.** Each value is parsed as JSON if it parses,
  otherwise kept as a string. A `script` step returning `JSON.stringify(...)`
  therefore comes back as structured data.
- **Selectors run against the live DOM**, after JavaScript has executed — that
  is the whole point of using a browser. If a selector finds nothing, add a
  `delay` before it or use `--wait-for-api` on the request.
- **Steps do not fail the run.** A click on a missing selector does not abort
  the workflow; later steps still execute. Check the extracted output rather
  than assuming every step landed.
