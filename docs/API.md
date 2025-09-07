# WebAssembly Game Engine API Documentation

## 🎮 GameEngine Class

The main class for creating and managing your game.

### Constructor

```javascript
const engine = new GameEngine(canvasId);
```

**Parameters:**
- `canvasId` (string): The ID of the HTML canvas element

### Methods

#### `async init()`
Initializes the WebAssembly module and sets up the engine.

**Returns:** `Promise<boolean>` - Success status

```javascript
const success = await engine.init();
if (success) {
    engine.start();
}
```

#### `start()`
Starts the game loop.

#### `stop()`
Stops the game loop.

#### `createSprite(options)`
Creates a new sprite in the game world.

**Parameters:**
- `options` (object):
  - `x` (number): X position (default: 0)
  - `y` (number): Y position (default: 0)
  - `width` (number): Width in pixels (default: 50)
  - `height` (number): Height in pixels (default: 50)
  - `color` (object): RGBA color (default: white)

**Returns:** Sprite object with `id`, `x`, `y`, `width`, `height`, `color`

```javascript
const player = engine.createSprite({
    x: 100,
    y: 100,
    width: 32,
    height: 32,
    color: GameEngine.Color.BLUE
});
```

#### `moveSprite(spriteId, x, y)`
Moves a sprite to a new position.

```javascript
engine.moveSprite(player.id, 200, 150);
```

#### `setSpriteColor(spriteId, color)`
Changes a sprite's color.

```javascript
engine.setSpriteColor(player.id, GameEngine.Color.RED);
```

#### `getSprite(spriteId)`
Gets sprite data by ID.

**Returns:** Sprite object or `undefined`

#### Input Methods

##### `isKeyPressed(keyCode)`
Checks if a key is currently pressed.

```javascript
if (engine.isKeyPressed(GameEngine.Keys.W)) {
    // Move up
}
```

##### `getMousePosition()`
Gets current mouse position.

**Returns:** `{x: number, y: number}`

##### `isMouseButtonPressed(button)`
Checks if a mouse button is pressed.

**Parameters:**
- `button` (number): 0 = left, 1 = middle, 2 = right

#### Utility Methods

##### `getCanvasSize()`
Gets canvas dimensions.

**Returns:** `{width: number, height: number}`

##### `resize()`
Handles canvas resize (called automatically).

---

## 🎨 Colors

### Predefined Colors

```javascript
GameEngine.Color.WHITE    // {r: 1.0, g: 1.0, b: 1.0, a: 1.0}
GameEngine.Color.BLACK    // {r: 0.0, g: 0.0, b: 0.0, a: 1.0}
GameEngine.Color.RED      // {r: 1.0, g: 0.0, b: 0.0, a: 1.0}
GameEngine.Color.GREEN    // {r: 0.0, g: 1.0, b: 0.0, a: 1.0}
GameEngine.Color.BLUE     // {r: 0.0, g: 0.0, b: 1.0, a: 1.0}
GameEngine.Color.YELLOW   // {r: 1.0, g: 1.0, b: 0.0, a: 1.0}
GameEngine.Color.MAGENTA  // {r: 1.0, g: 0.0, b: 1.0, a: 1.0}
GameEngine.Color.CYAN     // {r: 0.0, g: 1.0, b: 1.0, a: 1.0}
```

### Color Methods

#### `GameEngine.Color.fromHex(hex)`
Creates color from hexadecimal value.

```javascript
const purple = GameEngine.Color.fromHex(0x800080);
```

#### `GameEngine.Color.random()`
Generates a random color.

```javascript
const randomColor = GameEngine.Color.random();
```

---

## ⌨️ Input

### Key Constants

```javascript
GameEngine.Keys.W          // 'KeyW'
GameEngine.Keys.A          // 'KeyA'
GameEngine.Keys.S          // 'KeyS'
GameEngine.Keys.D          // 'KeyD'
GameEngine.Keys.SPACE      // 'Space'
GameEngine.Keys.ENTER      // 'Enter'
GameEngine.Keys.ESCAPE     // 'Escape'
GameEngine.Keys.ARROW_UP   // 'ArrowUp'
GameEngine.Keys.ARROW_DOWN // 'ArrowDown'
GameEngine.Keys.ARROW_LEFT // 'ArrowLeft'
GameEngine.Keys.ARROW_RIGHT// 'ArrowRight'
```

### Input Handling

The engine automatically handles input events. Use the query methods to check input state:

```javascript
function gameLoop() {
    // Keyboard input
    if (engine.isKeyPressed(GameEngine.Keys.W)) {
        movePlayerUp();
    }
    
    // Mouse input
    if (engine.isMouseButtonPressed(0)) {
        const mouse = engine.getMousePosition();
        shootAt(mouse.x, mouse.y);
    }
}
```

---

## 🔧 Advanced Usage

### Custom Game Loop

```javascript
let lastTime = 0;

function customGameLoop(currentTime) {
    const deltaTime = currentTime - lastTime;
    lastTime = currentTime;
    
    // Update game logic
    updatePlayer(deltaTime);
    updateEnemies(deltaTime);
    
    // The engine handles rendering automatically
    requestAnimationFrame(customGameLoop);
}

engine.init().then(() => {
    engine.start();
    requestAnimationFrame(customGameLoop);
});
```

### Sprite Animation

```javascript
class AnimatedSprite {
    constructor(engine, x, y) {
        this.sprite = engine.createSprite({x, y, width: 32, height: 32});
        this.velocity = {x: 2, y: 1};
        this.engine = engine;
    }
    
    update() {
        // Update position
        this.sprite.x += this.velocity.x;
        this.sprite.y += this.velocity.y;
        
        // Bounce off edges
        const canvas = this.engine.getCanvasSize();
        if (this.sprite.x <= 0 || this.sprite.x >= canvas.width - 32) {
            this.velocity.x *= -1;
        }
        if (this.sprite.y <= 0 || this.sprite.y >= canvas.height - 32) {
            this.velocity.y *= -1;
        }
        
        // Update sprite position
        this.engine.moveSprite(this.sprite.id, this.sprite.x, this.sprite.y);
    }
}
```

### Collision Detection

```javascript
function checkCollision(sprite1, sprite2) {
    return sprite1.x < sprite2.x + sprite2.width &&
           sprite1.x + sprite1.width > sprite2.x &&
           sprite1.y < sprite2.y + sprite2.height &&
           sprite1.y + sprite1.height > sprite2.y;
}

// Usage
if (checkCollision(player, enemy)) {
    // Handle collision
    engine.setSpriteColor(player.id, GameEngine.Color.RED);
}
```

---

## 🚀 Performance Tips

1. **Batch Operations**: Group multiple sprite updates together
2. **Object Pooling**: Reuse sprite objects instead of creating new ones
3. **Efficient Collision**: Use spatial partitioning for many objects
4. **Frame Rate**: Target 60 FPS, use `requestAnimationFrame`
5. **Canvas Size**: Optimize canvas resolution for target devices

---

## 🎯 Examples

See the `/examples` directory for complete working examples:

- `basic-demo.html` - Sprite creation and manipulation
- `space-game.html` - Complete game with collision detection

---

## 🐛 Troubleshooting

### Common Issues

**Engine fails to initialize:**
- Ensure browser supports WebAssembly
- Check console for error messages
- Verify all files are served from a web server

**Poor performance:**
- Reduce number of sprites
- Lower canvas resolution
- Optimize game loop logic

**Input not working:**
- Ensure canvas has focus
- Check key code constants
- Verify event listeners are set up

### Browser Compatibility

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

WebAssembly must be supported by the browser.
