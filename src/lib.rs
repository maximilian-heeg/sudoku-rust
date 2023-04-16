use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub struct SudokuGrid {
    pub fields: [u8; 81],
    pub guesses: usize,
    pub round: u8,
    pub steps: Vec<String>,
}

impl SudokuGrid {
    pub fn solve(mut self) -> Option<SudokuGrid> {
        let indent = "    ".repeat(self.guesses);
        while let Some(empty_fields) = self.empty_fields() {
            // println!(
            //     "{indent} Round {}. Guesses {} Empty fields: {}",
            //     self.round,
            //     self.guesses,
            //     empty_fields.len()
            // );

            // println!("{indent} Filling out fields that only have one option");

            // Check which fields only have one possible number.
            // Then fill them out
            let field_with_one_possibility: Vec<usize> = empty_fields
                .iter()
                .filter(|(_, v)| v.len() == 1)
                .map(|(&k, _)| k)
                .collect();
            if field_with_one_possibility.len() > 0 {
                // println!(
                //     "{indent} - {:?} field(s) with 1 possible number",
                //     field_with_one_possibility.len()
                // );
                let field = field_with_one_possibility[0];
                self.steps.push(format!(
                    "{indent} -- Row {} col {} must be {:?} ",
                    field / 9 + 1,
                    field % 9 + 1,
                    empty_fields[&field]
                ));

                self.fields[field] = empty_fields[&field][0];

                // Start while loop again
                self.round += 1;
                continue;
            }
           self.steps.push(format!("{indent} - no fields with 1 possible number found"));
            let field_with_least_possiblities = empty_fields
                .iter()
                .min_by(|a, b| a.1.len().cmp(&b.1.len()))
                .map(|(k, _v)| k)
                .unwrap();
            self.steps.push(format!(
                "{indent} -- {} {} has {} possibilities: {:?}. Guessing",
                field_with_least_possiblities / 9 + 1,
                field_with_least_possiblities % 9 + 1,
                empty_fields[field_with_least_possiblities].len(),
                empty_fields[field_with_least_possiblities]
            ));
            for v in &empty_fields[field_with_least_possiblities] {
                self.steps.push(format!("{indent} --- Guessing {} ", v));
                let mut grid = self.clone();
                grid.guesses += 1;
                grid.fields[*field_with_least_possiblities] = *v;
                let res = grid.solve();
                match res {
                    Some(i) => return Some(i),
                    None => continue,
                }
            }
            //
            self.steps.push(format!("{indent} No solution found."));
            return None;
        }

        self.steps.push(format!("Solved."));

        return Some(self);
    }

    fn empty_fields(&self) -> Option<HashMap<usize, Vec<u8>>> {
        // let index = self.fields.iter().position(|&r| r == 0);
        let index: Vec<usize> = self
            .fields
            .iter()
            .enumerate()
            .filter(|(_, &r)| r == 0)
            .map(|(index, _)| index)
            .collect();

        if index.len() > 0 {
            let mut fields_and_possible_values: HashMap<usize, Vec<u8>> = HashMap::new();
            for i in index {
                fields_and_possible_values.insert(i, self.possible_numbers(i / 9, i % 9));
            }
            return Some(fields_and_possible_values);
        } else {
            return None;
        }
    }

    fn is_valid_number(&self, row: usize, col: usize, number: u8) -> bool {
        let is_in_row = &self.is_in_row(row, number);
        let is_in_col = &self.is_in_col(col, number);
        let is_in_square = &self.is_in_square(row, col, number);
        let valid = !is_in_row && !is_in_col && !is_in_square;
        return valid;
    }

    fn is_in_row(&self, row: usize, number: u8) -> bool {
        let res = self.fields[(row * 9)..(row * 9 + 9)].contains(&number);
        return res;
    }

    fn is_in_col(&self, col: usize, number: u8) -> bool {
        for row in 0..9 {
            if self.fields[row * 9 + col] == number {
                return true;
            }
        }
        return false;
    }

    fn possible_numbers(&self, row: usize, col: usize) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        for i in 1..=9 {
            let is_valid = self.is_valid_number(row, col, i);
            if is_valid {
                v.push(i)
            }
        }
        return v;
    }

    fn is_in_square(&self, row: usize, col: usize, number: u8) -> bool {
        let x = row / 3;
        let y = col / 3;
        for i in 0..3 {
            let range = (x * 3 + i) * 9 + y * 3..((x * 3 + i) * 9 + y * 3 + 3);
            if self.fields[range].contains(&number) {
                return true;
            }
        }
        return false;
    }
}

