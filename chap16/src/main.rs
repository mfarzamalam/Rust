use std::thread;
use std::time::Duration;


///////////////////////// PART 1 /////////////////////////////////////


// fn main() {

//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!(" from spawned thread {} ",i);
//             thread::sleep(Duration::from_millis(1));
            
//         }

        
//     });

//     for i in 1..7 {
//         println!("from main thread {} ",i);
//         thread::sleep(Duration::from_millis(1));
//     }

//     handle.join().unwrap();

// }



///////////////////////// PART 2 /////////////////////////////////////

fn main() {
    let vextor = vec![1,2,3,4,5,6];

    let handle = thread::spawn(move || {
        println!(" from the spawn thread {:?}", vextor);
    });

    handle.join().unwrap();

}
