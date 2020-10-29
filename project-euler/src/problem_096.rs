use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug)]
pub struct Cell {
    number: u32,
    candidate: HashSet<u32>,
    snapshots: Vec<(u32, HashSet<u32>)>,
}

#[derive(Debug)]
pub struct Row {
    cells: Vec<Rc<RefCell<Cell>>>,
    comitted: RefCell<HashSet<u32>>,
    snapshots: RefCell<Vec<HashSet<u32>>>,
}

#[derive(Debug)]
pub struct Column {
    cells: Vec<Rc<RefCell<Cell>>>,
    comitted: RefCell<HashSet<u32>>,
    snapshots: RefCell<Vec<HashSet<u32>>>,
}

#[derive(Debug)]
pub struct Box {
    cells: Vec<Rc<RefCell<Cell>>>,
    comitted: RefCell<HashSet<u32>>,
    snapshots: RefCell<Vec<HashSet<u32>>>,
}

#[derive(Debug)]
pub struct Sudoku {
    rows: Vec<Row>,
    columns: Vec<Column>,
    boxes: Vec<Box>,
}

impl Cell {
    pub fn new(number: u32) -> Self {
        Self {
            number,
            candidate: if number == 0 {
                (1..=9).collect::<HashSet<_>>()
            } else {
                HashSet::new()
            },
            snapshots: vec![],
        }
    }

    fn snapshot(&mut self) {
        self.snapshots.push((self.number, self.candidate.clone()));
    }

    fn restore(&mut self) {
        let (n, c) = self.snapshots.pop().unwrap();
        self.number = n;
        self.candidate = c;
    }
}

impl Sudoku {
    pub fn new(question: &Vec<Vec<u32>>) -> Self {
        let rows = question
            .iter()
            .map(|row| Row {
                cells: row
                    .iter()
                    .map(|&n| Rc::new(RefCell::new(Cell::new(n))))
                    .collect::<Vec<_>>(),
                comitted: RefCell::new(HashSet::new()),
                snapshots: RefCell::new(vec![]),
            })
            .collect_vec();

        let mut columns = rows[0]
            .cells
            .iter()
            .map(|c| Column {
                cells: vec![Rc::clone(c)],
                comitted: RefCell::new(HashSet::new()),
                snapshots: RefCell::new(vec![]),
            })
            .collect_vec();

        for i in 1..rows.len() {
            for j in 0..rows[i].cells.len() {
                columns[j].cells.push(Rc::clone(&rows[i].cells[j]));
            }
        }

        let boxes = rows
            .chunks(3)
            .flat_map(|row| {
                (0..3)
                    .map(|i| i * 3)
                    .map(|i| Box {
                        cells: row[0].cells[i..i + 3]
                            .iter()
                            .chain(row[1].cells[i..i + 3].iter())
                            .chain(row[2].cells[i..i + 3].iter())
                            .map(|c| Rc::clone(c))
                            .collect(),
                        comitted: RefCell::new(HashSet::new()),
                        snapshots: RefCell::new(vec![]),
                    })
                    .collect_vec()
            })
            .collect_vec();

        let sudoku = Sudoku {
            rows,
            columns,
            boxes,
        };
        sudoku.update_comitted();

        sudoku
    }

    fn update_comitted(&self) {
        for row in &self.rows {
            row.comitted.borrow_mut().extend(
                row.cells
                    .iter()
                    .filter(|c| c.borrow().number != 0)
                    .map(|c| c.borrow().number),
            )
        }

        for col in &self.columns {
            col.comitted.borrow_mut().extend(
                col.cells
                    .iter()
                    .filter(|c| c.borrow().number != 0)
                    .map(|c| c.borrow().number),
            )
        }

        for box_ in &self.boxes {
            box_.comitted.borrow_mut().extend(
                box_.cells
                    .iter()
                    .filter(|c| c.borrow().number != 0)
                    .map(|c| c.borrow().number),
            )
        }
    }

    pub fn update_candidate(&self) -> bool {
        let mut updated = false;

        for i in 0..self.rows.len() {
            for j in 0..self.rows[i].cells.len() {
                let n = self.rows[i].cells[j].borrow().number;
                if n == 0 {
                    continue;
                }

                for cell in &self.rows[i].cells {
                    updated |= cell.borrow_mut().candidate.remove(&n);
                }
                for cell in &self.columns[j].cells {
                    updated |= cell.borrow_mut().candidate.remove(&n);
                }
                for cell in &self.get_box(i, j).cells {
                    updated |= cell.borrow_mut().candidate.remove(&n);
                }
            }
        }

        updated |= self.update_candidate_intersect_row();
        updated |= self.update_candidate_intersect_column();
        updated |= self.update_candidate_naked_row();
        updated |= self.update_candidate_naked_column();
        updated |= self.update_candidate_pointing_pair();
        updated |= self.update_candidate_hidden_pair();

        updated
    }

