import init, { PolyominoSolver, get_predefined_polyomino } from './pkg/polyomino_tiling.js';

class PolyominoApp {
    constructor() {
        this.solver = null;
        this.selectedPieces = [];
        this.isInitialized = false;
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
                const polyomino = get_predefined_polyomino(pieceName);
                this.solver.add_polyomino(polyomino);
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
}

// Initialize the application when the page loads
document.addEventListener('DOMContentLoaded', () => {
    console.log('DOM loaded, initializing app...');
    new PolyominoApp();
});