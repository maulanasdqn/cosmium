# patch 0016-strip-cdp-runtime-leaks

ChromeDriver / Puppeteer / Playwright leave fingerprintable artifacts even when `--disable-blink-features=AutomationControlled` is set. The CDP (Chrome DevTools Protocol) host page injects globals (`cdc_adoQpoasnfa76pfcZLmcfl_*`), and the V8 runtime exposes binding shapes that differ from non-DevTools sessions.

CreepJS `Headless` probe reported `chromium: true`, `38% like headless` — that score comes from these CDP-era leaks.

## Detection vector

```js
// 1. Property name probe (ChromeDriver glue)
for (const k in document) {
  if (k.startsWith('cdc_')) console.log('CDC leak:', k);
}
// Also check Window prototype chain:
for (const k of Object.getOwnPropertyNames(window)) {
  if (k.startsWith('cdc_')) console.log('CDC window leak:', k);
}

// 2. Runtime.evaluate marker (V8 binding shape)
const errorStack = new Error().stack;
// CDP-attached pages show '__puppeteer_evaluation_script__' or similar
// when scripts are injected via Runtime.evaluate

// 3. chrome.runtime.connect probe
// Real Chrome browsers have chrome.runtime defined; headless Chrome does not unless
// extensions are loaded. Detectors check shape.

// 4. Permissions API quirk
navigator.permissions.query({ name: 'notifications' }).then(p => {
  if (Notification.permission === 'denied' && p.state === 'prompt') {
    console.log('headless tell: notifications permission inconsistency');
  }
});

// 5. window.outerHeight === 0 in headless --headless=old; headless=new fixed it
// but stale detection scripts still check.
```

## Target files

| File | Change |
|---|---|
| `chrome/test/chromedriver/chrome/devtools_client_impl.cc` | Don't inject `cdc_*` symbols when cosmium switch present |
| `third_party/blink/renderer/bindings/core/v8/v8_binding_for_modules.cc` | Strip `Runtime.evaluate` source-URL markers from `Error.stack` |
| `content/renderer/render_frame_impl.cc` | Suppress `__puppeteer_evaluation_script__` marker |
| `third_party/blink/renderer/modules/permissions/permission_status.cc` | Fix Notification permission consistency |
| `third_party/blink/renderer/core/html/canvas/window_or_worker_global_scope.cc` | Ensure `window.chrome` object shape matches non-automation case |

## Profile fields read

None — this is a "strip" patch, no profile data needed. Add a single boolean switch `--cosmium-strip-automation-tells` that the patch checks; default off so non-automation users don't lose CDP entirely.

## Implementation notes

- **`cdc_` rename or remove.** ChromeDriver injects ~7 properties named `cdc_<random>_Array`, `cdc_<random>_Promise`, etc. They're used to keep references during page navigations. Option A: rename to less-distinctive names (still detectable by pattern). Option B: use closure-based references instead of globals (best). Patch should choose B but is a real undertaking.
- **`Error.stack` markers.** When CDP runs `Runtime.evaluate`, the resulting frames have URLs like `__puppeteer_evaluation_script__` or `eval at <anonymous>`. Filter these out of the `.stack` getter. Sites that inspect their own stack traces to detect injection will get clean output.
- **`window.chrome`.** Headless Chrome has `chrome.runtime === undefined`; real desktop Chrome has `chrome.runtime.id` etc. CreepJS checks the shape. Patch: in headless mode, lazy-define a `chrome.runtime` proxy that matches real-Chrome shape, returns `undefined` on most accesses but doesn't throw.
- **`navigator.plugins.length`.** Headless reports 0 (you saw "plugins (5)" — your build has CDM/Widevine bundled, so OK). Validator should warn if profile claims a typical Chrome plugin count but headless build has only the bundled few.
- **Don't break debugging.** Keep CDP fully functional when `--cosmium-strip-automation-tells` is off. Cosmium users running CDP-driven scrapers should opt in explicitly per session.

## Interaction with `--headless=new`

`--headless=new` (M108+) fixed the worst of `--headless=old` (no `outerHeight`, fake renderer, etc.). But CDC + stack markers persist. This patch is independent of headless mode — it strips even when running headed (which automation tools usually do for cosmium).

## Validation

```js
// Must all be false / undefined / clean stacks:
const leaks = {
  cdc_keys: Object.keys(window).filter(k => k.startsWith('cdc_')),
  puppeteer: new Error().stack.includes('puppeteer'),
  evaluation_script: new Error().stack.includes('evaluation_script'),
  chrome_runtime_missing: typeof chrome === 'undefined' || !chrome.runtime,
};
console.log(leaks);
```

Test sites: https://abrahamjuliot.github.io/creepjs/ (Headless section, target sub-1% "like headless"), https://bot.sannysoft.com/, https://arh.antoinevastel.com/bots/areyouheadless.
