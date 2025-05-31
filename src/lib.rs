use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

mod dancing_links;
mod polyomino_tiling;
mod pretty;

use polyomino_tiling::{PolyominoTiling, piece_placements_to_matrix_of_piece_ids};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Serialize, Deserialize)]
pub struct PolyominoSolution {
    pub piece_id: usize,
    pub placement: Vec<Vec<usize>>,
}

#[wasm_bindgen]
pub struct PolyominoSolver {
    width: usize,
    height: usize,
    polyominoes: Vec<Vec<Vec<usize>>>,
}

#[wasm_bindgen]
impl PolyominoSolver {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> PolyominoSolver {
        console_log!("Creating new PolyominoSolver with dimensions {}x{}", width, height);
        PolyominoSolver {
            width,
            height,
            polyominoes: Vec::new(),
        }
    }

    #[wasm_bindgen]
    pub fn add_polyomino(&mut self, polyomino_js: JsValue) -> Result<(), JsValue> {
        let polyomino: Vec<Vec<usize>> = serde_wasm_bindgen::from_value(polyomino_js)?;
        console_log!("Adding polyomino with {} rows", polyomino.len());
        self.polyominoes.push(polyomino);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn solve(&self) -> Result<JsValue, JsValue> {
        console_log!("Solving polyomino tiling problem...");
        let tiling = PolyominoTiling::new(self.width, self.height, self.polyominoes.clone());
        let solution = tiling.solve();
        
        match solution {
            Some(sol) => {
                console_log!("Found solution with {} pieces", sol.len());
                let js_solution: Vec<PolyominoSolution> = sol
                    .into_iter()
                    .map(|(piece_id, placement)| PolyominoSolution { piece_id, placement })
                    .collect();
                Ok(serde_wasm_bindgen::to_value(&js_solution)?)
            }
            None => {
                console_log!("No solution found");
                Ok(JsValue::NULL)
            }
        }
    }

    #[wasm_bindgen]
    pub fn get_solution_matrix(&self, solution_js: JsValue) -> Result<JsValue, JsValue> {
        if solution_js.is_null() {
            return Ok(JsValue::NULL);
        }
        
        let solution: Vec<PolyominoSolution> = serde_wasm_bindgen::from_value(solution_js)?;
        let piece_placements: Vec<(usize, Vec<Vec<usize>>)> = solution
            .into_iter()
            .map(|sol| (sol.piece_id, sol.placement))
            .collect();
        
        let matrix = piece_placements_to_matrix_of_piece_ids(&piece_placements, self.width, self.height);
        Ok(serde_wasm_bindgen::to_value(&matrix)?)
    }
}

// Predefined polyominoes from the original code
#[wasm_bindgen]
pub fn get_predefined_polyomino(name: &str) -> Result<JsValue, JsValue> {
    use pretty::str_to_matrix;
    
    let polyomino = match name {
        "L" => str_to_matrix(vec!["###", "#..", "#.."]),
        "l" => str_to_matrix(vec!["####", "#..."]),
        "I" => str_to_matrix(vec!["#####"]),
        "C" => str_to_matrix(vec!["##", "#.", "##"]),
        "S" => str_to_matrix(vec![".##", ".#.", "##."]),
        "s" => str_to_matrix(vec![".###", "##.."]),
        "X" => str_to_matrix(vec![".#.", "###", ".#."]),
        "F" => str_to_matrix(vec!["##.", ".##", ".#."]),
        "T" => str_to_matrix(vec!["###", ".#.", ".#."]),
        "t" => str_to_matrix(vec!["####", ".#.."]),
        "M" => str_to_matrix(vec![".##", "##.", "#.."]),
        "b" => str_to_matrix(vec!["#.", "##", "##"]),
        _ => return Err(JsValue::from_str("Unknown polyomino name")),
    };
    
    Ok(serde_wasm_bindgen::to_value(&polyomino)?)
}