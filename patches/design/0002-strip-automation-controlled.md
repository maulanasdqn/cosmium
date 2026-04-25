# patch 0002-strip-automation-controlled

Removes every code path that exposes "this browser is automated" — `navigator.webdriver`, the `--enable-automation` switch's side effects, the `Chrome is being controlled by automated test software` infobar, and the AutomationControlled feature flag's effects on Blink.

## Detection vector

The cheapest, oldest, and still most-checked bot signal:

```js
if (navigator.webdriver) bot = true;
```

CDP-level workarounds (`addScriptToEvaluateOnNewDocument` overriding the getter) are detected by checking `Object.getOwnPropertyDescriptor(Navigator.prototype, 'webdriver')` — the override's stack frame leaks. Only a C++ patch is undetectable.

The `--disable-blink-features=AutomationControlled` flag *partially* fixes this but is itself detectable: sites probe whether `Notification.permission` reports the value-pattern Chrome ships with that flag, which differs from real Chrome.

## Target files

| File | Change |
|---|---|
| `third_party/blink/renderer/core/frame/navigator_automation_information.cc` | Force `webdriver()` to return false unconditionally |
| `third_party/blink/renderer/core/frame/navigator_automation_information.idl` | Optionally remove the attribute entirely (cleaner, but tests for `'webdriver' in navigator` would then succeed where real Chrome returns false — keep the attribute, force the value) |
| `third_party/blink/public/common/features.cc` | Default `kAutomationControlled` to disabled, ensure no code path re-enables it |
| `chrome/browser/ui/views/frame/browser_view.cc` (or similar) | Suppress the "controlled by automated test software" infobar |
| `chrome/browser/chrome_content_browser_client.cc` | Strip `--enable-automation` cmdline propagation to renderer |

## Profile fields read

None. This patch is unconditional — webdriver is always false in cosmium. There is no legitimate reason to ever have it true.

## Implementation notes

- **Don't just delete the IDL attribute.** Sites probe both `navigator.webdriver === true` AND `'webdriver' in navigator`. Real Chrome returns false to the first and true to the second. Match that.
- **Worker threads.** `WorkerNavigator` has its own automation-information module. Patch both.
- **ServiceWorker / SharedWorker.** Same surface, separate file.
- **`window.cdc_*` strings from chromedriver.** If shipping chromedriver alongside cosmium, also strip the `cdc_*` evaluate-script names — but better: don't ship chromedriver at all. Drive cosmium via CDP directly.

## Validation

```js
// Should all be safe values:
console.log(navigator.webdriver);                                 // false
console.log('webdriver' in navigator);                            // true
console.log(WorkerNavigator.prototype.webdriver);                 // (worker context)
console.log(Object.getOwnPropertyDescriptor(Navigator.prototype, 'webdriver').get.toString());
// → 'function get webdriver() { [native code] }' (no override stack)
```

Sites: bot.sannysoft.com, intoli.com/blog/not-possible-to-block-chrome-headless, antoinevastel.com fingerprinting tests.
