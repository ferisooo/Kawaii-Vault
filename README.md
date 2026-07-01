<div align="center">

# 🛡️ Kawaii Vault

### A private, encrypted, cyberpunk-styled file vault — 100% on your machine.

*Lock your files behind a password. No cloud, no account, no tracking. Free & open-source.*

</div>

---

> ### 💜 **feris's idea & imagination**, built with **Claude's work**. 💜
> Support her → **[https://mez.ink/ferisooo](https://mez.ink/ferisooo)**

---

## ✨ Highlights

- 🔐 **Serious crypto** — AES-256-GCM + Argon2id, a unique key per file, no backdoor, no recovery.
- 🕵️ **Panic & stealth** — decoy Snake game login, stealth mode, duress-wipe PIN, self-destruct, `Ctrl+Shift+L` panic lock.
- 🗂️ **Pleasant to use** — built-in private browser (auto-imports downloads), watch-folder import, viewer/slideshow, categories, favorites, search, trash & restore.
- 🛟 **Safety nets** — encrypted export & full backup/restore, integrity checks, auto-lock, clipboard clearing, DiagBot health panel.
- 😎 **Looks incredible** — neon cyberpunk UI with 26 animated backgrounds.

**Why it's different:** everything stays on your device — no account, no telemetry, no ads, no subscription. It's private because of *how it's built*, not because you're asked to trust anyone. See [Privacy Policy](./PRIVACY_POLICY.md) & [Terms](./TERMS_OF_SERVICE.md).

---

## 🚀 Quick start (Windows 10/11)

**1. Install these (default options), then restart your PC:**
[Node.js LTS](https://nodejs.org) · [Rust](https://rustup.rs) · [Git](https://git-scm.com/downloads) · [C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

**2. In PowerShell, run:**

```bash
git clone https://github.com/ferisooo/KawaiiVault.git
cd KawaiiVault
npm install
npm run tauri dev
```

> ⏱️ First launch takes 5–15 min to build — that's normal, only slow once. A window opens when ready.
> 🪟 Shortcut: copy [`fresh-clone.bat`](./fresh-clone.bat) out of the folder and double-click it to do all of the above.

**Build an installable app:** `npm run tauri build` → output in `src-tauri/target/release/`.

| Command | Does |
|---------|------|
| `npm run tauri dev` | Run the full desktop app |
| `npm run tauri build` | Build the installable app |
| `npm run dev` / `npm run build` | Frontend only (dev / type-check + build) |

---

## 🦠 "Is this safe?"

It's open-source, so you don't have to trust anyone's word. It does **one** thing online: an optional daily version check ([`useUpdateChecker.ts`](./src/hooks/useUpdateChecker.ts)) that only *reads* a version number — nothing about you is ever sent. No analytics, telemetry, or cloud. Turn off Wi-Fi and it still works fully.

Worth reading if you're curious: [`vault.rs`](./src-tauri/src/vault.rs) (encryption) · [`phone_server.rs`](./src-tauri/src/phone_server.rs) (optional LAN phone feature) · [`lib.rs`](./src-tauri/src/lib.rs) (every allowed action) · [`package.json`](./package.json) & [`Cargo.toml`](./src-tauri/Cargo.toml) (all libraries).

> 🔒 **No recovery.** There's no master key — lose your password and the files are gone for good. That's exactly what keeps everyone else out, so write it down somewhere safe.

---

## 📚 More

[`ABOUT.md`](./ABOUT.md) · [`PRIVACY_POLICY.md`](./PRIVACY_POLICY.md) · [`TERMS_OF_SERVICE.md`](./TERMS_OF_SERVICE.md) · [`LICENSE`](./LICENSE)

Forking is welcome — just credit **feris's idea** and **Claude's work** ([details](./TERMS_OF_SERVICE.md)).

<div align="center">

💜 **Every part of this exists thanks to feris. If you love it, go support her →** [mez.ink/ferisooo](https://mez.ink/ferisooo) 💜

</div>
