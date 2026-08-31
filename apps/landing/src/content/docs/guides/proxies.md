---
title: Proxy rotation
description: The proxy pool, its health model, credential handling, and how rotation interacts with retries.
---

Browser patches do not change your IP, and IP reputation is where most
commercial anti-bot scoring starts. Cosmium therefore ships a proxy pool with
health tracking and automatic failover rather than a single `--proxy-server`
passthrough.

## Supplying proxies

Three sources, and they combine — a file plus an inline list yields one pool
containing both:

```bash
# a single proxy, no pool
--proxy http://user:pass@host:8080

# from a file, one URL per line
--proxy-file proxies.txt

# inline, comma-separated
--proxy-list http://a:8080,http://b:8080,http://c:8080
```

The file format is deliberately plain: one URL per line, blank lines skipped,
and lines starting with `#` treated as comments.

```text
# residential, US
http://user:pass@us-east.provider.net:8080
http://user:pass@us-west.provider.net:8080

# datacenter fallback
http://cheap-dc.example:3128
```

`--proxy` is only used when no pool exists. If you pass both `--proxy` and a
pool source, the pool wins.

## Rotation strategies

```bash
--proxy-rotation round-robin   # default
--proxy-rotation random
```

**Round-robin** advances an atomic cursor and then scans forward for the first
*available* proxy, so a proxy in cooldown is skipped rather than blocking the
rotation. **Random** picks uniformly among the available ones.

Both have the same fallback: if *nothing* is available — every proxy is in
cooldown — the pool returns one anyway rather than failing the request. A
degraded proxy is generally better than no attempt, and the alternative is a
hard stall when a whole provider has a bad minute.

## The health model

Each proxy carries state: `Healthy`, `Failed { consecutive_failures }`, or
`Cooldown`, plus lifetime counters for uses and failures.

The loop is driven by [block detection](/guides/scraping/#block-detection):

- Page comes back clean → `mark_success` → state resets to `Healthy`.
- Page comes back blocked → `mark_failed` → the failure is stamped with the
  current time and the consecutive counter increments.

A failed proxy becomes available again once `--proxy-cooldown` seconds have
elapsed since its last failure. Default is **60 seconds**.

```bash
--proxy-cooldown 300     # five minutes in the penalty box
```

Note that success resets the health state but not the lifetime counters, so
`total_uses` and `total_failures` remain an honest long-run picture.

## Rotation plus retries

The two flags work together:

```bash
cosmium scrape page --profile ./p.json \
  --proxy-file proxies.txt \
  --proxy-rotation round-robin \
  --proxy-cooldown 120 \
  --retries 3 \
  https://target.example
```

`--retries 3` allows up to four attempts. Each attempt draws a fresh proxy from
the pool, so a blocked attempt both penalizes the proxy that failed and moves to
a different egress for the next try. The run stops at the first clean page.

At `debug` log level, per-proxy stats are printed at the end of the run — URL,
uses, failures, and current availability:

```bash
COSMIUM_LOG=debug cosmium scrape page ...
```

## Authenticated proxies

Chromium does not accept credentials embedded in `--proxy-server`; it prompts
for them interactively, which is useless in headless mode. Cosmium works around
this with a **local forwarder**.

When the proxy URL contains userinfo, Cosmium binds an ephemeral listener on
`127.0.0.1`, points the browser at it with
`--proxy-server=http://127.0.0.1:<port>`, and the forwarder injects a
`Proxy-Authorization: Basic …` header into each request before tunneling it
upstream. `CONNECT` is handled properly, so HTTPS works.

If the forwarder cannot start, Cosmium falls back to a plain
`--proxy-server=<url>` flag and logs it. That fallback usually will not
authenticate — treat a sudden run of blocks after a forwarder failure as a
credential problem, not a fingerprint one.

## Choosing proxies

The pool mechanics are the easy half. The part that decides whether you get
through:

- **ASN type matters more than count.** Ten residential IPs beat a thousand
  datacenter ones against a vendor that scores IP type.
- **Keep a profile pinned to an egress.** A fingerprint that appears from three
  continents in ten minutes correlates trivially. If you rotate profiles and
  proxies independently, you are generating impossible machines — the exact
  problem [coherence validation](/guides/profiles/#coherence-rules) exists to
  prevent, one layer up.
- **Match the profile's timezone and locale to the egress.** A `America/Chicago`
  profile arriving from a Frankfurt IP is a free signal.
