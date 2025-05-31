import init, { PolyominoSolver, get_predefined_polyomino } from './pkg/polyomino_tiling.js';

class PolyominoApp {
    constructor() {
        this.solver = null;
        this.selectedPieces = [];
        this.customPolyominoes = [];
        this.isInitialized = false;
        this.editorGrid = [];
        this.editorSize = 4;
        this.init();
    }

    async init() {
        try {
            await init();
            this.isInitialized = true;
            this.setupEventListeners();
            this.updateStatus("Ready! Select polyominoes and click 'Solve Puzzle'");
        } catch (error) {
            console.error('Failed to initialize WASM:', error);
            this.updateStatus("Failed to initialize. Please refresh the page.", 'error');
        }
    }

    setupEventListeners() {
        // Polyomino selection buttons
        document.querySelectorAll('.polyomino-btn').forEach(btn => {
            btn.addEventListener('click', () => this.togglePiece(btn.dataset.piece));
        });

        // Solve button
        document.getElementById('solve-btn').addEventListener('click', () => this.solvePuzzle());

        // Clear button
        document.getElementById('clear-btn').addEventListener('click', () => this.clearSelection());

        // Board dimension inputs
        document.getElementById('board-width').addEventListener('change', () => this.updateBoard());
        document.getElementById('board-height').addEventListener('change', () => this.updateBoard());

        // Custom polyomino editor
        document.getElementById('add-mino-btn').addEventListener('click', () => this.openEditor());
        document.getElementById('close-modal').addEventListener('click', () => this.closeEditor());
        document.getElementById('editor-size').addEventListener('change', (e) => this.changeEditorSize(parseInt(e.target.value)));
        document.getElementById('clear-editor-btn').addEventListener('click', () => this.clearEditor());
        document.getElementById('save-mino-btn').addEventListener('click', () => this.saveCustomMino());
        
        // Close modal when clicking outside
        document.getElementById('custom-mino-modal').addEventListener('click', (e) => {
            if (e.target.id === 'custom-mino-modal') {
                this.closeEditor();
            }
        });
    }

    togglePiece(pieceName) {
        console.log('togglePiece called with:', pieceName);
        const btn = document.querySelector(`[data-piece="${pieceName}"]`);
        console.log('Button found:', btn);
        const index = this.selectedPieces.indexOf(pieceName);
        console.log('Current selected pieces:', this.selectedPieces);
        
        if (index === -1) {
            this.selectedPieces.push(pieceName);
            btn.classList.add('selected');
            console.log('Added piece:', pieceName);
        } else {
            this.selectedPieces.splice(index, 1);
            btn.classList.remove('selected');
            console.log('Removed piece:', pieceName);
        }
        
        this.updateSelectedList();
    }

    updateSelectedList() {
        const listElement = document.getElementById('selected-list');
        listElement.textContent = this.selectedPieces.length > 0 
            ? this.selectedPieces.join(', ') 
            : 'None selected';
    }

    clearSelection() {
        this.selectedPieces = [];
        document.querySelectorAll('.polyomino-btn').forEach(btn => {
            btn.classList.remove('selected');
        });
        this.updateSelectedList();
        this.clearBoard();
        this.updateStatus("Selection cleared. Choose polyominoes to solve.");
    }

    async solvePuzzle() {
        if (!this.isInitialized) {
            this.updateStatus("Application not ready yet. Please wait.", 'error');
            return;
        }

        if (this.selectedPieces.length === 0) {
            this.updateStatus("Please select at least one polyomino.", 'error');
            return;
        }

        const width = parseInt(document.getElementById('board-width').value);
        const height = parseInt(document.getElementById('board-height').value);

        if (width < 1 || height < 1 || width > 20 || height > 20) {
            this.updateStatus("Board dimensions must be between 1 and 20.", 'error');
            return;
        }

        this.updateStatus("Solving puzzle...", 'loading');
        document.getElementById('solve-btn').disabled = true;

        try {
            // Create solver
            this.solver = new PolyominoSolver(width, height);

            // Add selected polyominoes
            for (const pieceName of this.selectedPieces) {
                if (pieceName.startsWith('custom_')) {
                    // Custom polyomino
                    const customMino = this.customPolyominoes.find(m => m.name === pieceName);
                    if (customMino) {
                        this.solver.add_polyomino(customMino.matrix);
                    }
                } else {
                    // Predefined polyomino
                    const polyomino = get_predefined_polyomino(pieceName);
                    this.solver.add_polyomino(polyomino);
                }
            }

            // Solve
            const solution = this.solver.solve();
            
            if (solution === null) {
                this.updateStatus("No solution found for the selected pieces and board size.", 'error');
                this.clearBoard();
            } else {
                this.updateStatus(`Solution found! Used ${solution.length} pieces.`, 'success');
                this.displaySolution(solution);
            }
        } catch (error) {
            console.error('Solving error:', error);
            this.updateStatus(`Error solving puzzle: ${error.message}`, 'error');
        } finally {
            document.getElementById('solve-btn').disabled = false;
        }
    }

