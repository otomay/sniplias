# sniplias

A beautiful TUI for managing shell aliases and command snippets.

[⚠️ Vibe-coded project ⚠️](#user-warning)

---

## Screenshots

<img width="1901" height="1010" alt="image" src="https://github.com/user-attachments/assets/149bf9ed-f391-442d-ac28-c1023fbcc1bb" />

---

<img width="1901" height="999" alt="image" src="https://github.com/user-attachments/assets/3ffc4a2c-a869-4a12-832f-ce0943eaeb79" />

---

<img width="1898" height="1017" alt="image" src="https://github.com/user-attachments/assets/dc6a52fc-6bfc-440f-8737-9a65e315b0d1" />

---

## Installation

### Quickstart (Linux / macOS)

```bash
curl -sL https://raw.githubusercontent.com/otomay/sniplias/main/scripts/install.sh | sh
```

### Arch Linux (AUR)

```bash
yay -S sniplias
```

Or the pre-compiled version:

```bash
yay -S sniplias-bin
```

### Cargo (crates.io)

```bash
cargo install sniplias
```

### GitHub Releases

Download the latest binary for your platform from the [Releases](https://github.com/otomay/sniplias/releases) page.

### Manual Build

```bash
cargo build --release
sudo install target/release/sniplias /usr/local/bin/
```

---

## Usage

```bash
sniplias
```

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Tab` | Switch between Aliases/Snippets |
| `/` | Focus search |
| `n` | New alias/snippet |
| `e` | Edit selected |
| `d` | Delete selected |
| `Enter` | Execute/copy selected |
| `?` | Show help |
| `q` | Quit |

### Variables in Snippets

Create snippets with placeholders:

```
git clone {{repo}} -b {{branch:main}}
```

When executed, you'll be prompted for values. Defaults are optional.

### Configuration

Data is stored in `~/.config/sniplias/`

- `aliases.json` — Your shell aliases
- `snippets.json` — Your command snippets

---

## Features

- **Manage aliases** — View, create, edit, and delete shell aliases from your `.bashrc`/`.zshrc`
- **Command snippets** — Store reusable commands with variable interpolation
- **Smart variables** — Use `{{variable}}` syntax in snippets, with optional defaults: `{{branch:main}}`
- **Fast search** — Filter aliases and snippets instantly
- **Export to shell** — Execute commands directly or copy to clipboard
- **Beautiful UI** — Clean terminal interface with tabs, keyboard navigation

---
# User warning
## ⚠️ Important Notice: 100% AI-Generated Code

I was curious about how "vibecoders" work, so I decided to test a few tools using a language I had never worked with before: **Rust**.

I was already familiar with Ratatui, since I use several TUIs in my daily workflow. One thing I've always wanted was a simple alias and snippet manager.

For aliases, I wasn't using anything special; I would just edit my shell's source file directly.
For snippets, I was using [pet](https://github.com/knqyf263/pet) (which I still recommend if you're concerned about AI-generated projects).

That said, I'm not even sure "snippet" is the best term here. In practice, these are just commands meant to be executed in the terminal 🙂

The issue for me was that [pet](https://github.com/knqyf263/pet) isn't as simple to use as I would like. That's what motivated me to start this project.

---

### Why?

See above.

---

### Tools I Used

* [opencode](https://github.com/anomalyco/opencode) (pretty nice)
* [oh-my-opencode](https://github.com/code-yeongyu/oh-my-opencode) (just wow, the free models are awesome (I only used free models in this project))
* [vibe-kanban](https://github.com/BloopAI/vibe-kanban) (kinda laggy/buggy in my machine, I only used to fix a single issue)
* [picoclaw](https://github.com/sipeed/picoclaw) (probably just inserted a dozen lines, not extensivelly used)
* [aicommits](https://github.com/Nutlope/aicommits) (used a few times, but ended rewriting the history -- stil gona use it tho)

---

### Final Considerations

Although this was fun to build, I did **not** review or thoroughly check the code.

I do **not** recommend using projects that are 100% AI-generated for anything important, as they may carry serious security risks.
(Yes, this project only manages local text, but even so, caution is always a good idea.)


---

## License

MIT
