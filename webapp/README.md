# Polyomino Tiling Web Application

A web-based solver for polyomino tiling puzzles, powered by WebAssembly and Rust.

## Features

- Interactive web interface for selecting polyomino pieces
- Real-time puzzle solving using dancing links algorithm
- Visual representation of solutions
- Support for 12 different polyomino shapes (L, l, I, C, S, s, X, F, T, t, M, b)
- Configurable board dimensions (1x1 to 20x20)

## Usage

1. **Start a local server** (required for WASM modules):
   ```bash
   # From the webapp directory
   python3 -m http.server 8000
   # Or with Node.js
   npx serve .
   ```

2. **Open your browser** and navigate to:
   ```
   http://localhost:8000
   ```

3. **Configure the puzzle**:
   - Set board width and height
   - Click on polyomino buttons to select pieces
   - Click "Solve Puzzle" to find a solution

4. **View results**:
   - Solutions are displayed as a colorful grid
   - Each piece is shown in a different color
   - Hover over cells to see piece information

## Polyomino Pieces

- **L**: 4-cell L-shaped piece
- **l**: 5-cell L-shaped piece  
- **I**: 5-cell straight line
- **C**: 4-cell C-shaped piece
- **S**: 5-cell S-shaped piece
- **s**: 4-cell S-shaped piece
- **X**: 5-cell plus/cross shape
- **F**: 5-cell F-shaped piece
- **T**: 5-cell T-shaped piece
- **t**: 4-cell T-shaped piece
- **M**: 5-cell M-shaped piece
- **b**: 4-cell small block

## Technical Details

- Built with Rust and compiled to WebAssembly
- Uses dancing links algorithm for exact cover solving
- Supports piece rotation and mirroring
- Real-time visualization with CSS Grid

## Development

To rebuild the WASM module:
```bash
wasm-pack build --target web --out-dir pkg
```