impl fmt::Display for SudokuGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..9 {
            write!(
                f,
                "{} {} {} │ {} {} {} │ {} {} {}\n",
                &self.fields[(row * 9)],
                &self.fields[(row * 9 + 1)],
                &self.fields[(row * 9 + 2)],
                &self.fields[(row * 9 + 3)],
                &self.fields[(row * 9 + 4)],
                &self.fields[(row * 9 + 5)],
                &self.fields[(row * 9 + 6)],
                &self.fields[(row * 9 + 7)],
                &self.fields[(row * 9 + 8)],
            )?;
            if row == 2 || row == 5 {
                write!(f, "──────┼───────┼──────\n")?;
            }
        }
        Ok(())
    }
}

pub fn example_1() -> SudokuGrid {
    let grid = SudokuGrid {
        fields: [
            0, 0, 0, 2, 6, 0, 7, 0, 1, 6, 8, 0, 0, 7, 0, 0, 9, 0, 1, 9, 0, 0, 0, 4, 5, 0, 0, 8, 2,
            0, 1, 0, 0, 0, 4, 0, 0, 0, 4, 6, 0, 2, 9, 0, 0, 0, 5, 0, 0, 0, 3, 0, 2, 8, 0, 0, 9, 3,
            0, 0, 0, 7, 4, 0, 4, 0, 0, 5, 0, 0, 3, 6, 7, 0, 3, 0, 1, 8, 0, 0, 0,
        ],
        guesses: 0,
        round: 0,
        steps: vec![]
        // fields: [1; 81],
    };
    return grid;
}

pub fn example_2() -> SudokuGrid {
    let grid = SudokuGrid {
        fields: [
            0, 0, 0, 0, 0, 0, 6, 8, 0, 0, 0, 0, 0, 7, 3, 0, 0, 9, 3, 0, 9, 0, 0, 0, 0, 4, 5, 4, 9,
            0, 0, 0, 0, 0, 0, 0, 8, 0, 3, 0, 5, 0, 9, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 6, 9, 6, 0, 0,
            0, 0, 3, 0, 8, 7, 0, 0, 6, 8, 0, 0, 0, 0, 0, 2, 8, 0, 0, 0, 0, 0, 0,
        ],
        guesses: 0,
        round: 0,
        steps: vec![]
    };
    return grid;
}

