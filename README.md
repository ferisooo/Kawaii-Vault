<div align="center">

# 🛡️ CyberVault

### A private, encrypted, cyberpunk-styled file vault for your most personal files.

*Lock your files behind a password. Keep them 100% on your machine. Look incredible doing it.*

</div>

---

> # ✨ Only made possible by **feris** ✨
> ### 💜 CyberVault is built on **feris's idea and imagination**, brought to life with **Claude's work**. 💜
> ### Want to say hi or support her? Here are her socials:
> # 👉 **[https://mez.ink/ferisooo](https://mez.ink/ferisooo)** 👈
>
> **None of this exists without feris. Please show her some love.** 💜

---

## 🤯 Why you'll want this

These are the "okay, I *need* this" features — and every one of them is **free**
(there is no paid tier; everything is unlocked for everyone):

### 🔐 Genuinely serious security
- **AES-256-GCM encryption** for every file — authenticated, so tampering is
  detected, not just hidden.
- **Argon2id** password protection with OWASP-recommended settings (19 MiB
  memory, brute-force resistant) — not weak, fast-to-crack hashing.
- **A unique key for every single file** — each file gets its own random key
  that's then sealed with your master key, so cracking one tells an attacker
  nothing about the others.
- **No backdoor, no master key, no "forgot password."** Only *you* can open it.
  (So write your password down somewhere safe — there is genuinely no recovery.)
- **Optional key file** — require a specific file (like a USB stick) *in
  addition* to your password, for two-factor-style unlocking.
- **Memory hardening** — keys are kept XOR-masked, wiped from RAM when no longer
  needed, and (on Mac/Linux) locked out of the swap file.

### 🕵️ Panic & stealth features you won't find in a normal "photo vault"
- **🐍 Decoy Snake Game** — turn on stealth login and CyberVault disguises
  itself as a fully playable Snake game (9 modes, themes, power-ups,
  achievements). You secretly type your PIN while "playing." Anyone watching
  just sees a game.
- **🙈 Stealth Mode** — hide the vault entirely so it doesn't even appear to
  exist.
- **💣 Duress PIN** — set a *second* password that, when entered, **securely
  wipes the vault** instead of opening it — for situations where you're forced
  to "unlock."
- **🔥 Self-Destruct** — optionally wipe everything after too many wrong
  password attempts (you choose the number).
- **⚡ Panic Lock** — one keyboard shortcut (`Ctrl+Shift+L`) instantly locks the
  vault from anywhere.

### 🗂️ Actually pleasant to use
- **Built-in private browser** — browse and save media straight into the vault;
  downloads are **auto-encrypted and imported**, and the plaintext copy is
  deleted. It even filters ads/trackers.
- **Watch-folder auto-import** — point it at a folder and new files get pulled
  into the vault automatically.
- **Built-in viewer & slideshow** — preview images/video *inside* the vault so
  they never sit exposed on your desktop.
- **Categories, folders, favorites, search & sort** — stay organized as it grows.
- **Custom media pages** — group media into your own pages.
- **Trash & restore** — undo accidental deletes.

### 🛟 Safety nets & polish
- **Encrypted ZIP export** and **full encrypted vault backup/restore.**
- **Integrity checks** — verify with one click that nothing is corrupted or
  tampered with (CRC32 + hashing on every stored blob).
- **Auto-lock** after inactivity, plus **automatic clipboard clearing** so
  copied data doesn't linger.
- **DiagBot** — a built-in diagnostics panel that watches performance, memory,
  and vault health, and can export a report if something goes wrong.
- **Gorgeous neon cyberpunk UI** backed by a library of **26 animated background
  effects** (particles, scanlines, neon rain, starfields, neural webs, and more).

---

## 🆚 How CyberVault is different

Most "secure folder" or "photo vault" apps ask you to trust *them*. CyberVault
is built so you don't have to:

| Other apps often... | CyberVault... |
|---------------------|---------------|
| Upload your files to "the cloud" | Keeps **everything on your device** |
| Make you create an account | **No account, no email, no login to us** |
| Track you with analytics/ads | **Collects nothing** — zero tracking, no telemetry |
| Lock features behind a subscription | Is **100% free** — every feature unlocked |
| Hide their code | Is **fully open-source** — read every line |
| Can reset/recover your vault (so others can too) | Has **no backdoor** — only *you* hold the key |
| Just hide a folder | Has **duress wipe, self-destruct, and a decoy game** |
| Look like boring office software | Looks like it belongs in a cyberpunk movie 😎 |

In short: **it's private because of how it's built, not because we promise to be
nice.** See the [Privacy Policy](./PRIVACY_POLICY.md) and
[Terms of Service](./TERMS_OF_SERVICE.md) for the full story.

---

## 🚀 Setup guide (for people who have *never* touched code)

Don't worry — you don't need to "know how to program." Just follow the steps in
order. ☕ Grab a drink; the very first launch takes a little while.

### Step 1 — Install the free tools CyberVault needs

CyberVault is built from source, so you need a few free building blocks first.
Click each link, download, and run the installer with the **default options**:

1. **Node.js** → <https://nodejs.org> — pick the button that says **"LTS"**.
2. **Rust** → <https://rustup.rs> — run the installer and accept the defaults.
3. **Git** → <https://git-scm.com/downloads> — accept the defaults.

> 💡 **Windows users:** you may also need the free
> [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).
> **Mac users:** open the "Terminal" app and run `xcode-select --install`.
> **Linux users:** see the official list of system packages here →
> <https://tauri.app/start/prerequisites/>.

After installing, **restart your computer once** so everything is recognized.

### Step 2 — Download CyberVault

