use std::collections::{HashMap, HashSet};

use mobs::*;

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

#[test]
fn create_boss() {
    assert_eq!(
        Boss::new("Scarface", 43),
        Boss {
            name: "Scarface".to_owned(),
            age: 43
        }
    );
}

#[inline]
fn mobs() -> (Mob, Mob) {
    (
        Mob {
            name: "Hairy Giants".to_owned(),
            boss: Boss::new("Louie HaHa", 36),
            cities: ["San Francisco"].map(str::to_owned).into(),
            members: [
                (
                    "Benny Eggs",
                    Member {
                        role: Role::Soldier,
                        age: 28,
                    },
                ),
                (
                    "Jhonny",
                    Member {
                        role: Role::Associate,
                        age: 17,
                    },
                ),
                (
                    "Greasy Thumb",
                    Member {
                        role: Role::Soldier,
                        age: 30,
                    },
                ),
                (
                    "No Finger",
                    Member {
                        role: Role::Caporegime,
                        age: 32,
                    },
                ),
            ]
            .map(|(n, d)| (n.to_owned(), d))
            .into(),
            wealth: 100000,
        },
        Mob {
            name: "Red Thorns".to_owned(),
            boss: Boss::new("Big Tuna", 30),
            cities: ["San Jose"].map(str::to_owned).into(),
            members: [
                (
                    "Knuckles",
                    Member {
                        role: Role::Soldier,
                        age: 25,
                    },
                ),
                (
                    "Baldy Dom",
                    Member {
                        role: Role::Caporegime,
                        age: 36,
                    },
                ),
                (
                    "Crazy Joe",
                    Member {
                        role: Role::Underboss,
                        age: 23,
                    },
                ),
            ]
            .map(|(n, d)| (n.to_owned(), d))
            .into(),
            wealth: 70000,
        },
    )
}

#[test]
fn mob_recruit() {
    let (mut mob, _) = mobs();
    mob.recruit(("Rusty", 37));

    assert_eq!(
        mob.members.get("Rusty"),
        Some(&Member {
            role: Role::Associate,
            age: 37,
        })
    );

    mob.recruit(("Pedro", 14));

    assert_eq!(
        mob.members.get("Pedro"),
        Some(&Member {
            role: Role::Associate,
            age: 14,
        })
    );
}

#[test]
fn member_get_promotion() {
    let (mut mob, _) = mobs();

    let member = mob.members.get_mut("Benny Eggs").unwrap();

    member.get_promotion();
    assert_eq!(member.role, member::Role::Caporegime);
    member.get_promotion();
    assert_eq!(member.role, member::Role::Underboss);
}

#[test]
#[should_panic]
fn member_get_promotion_panic() {
    let (_, mut mob) = mobs();

    let member = mob.members.get_mut("Crazy Joe").unwrap();

    member.get_promotion();
}

#[test]
fn mob_steal() {
    let (mut a, mut b) = mobs();
    b.steal(&mut a, 10000);
    assert_eq!(a.wealth, 90000);
    assert_eq!(b.wealth, 80000);

    b.steal(&mut a, 1000000);
    assert_eq!(a.wealth, 0);
    assert_eq!(b.wealth, 170000);
}

#[test]
fn mob_attack() {
    let (mut a, mut b) = mobs();
    a.attack(&mut b);

    assert_eq!(a.members.len(), 3);
    assert_eq!(b.members.len(), 3);
}

#[test]
fn same_combat_power() {
    let (mut a, mut b) = mobs();

    a.recruit(("Stitches", 28));
    a.attack(&mut b);

    assert_eq!(a.members.len(), 4);
    assert_eq!(b.members.len(), 3);
}

#[test]
fn no_members_mob() {
    let (mut a, mut b) = mobs();
    b.attack(&mut a);
    a.attack(&mut b);
    b.attack(&mut a);
    b.attack(&mut a);

    assert_eq!(a.members.len(), 0);
    assert_eq!(a.cities.len(), 0);
    assert_eq!(a.wealth, 0);

    assert!(b
        .cities
        .is_superset(&["San Jose", "San Francisco"].map(str::to_owned).into()));
    assert_eq!(b.wealth, 170000);
}

#[test]
fn mob_conquer_city() {
    let (mut a, mut b) = mobs();

    b.conquer_city(&[&a], "Las Vegas".to_owned());
    assert!(b.cities.contains("Las Vegas"));

    a.conquer_city(&[&b], "Las Vegas".to_owned());
    assert_eq!(a.cities.len(), 1);
}
