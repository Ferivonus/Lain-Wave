# Lain Wave

Lain Wave is a highly optimized music management system built for speed and reliability. It is engineered to work entirely offline, ensuring that your library remains fully accessible without any internet dependency. The application includes an integrated tool to download tracks directly from YouTube, allowing you to build and maintain a permanent local archive effortlessly.

## System Capabilities

### 1. Performance and Independence

- Optimized Architecture: Built with a lightweight footprint to ensure minimal system resource usage and near-instant response times.
- YouTube Acquisition: Integrated functionality to download music from YouTube and automatically index it into your local collection.
- Local-First Design: The system is designed to function perfectly without an internet connection once your music is archived.

### 2. Intelligent Discovery and Radio

- Discover Panel: An analytical dashboard that tracks your listening habits to highlight your most played tracks and recent additions.
- Automated Radio Stations: A smart system that categorizes your existing collection into genre-based radio stations for automated playback.

### 3. Library and Collection Management

- Automated Indexing: Scans your storage to organize raw audio files into a structured and searchable local database.
- Interactive Playlists: Complete control over personal collections with intuitive drag-and-drop support for reordering.

### 4. Audio and Visual Standards

- High-Fidelity Support: Native playback for lossless formats like FLAC and WAV, as well as standard MP3, including specific quality indicators.
- Visual Protocols: Multiple aesthetic modes such as Modern Dark, Cyberpunk, Ghibli, and OLED to suit different environments.
- Presence Integration: Real-time synchronization with Discord to display your current track and playback progress.

## Technical Stack

- Frontend: Svelte 5 utilizing the Runes API.
- Framework: Tauri v2 for a secure and small desktop footprint.
- Styling: Tailwind CSS.
- Database: Local data persistence layer managed via Rust.

## Installation and Setup

To run this project, you must have Node.js and Rust installed on your system.

1. Clone the repository:
   git clone https://github.com/Ferivonus/lainwave.git
   cd lainwave

2. Install dependencies:
   npm install

3. Run in development mode:
   npm run tauri dev

4. Build the production installer:
   npm run tauri build

## Project Structure

- src/lib: Reusable UI components and logic modules.
- src/routes: Main application pages including Dashboard, Library, and Radio.
- src/store.svelte.ts: Global state management hub for player logic and data.
- src-tauri/src: Rust backend responsible for file operations, YouTube downloading, and Discord integration.

## Credits

Lain Wave was developed by Fahrettin Baştürk.

"No matter where you are, everyone is always connected."
