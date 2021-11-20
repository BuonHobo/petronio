#[derive(Debug)]
struct Transition {
    input: Vec<(usize, u8)>,
    output: Vec<(usize, u8)>,
}

impl Transition {
    fn is_active(&self, vect: &[i32]) -> bool {
        self.input.iter().all(|(i, u)| {
            vect[*i] >= *u as i32
        })
    }
    fn new(input: &[(i32, u8)], output: &[(i32, u8)]) -> Self {
        Transition {
            input: input.to_vec().iter().map(|(i, u)| (*i as usize, *u as u8)).collect(),
            output: output.to_vec().iter().map(|(i, u)| (*i as usize, *u as u8)).collect(),
        }
    }
    fn enable(&self, vect: &mut [i32]) -> () {
        self.input.iter().for_each(|(i, u)| vect[*i] -= *u as i32);
        self.output.iter().for_each(|(i, u)| vect[*i] += *u as i32);
    }
}

fn main() {
    let mut places: Vec<i32> = vec![0, 1, 0, 0, 1];
    let tra1 = Transition::new(&[(0, 1), (1, 1)], &[(2, 1),(1,1)]);
    let tra2 = Transition::new(&[(2, 1), (3, 1)], &[(4, 1)]);
    let tra3 = Transition::new(&[(4, 1)], &[(0, 1), (3, 1)]);
    let transizioni = vec![tra1, tra2, tra3];
    for (i, j) in transizioni.iter().enumerate() {
        println!("T{}: {{ input: {:?}, output: {:?} }}", i + 1, j.input, j.output);
    }
    println!("In -> {:?}", &places);
    let mut go_on;
    for _ in 0..5 {
        go_on = false;
        for (i, tra) in transizioni.iter().enumerate() {
            if tra.is_active(&places) {
                tra.enable(&mut places);
                println!("T{} -> {:?}", i + 1, &places);
                go_on = true
            }
        }
        if !go_on {
            println!("La rete è morta :((");
            break;
        }
    };
}