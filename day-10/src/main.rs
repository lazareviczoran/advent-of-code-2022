use std::fs::read_to_string;

fn main() {
    let ops = parse("input.txt");
    let mut cpu = Cpu::new(&ops);
    println!(
        "part1 solution: {}",
        cpu.sum_signal_strengths(&[20, 60, 100, 140, 180, 220])
    );

    cpu = Cpu::new(&ops);
    cpu.run_til_end();
    println!("part2 solution: \n{}", cpu.get_crt_output());
}

#[derive(Debug)]
struct Cpu<'a> {
    cycle: usize,
    register: isize,
    current_op_idx: usize,
    current_op_remaining: usize,
    signal_strength: isize,
    ops: &'a [Op],
    crt_output: Vec<Vec<char>>,
}
impl<'a> Cpu<'a> {
    fn new(ops: &'a [Op]) -> Self {
        Self {
            cycle: 0,
            current_op_idx: 0,
            current_op_remaining: ops.get(0).unwrap().get_duration(),
            signal_strength: 0,
            register: 1,
            ops,
            crt_output: vec![vec!['.'; 40]; 6],
        }
    }

    fn next_tick(&mut self) {
        self.draw_pixel();
        self.cycle += 1;
        self.current_op_remaining -= 1;
        self.signal_strength = self.compute_signal_strength();
        if self.current_op_remaining == 0 {
            if let Some(Op::Addx(value)) = self.ops.get(self.current_op_idx) {
                self.register += value;
            }
            self.current_op_idx += 1;
            self.current_op_remaining = self
                .ops
                .get(self.current_op_idx)
                .unwrap_or(&Op::Noop)
                .get_duration();
        }
    }

    fn run_til_end(&mut self) {
        while self.current_op_idx < self.ops.len() {
            self.next_tick();
        }
    }

    fn draw_pixel(&mut self) {
        let w = self.cycle % 40;
        let h = self.cycle / 40;
        if w as isize >= self.register - 1 && w as isize <= self.register + 1 {
            self.crt_output[h][w] = '#';
        }
    }

    fn get_crt_output(&self) -> String {
        let rows = self
            .crt_output
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>();
        rows.join("\n")
    }

    fn compute_signal_strength(&self) -> isize {
        self.cycle as isize * self.register
    }

    fn sum_signal_strengths(&mut self, cycle_samples: &[usize]) -> isize {
        (0..*cycle_samples.last().unwrap())
            .filter_map(|_| {
                self.next_tick();
                if cycle_samples.contains(&self.cycle) {
                    Some(self.signal_strength)
                } else {
                    None
                }
            })
            .sum()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Op {
    Noop,
    Addx(isize),
}
impl Op {
    fn get_duration(&self) -> usize {
        match self {
            Op::Addx(_) => 2,
            Op::Noop => 1,
        }
    }
}

fn parse(filename: &str) -> Vec<Op> {
    read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .filter_map(|line| {
            if line == "noop" {
                Some(Op::Noop)
            } else {
                Some(Op::Addx(line.strip_prefix("addx ")?.parse().ok()?))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{parse, Cpu};

    #[test]
    fn part1_test1() {
        let ops = parse("test-input.txt");
        let mut cpu = Cpu::new(&ops);

        cpu.next_tick();
        assert_eq!(cpu.register, 1);
        cpu.next_tick();
        assert_eq!(cpu.register, 1);
        cpu.next_tick();
        assert_eq!(cpu.register, 4);
        cpu.next_tick();
        assert_eq!(cpu.register, 4);
        cpu.next_tick();
        assert_eq!(cpu.register, -1);
    }

    #[test]
    fn part1_test2() {
        let ops = parse("test-input2.txt");
        let mut cpu = Cpu::new(&ops);

        fn run_n_cycles(cpu: &mut Cpu, n: usize) {
            (0..n).for_each(|_| cpu.next_tick());
        }

        run_n_cycles(&mut cpu, 20);
        assert_eq!(cpu.signal_strength, 420);
        cpu = Cpu::new(&ops);
        run_n_cycles(&mut cpu, 60);
        assert_eq!(cpu.signal_strength, 1140);
        cpu = Cpu::new(&ops);
        run_n_cycles(&mut cpu, 100);
        assert_eq!(cpu.signal_strength, 1800);
        cpu = Cpu::new(&ops);
        run_n_cycles(&mut cpu, 140);
        assert_eq!(cpu.signal_strength, 2940);
        cpu = Cpu::new(&ops);
        run_n_cycles(&mut cpu, 180);
        assert_eq!(cpu.signal_strength, 2880);
        cpu = Cpu::new(&ops);
        run_n_cycles(&mut cpu, 220);
        assert_eq!(cpu.signal_strength, 3960);
        cpu = Cpu::new(&ops);

        assert_eq!(
            cpu.sum_signal_strengths(&[20, 60, 100, 140, 180, 220]),
            13140
        );
    }

    #[test]
    fn part2_test() {
        let ops = parse("test-input2.txt");
        let mut cpu = Cpu::new(&ops);
        cpu.run_til_end();
        assert_eq!(
            cpu.get_crt_output(),
            r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#
            .trim()
        )
    }
}