pub fn string_to_sudoku(content: &str) -> Result<[u8; 81], Vec<u8>> {
    let arr: [u8; 81] = content
        .chars()
        .map(|b| b.to_string().parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
        .try_into()?;
    return Ok(arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_number() {
        let grid = example_1();
        assert_eq!(true, grid.is_valid_number(0, 0, 3));
    }

    #[test]
    fn invalid_number() {
        let grid = example_1();
        assert_eq!(false, grid.is_valid_number(0, 0, 1));
    }

    #[test]
    fn valid_number_2() {
        let grid = example_1();
        assert_eq!(true, grid.is_valid_number(4, 2, 7));
    }

    #[test]
    fn invalid_number_2() {
        let grid = example_1();
        assert_eq!(false, grid.is_valid_number(7, 7, 2));
    }

    #[test]
    fn number_in_row() {
        let grid = example_1();
        assert_eq!(true, grid.is_in_row(3, 8));
    }

    #[test]
    fn number_not_in_row() {
        let grid = example_1();
        assert_eq!(false, grid.is_in_row(8, 4));
    }

    #[test]
    fn number_not_in_col() {
        let grid = example_1();
        assert_eq!(false, grid.is_in_row(5, 9));
    }

    #[test]
    fn number_in_col() {
        let grid = example_1();
        assert_eq!(true, grid.is_in_row(2, 9));
    }

    #[test]
    fn number_not_in_square() {
        let grid = example_1();
        assert_eq!(false, grid.is_in_square(8, 5, 4));
    }

    #[test]
    fn number_in_square() {
        let grid = example_1();
        assert_eq!(true, grid.is_in_square(5, 7, 8));
    }

    #[test]
    fn possible_numbers() {
        let grid = example_1();
        assert_eq!(vec![3, 5, 7], grid.possible_numbers(4, 8));
    }

    #[test]
    fn possible_numbers_2() {
        let grid = example_1();
        assert_eq!(vec![1, 6], grid.possible_numbers(6, 1));
    }

    #[test]
    fn solve_example_1() {
        let grid = example_1();
        let res: [u8; 81] = [
            4, 3, 5, 2, 6, 9, 7, 8, 1, 6, 8, 2, 5, 7, 1, 4, 9, 3, 1, 9, 7, 8, 3, 4, 5, 6, 2, 8, 2,
            6, 1, 9, 5, 3, 4, 7, 3, 7, 4, 6, 8, 2, 9, 1, 5, 9, 5, 1, 7, 4, 3, 6, 2, 8, 5, 1, 9, 3,
            2, 6, 8, 7, 4, 2, 4, 8, 9, 5, 7, 1, 3, 6, 7, 6, 3, 4, 1, 8, 2, 5, 9,
        ];

        let solution = grid.solve().unwrap().fields;
        assert_eq!(solution, res);
    }

    #[test]
    fn solve_example_2() {
        let grid = example_2();
        let res: [u8; 81] = [
            1, 7, 2, 5, 4, 9, 6, 8, 3, 6, 4, 5, 8, 7, 3, 2, 1, 9, 3, 8, 9, 2, 6, 1, 7, 4, 5, 4, 9,
            6, 3, 2, 7, 8, 5, 1, 8, 1, 3, 4, 5, 6, 9, 7, 2, 2, 5, 7, 1, 9, 8, 4, 3, 6, 9, 6, 4, 7,
            1, 5, 3, 2, 8, 7, 3, 1, 6, 8, 2, 5, 9, 4, 5, 2, 8, 9, 3, 4, 1, 6, 7,
        ];

        let solution = grid.solve().unwrap().fields;
        assert_eq!(solution, res);
    }

    #[test]
    fn solve_20_puzzles() {
        let puzzles = "\
004300209005009001070060043006002087190007400050083000600000105003508690042910300,864371259325849761971265843436192587198657432257483916689734125713528694542916378
040100050107003960520008000000000017000906800803050620090060543600080700250097100,346179258187523964529648371965832417472916835813754629798261543631485792254397186
600120384008459072000006005000264030070080006940003000310000050089700000502000190,695127384138459672724836915851264739273981546946573821317692458489715263562348197
497200000100400005000016098620300040300900000001072600002005870000600004530097061,497258316186439725253716498629381547375964182841572639962145873718623954534897261
005910308009403060027500100030000201000820007006007004000080000640150700890000420,465912378189473562327568149738645291954821637216397854573284916642159783891736425
100005007380900000600000480820001075040760020069002001005039004000020100000046352,194685237382974516657213489823491675541768923769352841215839764436527198978146352
009065430007000800600108020003090002501403960804000100030509007056080000070240090,289765431317924856645138729763891542521473968894652173432519687956387214178246395
000000657702400100350006000500020009210300500047109008008760090900502030030018206,894231657762495183351876942583624719219387564647159328128763495976542831435918276
503070190000006750047190600400038000950200300000010072000804001300001860086720005,563472198219386754847195623472638519951247386638519472795864231324951867186723945
060720908084003001700100065900008000071060000002010034000200706030049800215000090,163725948584693271729184365946358127371462589852917634498231756637549812215876493
004083002051004300000096710120800006040000500830607900060309040007000205090050803,974183652651274389283596714129835476746912538835647921568329147317468295492751863
000060280709001000860320074900040510007190340003006002002970000300800905500000021,431567289729481653865329174986243517257198346143756892612975438374812965598634721
004300000890200670700900050500008140070032060600001308001750900005040012980006005,254367891893215674716984253532698147178432569649571328421753986365849712987126435
008070100120090054000003020604010089530780010009062300080040607007506000400800002,958274163123698754746153928674315289532789416819462375285941637397526841461837592
065370002000001370000640800097004028080090001100020940040006700070018050230900060,865379412924581376713642895397164528482795631156823947541236789679418253238957164
005710329000362800004000000100000980083900250006003100300106000409800007070029500,865714329917362845234598761142657983783941256596283174358176492429835617671429538
200005300000073850000108904070009001651000040040200080300050000580760100410030096,268495317194673852735128964872549631651387249943216785326951478589764123417832596
040800500080760092001005470056309000009001004320500010000200700700090030005008026,947812563583764192261935478156349287879621354324587619698253741712496835435178926
050083017000100400304005608000030009090824500006000070009000050007290086103607204,652483917978162435314975628825736149791824563436519872269348751547291386183657294
700084005300701020080260401624109038803600010000000002900000000001005790035400006,712984365346751829589263471624179538853642917197538642978316254461825793235497186";
        for line in puzzles.lines() {
            let parts = line.split(",").collect::<Vec<&str>>();
            let arr = string_to_sudoku(parts[0]).unwrap();
            let grid = SudokuGrid {
                fields: arr,
                guesses: 0,
                round: 0,
                steps: vec![]
            };
            let solved = grid.solve().unwrap();

            let solution = string_to_sudoku(parts[1]).unwrap();
            assert_eq!(solved.fields, solution);
        }
    }
}
