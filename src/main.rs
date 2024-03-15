use clap::{command, Arg, ArgGroup, Command};

fn main() {
    let match_result = command!()
                                .subcommand(
                                    Command::new("register-person")
                                        .arg(Arg::new("firstname")
                                        .short('f')
                                        .long("first-name")
                                        .aliases(["fname", "firstname"])
                                        .required(true)
                                        .help("The person's first name")
                                        // .conflicts_with("lastname")
                                    )
                                .arg(Arg::new("lastname")
                                        .short('l')
                                        .long("last-name")
                                        .aliases(["lname", "lastname"])
                                        .required(true)
                                        .help("The person's last name")
                                    )
                                )
                                .subcommand(
                                    Command::new("register-pet")
                                        .arg(Arg::new("pet-name")
                                        .long("pet-name")
                                        .short('n')
                                        .required(true)
                                    )
                                )
                                .about("This app registers people with their doctor's office.")
                                // .group(
                                //     ArgGroup::new("person-register")
                                //         .arg("firstname")
                                //         .arg("lastname")
                                // )
                                // .group(
                                //     ArgGroup::new("dog-register")
                                //         .arg("pet-name")
                                // )
                                .arg(Arg::new("fluffy")
                                    .long("fluffy")
                                    .aliases(["flf"])
                                    .help("Is the person wearing a fluffy coat or not")
                                )
                                .get_matches();
    
    // println!("{}", match_result.get_one::<String>("pet-name").unwrap_or(&"NO PET NAME".to_string()));
    // let pet_args = match_result.subcommand_matches("register-pet").unwrap().contains_id("pet-name");
    // let pet_args = match_result.subcommand_matches("register-pet").unwrap().get_one::<String>("pet-name").unwrap();
    // println!("Does pet name exists? {}", pet_args);

    let person_args = match_result.subcommand_matches("register-person").unwrap();
    let fname = person_args.get_one::<String>("firstname").unwrap();
    let lname = person_args.get_one::<String>("lastname").unwrap();
    println!("First Name: {} Last Name: {}", fname, lname);
}
