use rand::seq::IteratorRandom;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
/*Una transizione contiene un vettore di archi entranti e uno di archi uscenti.
Ogni arco è composto da (usize,u8) dove usize è l'indice del posto associato
nel vettore della marcatura, e u8 è il peso dell'arco (negativo se è inibitore).*/
struct Transition {
    id: u8,
    input: Vec<(usize, i8)>,
    output: Vec<(usize, i8)>,
    duration: Duration,
    processing: bool,
}

impl Transition {
    //Verifica che i gettoni in ognuno dei posti in input siano maggiori o uguali al peso.
    fn is_active(&self, vect: &[i32]) -> bool {
        if !self.processing {
            //La transazione deve essere libera, non occupata
            self.input.iter().all(|(i, u)| {
                if u.is_negative() {
                    //u negativo vuol dire che è inibitore
                    !(vect[*i] >= -*u as i32)
                } else {
                    vect[*i] >= *u as i32
                }
            })
        } else {
            false
        }
    }
    //Crea una transizione partendo da una slice di archi in input e una in output.
    fn new(id: u8, input: &[(i32, i8)], output: &[(i32, i8)], duration: u64) -> Self {
        let closure = |slice: &[(i32, i8)]| {
            slice
                .iter()
                .map(|(index, weight)| (*index as usize, *weight as i8))
                .collect()
        };
        Transition {
            id,
            input: closure(input),
            output: closure(output),
            processing: false,
            duration: Duration::new(duration, 0),
        }
    }
    //Fa scattare la transizione consumando i gettoni dai posti in input e generandoli in output.
    fn enable(&self, vect: &mut [i32]) -> () {
        self.input.iter().for_each(|(i, u)| {
            if !u.is_negative() {
                vect[*i] -= *u as i32
            }
        });
        self.output.iter().for_each(|(i, u)| {
            if !u.is_negative() {
                vect[*i] += *u as i32
            }
        });
    }
}

fn lockunwrap<T>(input: &Arc<Mutex<T>>) -> MutexGuard<T> {
    input.lock().unwrap()
}

fn main() {
    //Dati della rete
    let places: Vec<i32> = vec![2, 0, 0, 0]; //marcatura iniziale
    let trans: Vec<Transition> = vec![
        //transizioni
        //(ID, lista con (posto entrante, peso), lista con (posto uscente, peso), durata).
        Transition::new(1, &[(0, 1), (2, -1)], &[(1, 1)], 0),
        Transition::new(2, &[(0, 1)], &[(2, 1)], 0),
        Transition::new(3, &[(1, 1)], &[(0, 1)], 6),
        Transition::new(4, &[(2, 1)], &[(3, 1)], 3),
        Transition::new(5, &[(3, 1)], &[(0, 1)], 4),
    ];
    let n = 20;

    //Preparo i dati al multithreading
    let shared_trans: Vec<Arc<Mutex<Transition>>> = trans
        .iter()
        .map(|tran| Arc::new(Mutex::new(tran.clone())))
        .collect();

    let shared_places: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(places));

    //Stampo le transizioni e la marcatura iniziale
    // trans.iter().for_each(|j|
    //     println!("T{}: {:?} -> {:?}", j.id, j.input, j.output));
    println!("In -> {:?}", lockunwrap(&shared_places));

    //preparo il seed per la scelta randomica, il canale per la comunicazione tra thread e roba per dare info
    let mut rng = rand::thread_rng();
    let (tx, rx) = channel();
    let start = std::time::SystemTime::now();
    let fn_printer = |id: u8, places: MutexGuard<Vec<i32>>, secs: u64| {
        println!("T{} -> {:?}. Tempo: {:?}.", id, places, secs);
    };

    //Faccio al massimo n scatti
    let mut i = 0;
    while i <= n {
        //trovo le transizioni attive
        let active_trans: Vec<Arc<Mutex<Transition>>> = shared_trans
            .iter()
            .filter(|t| lockunwrap(t).is_active(lockunwrap(&shared_places).as_slice()))
            .map(|t| t.clone())
            .collect();

        let attive = active_trans
            .iter()
            .map(|t| lockunwrap(t).id)
            .collect::<Vec<u8>>();
        //se non ce ne sono aspetto, o forse è morta
        if attive.is_empty() {
            if rx.recv().is_err() {
                println!("morta su {:?}", lockunwrap(&shared_places));
                break;
            };
            continue;
        }

        let rand_trans = active_trans.iter().choose(&mut rng).unwrap();
        let mut rand_trans_lock = lockunwrap(rand_trans);

        //println!("Transizioni attive: {:?}. Attivo T{}", attive,rand_trans_lock.id);

        //se è istantanea la faccio subito
        if rand_trans_lock.duration.is_zero() {
            rand_trans_lock.enable(lockunwrap(&shared_places).as_mut_slice());

            fn_printer(
                rand_trans_lock.id,
                lockunwrap(&shared_places),
                start.elapsed().unwrap().as_secs(),
            );
        } else {
            //altrimenti la faccio fare da un thread
            rand_trans_lock.processing = true;

            let curr_trans = Arc::clone(&rand_trans);
            let curr_places = Arc::clone(&shared_places);
            let tx = tx.clone();

            thread::spawn(move || {
                thread::sleep(lockunwrap(&curr_trans).duration);

                let mut curr_trans_lock = lockunwrap(&curr_trans);
                let mut curr_places_lock = lockunwrap(&curr_places);

                curr_trans_lock.enable(curr_places_lock.as_mut_slice());
                curr_trans_lock.processing = false;
                tx.send(Some(())).unwrap();

                fn_printer(
                    curr_trans_lock.id,
                    curr_places_lock,
                    start.elapsed().unwrap().as_secs(),
                );
            });
        }
        i += 1;
    }
}
