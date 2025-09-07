/**
 * WebAssembly Game Engine JavaScript Bindings
 * High-level API for easy game development
 */

import { SimpleGameEngine } from './simple-engine.js';

class GameEngine {
    constructor(canvasId) {
        this.canvas = document.getElementById(canvasId);
        if (!this.canvas) {
            throw new Error(`Canvas with id "${canvasId}" not found`);
        }
        
        this.wasmEngine = null;
        this.fallbackEngine = null;
        this.useWasm = false;
        this.isRunning = false;
        this.lastTime = 0;
        this.sprites = new Map();
        this.nextSpriteId = 1;
        
        // Input state
        this.keys = {};
        this.mouse = { x: 0, y: 0, buttons: {} };
        
        this.setupEventListeners();
    }
    
    async init() {
        try {
            // Try WebAssembly first
            try {
                const wasm = await import('../pkg/wasm_game_engine.js');
                await wasm.default();
                
                this.wasmEngine = new wasm.WasmGameEngine(this.canvas.id);
                this.useWasm = true;
                console.log('🎮 WebAssembly Game Engine initialized successfully!');
                return true;
            } catch (wasmError) {
                console.log('WebAssembly not available, falling back to JavaScript engine');
                // Fall back to JavaScript implementation
                this.fallbackEngine = new SimpleGameEngine(this.canvas.id);
                await this.fallbackEngine.init();
                this.useWasm = false;
                console.log('🎮 JavaScript Game Engine initialized successfully!');
                return true;
            }
        } catch (error) {
            console.error('Failed to initialize any game engine:', error);
            return false;
        }
    }
    
    start() {
        const engine = this.useWasm ? this.wasmEngine : this.fallbackEngine;
        if (!engine) {
            throw new Error('Engine not initialized. Call init() first.');
        }
        
        if (this.useWasm) {
            if (this.isRunning) return;
            this.isRunning = true;
            this.lastTime = performance.now();
            this.gameLoop();
        } else {
            this.fallbackEngine.start();
        }
        console.log('🚀 Game loop started!');
    }
    
    stop() {
        if (this.useWasm) {
            this.isRunning = false;
        } else {
            this.fallbackEngine.stop();
        }
        console.log('⏸️ Game loop stopped.');
    }
    
    gameLoop = (currentTime) => {
        if (!this.isRunning || !this.useWasm) return;
        
        const deltaTime = currentTime - this.lastTime;
        this.lastTime = currentTime;
        
        // Update engine
        this.wasmEngine.update(deltaTime);
        
        // Render frame
        this.wasmEngine.render();
        
        // Continue loop
        requestAnimationFrame(this.gameLoop);
    }
    
