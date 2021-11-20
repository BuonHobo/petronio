use rand::seq::SliceRandom;

#[derive(Debug)]
/*Una transizione contiene un vettore di archi entranti e uno di archi uscenti.
Ogni arco è composto da (usize,u8) dove usize è l'indice del posto associato
nel vettore della marcatura, e u8 è il peso dell'arco.*/
struct Transition {
    input: Vec<(usize, u8)>,
    output: Vec<(usize, u8)>,
}

impl Transition {
    //Verifica che i gettoni in ognuno dei posti in input siano maggiori o uguali al peso.
    fn is_active(&self, vect: &[i32]) -> bool {
        self.input.iter().all(|(i, u)| {
            vect[*i] >= *u as i32
        })
    }
    //Crea una transizione partendo da una slice di archi in input e una in output.
    fn new(input: &[(i32, u8)], output: &[(i32, u8)]) -> Self {
        Transition {
            input: input.to_vec().iter().map(|(i, u)| (*i as usize, *u as u8)).collect(),
            output: output.to_vec().iter().map(|(i, u)| (*i as usize, *u as u8)).collect(),
        }
    }
    //Fa scattare la transizione consumando i gettoni dai posti in input e generandoli in output.
    fn enable(&self, vect: &mut [i32]) -> () {
        self.input.iter().for_each(|(i, u)| vect[*i] -= *u as i32);
        self.output.iter().for_each(|(i, u)| vect[*i] += *u as i32);
    }
}

fn main() {
    //Creo la marcatura iniziale e le transizioni
    let mut places: Vec<i32> = vec![0, 0, 1, 0, 1];
    let transizioni = vec![Transition::new(&[(0, 1)], &[(1, 1), (4, 1)]),
                               Transition::new(&[(3, 1)], &[(4, 1)]),
                               Transition::new(&[(1, 1)], &[(2, 1)]),
                               Transition::new(&[(4, 1)], &[(3, 1)]),
                               Transition::new(&[(2, 1)], &[(0, 1)])];
    let len=transizioni.len();
    let mut ordine: Vec<usize> = (0..len).collect();

    //Stampo le transizioni e la marcatura iniziale
    for (i, j) in transizioni.iter().enumerate() {
        println!("T{}: {{ input: {:?}, output: {:?} }}", i + 1, j.input, j.output);
    }
    println!("In -> {:?}", &places);

    let mut rng = rand::thread_rng();
    let mut tra: &Transition;

    //Simulo la rete
    let mut go_on;
    for _ in 0..10 {
        go_on = false;
        ordine.shuffle(&mut rng);
        for i in ordine.iter() {
            tra = &transizioni[*i];
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