1. Open a terminal:
   - **Windows:** press the Start key, type **PowerShell**, press Enter.
   - **Mac:** open the **Terminal** app.
   - **Linux:** open your **Terminal**.
2. Copy and paste these lines **one at a time**, pressing Enter after each:

```bash
git clone https://github.com/ferisooo/CyberVault.git
cd CyberVault
```

### Step 3 — Install the app's parts

Paste this and press Enter (this downloads the pieces the app is made of):

```bash
npm install
```

### Step 4 — Launch CyberVault! 🎉

```bash
npm run tauri dev
```

> ⏱️ **The very first launch can take 5–15 minutes** while your computer builds
> the app. That's totally normal — it's only slow the *first* time. A window
> opens by itself when it's ready. Keep the terminal open while you use the app.

### Step 5 (optional) — Make a permanent app you can double-click

Once you're happy, build a real installable app:

```bash
npm run tauri build
```

The finished installer lands in `src-tauri/target/release/`.

> 🪟 **Windows shortcut:** there's a helper file called
> [`fresh-clone.bat`](./fresh-clone.bat) that downloads, installs, and launches
> everything for you. Copy it **out** of the project folder first, then
> double-click it.

---

## 🦠 "How do I know this isn't a virus / spyware?"

Great question — and you're right to be careful. The honest answer:
**CyberVault is open-source, so you don't have to trust anyone's word — you (or
a tech-savvy friend) can check for yourself.** Here's exactly where to look.

### 🌐 What does it send over the internet? (Short answer: almost nothing.)

There is **only one** thing CyberVault does online on its own:

- ✅ **An optional update check.** Roughly once a day it does a simple *read*
  (an HTTP GET) of a small text file at
  `https://raw.githubusercontent.com/ferisooo/CybertronUpdate/main/latest.json`
  to see if a newer version exists. **No files, no personal info, nothing about
  you is sent** — it only reads a version number. You can see this yourself in
  [`src/hooks/useUpdateChecker.ts`](./src/hooks/useUpdateChecker.ts).

Everything else stays on your machine. There is **no analytics, no telemetry,
no crash reporting, no ads, no account, and no cloud.** (The code contains an
old, *unused* Gumroad licensing path — the app is fully free and that check is
never performed; you can confirm `isPro` is hard-set to `true` in
[`src/hooks/useLicense.ts`](./src/hooks/useLicense.ts).)

> 🔌 Want to be 100% certain? **Turn off your Wi-Fi** — the vault works fully
> offline.

### 📂 The exact files to read if you're suspicious

- 📜 **[`PRIVACY_POLICY.md`](./PRIVACY_POLICY.md)** — plain-language promise that
  **nothing is collected** from you.
- 📜 **[`TERMS_OF_SERVICE.md`](./TERMS_OF_SERVICE.md)** — your rights and the
  legal terms.
- 🔐 **[`src-tauri/src/vault.rs`](./src-tauri/src/vault.rs)** — the **actual
  encryption code** (where your files get locked) plus the duress-wipe and
  self-destruct logic. You'll see real, well-known libraries: `aes-gcm`
  (AES-256) and `argon2`.
- 📱 **[`src-tauri/src/phone_server.rs`](./src-tauri/src/phone_server.rs)** — the
  **only** part that ever opens a network connection on your device, and it's
  the *optional* phone feature: TLS-encrypted, on your **own Wi-Fi only**, off
  until you switch it on, and it never sends your password (it uses a
  challenge/response so the password never leaves your phone).
- 📡 **[`src/hooks/useUpdateChecker.ts`](./src/hooks/useUpdateChecker.ts)** — the
  one automatic internet call, described above. Short and readable.
- 🧩 **[`src-tauri/src/lib.rs`](./src-tauri/src/lib.rs)** — the full list of every
  action the app is allowed to perform.
- 📦 **[`package.json`](./package.json)** & **[`src-tauri/Cargo.toml`](./src-tauri/Cargo.toml)**
  — every outside library the app uses, listed openly.

> 🛡️ Still unsure? Paste any of those files into an AI assistant or show a
> developer friend and ask "does this do anything sketchy?" Because you **build
> it yourself** from this readable source (instead of running a mystery `.exe`),
> there's no hidden pre-packaged program that could smuggle something in.

---

## 🧰 Available commands (for the curious)

| Command | What it does |
|---------|--------------|
| `npm run dev` | Start just the visual frontend (Vite dev server) |
| `npm run build` | Type-check and build the frontend |
| `npm run tauri dev` | Run the full desktop app (what you'll normally use) |
| `npm run tauri build` | Build the finished, installable desktop app |

> 🔒 Don't forget your password. Because of how the encryption works, there is
> **no master key and no recovery** — if you lose it, the files are gone for
> good (which is exactly what keeps everyone else out).

---

## 📚 More reading

- [`ABOUT.md`](./ABOUT.md) — the story behind CyberVault.
- [`PRIVACY_POLICY.md`](./PRIVACY_POLICY.md) — what we collect (spoiler: nothing).
- [`TERMS_OF_SERVICE.md`](./TERMS_OF_SERVICE.md) — terms, and how to fork it.
- [`LICENSE`](./LICENSE) — the open-source license.

> 🔁 **Want to fork or remix CyberVault?** You're welcome to — just credit
> **feris's idea** and **Claude's work**. Details in the
> [Terms of Service](./TERMS_OF_SERVICE.md).

---

<div align="center">

### 💜 CyberVault — **feris's idea & imagination**, built with **Claude**. 💜
### 👉 [https://mez.ink/ferisooo](https://mez.ink/ferisooo) 👈

**Every part of this exists thanks to feris. If you love it, go support her.**

</div>
