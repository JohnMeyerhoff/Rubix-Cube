
use rand::prelude::*; // For random number generation
pub struct Side {
    pub faces: Vec<char>,
}
impl Side {
    fn copy_side(&self) -> Side {
        let old_faces = &self.faces;
        Side {
            faces: old_faces.clone(),
        }
    }
    pub fn is_solved(&self) -> bool {
        //A side is solved if all faces are equal
        let first_face = &self.faces[0];
        for face in &self.faces {
            if first_face != face {
                return false;
            }
        }
        true
    }
}
pub struct Cube {
    pub sides: Vec<Side>,
    pub previous_moves: Vec<char>,
    pub num_moves: u8,
    pub test : Vec<String>,
    //test = vec!["F".to_string(),"F".to_string(),"F".to_string(),"F".to_string(),"F".to_string(),"F".to_string(),"F".to_string(),"F".to_string(),],
        
}
impl Cube {
    pub fn copy_cube(&self) -> Cube {
        Cube {
            sides: vec![
                self.sides[0].copy_side(),
                self.sides[1].copy_side(),
                self.sides[2].copy_side(),
                self.sides[3].copy_side(),
                self.sides[4].copy_side(),
                self.sides[5].copy_side(),
            ],
            previous_moves: self.previous_moves.clone(),
            num_moves: self.num_moves,
            test : vec!["F".to_string(),"F".to_string(),"F".to_string(),"F".to_string(),"F".to_string(),"F".to_string(),"F".to_string(),"F".to_string(),],
        }
    }
    pub fn is_solved(&self) -> bool {
       // println!("Last2Moves: {:?}",&self.previous_moves.iter().rev().take(3).collect::<Vec<_>>());
        /*println!(
            "&self.previous_moves.count() : {:?}",&self.previous_moves.iter());// outputs : Some('u')
        //A cube is solved if all sides have all the same colours on their faces.*/
        if &self.previous_moves.len()<= &self.test.len(){
             return false; } //Assert less than
        else{
        for side in &self.sides {
            if side.is_solved() == false {
                return false;
            }
        }
        true
    }
    }

