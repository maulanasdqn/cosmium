---
title: LLM profile authoring
description: Generate, repair, and mutate coherent fingerprint profiles through OpenRouter.
---

Writing a coherent fingerprint by hand means getting a dozen interlocking fields
right at once. Three subcommands delegate that to a model and then verify the
result with the same validator you would have used anyway.

:::note
These three subcommands are the only part of Cosmium that needs an API key.
Validation, listing, running, scraping, and testing all work without one.
:::

## Setup

```bash
export OPENROUTER_API_KEY=sk-or-v1-...

# optional
export OPENROUTER_MODEL=anthropic/claude-sonnet-4.6     # the default
export OPENROUTER_BASE_URL=https://openrouter.ai/api/v1
export OPENROUTER_REFERER=https://your-app.example      # sent as HTTP-Referer
export OPENROUTER_TITLE="Your App"                      # sent as X-Title
```

A `.env` at the repo root is loaded automatically.

Any OpenRouter model with JSON mode works. The client is a plain
OpenAI-compatible chat adapter, so `OPENROUTER_BASE_URL` also points at other
compatible providers.

## `generate` — persona to profile

```bash
cosmium profile generate \
  --persona "Windows 11 gaming PC, RTX 4070, 16-core CPU, 32GB RAM, en-US, Central time" \
  --name win11_rtx4070_en-us \
  --save
```

`--save` writes into `COSMIUM_PROFILES_DIR`. Use `--output PATH` to write
somewhere specific instead; the two are alternatives.

Personas work best when they describe a *machine and its owner* rather than a
list of fields. "Corporate MacBook Pro M2, Amsterdam, Dutch and English, managed
by IT" produces a more internally consistent document than an enumeration of
values, because the coherent choices follow from the story.

## `repair` — fix a failing profile

```bash
cosmium profile repair profiles/some-broken.json
cosmium profile repair ./broken.json --output ./fixed.json
```

Sends the profile *and its validation diagnostics* to the model, then
re-validates whatever comes back. This closes the loop on a hand-edited profile
that drifted — bump a UA version and forget the matching Client Hints brand, and
`repair` is faster than hunting the mismatch.

## `mutate` — a fleet from one profile

```bash
# five variants, saved to the profiles directory
cosmium profile mutate profiles/win11_rtx3060_en-us.json --count 5 --save

# steer which dimensions vary
cosmium profile mutate profiles/win11_rtx3060_en-us.json \
  --count 8 \
  --hint "vary primarily by locale and timezone, keep Windows + RTX-class GPU" \
  --save
```

`--count` defaults to `5`. Output goes to `COSMIUM_PROFILES_DIR` with `--save`,
or to a directory of your choosing with `--output-dir DIR`.

This is the intended way to build a rotation fleet. Without a `--hint` the model
varies broadly; with one you keep a family resemblance, which matters when a
target expects a particular platform mix.

## Why the output is trustworthy

The interesting part is not the prompt, it is what surrounds it:

- **Ground truth is embedded in the prompt.** `generate_profile.rs` pulls
  `profiles/schema.json` and the reference profile
  `profiles/win11_rtx3060_en-us.json` in via `include_str!`. The model sees the
  exact schema and a known-good example, not a paraphrase that can drift from
  the code.
- **JSON mode is enforced.** Requests set `response_format: json_object`, so
  compatible models cannot return prose around the document.
- **Every output is validated before it is saved.** Generated, repaired, and
  mutated profiles all run through the same coherence validator as
  `cosmium profile validate`. Diagnostics print; warnings are advisory, errors
  block the save.
- **`repair` closes the loop.** When validation does fail, the diagnostics go
  back to the model rather than to you.

The model is a drafting tool sitting behind a deterministic gate. That gate —
not the model — is what makes the profile safe to use.

## Verify before you trust

Coherence is necessary but not sufficient. A profile can pass every rule and
still describe a machine that does not exist in the wild — an unusual
resolution, a GPU that never shipped with that CPU. Run the generated profile
through the probe suite and a detection site before putting it in rotation:

```bash
cosmium profile validate win11_rtx4070_en-us --strict
cosmium test fingerprint --profile win11_rtx4070_en-us
cosmium test stealth --profile win11_rtx4070_en-us
```

See [testing stealth](/guides/testing/).