    fn update_candidate_intersect_row(&self) -> bool {
        let cand_set: HashSet<_> = (1..=9).collect();
        let mut updated = false;

        for i in 0..self.rows.len() {
            for check_number in (&cand_set - &self.rows[i].comitted.borrow()).iter() {
                let bit_flag = self.rows[i]
                    .cells
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| {
                        let c = c.borrow();
                        c.number == 0 && c.candidate.contains(check_number)
                    })
                    .map(|(i, _)| 1 << i)
                    .sum::<u32>();

                if bit_flag == 0 {
                    continue;
                }

                for &(mask, index) in &[(0b111111000, 0), (0b111000111, 3), (0b000111111, 6)] {
                    if bit_flag & mask == 0 {
                        for (_, c) in self
                            .get_box(i, index)
                            .cells
                            .iter()
                            .enumerate()
                            .filter(|(j, _)| i % 3 != j / 3)
                        {
                            updated |= c.borrow_mut().candidate.remove(&check_number);
                        }
                    }
                }
            }
        }

        updated
    }

    fn update_candidate_intersect_column(&self) -> bool {
        let cand_set: HashSet<_> = (1..=9).collect();
        let mut updated = false;

        for j in 0..self.columns.len() {
            for check_number in (&cand_set - &self.columns[j].comitted.borrow()).iter() {
                let bit_flag = self.columns[j]
                    .cells
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| {
                        let c = c.borrow();
                        c.number == 0 && c.candidate.contains(check_number)
                    })
                    .map(|(i, _)| 1 << i)
                    .sum::<u32>();

                if bit_flag == 0 {
                    continue;
                }

                for &(mask, index) in &[(0b111111000, 0), (0b111000111, 3), (0b000111111, 6)] {
                    if bit_flag & mask == 0 {
                        for (_, c) in self
                            .get_box(index, j)
                            .cells
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| i % 3 != j % 3)
                        {
                            updated |= c.borrow_mut().candidate.remove(&check_number);
                        }
                    }
                }
            }
        }

        updated
    }

    fn update_candidate_naked_row(&self) -> bool {
        let mut updated = false;

        for i in 0..self.rows.len() {
            for j in (0..3).map(|j| j * 3) {
                for (a, b) in self.rows[i].cells[j..j + 3].iter().tuple_combinations() {
                    let borrow_a = a.borrow();
                    let borrow_b = b.borrow();

                    if borrow_a.candidate != borrow_b.candidate || borrow_a.candidate.len() != 2 {
                        continue;
                    }

                    for cell in self.rows[i]
                        .cells
                        .iter()
                        .chain(self.get_box(i, j).cells.iter())
                    {
                        if cell.as_ptr() == a.as_ptr() || cell.as_ptr() == b.as_ptr() {
                            continue;
                        }

                        let mut c = cell.borrow_mut();
                        if c.number != 0 {
                            continue;
                        }

                        for n in &borrow_a.candidate {
                            updated |= c.candidate.remove(n);
                        }
                    }
                }
            }

            for (a, b) in self.rows[i]
                .cells
                .iter()
                .filter(|c| c.borrow().number == 0)
                .tuple_combinations()
            {
                let borrow_a = a.borrow();
                let borrow_b = b.borrow();

                if borrow_a.candidate != borrow_b.candidate || borrow_a.candidate.len() != 2 {
                    continue;
                }

                for cell in self.rows[i].cells.iter() {
                    if cell.as_ptr() == a.as_ptr() || cell.as_ptr() == b.as_ptr() {
                        continue;
                    }

                    let mut c = cell.borrow_mut();
                    if c.number != 0 {
                        continue;
                    }

                    for n in &borrow_a.candidate {
                        updated |= c.candidate.remove(n);
                    }
                }
            }
        }

        updated
    }

    fn update_candidate_naked_column(&self) -> bool {
        let mut updated = false;

        for j in 0..self.columns.len() {
            for i in (0..3).map(|i| i * 3) {
                for (a, b) in self.columns[j].cells[i..i + 3].iter().tuple_combinations() {
                    let borrow_a = a.borrow();
                    let borrow_b = b.borrow();

                    if borrow_a.candidate != borrow_b.candidate || borrow_a.candidate.len() != 2 {
                        continue;
                    }

                    for cell in self.columns[j]
                        .cells
                        .iter()
                        .chain(self.get_box(i, j).cells.iter())
                    {
                        if cell.as_ptr() == a.as_ptr() || cell.as_ptr() == b.as_ptr() {
                            continue;
                        }

                        let mut c = cell.borrow_mut();
                        if c.number != 0 {
                            continue;
                        }

                        for n in &borrow_a.candidate {
                            updated |= c.candidate.remove(n);
                        }
                    }
                }
            }

            for (a, b) in self.columns[j]
                .cells
                .iter()
                .filter(|c| c.borrow().number == 0)
                .tuple_combinations()
            {
                let borrow_a = a.borrow();
                let borrow_b = b.borrow();

                if borrow_a.candidate != borrow_b.candidate || borrow_a.candidate.len() != 2 {
                    continue;
                }

                for cell in self.columns[j].cells.iter() {
                    if cell.as_ptr() == a.as_ptr() || cell.as_ptr() == b.as_ptr() {
                        continue;
                    }

                    let mut c = cell.borrow_mut();
                    if c.number != 0 {
                        continue;
                    }

                    for n in &borrow_a.candidate {
                        updated |= c.candidate.remove(n);
                    }
                }
            }
        }

        updated
    }

    fn update_candidate_pointing_pair(&self) -> bool {
        let cand_set: HashSet<_> = (1..=9).collect();
        let mut updated = false;

        for box_i in 0..self.boxes.len() {
            for check_number in (&cand_set - &self.boxes[box_i].comitted.borrow()).iter() {
                let bit_flag = self.boxes[box_i]
                    .cells
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| {
                        let c = c.borrow();
                        c.number == 0 && c.candidate.contains(check_number)
                    })
                    .map(|(i, _)| 1 << i)
                    .sum::<u32>();

                if bit_flag == 0 {
                    continue;
                }

                for &(mask, index) in &[(0b111111000, 0), (0b111000111, 1), (0b000111111, 2)] {
                    if bit_flag & mask == 0 {
                        for (_, c) in self.rows[(box_i / 3) * 3 + index]
                            .cells
                            .iter()
                            .enumerate()
                            .filter(|(j, _)| box_i % 3 != j / 3)
                        {
                            updated |= c.borrow_mut().candidate.remove(&check_number);
                        }
                    }
                }

                for &(mask, index) in &[(0b110110110, 0), (0b101101101, 1), (0b011011011, 2)] {
                    if bit_flag & mask == 0 {
                        for (_, c) in self.columns[(box_i % 3) * 3 + index]
                            .cells
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| box_i / 3 != i / 3)
                        {
                            updated |= c.borrow_mut().candidate.remove(&check_number);
                        }
                    }
                }
            }
        }

        updated
    }

    fn update_candidate_hidden_pair(&self) -> bool {
        let cand_set: HashSet<_> = (1..=9).collect();
        let mut updated = false;

        for i in 0..self.rows.len() {
            for pair in (&cand_set - &self.rows[i].comitted.borrow())
                .iter()
                .combinations(2)
                .map(|combi| combi.into_iter().copied().collect::<HashSet<_>>())
            {
                let v = self.rows[i]
                    .cells
                    .iter()
                    .filter(|c| {
                        let c = c.borrow();
                        c.number == 0 && c.candidate.intersection(&pair).count() != 0
                    })
                    .collect_vec();

                if v.len() == 2 {
                    for c in v {
                        let mut c = c.borrow_mut();
                        for n in &c.candidate - &pair {
                            updated |= c.candidate.remove(&n);
                        }
                    }
                }
            }
        }

        for j in 0..self.columns.len() {
            for pair in (&cand_set - &self.columns[j].comitted.borrow())
                .iter()
                .combinations(2)
                .map(|combi| combi.into_iter().copied().collect::<HashSet<_>>())
            {
                let v = self.columns[j]
                    .cells
                    .iter()
                    .filter(|c| {
                        let c = c.borrow();
                        c.number == 0 && c.candidate.intersection(&pair).count() != 0
                    })
                    .collect_vec();

                if v.len() == 2 {
                    for c in v {
                        let mut c = c.borrow_mut();
                        for n in &c.candidate - &pair {
                            updated |= c.candidate.remove(&n);
                        }
                    }
                }
            }
        }

        for i in 0..self.boxes.len() {
            for pair in (&cand_set - &self.boxes[i].comitted.borrow())
                .iter()
                .combinations(2)
                .map(|combi| combi.into_iter().copied().collect::<HashSet<_>>())
            {
                let v = self.boxes[i]
                    .cells
                    .iter()
                    .filter(|c| {
                        let c = c.borrow();
                        c.number == 0 && c.candidate.intersection(&pair).count() != 0
                    })
                    .collect_vec();

                if v.len() == 2 {
                    for c in v {
                        let mut c = c.borrow_mut();
                        for n in &c.candidate - &pair {
                            updated |= c.candidate.remove(&n);
                        }
                    }
                }
            }
        }

        updated
    }

    pub fn discovery_and_commit(&self) {
        for row in &self.rows {
            for cell in &row.cells {
                let n = &mut cell.borrow_mut();
                if n.number != 0 {
                    continue;
                }

                if n.candidate.len() == 1 {
                    n.number = *n.candidate.iter().next().unwrap();
                    n.candidate.clear();
                }
            }
        }

        self.update_comitted();
    }

    pub fn discovery_and_commit2(&self) {
        let cand_set: HashSet<_> = (1..=9).collect();

        for box_ in &self.boxes {
            for check_number in cand_set.difference(&box_.comitted.borrow()) {
                let cs = box_
                    .cells
                    .iter()
                    .filter(|c| {
                        let c = c.borrow();
                        c.number == 0 && c.candidate.contains(check_number)
                    })
                    .collect_vec();

                if cs.len() == 1 {
                    let mut n = cs[0].borrow_mut();
                    n.number = *check_number;
                    n.candidate.clear();
                }
            }
        }

        self.update_comitted();
    }

    pub fn discovery_and_commit3(&self, number: usize) {
        let c = self
            .rows
            .iter()
            .flat_map(|row| &row.cells)
            .filter(|c| {
                let c = c.borrow();
                c.number == 0 && c.candidate.len() == 2
            })
            .skip(number / 2)
            .next()
            .unwrap();

        {
            let mut c = c.borrow_mut();
            let n = c.candidate.iter().sorted().skip(number % 2).next().unwrap();
            c.number = *n;
            c.candidate.clear();
        } // release borrow mut
        self.update_comitted();
    }

    pub fn get_left_up_3digit(&self) -> u32 {
        self.rows[0].cells[0..3]
            .iter()
            .map(|c| c.borrow().number)
            .join("")
            .parse()
            .unwrap()
    }

    pub fn snapshot(&self) {
        for i in 0..self.rows.len() {
            for j in 0..self.rows[i].cells.len() {
                self.rows[i].cells[j].borrow_mut().snapshot();
            }
            self.rows[i]
                .snapshots
                .borrow_mut()
                .push(self.rows[i].comitted.borrow().clone());
        }

        for column in &self.columns {
            column
                .snapshots
                .borrow_mut()
                .push(column.comitted.borrow().clone());
        }

        for box_ in &self.boxes {
            box_.snapshots
                .borrow_mut()
                .push(box_.comitted.borrow().clone());
        }
    }

    pub fn restore(&self) {
        for i in 0..self.rows.len() {
            for j in 0..self.rows[i].cells.len() {
                self.rows[i].cells[j].borrow_mut().restore();
            }

            *self.rows[i].comitted.borrow_mut() =
                self.rows[i].snapshots.borrow_mut().pop().unwrap();
        }

        for column in &self.columns {
            *column.comitted.borrow_mut() = column.snapshots.borrow_mut().pop().unwrap();
        }

        for box_ in &self.boxes {
            *box_.comitted.borrow_mut() = box_.snapshots.borrow_mut().pop().unwrap();
        }
    }

    pub fn solve(&self) {
        let mut ss = 0;

        loop {
            while self.update_candidate() {
                self.discovery_and_commit();
                self.discovery_and_commit2();
            }

            if self.is_valid() {
                return;
            } else if ss == 2 {
                panic!("unsolved!");
            } else {
                if ss != 0 {
                    self.restore();
                }
                self.snapshot();

                self.discovery_and_commit3(ss);
                ss += 1;
            }
        }
    }

    fn get_box(&self, i: usize, j: usize) -> &Box {
        &self.boxes[(i / 3) * 3 + (j / 3)]
    }

    fn is_valid(&self) -> bool {
        self.rows.iter().all(|row| {
            row.cells
                .iter()
                .map(|c| c.borrow().number)
                .collect::<HashSet<_>>()
                .len()
                == 9
        })
    }

    // pub fn print(&self) {
    //     for row in &self.rows {
    //         println!(
    //             "{:?}",
    //             row.cells
    //                 .iter()
    //                 .map(|c| c.borrow().number)
    //                 .collect::<Vec<_>>()
    //         );
    //     }
    // }
}

// #[test]
// fn test() {
//     let question = GRID
//         .split("\n")
//         .map(|line| {
//             line.chars()
//                 .filter_map(|c| c.to_digit(10))
//                 .collect::<Vec<_>>()
//         })
//         .collect::<Vec<_>>();

//     let sudoku = Sudoku::new(&question);
//     sudoku.solve();

//     sudoku.print();

//     // println!("---");
//     // for row in sudoku.rows {
//     //     println!("{:?}", row);
//     // }
//     // println!("---");
//     // for column in sudoku.columns {
//     //     println!("{:?}", column);
//     // }
//     // println!("---");
//     // for box_ in sudoku.boxes {
//     //     println!("{:?}", box_);
//     // }
// }

// const GRID: &str = "\
// 043080250
// 600000000
// 000001094
// 900004070
// 000608000
// 010200003
// 820500000
// 000000005
// 034090710";
