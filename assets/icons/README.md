# Icon Themes Directory

This directory contains icon theme packs used by Weaver Desktop but **is not included in git** due to big size.

## Optional Enhancement

**The application runs fine without icons** — it uses emoji fallbacks (📁, 💻, ⌨️, etc.) when icon themes are missing. This is intentional for easy testing and quick starts.

To see full icon themes instead of emoji fallbacks, install at least one icon theme below.

## Quick Setup

### Option 1: Download Icon Themes (Recommended)

Download the icon themes from their official sources:

**Papirus (Primary, ~6.6MB)** - Used by default

```bash
cd assets/icons
git clone https://github.com/PapirusDevelopmentTeam/papirus-icon-theme.git
```

**Numix Circle (~34MB)** - Alternative theme

```bash
cd assets/icons
git clone https://github.com/numixproject/numix-icon-theme-circle.git
```

**Vimix (~108MB)** - Alternative theme

```bash
cd assets/icons
git clone https://github.com/vinceliuice/Vimix-icon-theme.git
```

### Option 2: Use System-Installed Icons (Linux)

If you're on Linux with icon themes already installed in `/usr/share/icons/`, Weaver will automatically fall back to system paths:

- `~/.local/share/icons/`
- `~/.icons/`
- `/usr/local/share/icons/`
- `/usr/share/icons/`

Just install your preferred icon theme via your package manager:

```bash
# Debian/Ubuntu
sudo apt install papirus-icon-theme

# Arch
sudo pacman -S papirus-icon-theme

# Fedora
sudo dnf install papirus-icon-theme
```

### Option 3: Skip Icons (Minimal Testing)

The application will run without icons (they'll be missing from the UI). This is fine for testing core functionality but not recommended for evaluating the full desktop environment experience.

## Icon Theme Structure

Each icon theme should follow the freedesktop.org Icon Theme Specification:

```text
assets/icons/
├── papirus-icon-theme/
│   └── Papirus/
│       ├── 16x16/
│       ├── 22x22/
│       ├── 24x24/
│       ├── 32x32/
│       ├── 48x48/
│       └── 64x64/
│           ├── actions/
│           ├── apps/
│           ├── devices/
│           ├── places/
│           └── ...
```

## Custom Icon Themes

You can use any freedesktop.org-compatible icon theme by:

1. Extracting it to `assets/icons/your-theme-name/`
2. Modifying `src/app.rs` to update `DEFAULT_ICON_THEME_PATH`

## Why Not Included in Git?

Icon themes are large binary assets (6MB-108MB each) that would:

- Bloat the repository size unnecessarily
- Slow down clones significantly
- Contain mostly unchanged upstream data

These are upstream dependencies that should be fetched separately, similar to how you'd install system libraries or download assets for any other project.
