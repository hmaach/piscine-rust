use std::collections::{HashMap, HashSet};

// use mobs::member::Member;
use mobs::{boss::Boss, member::*, Mob};

fn main() {
    let mut members_1 = HashMap::new();
    members_1.insert(
        "Lfarssi".to_owned(),
        Member {
            age: 90,
            role: Role::Soldier,
        },
    );
    members_1.insert(
        "Fahd".to_owned(),
        Member {
            age: 90,
            role: Role::Caporegime,
        },
    );
    members_1.insert(
        "Madara".to_owned(),
        Member {
            age: 90,
            role: Role::Associate,
        },
    );

    let mut mob_1 = Mob {
        name: "Zone01 Oujda".to_owned(),
        boss: Boss {
            age: 18,
            name: "Hamza Maach".to_owned(),
        },
        cities: HashSet::from(["Berkane".to_owned(), "Oujda".to_owned()]),
        wealth: 5000,
        members: members_1,
    };
    // -------------------------------------------------------
    let mut members_2 = HashMap::new();
    members_2.insert(
        "Yassine".to_owned(),
        Member {
            age: 70,
            role: Role::Soldier,
        },
    );
    members_2.insert(
        "Isayen".to_owned(),
        Member {
            age: 80,
            role: Role::Caporegime,
        },
    );
    members_2.insert(
        "Nachti".to_owned(),
        Member {
            age: 40,
            role: Role::Associate,
        },
    );

    let mut mob_2 = Mob {
        name: "Zone01 Sous".to_owned(),
        boss: Boss {
            age: 45,
            name: "Ahmed Baid".to_owned(),
        },
        cities: HashSet::from(["Agadir".to_owned(), "Tiznit".to_owned()]),
        wealth: 800,
        members: members_2,
    };
    // -------------------------------------------------------
    // -------------------------------------------------------
    // -------------------------------------------------------
    // -------------------------------------------------------
    println!("{:?}", mob_1);
    println!("{:?}", mob_2);
    mob_1.attack(&mut mob_2);
    println!("{:?}", mob_2);
}
