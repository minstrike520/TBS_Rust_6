use std::collections::HashMap;


mod game;
mod tests;

// HashMap<&str, Vec<T: Tuner> >

/*
        HashMap::from([
            ("atk", vec![ TunerFlat(-20) ]),
            ("spd", vec![ TunerPerc(30), TunerFlat(5) ])
        ]) 
*/

fn main() {
    tests::test_all();
}