    displaySolution(solution) {
        const width = parseInt(document.getElementById('board-width').value);
        const height = parseInt(document.getElementById('board-height').value);
        
        try {
            const solutionMatrix = this.solver.get_solution_matrix(solution);
            this.renderBoard(solutionMatrix, width, height);
        } catch (error) {
            console.error('Display error:', error);
            this.updateStatus(`Error displaying solution: ${error.message}`, 'error');
        }
    }

    renderBoard(matrix, width, height) {
        const board = document.getElementById('board');
        board.style.gridTemplateColumns = `repeat(${width}, 1fr)`;
        board.innerHTML = '';

        for (let row = 0; row < height; row++) {
            for (let col = 0; col < width; col++) {
                const cell = document.createElement('div');
                cell.className = 'cell';
                
                const pieceId = matrix[row][col];
                if (pieceId === null) {
                    cell.classList.add('empty');
                } else {
                    cell.classList.add(`piece-${pieceId}`);
                    cell.title = `Piece ${this.selectedPieces[pieceId]} (ID: ${pieceId})`;
                }
                
                board.appendChild(cell);
            }
        }
    }

    clearBoard() {
        const board = document.getElementById('board');
        board.innerHTML = '';
    }

    updateBoard() {
        this.clearBoard();
        if (this.selectedPieces.length > 0) {
            this.updateStatus("Board dimensions changed. Click 'Solve Puzzle' to solve again.");
        }
    }

    updateStatus(message, type = '') {
        const statusElement = document.getElementById('status-message');
        const statusContainer = statusElement.parentElement;
        
        statusElement.textContent = message;
        
        // Remove existing status classes
        statusContainer.classList.remove('success', 'error', 'loading');
        
        // Add new status class if provided
        if (type) {
            statusContainer.classList.add(type);
        }
    }

    // Custom Polyomino Editor Methods
    openEditor() {
        const modal = document.getElementById('custom-mino-modal');
        modal.classList.remove('hidden');
        this.initializeEditor();
    }

    closeEditor() {
        const modal = document.getElementById('custom-mino-modal');
        modal.classList.add('hidden');
        this.clearEditor();
    }

    initializeEditor() {
        this.editorSize = parseInt(document.getElementById('editor-size').value);
        this.createEditorGrid();
        this.updatePreview();
    }

    createEditorGrid() {
        const editor = document.getElementById('mino-editor');
        editor.innerHTML = '';
        editor.style.gridTemplateColumns = `repeat(${this.editorSize}, 1fr)`;
        
        this.editorGrid = [];
        for (let row = 0; row < this.editorSize; row++) {
            this.editorGrid[row] = [];
            for (let col = 0; col < this.editorSize; col++) {
                this.editorGrid[row][col] = false;
                
                const cell = document.createElement('div');
                cell.className = 'editor-cell';
                cell.dataset.row = row;
                cell.dataset.col = col;
                
                cell.addEventListener('click', () => this.toggleEditorCell(row, col));
                
                editor.appendChild(cell);
            }
        }
    }

    toggleEditorCell(row, col) {
        this.editorGrid[row][col] = !this.editorGrid[row][col];
        
        const cell = document.querySelector(`[data-row="${row}"][data-col="${col}"]`);
        if (this.editorGrid[row][col]) {
            cell.classList.add('active');
        } else {
            cell.classList.remove('active');
        }
        
        this.updatePreview();
    }

    changeEditorSize(newSize) {
        this.editorSize = newSize;
        this.createEditorGrid();
        this.updatePreview();
    }

    clearEditor() {
        this.editorGrid = [];
        this.createEditorGrid();
        this.updatePreview();
        document.getElementById('custom-mino-name').value = '';
    }

    updatePreview() {
        const preview = document.getElementById('mino-preview');
        preview.innerHTML = '';
        
        if (this.editorGrid.length === 0) return;
        
        // Find bounding box of active cells
        let minRow = this.editorSize, maxRow = -1;
        let minCol = this.editorSize, maxCol = -1;
        
        for (let row = 0; row < this.editorSize; row++) {
            for (let col = 0; col < this.editorSize; col++) {
                if (this.editorGrid[row][col]) {
                    minRow = Math.min(minRow, row);
                    maxRow = Math.max(maxRow, row);
                    minCol = Math.min(minCol, col);
                    maxCol = Math.max(maxCol, col);
                }
            }
        }
        
        if (maxRow === -1) return; // No active cells
        
        const height = maxRow - minRow + 1;
        const width = maxCol - minCol + 1;
        
        preview.style.gridTemplateColumns = `repeat(${width}, 1fr)`;
        
        for (let row = minRow; row <= maxRow; row++) {
            for (let col = minCol; col <= maxCol; col++) {
                const cell = document.createElement('div');
                cell.className = 'preview-cell';
                if (this.editorGrid[row][col]) {
                    cell.classList.add('active');
                }
                preview.appendChild(cell);
            }
        }
    }

