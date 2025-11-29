/**
 * Simple JavaScript Game Engine (WebAssembly fallback)
 * High-performance 2D rendering with WebGL
 */

class SimpleGameEngine {
    constructor(canvasId) {
        this.canvas = document.getElementById(canvasId);
        if (!this.canvas) {
            throw new Error(`Canvas with id "${canvasId}" not found`);
        }
        
        this.gl = this.canvas.getContext('webgl2') || this.canvas.getContext('webgl');
        if (!this.gl) {
            throw new Error('WebGL not supported');
        }
        
        this.isRunning = false;
        this.lastTime = 0;
        this.sprites = new Map();
        this.nextSpriteId = 1;
        
        // Input state
        this.keys = {};
        this.mouse = { x: 0, y: 0, buttons: {} };
        
        // WebGL resources
        this.shaderProgram = null;
        this.vertexBuffer = null;
        this.indexBuffer = null;
        
        this.setupWebGL();
        this.setupEventListeners();
    }
    
    async init() {
        try {
            console.log('🎮 Simple Game Engine initialized successfully!');
            return true;
        } catch (error) {
            console.error('Failed to initialize simple engine:', error);
            return false;
        }
    }
    
    setupWebGL() {
        const gl = this.gl;
        
        // Vertex shader
        const vertexShaderSource = `
            attribute vec2 a_position;
            uniform mat4 u_matrix;
            uniform vec2 u_size;
            uniform vec2 u_offset;
            
            void main() {
                vec2 position = a_position * u_size + u_offset;
                gl_Position = u_matrix * vec4(position, 0.0, 1.0);
            }
        `;
        
        // Fragment shader
        const fragmentShaderSource = `
            precision mediump float;
            uniform vec4 u_color;
            
            void main() {
                gl_FragColor = u_color;
            }
        `;
        
        // Create shaders
        const vertexShader = this.createShader(gl.VERTEX_SHADER, vertexShaderSource);
        const fragmentShader = this.createShader(gl.FRAGMENT_SHADER, fragmentShaderSource);
        
        // Create program
        this.shaderProgram = gl.createProgram();
        gl.attachShader(this.shaderProgram, vertexShader);
        gl.attachShader(this.shaderProgram, fragmentShader);
        gl.linkProgram(this.shaderProgram);
        
        if (!gl.getProgramParameter(this.shaderProgram, gl.LINK_STATUS)) {
            throw new Error('Program link error: ' + gl.getProgramInfoLog(this.shaderProgram));
        }
        
        // Get attribute and uniform locations
        this.positionAttributeLocation = gl.getAttribLocation(this.shaderProgram, 'a_position');
        this.matrixUniformLocation = gl.getUniformLocation(this.shaderProgram, 'u_matrix');
        this.sizeUniformLocation = gl.getUniformLocation(this.shaderProgram, 'u_size');
        this.offsetUniformLocation = gl.getUniformLocation(this.shaderProgram, 'u_offset');
        this.colorUniformLocation = gl.getUniformLocation(this.shaderProgram, 'u_color');
        
        // Create quad geometry
        const vertices = new Float32Array([
            0, 0,  // bottom left
            1, 0,  // bottom right
            0, 1,  // top left
            1, 1,  // top right
        ]);
        
        const indices = new Uint16Array([
            0, 1, 2,  // first triangle
            2, 1, 3   // second triangle
        ]);
        
        // Create buffers
        this.vertexBuffer = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, this.vertexBuffer);
        gl.bufferData(gl.ARRAY_BUFFER, vertices, gl.STATIC_DRAW);
        