    // Sprite management
    createSprite(options = {}) {
        if (this.useWasm) {
            const {
                x = 0,
                y = 0,
                width = 50,
                height = 50,
                color = { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
            } = options;
            
            const wasmSpriteId = this.wasmEngine.create_sprite(x, y, width, height);
            const spriteId = `sprite_${this.nextSpriteId++}`;
            
            const sprite = {
                id: spriteId,
                wasmId: wasmSpriteId,
                x, y, width, height,
                color: { ...color }
            };
            
            this.sprites.set(spriteId, sprite);
            
            // Set initial color
            this.wasmEngine.set_sprite_color(wasmSpriteId, color.r, color.g, color.b, color.a);
            
            return sprite;
        } else {
            return this.fallbackEngine.createSprite(options);
        }
    }
    
    moveSprite(spriteId, x, y) {
        if (this.useWasm) {
            const sprite = this.sprites.get(spriteId);
            if (sprite) {
                sprite.x = x;
                sprite.y = y;
                this.wasmEngine.set_sprite_position(sprite.wasmId, x, y);
            }
        } else {
            this.fallbackEngine.moveSprite(spriteId, x, y);
        }
    }
    
    setSpriteColor(spriteId, color) {
        if (this.useWasm) {
            const sprite = this.sprites.get(spriteId);
            if (sprite) {
                sprite.color = { ...color };
                this.wasmEngine.set_sprite_color(sprite.wasmId, color.r, color.g, color.b, color.a);
            }
        } else {
            this.fallbackEngine.setSpriteColor(spriteId, color);
        }
    }
    
    getSprite(spriteId) {
        if (this.useWasm) {
            return this.sprites.get(spriteId);
        } else {
            return this.fallbackEngine.getSprite(spriteId);
        }
    }
    
    // Input handling
    setupEventListeners() {
        // Keyboard events
        window.addEventListener('keydown', (e) => {
            this.keys[e.code] = true;
            if (this.wasmEngine) {
                this.wasmEngine.handle_key_down(e.keyCode);
            }
        });
        
        window.addEventListener('keyup', (e) => {
            this.keys[e.code] = false;
            if (this.wasmEngine) {
                this.wasmEngine.handle_key_up(e.keyCode);
            }
        });
        
        // Mouse events
        this.canvas.addEventListener('mousemove', (e) => {
            const rect = this.canvas.getBoundingClientRect();
            this.mouse.x = e.clientX - rect.left;
            this.mouse.y = e.clientY - rect.top;
            
            if (this.wasmEngine) {
                this.wasmEngine.handle_mouse_move(this.mouse.x, this.mouse.y);
            }
        });
        
        this.canvas.addEventListener('mousedown', (e) => {
            this.mouse.buttons[e.button] = true;
            if (this.wasmEngine) {
                this.wasmEngine.handle_mouse_down(e.button, this.mouse.x, this.mouse.y);
            }
        });
        
        this.canvas.addEventListener('mouseup', (e) => {
            this.mouse.buttons[e.button] = false;
            if (this.wasmEngine) {
                this.wasmEngine.handle_mouse_up(e.button, this.mouse.x, this.mouse.y);
            }
        });
        
        // Resize handling
        window.addEventListener('resize', () => {
            this.resize();
        });
        
        // Initial resize
        this.resize();
    }
    
    resize() {
        const rect = this.canvas.getBoundingClientRect();
        this.canvas.width = rect.width;
        this.canvas.height = rect.height;
        
        if (this.wasmEngine) {
            this.wasmEngine.resize(this.canvas.width, this.canvas.height);
        }
    }
    
    // Input queries
    isKeyPressed(keyCode) {
        if (this.useWasm) {
            return this.keys[keyCode] || false;
        } else {
            return this.fallbackEngine.isKeyPressed(keyCode);
        }
    }
    
    getMousePosition() {
        if (this.useWasm) {
            return { x: this.mouse.x, y: this.mouse.y };
        } else {
            return this.fallbackEngine.getMousePosition();
        }
    }
    
    isMouseButtonPressed(button) {
        if (this.useWasm) {
            return this.mouse.buttons[button] || false;
        } else {
            return this.fallbackEngine.isMouseButtonPressed(button);
        }
    }
    
    // Utility methods
    getCanvasSize() {
        if (this.useWasm) {
            return {
                width: this.canvas.width,
                height: this.canvas.height
            };
        } else {
            return this.fallbackEngine.getCanvasSize();
        }
    }
    
    // Color utilities
    static Color = {
        WHITE: { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
        BLACK: { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
        RED: { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
        GREEN: { r: 0.0, g: 1.0, b: 0.0, a: 1.0 },
        BLUE: { r: 0.0, g: 0.0, b: 1.0, a: 1.0 },
        YELLOW: { r: 1.0, g: 1.0, b: 0.0, a: 1.0 },
        MAGENTA: { r: 1.0, g: 0.0, b: 1.0, a: 1.0 },
        CYAN: { r: 0.0, g: 1.0, b: 1.0, a: 1.0 },
        
        // Create color from hex
        fromHex(hex) {
            const r = ((hex >> 16) & 0xFF) / 255;
            const g = ((hex >> 8) & 0xFF) / 255;
            const b = (hex & 0xFF) / 255;
            return { r, g, b, a: 1.0 };
        },
        
        // Create random color
        random() {
            return {
                r: Math.random(),
                g: Math.random(),
                b: Math.random(),
                a: 1.0
            };
        }
    };
}

// Key code constants for convenience
GameEngine.Keys = {
    W: 'KeyW',
    A: 'KeyA',
    S: 'KeyS',
    D: 'KeyD',
    SPACE: 'Space',
    ENTER: 'Enter',
    ESCAPE: 'Escape',
    ARROW_UP: 'ArrowUp',
    ARROW_DOWN: 'ArrowDown',
    ARROW_LEFT: 'ArrowLeft',
    ARROW_RIGHT: 'ArrowRight'
};

export { GameEngine };