    pub fn scramble_cube(&self, n: u8) -> Cube {
        //Applies n random moves to cube.
        let mut new_cube = self.copy_cube();
        for _ in 0..n {
            //Loop runs n times. Underscore for variable name means we don't use the index for anything.
            let rng = thread_rng().gen_range(0, 12); //Random int from 0 to 11
            let rotated_cube = match rng {
                0 => new_cube.rotate_back_clockwise(),
                1 => new_cube.rotate_back_counter_clockwise(),
                2 => new_cube.rotate_down_clockwise(),
                3 => new_cube.rotate_down_counter_clockwise(),
                4 => new_cube.rotate_front_clockwise(),
                5 => new_cube.rotate_front_counter_clockwise(),
                6 => new_cube.rotate_left_clockwise(),
                7 => new_cube.rotate_left_counter_clockwise(),
                8 => new_cube.rotate_right_clockwise(),
                9 => new_cube.rotate_right_counter_clockwise(),
                10 => new_cube.rotate_up_clockwise(),
                11 => new_cube.rotate_up_counter_clockwise(),
                _ => panic!("RNG ran out of bounds!"), //Should never happen
            };
            new_cube = rotated_cube;
        }
        let mut new_predeterminedcube = self.copy_cube();
        /*B2 U R2 F2 R2
         B2 F2 R2 U'
        L2 D2 F' R'
        B2 R F2
        L' U F2 L2*/
        /*let turned_cube = new_predeterminedcube.rotate_bottom_clockwise();
        new_predeterminedcube = turned_cube;*/
        new_predeterminedcube = new_predeterminedcube.rotate_back_clockwise(); //B
        new_predeterminedcube = new_predeterminedcube.rotate_back_clockwise(); //B
        new_predeterminedcube = new_predeterminedcube.rotate_up_clockwise(); //U
        new_predeterminedcube = new_predeterminedcube.rotate_right_clockwise(); //R
        new_predeterminedcube = new_predeterminedcube.rotate_right_clockwise(); //R
        new_predeterminedcube = new_predeterminedcube.rotate_front_clockwise(); //F
        new_predeterminedcube = new_predeterminedcube.rotate_front_clockwise(); //F
        new_predeterminedcube = new_predeterminedcube.rotate_right_clockwise(); //R
        new_predeterminedcube = new_predeterminedcube.rotate_right_clockwise(); //R
                                                                                /*
                                                                                DONE: B2 U R2 F2 R2 */
        new_predeterminedcube = new_predeterminedcube.rotate_back_clockwise(); //B
        new_predeterminedcube = new_predeterminedcube.rotate_back_clockwise(); //B
        new_predeterminedcube = new_predeterminedcube.rotate_front_clockwise(); //F
        new_predeterminedcube = new_predeterminedcube.rotate_front_clockwise(); //F
        new_predeterminedcube = new_predeterminedcube.rotate_right_clockwise(); //R
        new_predeterminedcube = new_predeterminedcube.rotate_right_clockwise(); //R
        new_predeterminedcube = new_predeterminedcube.rotate_up_counter_clockwise(); //U'
                                                                                     /*
                                                                                     DONE: B2 F2 R2 U' */
        new_predeterminedcube = new_predeterminedcube.rotate_left_clockwise(); //L
        new_predeterminedcube = new_predeterminedcube.rotate_left_clockwise(); //L
        new_predeterminedcube = new_predeterminedcube.rotate_down_clockwise(); //D
        new_predeterminedcube = new_predeterminedcube.rotate_down_clockwise(); //D
        new_predeterminedcube = new_predeterminedcube.rotate_front_counter_clockwise(); //F'
        new_predeterminedcube = new_predeterminedcube.rotate_right_counter_clockwise(); //R'
                                                                                        /*
                                                                                        DONE: L2 D2 F' R' */
        new_predeterminedcube = new_predeterminedcube.rotate_back_clockwise(); //B
        new_predeterminedcube = new_predeterminedcube.rotate_back_clockwise(); //B
        new_predeterminedcube = new_predeterminedcube.rotate_right_clockwise(); //R
        new_predeterminedcube = new_predeterminedcube.rotate_front_clockwise(); //F
        new_predeterminedcube = new_predeterminedcube.rotate_front_clockwise(); //F
                                                                                /*
                                                                                DONE: B2 R F2 */
        new_predeterminedcube = new_predeterminedcube.rotate_left_counter_clockwise(); //L'
        new_predeterminedcube = new_predeterminedcube.rotate_up_clockwise(); //U
        new_predeterminedcube = new_predeterminedcube.rotate_front_clockwise(); //F
        new_predeterminedcube = new_predeterminedcube.rotate_front_clockwise(); //F
        new_predeterminedcube = new_predeterminedcube.rotate_left_clockwise(); //L
        new_predeterminedcube = new_predeterminedcube.rotate_left_clockwise(); //L
                                                                               /*
                                                                               DONE: L' U F2 L2 */

        new_predeterminedcube
    }
    pub fn forget_moves(&self) -> Cube {
        //Blanks the list of previous moves, keeps the state of the cube intact.
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves = vec![];
        new_cube.num_moves = 0;
        new_cube
    }
    pub fn print_cube(&self) -> () {
        println!(
            "TOP (WHITE)
             {}{}{}
             {}{}{}
             {}{}{}
             ------
             FRONT (GREEN)| RIGHT (RED)
             {}{}{}     |{}{}{}|{}{}{}|{}{}{}
             {}{}{}     |{}{}{}|{}{}{}|{}{}{}
             {}{}{}     |{}{}{}|{}{}{}|{}{}{}
             ------
             BOTTOM (YELLOW)
             {}{}{}
             {}{}{}
             {}{}{}
        ",
            //top cube
            self.sides[1].faces[0],
            self.sides[1].faces[1],
            self.sides[1].faces[2],
            self.sides[1].faces[3],
            self.sides[1].faces[4],
            self.sides[1].faces[5],
            self.sides[1].faces[6],
            self.sides[1].faces[7],
            self.sides[1].faces[8],
            //middle cubes, first row
            self.sides[4].faces[0],
            self.sides[4].faces[1],
            self.sides[4].faces[2],
            self.sides[0].faces[0],
            self.sides[0].faces[1],
            self.sides[0].faces[2],
            self.sides[2].faces[0],
            self.sides[2].faces[1],
            self.sides[2].faces[2],
            self.sides[5].faces[0],
            self.sides[5].faces[1],
            self.sides[5].faces[2],
            //middle cubes, second row
            self.sides[4].faces[3],
            self.sides[4].faces[4],
            self.sides[4].faces[5],
            self.sides[0].faces[3],
            self.sides[0].faces[4],
            self.sides[0].faces[5],
            self.sides[2].faces[3],
            self.sides[2].faces[4],
            self.sides[2].faces[5],
            self.sides[5].faces[3],
            self.sides[5].faces[4],
            self.sides[5].faces[5],
            //middle cubes, third row
            self.sides[4].faces[6],
            self.sides[4].faces[7],
            self.sides[4].faces[8],
            self.sides[0].faces[6],
            self.sides[0].faces[7],
            self.sides[0].faces[8],
            self.sides[2].faces[6],
            self.sides[2].faces[7],
            self.sides[2].faces[8],
            self.sides[5].faces[6],
            self.sides[5].faces[7],
            self.sides[5].faces[8],
            //bottom cube
            self.sides[3].faces[0],
            self.sides[3].faces[1],
            self.sides[3].faces[2],
            self.sides[3].faces[3],
            self.sides[3].faces[4],
            self.sides[3].faces[5],
            self.sides[3].faces[6],
            self.sides[3].faces[7],
            self.sides[3].faces[8]
        )
    }
    pub fn rotate_front_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push('F');
        new_cube.num_moves += 1;
        //rotate top side
        new_cube.sides[0].faces[0] = self.sides[0].faces[6];
        new_cube.sides[0].faces[1] = self.sides[0].faces[3];
        new_cube.sides[0].faces[2] = self.sides[0].faces[0];
        new_cube.sides[0].faces[3] = self.sides[0].faces[7];
        new_cube.sides[0].faces[5] = self.sides[0].faces[1];
        new_cube.sides[0].faces[6] = self.sides[0].faces[8];
        new_cube.sides[0].faces[7] = self.sides[0].faces[5];
        new_cube.sides[0].faces[8] = self.sides[0].faces[2];
        //rotate left side, top row
        new_cube.sides[4].faces[2] = self.sides[3].faces[0];
        new_cube.sides[4].faces[5] = self.sides[3].faces[1];
        new_cube.sides[4].faces[8] = self.sides[3].faces[2];
        //rotate Up side, top row
        new_cube.sides[1].faces[6] = self.sides[4].faces[2];
        new_cube.sides[1].faces[7] = self.sides[4].faces[5];
        new_cube.sides[1].faces[8] = self.sides[4].faces[8];
        //rotate Right side, top row
        new_cube.sides[2].faces[0] = self.sides[1].faces[6];
        new_cube.sides[2].faces[3] = self.sides[1].faces[7];
        new_cube.sides[2].faces[6] = self.sides[1].faces[8];
        //rotate Down side, top row
        new_cube.sides[3].faces[0] = self.sides[2].faces[0];
        new_cube.sides[3].faces[1] = self.sides[2].faces[3];
        new_cube.sides[3].faces[2] = self.sides[2].faces[6];
        new_cube
    }
    pub fn rotate_front_counter_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push('f');
        new_cube.num_moves += 1;
        //rotate Facing side
        new_cube.sides[0].faces[6] = self.sides[0].faces[0];
        new_cube.sides[0].faces[3] = self.sides[0].faces[1];
        new_cube.sides[0].faces[0] = self.sides[0].faces[2];
        new_cube.sides[0].faces[7] = self.sides[0].faces[3];
        new_cube.sides[0].faces[1] = self.sides[0].faces[5];
        new_cube.sides[0].faces[8] = self.sides[0].faces[6];
        new_cube.sides[0].faces[5] = self.sides[0].faces[7];
        new_cube.sides[0].faces[2] = self.sides[0].faces[8];
        //rotate Down side, top row
        new_cube.sides[3].faces[0] = self.sides[4].faces[2];
        new_cube.sides[3].faces[1] = self.sides[4].faces[5];
        new_cube.sides[3].faces[2] = self.sides[4].faces[8];
        //rotate Left side, top row
        new_cube.sides[4].faces[2] = self.sides[1].faces[6];
        new_cube.sides[4].faces[5] = self.sides[1].faces[7];
        new_cube.sides[4].faces[8] = self.sides[1].faces[8];
        //rotate Up side, top row
        new_cube.sides[1].faces[6] = self.sides[2].faces[0];
        new_cube.sides[1].faces[7] = self.sides[2].faces[3];
        new_cube.sides[1].faces[8] = self.sides[2].faces[6];
        //rotate Right side, top row
        new_cube.sides[2].faces[0] = self.sides[3].faces[0];
        new_cube.sides[2].faces[3] = self.sides[3].faces[1];
        new_cube.sides[2].faces[6] = self.sides[3].faces[2];
        new_cube
    }
    pub fn rotate_up_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push('U');
        new_cube.num_moves += 1;
        //rotate up side
        new_cube.sides[1].faces[0] = self.sides[1].faces[6];
        new_cube.sides[1].faces[1] = self.sides[1].faces[3];
        new_cube.sides[1].faces[2] = self.sides[1].faces[0];
        new_cube.sides[1].faces[3] = self.sides[1].faces[7];
        new_cube.sides[1].faces[5] = self.sides[1].faces[1];
        new_cube.sides[1].faces[6] = self.sides[1].faces[8];
        new_cube.sides[1].faces[7] = self.sides[1].faces[5];
        new_cube.sides[1].faces[8] = self.sides[1].faces[2];
        //rotate left side
        new_cube.sides[4].faces[0] = self.sides[0].faces[0];
        new_cube.sides[4].faces[1] = self.sides[0].faces[1];
        new_cube.sides[4].faces[2] = self.sides[0].faces[2];
        //rotate bottom side
        new_cube.sides[5].faces[0] = self.sides[4].faces[0];
        new_cube.sides[5].faces[1] = self.sides[4].faces[1];
        new_cube.sides[5].faces[2] = self.sides[4].faces[2];
        //rotate Right side
        new_cube.sides[2].faces[0] = self.sides[5].faces[0];
        new_cube.sides[2].faces[1] = self.sides[5].faces[1];
        new_cube.sides[2].faces[2] = self.sides[5].faces[2];
        //rotate Down side
        new_cube.sides[0].faces[0] = self.sides[2].faces[0];
        new_cube.sides[0].faces[1] = self.sides[2].faces[1];
        new_cube.sides[0].faces[2] = self.sides[2].faces[2];
        new_cube
    }
    pub fn rotate_up_counter_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push('u');
        new_cube.num_moves += 1;
        //rotate up side
        new_cube.sides[1].faces[6] = self.sides[1].faces[0];
        new_cube.sides[1].faces[3] = self.sides[1].faces[1];
        new_cube.sides[1].faces[0] = self.sides[1].faces[2];
        new_cube.sides[1].faces[7] = self.sides[1].faces[3];
        new_cube.sides[1].faces[1] = self.sides[1].faces[5];
        new_cube.sides[1].faces[8] = self.sides[1].faces[6];
        new_cube.sides[1].faces[5] = self.sides[1].faces[7];
        new_cube.sides[1].faces[2] = self.sides[1].faces[8];
        //rotate top side
        new_cube.sides[0].faces[0] = self.sides[4].faces[0];
        new_cube.sides[0].faces[1] = self.sides[4].faces[1];
        new_cube.sides[0].faces[2] = self.sides[4].faces[2];
        //rotate left side
        new_cube.sides[4].faces[0] = self.sides[5].faces[0];
        new_cube.sides[4].faces[1] = self.sides[5].faces[1];
        new_cube.sides[4].faces[2] = self.sides[5].faces[2];
        //rotate Bottom side
        new_cube.sides[5].faces[0] = self.sides[2].faces[0];
        new_cube.sides[5].faces[1] = self.sides[2].faces[1];
        new_cube.sides[5].faces[2] = self.sides[2].faces[2];
        //rotate right side
        new_cube.sides[2].faces[0] = self.sides[0].faces[0];
        new_cube.sides[2].faces[1] = self.sides[0].faces[1];
        new_cube.sides[2].faces[2] = self.sides[0].faces[2];
        new_cube
    }
    pub fn rotate_right_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push('R');
        new_cube.num_moves += 1;
        //rotate right side
        new_cube.sides[2].faces[0] = self.sides[2].faces[6];
        new_cube.sides[2].faces[1] = self.sides[2].faces[3];
        new_cube.sides[2].faces[2] = self.sides[2].faces[0];
        new_cube.sides[2].faces[3] = self.sides[2].faces[7];
        new_cube.sides[2].faces[5] = self.sides[2].faces[1];
        new_cube.sides[2].faces[6] = self.sides[2].faces[8];
        new_cube.sides[2].faces[7] = self.sides[2].faces[5];
        new_cube.sides[2].faces[8] = self.sides[2].faces[2];
        //rotate Down side
        new_cube.sides[3].faces[2] = self.sides[5].faces[0];
        new_cube.sides[3].faces[5] = self.sides[5].faces[3];
        new_cube.sides[3].faces[8] = self.sides[5].faces[6];
        //rotate bottom side
        new_cube.sides[5].faces[0] = self.sides[1].faces[2];
        new_cube.sides[5].faces[3] = self.sides[1].faces[5];
        new_cube.sides[5].faces[6] = self.sides[1].faces[8];
        //rotate Front side
        new_cube.sides[0].faces[2] = self.sides[3].faces[2];
        new_cube.sides[0].faces[5] = self.sides[3].faces[5];
        new_cube.sides[0].faces[8] = self.sides[3].faces[8];
        //rotate up side
        new_cube.sides[1].faces[2] = self.sides[0].faces[2];
        new_cube.sides[1].faces[5] = self.sides[0].faces[5];
        new_cube.sides[1].faces[8] = self.sides[0].faces[8];
        new_cube
    }
    pub fn rotate_right_counter_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push('r');
        new_cube.num_moves += 1;
        //rotate right side
        new_cube.sides[2].faces[6] = self.sides[2].faces[0];
        new_cube.sides[2].faces[3] = self.sides[2].faces[1];
        new_cube.sides[2].faces[0] = self.sides[2].faces[2];
        new_cube.sides[2].faces[7] = self.sides[2].faces[3];
        new_cube.sides[2].faces[1] = self.sides[2].faces[5];
        new_cube.sides[2].faces[8] = self.sides[2].faces[6];
        new_cube.sides[2].faces[5] = self.sides[2].faces[7];
        new_cube.sides[2].faces[2] = self.sides[2].faces[8];
        //rotate bottom side
        new_cube.sides[5].faces[0] = self.sides[3].faces[2];
        new_cube.sides[5].faces[3] = self.sides[3].faces[5];
        new_cube.sides[5].faces[6] = self.sides[3].faces[8];
        //rotate up side
        new_cube.sides[1].faces[2] = self.sides[5].faces[0];
        new_cube.sides[1].faces[5] = self.sides[5].faces[3];
        new_cube.sides[1].faces[8] = self.sides[5].faces[6];
        //rotate Down side
        new_cube.sides[3].faces[2] = self.sides[0].faces[2];
        new_cube.sides[3].faces[5] = self.sides[0].faces[5];
        new_cube.sides[3].faces[8] = self.sides[0].faces[8];
        //rotate Front side
        new_cube.sides[0].faces[2] = self.sides[1].faces[2];
        new_cube.sides[0].faces[5] = self.sides[1].faces[5];
        new_cube.sides[0].faces[8] = self.sides[1].faces[8];
        new_cube
    }
    pub fn rotate_down_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push('D');
        new_cube.num_moves += 1;
        //rotate down side
        new_cube.sides[3].faces[0] = self.sides[3].faces[6];
        new_cube.sides[3].faces[1] = self.sides[3].faces[3];
        new_cube.sides[3].faces[2] = self.sides[3].faces[0];
        new_cube.sides[3].faces[3] = self.sides[3].faces[7];
        new_cube.sides[3].faces[5] = self.sides[3].faces[1];
        new_cube.sides[3].faces[6] = self.sides[3].faces[8];
        new_cube.sides[3].faces[7] = self.sides[3].faces[5];
        new_cube.sides[3].faces[8] = self.sides[3].faces[2];
        //rotate Front side
        new_cube.sides[0].faces[6] = self.sides[4].faces[6];
        new_cube.sides[0].faces[7] = self.sides[4].faces[7];
        new_cube.sides[0].faces[8] = self.sides[4].faces[8];
        //rotate Right side
        new_cube.sides[2].faces[6] = self.sides[0].faces[6];
        new_cube.sides[2].faces[7] = self.sides[0].faces[7];
        new_cube.sides[2].faces[8] = self.sides[0].faces[8];
        //rotate Bottom side
        new_cube.sides[5].faces[6] = self.sides[2].faces[6];
        new_cube.sides[5].faces[7] = self.sides[2].faces[7];
        new_cube.sides[5].faces[8] = self.sides[2].faces[8];
        //rotate left side
        new_cube.sides[4].faces[6] = self.sides[5].faces[6];
        new_cube.sides[4].faces[7] = self.sides[5].faces[7];
        new_cube.sides[4].faces[8] = self.sides[5].faces[8];
        new_cube
    }
    pub fn rotate_down_counter_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push('d');
        new_cube.num_moves += 1;
        //rotate down side
        new_cube.sides[3].faces[6] = self.sides[3].faces[0];
        new_cube.sides[3].faces[3] = self.sides[3].faces[1];
        new_cube.sides[3].faces[0] = self.sides[3].faces[2];
        new_cube.sides[3].faces[7] = self.sides[3].faces[3];
        new_cube.sides[3].faces[1] = self.sides[3].faces[5];
        new_cube.sides[3].faces[8] = self.sides[3].faces[6];
        new_cube.sides[3].faces[5] = self.sides[3].faces[7];
        new_cube.sides[3].faces[2] = self.sides[3].faces[8];
        //rotate Front side
        new_cube.sides[4].faces[6] = self.sides[0].faces[6];
        new_cube.sides[4].faces[7] = self.sides[0].faces[7];
        new_cube.sides[4].faces[8] = self.sides[0].faces[8];
        //rotate Right side
        new_cube.sides[0].faces[6] = self.sides[2].faces[6];
        new_cube.sides[0].faces[7] = self.sides[2].faces[7];
        new_cube.sides[0].faces[8] = self.sides[2].faces[8];
        //rotate Bottom side
        new_cube.sides[2].faces[6] = self.sides[5].faces[6];
        new_cube.sides[2].faces[7] = self.sides[5].faces[7];
        new_cube.sides[2].faces[8] = self.sides[5].faces[8];
        //rotate left side
        new_cube.sides[5].faces[6] = self.sides[4].faces[6];
        new_cube.sides[5].faces[7] = self.sides[4].faces[7];
        new_cube.sides[5].faces[8] = self.sides[4].faces[8];
        new_cube
    }
    pub fn rotate_left_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push('L');
        new_cube.num_moves += 1;
        //rotate down side
        new_cube.sides[4].faces[0] = self.sides[4].faces[6];
        new_cube.sides[4].faces[1] = self.sides[4].faces[3];
        new_cube.sides[4].faces[2] = self.sides[4].faces[0];
        new_cube.sides[4].faces[3] = self.sides[4].faces[7];
        new_cube.sides[4].faces[5] = self.sides[4].faces[1];
        new_cube.sides[4].faces[6] = self.sides[4].faces[8];
        new_cube.sides[4].faces[7] = self.sides[4].faces[5];
        new_cube.sides[4].faces[8] = self.sides[4].faces[2];
        //rotate Front side
        new_cube.sides[0].faces[0] = self.sides[1].faces[0];
        new_cube.sides[0].faces[3] = self.sides[1].faces[3];
        new_cube.sides[0].faces[6] = self.sides[1].faces[6];
        //rotate Down side
        new_cube.sides[3].faces[0] = self.sides[0].faces[0];
        new_cube.sides[3].faces[3] = self.sides[0].faces[3];
        new_cube.sides[3].faces[6] = self.sides[0].faces[6];
        //rotate Up side
        new_cube.sides[1].faces[0] = self.sides[5].faces[2];
        new_cube.sides[1].faces[3] = self.sides[5].faces[5];
        new_cube.sides[1].faces[6] = self.sides[5].faces[8];
        //rotate Bottom side
        new_cube.sides[5].faces[2] = self.sides[3].faces[0];
        new_cube.sides[5].faces[5] = self.sides[3].faces[3];
        new_cube.sides[5].faces[8] = self.sides[3].faces[6];
        new_cube
    }
    pub fn rotate_left_counter_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push('l');
        new_cube.num_moves += 1;
        //rotate Left side
        new_cube.sides[4].faces[6] = self.sides[4].faces[0];
        new_cube.sides[4].faces[3] = self.sides[4].faces[1];
        new_cube.sides[4].faces[0] = self.sides[4].faces[2];
        new_cube.sides[4].faces[7] = self.sides[4].faces[3];
        new_cube.sides[4].faces[1] = self.sides[4].faces[5];
        new_cube.sides[4].faces[8] = self.sides[4].faces[6];
        new_cube.sides[4].faces[5] = self.sides[4].faces[7];
        new_cube.sides[4].faces[2] = self.sides[4].faces[8];
        //rotate Up side
        new_cube.sides[1].faces[0] = self.sides[0].faces[0];
        new_cube.sides[1].faces[3] = self.sides[0].faces[3];
        new_cube.sides[1].faces[6] = self.sides[0].faces[6];
        //rotate Front side
        new_cube.sides[0].faces[0] = self.sides[3].faces[0];
        new_cube.sides[0].faces[3] = self.sides[3].faces[3];
        new_cube.sides[0].faces[6] = self.sides[3].faces[6];
        //rotate Bottom side
        new_cube.sides[5].faces[2] = self.sides[1].faces[0];
        new_cube.sides[5].faces[5] = self.sides[1].faces[3];
        new_cube.sides[5].faces[8] = self.sides[1].faces[6];
        //rotate down side
        new_cube.sides[3].faces[0] = self.sides[5].faces[2];
        new_cube.sides[3].faces[3] = self.sides[5].faces[5];
        new_cube.sides[3].faces[6] = self.sides[5].faces[8];
        new_cube
    }
    pub fn rotate_back_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push('B');
        new_cube.num_moves += 1;
        //rotate bottom side
        new_cube.sides[5].faces[0] = self.sides[5].faces[6];
        new_cube.sides[5].faces[1] = self.sides[5].faces[3];
        new_cube.sides[5].faces[2] = self.sides[5].faces[0];
        new_cube.sides[5].faces[3] = self.sides[5].faces[7];
        new_cube.sides[5].faces[5] = self.sides[5].faces[1];
        new_cube.sides[5].faces[6] = self.sides[5].faces[8];
        new_cube.sides[5].faces[7] = self.sides[5].faces[5];
        new_cube.sides[5].faces[8] = self.sides[5].faces[2];
        //rotate Up side
        new_cube.sides[1].faces[0] = self.sides[4].faces[0];
        new_cube.sides[1].faces[1] = self.sides[4].faces[3];
        new_cube.sides[1].faces[2] = self.sides[4].faces[6];
        //rotate Right side
        new_cube.sides[2].faces[2] = self.sides[1].faces[0];
        new_cube.sides[2].faces[5] = self.sides[1].faces[1];
        new_cube.sides[2].faces[8] = self.sides[1].faces[2];
        //rotate Down side
        new_cube.sides[3].faces[6] = self.sides[2].faces[2];
        new_cube.sides[3].faces[7] = self.sides[2].faces[5];
        new_cube.sides[3].faces[8] = self.sides[2].faces[8];
        //rotate Left side
        new_cube.sides[4].faces[0] = self.sides[3].faces[6];
        new_cube.sides[4].faces[3] = self.sides[3].faces[7];
        new_cube.sides[4].faces[6] = self.sides[3].faces[8];
        new_cube
    }
    pub fn rotate_back_counter_clockwise(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push('b');
        new_cube.num_moves += 1;
        //rotate bottom side
        new_cube.sides[5].faces[6] = self.sides[5].faces[0];
        new_cube.sides[5].faces[3] = self.sides[5].faces[1];
        new_cube.sides[5].faces[0] = self.sides[5].faces[2];
        new_cube.sides[5].faces[7] = self.sides[5].faces[3];
        new_cube.sides[5].faces[1] = self.sides[5].faces[5];
        new_cube.sides[5].faces[8] = self.sides[5].faces[6];
        new_cube.sides[5].faces[5] = self.sides[5].faces[7];
        new_cube.sides[5].faces[2] = self.sides[5].faces[8];
        //rotate front side
        new_cube.sides[4].faces[0] = self.sides[1].faces[0];
        new_cube.sides[4].faces[3] = self.sides[1].faces[1];
        new_cube.sides[4].faces[6] = self.sides[1].faces[2];
        //rotate up side
        new_cube.sides[1].faces[0] = self.sides[2].faces[2];
        new_cube.sides[1].faces[1] = self.sides[2].faces[5];
        new_cube.sides[1].faces[2] = self.sides[2].faces[8];
        //rotate right side
        new_cube.sides[2].faces[2] = self.sides[3].faces[6];
        new_cube.sides[2].faces[5] = self.sides[3].faces[7];
        new_cube.sides[2].faces[8] = self.sides[3].faces[8];
        //rotate down side
        new_cube.sides[3].faces[6] = self.sides[4].faces[0];
        new_cube.sides[3].faces[7] = self.sides[4].faces[3];
        new_cube.sides[3].faces[8] = self.sides[4].faces[6];
        new_cube
    }
    //DECLARE ghjklo !!!!!
    //g is back
    //h is front
    //j is down
    //k is up
    //l is left
    //o is right
    pub fn rotate_back_twice(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push('g');
        new_cube.num_moves += 1;
        //rotate bottom side
        new_cube.rotate_back_clockwise().rotate_back_clockwise();
        new_cube.previous_moves.pop();
        new_cube.previous_moves.pop();
        new_cube
    }
    pub fn rotate_front_twice(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push('h');
        new_cube.num_moves += 1;
        //rotate bottom side
        new_cube.rotate_front_clockwise().rotate_front_clockwise();
        new_cube.previous_moves.pop();
        new_cube.previous_moves.pop();
        new_cube
    }
    pub fn rotate_down_twice(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push('j');
        new_cube.num_moves += 1;
        //rotate bottom side
        new_cube.rotate_down_clockwise().rotate_down_clockwise();
        new_cube.previous_moves.pop();
        new_cube.previous_moves.pop();
        new_cube
    }
    pub fn rotate_up_twice(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push('k');
        new_cube.num_moves += 1;
        //rotate bottom side
        new_cube.rotate_up_clockwise().rotate_up_clockwise();
        new_cube.previous_moves.pop();
        new_cube.previous_moves.pop();
        new_cube
    }
    pub fn rotate_left_twice(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push('l');
        new_cube.num_moves += 1;
        //rotate bottom side
        new_cube.rotate_left_clockwise().rotate_left_clockwise();
        new_cube.previous_moves.pop();
        new_cube.previous_moves.pop();
        new_cube
    }
    pub fn rotate_right_twice(&self) -> Cube {
        let mut new_cube = self.copy_cube();
        new_cube.previous_moves.push('o');
        new_cube.num_moves += 1;
        //rotate bottom side
        new_cube.rotate_right_clockwise().rotate_right_clockwise();
        new_cube.previous_moves.pop();
        new_cube.previous_moves.pop();
        new_cube
    }
    
    
}