        this.indexBuffer = gl.createBuffer();
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.indexBuffer);
        gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, indices, gl.STATIC_DRAW);
        
        // Setup projection matrix
        this.updateProjectionMatrix();
        
        // Enable blending
        gl.enable(gl.BLEND);
        gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
    }
    
    createShader(type, source) {
        const gl = this.gl;
        const shader = gl.createShader(type);
        gl.shaderSource(shader, source);
        gl.compileShader(shader);
        
        if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
            throw new Error('Shader compile error: ' + gl.getShaderInfoLog(shader));
        }
        
        return shader;
    }
    
    updateProjectionMatrix() {
        const width = this.canvas.width;
        const height = this.canvas.height;
        
        // Orthographic projection matrix
        this.projectionMatrix = new Float32Array([
            2 / width,  0,           0, 0,
            0,         -2 / height,  0, 0,
            0,          0,          -1, 0,
           -1,          1,           0, 1
        ]);
    }
    
    start() {
        if (this.isRunning) return;
        
        this.isRunning = true;
        this.lastTime = performance.now();
        this.gameLoop();
        console.log('🚀 Game loop started!');
    }
    
    stop() {
        this.isRunning = false;
        console.log('⏸️ Game loop stopped.');
    }
    
    gameLoop = (currentTime) => {
        if (!this.isRunning) return;
        
        const deltaTime = currentTime - this.lastTime;
        this.lastTime = currentTime;
        
        this.render();
        
        requestAnimationFrame(this.gameLoop);
    }
    
    render() {
        const gl = this.gl;
        
        // Clear
        gl.viewport(0, 0, this.canvas.width, this.canvas.height);
        gl.clearColor(0.2, 0.3, 0.3, 1.0);
        gl.clear(gl.COLOR_BUFFER_BIT);
        
        // Use shader program
        gl.useProgram(this.shaderProgram);
        
        // Set projection matrix
        gl.uniformMatrix4fv(this.matrixUniformLocation, false, this.projectionMatrix);
        
        // Bind buffers
        gl.bindBuffer(gl.ARRAY_BUFFER, this.vertexBuffer);
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, this.indexBuffer);
        
        // Enable position attribute
        gl.enableVertexAttribArray(this.positionAttributeLocation);
        gl.vertexAttribPointer(this.positionAttributeLocation, 2, gl.FLOAT, false, 0, 0);
        
        // Render each sprite
        for (const sprite of this.sprites.values()) {
            // Set uniforms
            gl.uniform2f(this.sizeUniformLocation, sprite.width, sprite.height);
            gl.uniform2f(this.offsetUniformLocation, sprite.x, sprite.y);
            gl.uniform4f(this.colorUniformLocation, sprite.color.r, sprite.color.g, sprite.color.b, sprite.color.a);
            
            // Draw
            gl.drawElements(gl.TRIANGLES, 6, gl.UNSIGNED_SHORT, 0);
        }
    }
    
    createSprite(options = {}) {
        const {
            x = 0,
            y = 0,
            width = 50,
            height = 50,
            color = { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
        } = options;
        
        const spriteId = `sprite_${this.nextSpriteId++}`;
        
        const sprite = {
            id: spriteId,
            x, y, width, height,
            color: { ...color }
        };
        
        this.sprites.set(spriteId, sprite);
        return sprite;
    }
    
    moveSprite(spriteId, x, y) {
        const sprite = this.sprites.get(spriteId);
        if (sprite) {
            sprite.x = x;
            sprite.y = y;
        }
    }
    
    setSpriteColor(spriteId, color) {
        const sprite = this.sprites.get(spriteId);
        if (sprite) {
            sprite.color = { ...color };
        }
    }
    
    getSprite(spriteId) {
        return this.sprites.get(spriteId);
    }
    
    removeSprite(spriteId) {
        return this.sprites.delete(spriteId);
    }
    
    clearAllSprites() {
        this.sprites.clear();
    }
    
    setupEventListeners() {
        // Keyboard events
        window.addEventListener('keydown', (e) => {
            this.keys[e.code] = true;
        });
        
        window.addEventListener('keyup', (e) => {
            this.keys[e.code] = false;
        });
        
        // Mouse events
        this.canvas.addEventListener('mousemove', (e) => {
            const rect = this.canvas.getBoundingClientRect();
            this.mouse.x = e.clientX - rect.left;
            this.mouse.y = e.clientY - rect.top;
        });
        
        this.canvas.addEventListener('mousedown', (e) => {
            this.mouse.buttons[e.button] = true;
        });
        
        this.canvas.addEventListener('mouseup', (e) => {
            this.mouse.buttons[e.button] = false;
        });
        
        // Resize handling
        window.addEventListener('resize', () => {
            this.resize();
        });
        
        this.resize();
    }
    
    resize() {
        const rect = this.canvas.getBoundingClientRect();
        this.canvas.width = rect.width;
        this.canvas.height = rect.height;
        this.updateProjectionMatrix();
    }
    
    isKeyPressed(keyCode) {
        return this.keys[keyCode] || false;
    }
    
    getMousePosition() {
        return { x: this.mouse.x, y: this.mouse.y };
    }
    
    isMouseButtonPressed(button) {
        return this.mouse.buttons[button] || false;
    }
    
    getCanvasSize() {
        return {
            width: this.canvas.width,
            height: this.canvas.height
        };
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
        
        fromHex(hex) {
            const r = ((hex >> 16) & 0xFF) / 255;
            const g = ((hex >> 8) & 0xFF) / 255;
            const b = (hex & 0xFF) / 255;
            return { r, g, b, a: 1.0 };
        },
        
        random() {
            return {
                r: Math.random(),
                g: Math.random(),
                b: Math.random(),
                a: 1.0
            };
        }
    };
    
    static Keys = {
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
}

export { SimpleGameEngine };