    isConnected() {
        // Check if all active cells form a connected shape
        const activeCells = [];
        for (let row = 0; row < this.editorSize; row++) {
            for (let col = 0; col < this.editorSize; col++) {
                if (this.editorGrid[row][col]) {
                    activeCells.push([row, col]);
                }
            }
        }
        
        if (activeCells.length === 0) return false;
        if (activeCells.length === 1) return true;
        
        // BFS to check connectivity
        const visited = new Set();
        const queue = [activeCells[0]];
        visited.add(`${activeCells[0][0]},${activeCells[0][1]}`);
        
        while (queue.length > 0) {
            const [row, col] = queue.shift();
            
            // Check 4 directions
            for (const [dr, dc] of [[-1,0], [1,0], [0,-1], [0,1]]) {
                const newRow = row + dr;
                const newCol = col + dc;
                const key = `${newRow},${newCol}`;
                
                if (newRow >= 0 && newRow < this.editorSize && 
                    newCol >= 0 && newCol < this.editorSize && 
                    this.editorGrid[newRow][newCol] && 
                    !visited.has(key)) {
                    visited.add(key);
                    queue.push([newRow, newCol]);
                }
            }
        }
        
        return visited.size === activeCells.length;
    }

    saveCustomMino() {
        const nameInput = document.getElementById('custom-mino-name');
        let name = nameInput.value.trim();
        
        if (!name) {
            alert('Please enter a name for your polyomino.');
            return;
        }
        
        // Check if shape has any active cells
        const hasActiveCells = this.editorGrid.some(row => row.some(cell => cell));
        if (!hasActiveCells) {
            alert('Please draw a polyomino shape first.');
            return;
        }
        
        // Check if shape is connected
        if (!this.isConnected()) {
            alert('Polyomino must be a connected shape (all cells must touch).');
            return;
        }
        
        // Convert to matrix format (trim to bounding box)
        let minRow = this.editorSize, maxRow = -1;
        let minCol = this.editorSize, maxCol = -1;
        
        for (let row = 0; row < this.editorSize; row++) {
            for (let col = 0; col < this.editorSize; col++) {
                if (this.editorGrid[row][col]) {
                    minRow = Math.min(minRow, row);
                    maxRow = Math.max(maxRow, row);
                    minCol = Math.min(minCol, col);
                    maxCol = Math.max(maxCol, col);
                }
            }
        }
        
        const matrix = [];
        for (let row = minRow; row <= maxRow; row++) {
            const matrixRow = [];
            for (let col = minCol; col <= maxCol; col++) {
                matrixRow.push(this.editorGrid[row][col] ? 1 : 0);
            }
            matrix.push(matrixRow);
        }
        
        // Make sure name is unique
        const customName = `custom_${name}`;
        if (this.customPolyominoes.some(m => m.name === customName) || 
            this.selectedPieces.includes(customName)) {
            alert('A polyomino with this name already exists.');
            return;
        }
        
        // Save custom polyomino
        this.customPolyominoes.push({
            name: customName,
            displayName: name,
            matrix: matrix
        });
        
        // Add button to selection area
        this.addCustomPolyominoButton(customName, name);
        
        // Close editor
        this.closeEditor();
        
        this.updateStatus(`Custom polyomino "${name}" added successfully!`, 'success');
    }

    addCustomPolyominoButton(fullName, displayName) {
        const buttonsContainer = document.querySelector('.polyomino-buttons');
        
        const button = document.createElement('button');
        button.className = 'polyomino-btn';
        button.dataset.piece = fullName;
        
        // Get the custom polyomino matrix
        const customMino = this.customPolyominoes.find(m => m.name === fullName);
        
        // Create visual representation
        const shapeDiv = document.createElement('div');
        shapeDiv.className = 'polyomino-shape custom-shape';
        shapeDiv.style.display = 'grid';
        shapeDiv.style.gap = '0px';
        shapeDiv.style.gridTemplateColumns = `repeat(${customMino.matrix[0].length}, 6px)`;
        shapeDiv.style.justifyContent = 'center';
        
        // Add cells for the custom shape
        for (let row = 0; row < customMino.matrix.length; row++) {
            for (let col = 0; col < customMino.matrix[row].length; col++) {
                const cell = document.createElement('div');
                cell.style.width = '6px';
                cell.style.height = '6px';
                cell.style.borderRadius = '1px';
                
                if (customMino.matrix[row][col] === 1) {
                    cell.style.background = '#007bff';
                } else {
                    cell.style.background = 'transparent';
                }
                
                shapeDiv.appendChild(cell);
            }
        }
        
        const labelSpan = document.createElement('span');
        labelSpan.className = 'polyomino-label';
        labelSpan.textContent = displayName;
        
        button.appendChild(shapeDiv);
        button.appendChild(labelSpan);
        
        button.addEventListener('click', () => this.togglePiece(fullName));
        
        buttonsContainer.appendChild(button);
    }
}

// Initialize the application when the page loads
document.addEventListener('DOMContentLoaded', () => {
    console.log('DOM loaded, initializing app...');
    new PolyominoApp();
});