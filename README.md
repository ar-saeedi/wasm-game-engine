# 🎮 WebAssembly Game Engine

A high-performance game engine built with Rust and compiled to WebAssembly for near-native performance in the browser.

## ⚡ Features

- **High Performance**: Rust + WebAssembly for near-native speed
- **Modern Graphics**: WebGL 2.0 rendering pipeline
- **ECS Architecture**: Entity-Component-System for scalability
- **Physics Engine**: Built-in 2D physics with collision detection
- **Audio System**: Web Audio API integration
- **Cross Platform**: Runs in any modern browser
- **Developer Friendly**: Simple JavaScript API

## 🚀 Quick Start

### Prerequisites

1. **Install Rust**: https://rustup.rs/
2. **Install wasm-pack**: 
   ```bash
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```
3. **Install Node.js**: https://nodejs.org/

### Setup

```bash
# Clone and setup
cd wasm-game-engine
npm install

# Build WebAssembly module
wasm-pack build --target web --out-dir pkg

# Start development server
npm run dev
```

### Basic Usage

```javascript
import { GameEngine } from './pkg/wasm_game_engine.js';

// Initialize engine
const engine = new GameEngine('canvas');

// Create a sprite
const player = engine.create_sprite({
    x: 100, y: 100,
    width: 32, height: 32,
    texture: 'player.png'
});

// Game loop
engine.start();
```

## 🏗️ Architecture

```
src/
├── core/           # Core engine systems
├── graphics/       # WebGL renderer
├── physics/        # 2D physics engine
├── audio/          # Audio management
├── input/          # Input handling
└── utils/          # Utilities
```

## 📚 Documentation

See `/docs` for complete API documentation and tutorials.

## 🎯 Roadmap

- [x] Core ECS system
- [x] WebGL rendering
- [x] Input handling
- [x] 2D Physics
- [x] Audio integration
- [ ] 3D support
- [ ] Networking
- [ ] Advanced particles
- [ ] Level editor

## 🤝 Contributing

1. Fork the repository
2. Create feature branch
3. Make changes
4. Add tests
5. Submit pull request

## 📄 License

MIT License - see LICENSE file for details.

---

**⭐ Star this repo if you find it useful!**
