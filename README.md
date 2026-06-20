# AULA F75 Linux Driver

Makes the AULA F75 keyboard usable on Linux, especially with a **Finnish keyboard layout** on an ANSI keyboard.

## Finnish Layout Fixes

On a standard Finnish layout (`fi`), the `*` key is only available on an ISO key that doesn't physically exist on an ANSI keyboard. This driver:

- Remaps **Delete** → `*` (via Keypad Asterisk)
- Remaps right Fn → RAlt (AltGr) for easier special character access

## Usage

```bash
# Build
cargo build --release

# Apply keymap
sudo ./target/release/driver
```

Edit `test.toml` to customize key mappings.

## How it Works

Uses HID feature reports to read/write the keyboard's internal keymap. Supports key remapping across all layers, custom lighting, and device info queries.
