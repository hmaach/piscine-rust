use shopping_mall::*;

fn main() {
    let mut mall = Mall::new(
        "La Vie Funchal",
        [
            (
                "John Oliver",
                Guard {
                    age: 34,
                    years_experience: 7,
                },
            ),
            (
                "Bob Schumacher",
                Guard {
                    age: 53,
                    years_experience: 15,
                },
            ),
        ]
        .into(),
        [
            (
                "Ground Floor",
                Floor::new(
                    [
                        (
                            "Footzo",
                            Store::new(
                                [
                                    (
                                        "Finbar Haines",
                                        Employee {
                                            age: 36,
                                            working_hours: (9, 14),
                                            salary: 650.88,
                                        },
                                    ),
                                    (
                                        "Sienna-Rose Penn",
                                        Employee {
                                            age: 26,
                                            working_hours: (9, 22),
                                            salary: 1000.43,
                                        },
                                    ),
                                ]
                                .into(),
                                50,
                            ),
                        ),
                        (
                            "Swashion",
                            Store::new(
                                [
                                    (
                                        "Abdallah Stafford",
                                        Employee {
                                            age: 54,
                                            working_hours: (8, 22),
                                            salary: 1234.21,
                                        },
                                    ),
                                    (
                                        "Marian Snyder",
                                        Employee {
                                            age: 21,
                                            working_hours: (8, 14),
                                            salary: 831.9,
                                        },
                                    ),
                                ]
                                .into(),
                                43,
                            ),
                        ),
                    ]
                    .into(),
                    300,
                ),
            ),
            (
                "Supermarket",
                Floor::new(
                    [(
                        "Pretail",
                        Store::new(
                            [
                                (
                                    "Yara Wickens",
                                    Employee {
                                        age: 39,
                                        working_hours: (9, 14),
                                        salary: 853.42,
                                    },
                                ),
                                (
                                    "Indiana Baxter",
                                    Employee {
                                        age: 33,
                                        working_hours: (13, 20),
                                        salary: 991.71,
                                    },
                                ),
                                (
                                    "Jadine Page",
                                    Employee {
                                        age: 48,
                                        working_hours: (13, 20),
                                        salary: 743.21,
                                    },
                                ),
                                (
                                    "Tyler Hunt",
                                    Employee {
                                        age: 63,
                                        working_hours: (13, 20),
                                        salary: 668.25,
                                    },
                                ),
                                (
                                    "Mohsin Mcgee",
                                    Employee {
                                        age: 30,
                                        working_hours: (19, 24),
                                        salary: 703.83,
                                    },
                                ),
                                (
                                    "Antoine Goulding",
                                    Employee {
                                        age: 45,
                                        working_hours: (19, 24),
                                        salary: 697.12,
                                    },
                                ),
                                (
                                    "Mark Barnard",
                                    Employee {
                                        age: 53,
                                        working_hours: (19, 24),
                                        salary: 788.81,
                                    },
                                ),
                            ]
                            .into(),
                            950,
                        ),
                    )]
                    .into(),
                    1000,
                ),
            ),
        ]
        .into(),
    );

    // returns the biggest store
    println!("Biggest store: {:#?}", biggest_store(&mall));

    // returns the list with the highest paid employees
    println!("Highest paid employee: {:#?}", highest_paid_employee(&mall));

    // returns the number of employees
    println!("Number of employees: {}", nbr_of_employees(&mall));

    // checks if it is needed to add securities
    check_for_securities(
        &mut mall,
        [
            (
                "Peter Solomons",
                Guard {
                    age: 45,
                    years_experience: 20,
                },
            ),
            (
                "William Charles",
                Guard {
                    age: 32,
                    years_experience: 10,
                },
            ),
            (
                "Leonardo Changretta",
                Guard {
                    age: 23,
                    years_experience: 0,
                },
            ),
            (
                "Vlad Levi",
                Guard {
                    age: 38,
                    years_experience: 8,
                },
            ),
            (
                "Faruk Berkai",
                Guard {
                    age: 40,
                    years_experience: 15,
                },
            ),
            (
                "Christopher Smith",
                Guard {
                    age: 35,
                    years_experience: 9,
                },
            ),
            (
                "Jason Mackie",
                Guard {
                    age: 26,
                    years_experience: 2,
                },
            ),
            (
                "Kenzie Mair",
                Guard {
                    age: 34,
                    years_experience: 8,
                },
            ),
            (
                "Bentley Larson",
                Guard {
                    age: 33,
                    years_experience: 10,
                },
            ),
            (
                "Ray Storey",
                Guard {
                    age: 37,
                    years_experience: 12,
                },
            ),
        ]
        .map(|(n, d)| (n.to_owned(), d))
        .into(),
    );

    // raises or cuts the salary of every employee
    cut_or_raise(&mut mall);

    println!("{:#?}", mall);
}