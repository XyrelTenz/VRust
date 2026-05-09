# Vrust

Vrust is a high-performance, minimalist Android emulator runner and manager built for the Neovim ecosystem. By leveraging Rust and Tauri, Vrust eliminates the overhead of a full IDE, providing a standalone, gRPC-powered window that streams a headless Android environment directly to your workspace. Designed for developers who live in the terminal and demand speed.

## Features

- Headless Emulator Execution: Launch Android Virtual Devices without the default bulky emulator window.
- Real-time Streaming: High-performance screen streaming using gRPC protocol for low-latency feedback.
- System Navigation: Dedicated controls for Home, Back, Recent Apps, and Power functions.
- Boot Management: Support for multiple launch modes including Normal Boot, Cold Start, and Wipe Data.
- Automatic Device Detection: Automatically lists all available Android Virtual Devices (AVDs) configured on the system.
- System Tray Integration: Quick access and application management from the system tray.
- Desktop Notifications: Visual feedback for emulator boot status and connectivity.
- Modern UI: Sleek, dark-themed interface built with Vue 3 and Tailwind CSS.

## Current Status

Vrust is currently in active development. While it successfully streams the emulator environment, the interaction is currently limited to read-only mode. Touch events have not yet been implemented, so the emulator is not yet responsive to direct manipulation (such as opening apps or navigating settings) like a standard mobile device.


