# Kawaii Vault — Updates

---

* **Date:** 07/01/2026
* **Updated at:** 2:00 PM (L.A. time)
* **Version:** 0.3.0

### TL;DR
New feature: a remote "panic wipe." Text a secret phrase to your own Telegram bot from anywhere, and your PC destroys every vault and restarts itself. Works on any network, and catches the message even if the PC was off when you sent it.

### Bug
* No way to wipe the vault remotely if your PC was taken or you were away from it.

### Fixes / New
* **Panic Remote Wipe (new):** Set it up in Settings → Tools → "🧨 Panic Remote Wipe."
  * You make a free Telegram bot (steps are in the app), paste its token, and pick a trigger phrase.
  * Send that phrase to the bot from your phone — on the same Wi-Fi or from cellular, anywhere — and the PC wipes all vaults, scrubs free space, and restarts.
  * If the PC was **off** when you sent it, the message waits; the wipe fires the moment the PC turns on and you log in.
  * You choose what happens: wipe + scrub + restart (default), wipe + restart, wipe + shut down, or wipe only.
* **Honest limit (please read):** this only works while Kawaii Vault is running (on or after login). It **cannot** wipe a PC that is powered off and kept offline, or one that someone boots from a USB stick or by pulling the drive. Against a stolen, powered-off machine, your real protection is the vault's encryption — a thief just sees scrambled data. Think of remote wipe as a "burn it now" button for when the PC is still yours and online.

---

* **Date:** 07/01/2026
* **Updated at:** 1:30 PM (L.A. time)
* **Version:** 0.2.0

### TL;DR
Security audit done. Found and fixed 9 issues. Your vault is now harder to break into, and deleted files are much harder to recover. Also fixed a memory leak and a freeze when using a big video as a background.

### Bug
* Closing the app did not properly erase the secret key from the computer's memory.
* "Deleted" files could sometimes still be recovered from the disk with special tools.
* Thumbnails (small previews of your files) were saved on disk **unencrypted** — anyone with access to the computer could see previews of your private files.
* Leftover temporary files from browser downloads were deleted the normal way, which means they could be undeleted.
* A malicious website opened in the private browser could trick the app into talking to other programs on your computer.
* The app slowly ate more and more memory the longer you scrolled through videos.
* Picking a large video as your background could freeze the app.

### Fixes
* The secret key is now wiped (zeroed out) from memory the moment you close the app, lock the vault, or delete the vault.
* Deleted files are now **overwritten with random data** before being removed, and their encryption keys are destroyed — making recovery practically impossible.
* Thumbnails are now encrypted with a key tied to your vault. Locked vault = unreadable previews. Old unencrypted previews are automatically wiped.
* All temporary files (downloads, video grabs) are now shredded (overwritten, then deleted) instead of just deleted.
* The private browser's download feature now refuses suspicious addresses that point back into your own computer.
* Fixed the memory leak from video previews — the app stays light during long sessions.
* Backgrounds now stream directly from the vault instead of loading the whole file into memory — no more freeze.
* Your password is also wiped from memory immediately after every unlock attempt.
