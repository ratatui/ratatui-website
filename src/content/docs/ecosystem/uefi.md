---
title: UEFI
sidebar:
  order: 4
---

These crates allow you to build Ratatui apps that run before operating system, as EFI applications.

## ratatuefi

> Provides a [uefi](https://docs.rs/uefi) backend for Ratatui (`no_std`).

![ratatuefi banner](https://raw.githubusercontent.com/sermuns/ratatuefi/refs/heads/main/media/banner.svg)

<video controls src="https://github.com/user-attachments/assets/9c12cd86-6aef-4f9b-9006-e8e9d527dcc2"></video>

- [GitHub](https://github.com/sermuns/ratatuefi)
- [API Documentation](https://docs.rs/ratatuefi)
- [Examples](https://github.com/sermuns/ratatuefi/tree/main/examples)

:::note[Example Project]

[efimux](https://github.com/sermuns/efimux) is a bootloader-like EFI application built with Ratatui and `ratatuefi`. It can discovers and boot other EFI applications on detected filesystems.

<video controls src="https://github.com/user-attachments/assets/c37e6d62-5e2a-4e62-86b2-3bcd2e930065"></video>

:::

## tui-uefi

> Also provides a [uefi](https://docs.rs/uefi) backend for Ratatui (`no_std`).

![screenshot](https://github.com/user-attachments/assets/29a559ff-f2c3-4059-8725-95602fdcba63)

- [GitHub](https://github.com/reubeno/tui-uefi)
- [Examples](https://github.com/reubeno/tui-uefi/tree/main/examples)
