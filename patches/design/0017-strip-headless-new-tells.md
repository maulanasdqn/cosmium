# patch 0017-strip-headless-new-tells

`--headless=new` (M108+) is much better than `--headless=old`, but it still leaves a few surfaces with subtly-different values that fingerprinters latch onto. Cosmium typically runs *headed*, but for batch scraping headless is the default, and these tells should not light up.

## Detection vectors that survive `--headless=new`

| Surface | Headed value | `--headless=new` value | Fix |
|---|---|---|---|
| `navigator.webdriver` | `false` | `true` | Patch 0001 already covers |
| `chrome.app` / `chrome.csi` | populated | undefined | Define stubs |
| `Notification.permission` | `default` | `denied` | Force `default` unless explicitly set |
| `Permissions.query({name:'notifications'}).state` | `prompt` | inconsistent with above | Force-align with `Notification.permission` |
| `outerWidth/outerHeight` | matches `innerWidth/Height` + chrome | inconsistent | Apply expected chrome offset |
| `screen.availLeft`/`availTop` | platform-specific value | always 0 | Read from profile |
| `window.devicePixelRatio` | matches profile/screen | sometimes forced to 1 | Honor profile |
| Service Worker registration error codes | subtly different timing | timing-side-channel | (deferred) |
| `Sec-CH-UA-Mobile` Client Hint | matches `?0` | matches | OK |

## Target files

| File | Change |
|---|---|
| `headless/lib/browser/headless_browser_impl.cc` | Strip headless-specific overrides for Notification + chrome.app |
| `third_party/blink/renderer/modules/notifications/notification.cc` | `Notification.permission` returns `default` when no explicit grant/deny |
| `third_party/blink/renderer/core/frame/dom_window.cc` | `outerWidth`/`outerHeight` adjustments |
| `third_party/blink/renderer/core/frame/screen.cc` | `availLeft`/`availTop` reading cosmium switches |

## Profile fields read

```
screen.avail_left   → uint (new field; default 0)
screen.avail_top    → uint (new field; default 0; ~25 on macOS for menu bar)
```

Add these two fields to the `Screen` struct in Rust side (`apps/engine/src/domain/profile/screen.rs`) — currently absent. Default 0; macOS profiles should set `avail_top=25` and reduce `avail_height` correspondingly.

## Implementation notes

- **`chrome.app` and `chrome.csi`.** Define as stub functions returning empty objects. Real Chrome's `chrome.app.isInstalled` returns `false`, `chrome.csi()` returns timing object with `onloadT`, `startE`, `pageT`, `tran` keys. Compute the timing values from `performance.timing` so they're internally consistent.
- **`Notification.permission`.** Three values: `default`, `granted`, `denied`. Headless always reports `denied`. Force `default` when no explicit grant. Coupled with permissions.query to keep them aligned.
- **Window chrome offset.** Real desktop Chrome reserves ~88px top (tabs + URL bar + bookmarks) and 2px each side. Patch should compute `outerHeight = innerHeight + 88` when running headless (best approximation). Skip in headed (real values).
- **`Sec-CH-UA-Mobile`.** Already correct in patch 0003 — flag.
- **`screen.availLeft`/`availTop`.** Real macOS reserves top 25px for menu bar; Windows usually 0; Linux varies. Profile should specify; patch reads switch.

## Headed vs headless toggle

Whole patch should gate on `--headless=new` detection (check `--headless` switch presence). When running headed, don't apply any overrides — they'd cause real-Chrome value drift in the wrong direction.

## Validation

```js
const tells = {
  webdriver: navigator.webdriver,
  notification_perm: Notification.permission,
  chrome_app: typeof chrome?.app,
  outer_inner_ratio: window.outerHeight / window.innerHeight,
  avail_top: screen.availTop,
};
console.log(tells);
// Headed expected: { webdriver: false, notification_perm: 'default',
//                    chrome_app: 'object', ratio: ~1.07-1.10, avail_top: <profile> }
```

Test sites: https://bot.sannysoft.com/ (must show all green), https://abrahamjuliot.github.io/creepjs/ (Headless section <5%).
