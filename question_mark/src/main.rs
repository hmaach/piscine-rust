use question_mark::*;

fn main() {
    let a = One {
        first_layer: Some(Two {
            second_layer: Some(Three {
                third_layer: Some(Four {
                    fourth_layer: Some(1000),
                }),
            }),
        }),
    };

    println!("{:?}", a.get_fourth_layer());
}
