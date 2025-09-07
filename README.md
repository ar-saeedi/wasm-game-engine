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
- **JavaScript Fallback**: Works even without WebAssembly compilation

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
git clone https://github.com/ar-saeedi/wasm-game-engine.git
cd wasm-game-engine
npm install

# Build WebAssembly module (optional - JavaScript fallback available)
wasm-pack build --target web --out-dir pkg

# Start development server
npm run dev
```

### Basic Usage

```javascript
import { GameEngine } from './js/game-engine.js';

// Initialize engine (automatically selects WebAssembly or JavaScript)
const engine = new GameEngine('canvas');
await engine.init();

// Create a sprite
const player = engine.createSprite({
    x: 100, y: 100,
    width: 32, height: 32,
    color: GameEngine.Color.BLUE
});

// Start game loop
engine.start();
```

## 🎮 Live Examples

- **[Interactive Demo](examples/basic-demo.html)** - Sprite creation and animation
- **[Space Shooter Game](examples/space-game.html)** - Complete playable game

## 🏗️ Architecture

```
src/
├── core/           # Core engine systems (ECS, time management)
├── graphics/       # WebGL renderer with shaders
├── physics/        # 2D physics engine with collision detection
├── audio/          # Web Audio API integration
├── input/          # Cross-platform input handling
├── math/           # Vector and matrix mathematics
└── utils/          # Utilities and helpers

js/
├── game-engine.js  # Main JavaScript API
└── simple-engine.js # WebGL fallback engine

examples/
├── basic-demo.html # Interactive sprite demo
└── space-game.html # Complete space shooter game
```

## 📚 Documentation

- **[API Documentation](docs/API.md)** - Complete API reference
- **[Examples](examples/)** - Working code samples
- **[Project Overview](index.html)** - Interactive showcase

## 🎯 Features Completed

- [x] **Core ECS system** - Entity-Component-System architecture
- [x] **WebGL rendering** - Hardware-accelerated graphics pipeline
- [x] **Input handling** - Keyboard, mouse, and touch support
- [x] **2D Physics** - Collision detection and response
- [x] **Audio integration** - Web Audio API support
- [x] **JavaScript fallback** - Works without WebAssembly compilation
- [x] **Matrix math** - Complete 2D/3D mathematics library
- [x] **Examples** - Interactive demos and games

## 🚀 Future Roadmap

- [ ] **3D support** - WebGL 2.0 3D rendering
- [ ] **Networking** - Multiplayer capabilities
- [ ] **Advanced particles** - Particle system effects
- [ ] **Level editor** - Visual game development tools
- [ ] **Asset pipeline** - Texture and audio management
- [ ] **Mobile optimization** - Touch and performance improvements

## 🤝 Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Make changes and test thoroughly
4. Add examples if applicable
5. Submit pull request

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

---

**⭐ Star this repo if you find it useful!**

Built with ❤️ using Rust, WebAssembly, and WebGL
