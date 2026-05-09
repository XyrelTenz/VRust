# Vrust

Vrust is a high-performance, minimalist Android emulator runner and manager built for the Neovim ecosystem. By leveraging Rust and Tauri, Vrust eliminates the overhead of a full IDE, providing a standalone, gRPC-powered window that streams a headless Android environment directly to your workspace. Designed for developers who live in the terminal and demand speed.

## Features

- Headless Emulator Execution: Launch Android Virtual Devices without the default bulky emulator window.
- Real-time Streaming: High-performance screen streaming using gRPC protocol for low-latency feedback.
- Interactive Touch Control: Direct interaction with the emulator screen using mouse-to-touch coordinate mapping.
- System Navigation: Dedicated controls for Home, Back, Recent Apps, and Power functions.
- Boot Management: Support for multiple launch modes including Normal Boot, Cold Start, and Wipe Data.
- Automatic Device Detection: Automatically lists all available Android Virtual Devices (AVDs) configured on the system.
- System Tray Integration: Quick access and application management from the system tray.
- Desktop Notifications: Visual feedback for emulator boot status and connectivity.
- Modern UI: Sleek, dark-themed interface built with Vue 3 and Tailwind CSS.

## Prerequisites

Before setting up the project, ensure you have the following installed:

- Android SDK: The emulator command must be available in your system PATH.
- Rust and Cargo: Required for building the Tauri backend.
- Node.js: Required for the frontend development environment.
- Bun: The recommended package manager for this project.

## Setup Instructions

Follow these steps to get the application running on your local machine:

1. Clone the repository
Get the source code from the repository.

2. Install dependencies
Run the following command in the project root to install all required frontend and backend dependencies:
bun install

3. Run in development mode
To start the application in development mode with hot-reloading:
bun run tauri dev

4. Build for production
To create a production-ready executable for your platform:
bun run tauri build

## Configuration

The application uses the default Android SDK configuration. Ensure your AVDs are correctly set up in Android Studio or via the command line before launching them through Vrust.

## Technical Details

- Frontend: Vue 3 with TypeScript and Tailwind CSS.
- Backend: Rust with Tauri framework.
- Communication: gRPC for emulator interaction and screen streaming.
- Window Management: Frameless, transparent window design for a premium feel.
