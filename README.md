# README BY THE HUMAN:

## ⚠️ Important Notice: 100% AI-Generated Code

### Why?

I was curious about how “vibecoders” work, so I decided to test a few tools using a language I had never worked with before: **Rust**.

I was already familiar with Ratatui, since I use several TUIs in my daily workflow. One thing I’ve always wanted was a simple alias and snippet manager.

For aliases, I wasn’t using anything special; I would just edit my shell’s source file directly.
For snippets, I was using `pet` (which I still recommend if you’re concerned about AI-generated projects).

That said, I’m not even sure “snippet” is the best term here. In practice, these are just commands meant to be executed in the terminal 🙂

The issue for me was that `pet` isn’t as simple to use as I would like. That’s what motivated me to start this project.

---

### Tools I Used

* `opencode`
* `oh-my-opencode`
* `vibekanban`
* `picoclaw`

---

### Final Considerations

Although this was fun to build, I did **not** review or thoroughly check the code.

I do **not** recommend using projects that are 100% AI-generated for anything important, as they may carry serious security risks.
(Yes, this project only manages local text, but even so, caution is always a good idea.)


# README BY THE ROBOT:

# sniplias

A beautiful TUI for managing shell aliases and command snippets.

## Features

- **Manage aliases** — View, create, edit, and delete shell aliases from your `.bashrc`/`.zshrc`
- **Command snippets** — Store reusable commands with variable interpolation
- **Smart variables** — Use `{{variable}}` syntax in snippets, with optional defaults: `{{branch:main}}`
- **Fast search** — Filter aliases and snippets instantly
- **Export to shell** — Execute commands directly or copy to clipboard
- **Beautiful UI** — Clean terminal interface with tabs, keyboard navigation

## Installation

### Arch Linux (AUR)

```bash
yay -S sniplias-bin
```

### Debian/Ubuntu

```bash
sudo dpkg -i sniplias_*.deb
```

### From binary

Download from [Releases](https://github.com/otomay/sniplias/releases)

### From source

```bash
cargo build --release
sudo install target/release/sniplias /usr/local/bin/
```

### With cargo-binstall

```bash
cargo binstall sniplias
```

## Usage

```bash
sniplias
```

### Keyboard shortcuts

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

### Variables in snippets

Create snippets with placeholders:

```
git clone {{repo}} -b {{branch:main}}
```

When executed, you'll be prompted for values. Defaults are optional.

## Configuration

Data is stored in `~/.config/sniplias/`

- `aliases.json` — Your shell aliases
- `snippets.json` — Your command snippets

## License

MIT
