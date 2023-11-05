use crate::room_class_stuff::Room;

pub struct World {
    pub evil_bad_error_room: Room,
    pub bulldog_alley_central: Room,
    pub bulldog_alley_east: Room,
    pub college_hall: Room,
    pub crosby: Room,
    pub desmet: Room,
    pub foley_lawn: Room,
    pub foley_library: Room,
    pub hemmingson: Room,
    pub herak_quad: Room,
    pub hughes: Room,
    pub pathways: Room,
    pub rosauer: Room,
    pub welch: Room,
}

impl World {
    pub fn create_world(&mut self) {
        self.evil_bad_error_room = Default::default();
        self.evil_bad_error_room.description = "A very bad an evil room.".to_string();
        self.evil_bad_error_room.address = "NULL".to_string();

        //connections

        /*
        self.NAME = Default::default();
        self.NAME.description = "Blah blah blah.".to_string();
        self.NAME.address = "NAME".to_string();
        self.NAME.add_connection("DESTINATION".to_string(), "PLACEHOLDER".to_string());
        */

        self.bulldog_alley_central = Default::default();
        self.bulldog_alley_central.description = "You are in the center of Bulldog Alley, from here \
        you can see (College Hall), (Crosby), (Desmet), (Herak Quad), and further down Bulldog Alley to the (East). Although Bulldog Alley is usually quite crowded, it is empty and barren.
        There is a (Tree) and a (Bush) nearby.".to_string();
        self.bulldog_alley_central.address = "bulldog_alley_central".to_string();
        self.bulldog_alley_central.add_connection("college_hall".to_string(), "COLLEGE HALL".to_string());
        self.bulldog_alley_central.add_connection("crosby".to_string(), "CROSBY".to_string());
        self.bulldog_alley_central.add_connection("desmet".to_string(), "DESMET".to_string());
        self.bulldog_alley_central.add_connection("herak_quad".to_string(), "HERAK QUAD".to_string());
        self.bulldog_alley_central.add_connection("bulldog_alley_east".to_string(), "EAST".to_string());
        self.bulldog_alley_central.add_objects(
            vec!("TREE".to_string(), "CORPSE".to_string(), "BUSH".to_string()),
            vec!(
                "For a moment, you are convinced your eyes decieve you, but as you look on, you become convinced of what you see. A (Corpse) is stuck in the tree.".to_string(), 
                "You are hesistant to approach, but as you do so, you see that the corpse is heavily damaged. It appears to be from some kind of explosion.".to_string(), 
                "There is a pile of (Robot Scrap) lying inside the bush.".to_string()
            ),
            vec!("Tree".to_string(), "Corpse".to_string(), "Bush".to_string()),
        );
        self.bulldog_alley_central.add_item(&"ROBOT SCRAP".to_string(), &"A pile of robot scrap. Maybe you could learn more about it in the library.".to_string(), &"Robot Scrap".to_string());

        self.bulldog_alley_east = Default::default();
        self.bulldog_alley_east.description = "You are in the east part of Bulldog Alley, from here \
        you can see (Crosby), (Desmet), (Foley Lawn), (Hemmingson), (Rosauer), (Welch), further down Bulldog Alley to the (West), and the (Pathways) between Welch and Desmet.".to_string();
        self.bulldog_alley_east.address = "bulldog_alley_east".to_string();
        self.bulldog_alley_east.add_connection("bulldog_alley_central".to_string(), "WEST".to_string());
        self.bulldog_alley_east.add_connection("crosby".to_string(), "CROSBY".to_string());
        self.bulldog_alley_east.add_connection("desmet".to_string(), "DESMET".to_string());
        self.bulldog_alley_east.add_connection("foley_lawn".to_string(), "FOLEY LAWN".to_string());
        self.bulldog_alley_east.add_connection("hemmingson".to_string(), "HEMMINGSON".to_string());
        self.bulldog_alley_east.add_connection("rosauer".to_string(), "ROSAUER".to_string());
        self.bulldog_alley_east.add_connection("welch".to_string(), "WELCH".to_string());
        self.bulldog_alley_east.add_connection("pathways".to_string(), "PATHWAYS".to_string());

        self.college_hall = Default::default();
        self.college_hall.description = "You are inside of College Hall; you can see the door back out to (Bulldog Alley). There's an (Unlocked Door) in the hallway ahead.".to_string();
        self.college_hall.address = "college_hall".to_string();
        self.college_hall.add_connection("bulldog_alley_central".to_string(), "BULLDOG ALLEY".to_string());
        self.college_hall.add_objects(
            vec!("UNLOCKED DOOR".to_string(), "SODEXO CORPSE".to_string(),),
            vec!(
                "An unlocked door. It is slightly opened, and light pours in from the other side.".to_string(), 
                "A corpse of someone in a Sodexo uniform. Their face is intact. There is a look of absolute terror on them. You don't dare to investigate their wounds. There's an (Id Card) on their uniform.".to_string(), 
            ),
            vec!("Unlocked Door".to_string(), "Sodexo Corpse".to_string(),),
        );
        self.college_hall.add_item(&"ID CARD".to_string(), &"A Sodexo employee Id Card. Could be useful to get in somewhere.".to_string(), &"Id Card".to_string());


        self.crosby = Default::default();
        self.crosby.description = "You are inside of Crosby, you can see the door(s) back out to (Central) Bulldog Alley, (East) Bulldog Alley, (Foley Lawn), and (Herak Quad).\
        \nThere are two rooms here, the (Career Office) and a (Server Room)go college ".to_string();
        self.crosby.address = "crosby".to_string();
        self.crosby.add_connection("bulldog_alley_central".to_string(), "CENTRAL".to_string());
        self.crosby.add_connection("bulldog_alley_east".to_string(), "EAST".to_string());
        self.crosby.add_connection("foley_lawn".to_string(), "FOLEY LAWN".to_string());
        self.crosby.add_connection("herak_quad".to_string(), "HERAK QUAD".to_string());
        self.crosby.add_objects(vec!("CAREER OFFICE".to_string(), "SERVER ROOM".to_string()),
                                vec!(
                                    "As you enter the Career Office, you see on the desk two papers. One of them appears to be a (job application). The other is a (small notecard).".to_string(),
                                    "You look inside the Server Room. It has servers in it. Perfectly normal, but behind them there is a small safe with a (book) on top".to_string()),
                                vec!("Career Office".to_string(), "Server Room".to_string()));
        self.crosby.add_items(vec!("JOB APPLICATION".to_string(), "SMALL NOTECARD".to_string(), "HACKER LAPTOP".to_string(), "BOOK".to_string()),
                              vec!(
"Percival Nightshade
123 Boone Ave, Spokane, WA 99256
Email: xxx_420_PercyHackz_1@email.com | Phone: (123) 456-7890
Dec 12, 2023

Hiring Manager
Sodexo, 789 Security Avenue, Secure City, TX 54321

Dear Hiring Manager,

I am a seasoned Cybersecurity Analyst with 5 years of experience, specializing in network security and incident response.
I am passionate about protecting critical assets and staying current with evolving threats.

Key Qualifications:
- Security Architecture, Threat Assessment, and Risk Management.
- Incident Response Planning and Execution.
- Compliance with industry standards (PCI DSS, HIPAA, GDPR).
- Penetration Testing and Vulnerability Assessments.
- Security Awareness Training.

Education:
- Bachelor of Science in Cybersecurity, Gonzaga University, 2017.

Professional Experience:
Cybersecurity Analyst - Two Barrels
- Led comprehensive cybersecurity strategy, reducing incidents by 30%.
- Managed a team of 4 cybersecurity professionals.
- Developed security policies and ensured industry compliance.

I am excited to contribute my expertise to Sodexo's cybersecurity efforts. Thank you for considering my application. Let's discuss how I can enhance your team's capabilities.

Sincerely,
Percival Nightshade
".to_string(),
"Password: 8975 (This could be used on a safe)".to_string(),
"Property of Percival Nightshade
Use in hemmingson".to_string(),
"The food delivery revolution and its consequences have been a disaster for the human race. It's a
grim reality that we must confront head-on, a crisis born from our insatiable appetite for convenience
at the expense of our humanity. In this age of instant gratification, the Sodexo food robots have become
emblematic of our collective indifference to the very essence of what it means to be human.

In the pursuit of efficiency, we have allowed the heart of our communities to wither away. We've relinquished
the kitchen hearth, the family dinner table, and the neighborhood bistro in exchange for an assembly
line of soulless machines. The Sodexo food robots, those metallic servants of our culinary desires,
may promise speed and convenience, but they have stripped away the soul, the art, and the joy of sharing
a meal.

The act of preparing and sharing food has been at the core of our human experience for millennia.
It has been the catalyst for conversation, laughter, and understanding. Yet, in our reckless quest for
efficiency, we've relegated our food culture to the cold embrace of machines. The Sodexo food robots,
no matter how advanced their algorithms, can never replicate the warmth of a grandmother's recipe, the
secret ingredient passed down through generations, or the smile of a chef who takes pride in their craft.

It is not the fault of the robots themselves but our complacency, our willingness to sacrifice the human
touch for the sake of a few minutes saved. We have allowed the algorithms to replace the soul in our
culinary endeavors, sacrificing our connection to our food, our communities, and ultimately ourselves.
In doing so, we've surrendered the richness of human experience at the altar of convenience.

We must rise against this dehumanizing wave, reclaim our kitchens, our dining rooms, and our neighborhood
restaurants. The Sodexo food robots should not be the symbols of our culinary future, but a stark reminder
of what we have lost. It's time to celebrate the diversity of flavors, the creativity of chefs, and
the human connections that only a shared meal can provide. Let us revive the art of cooking, the joy
of breaking bread together, and restore the heart and soul to our dining experiences.

The food delivery revolution and the Sodexo food robots may have ushered in an era of convenience, but
it's a convenience that has come at a heavy cost. It's time to reclaim our humanity, rediscover the
beauty of real food, and forge a future where our love for cooking and sharing meals transcends the
allure of automation. Our manifesto is a call to action, a plea to rediscover the soul of food and rekindle
the warmth of our communal hearths, for in doing so, we restore what it truly means to be human.".to_string()),
                              vec!("Job Application".to_string(), "Small Notecard".to_string(), "HACKER LAPTOP".to_string(), "Book".to_string())
        );

        self.desmet = Default::default();
        self.desmet.description = "You are inside of Desmet, you can see the door(s) back out to (Central) Bulldog Alley, (East) Bulldog Alley, and the (Pathways).".to_string();
        self.desmet.address = "desmet".to_string();
        self.desmet.add_connection("bulldog_alley_east".to_string(), "EAST".to_string());
        self.desmet.add_connection("bulldog_alley_central".to_string(), "CENTRAL".to_string());
        self.desmet.add_connection("pathways".to_string(), "PATHWAYS".to_string());

        self.foley_lawn = Default::default();
        self.foley_lawn.description = "You are on the Foley Lawn. From here you can see (Crosby), (Foley Library), (Hemmingson), the (East) part of Bulldog Alley, and the (Pathways) in between Desmet and Welch.".to_string();
        self.foley_lawn.address = "foley_lawn".to_string();
        self.foley_lawn.add_connection("crosby".to_string(), "CROSBY".to_string());
        self.foley_lawn.add_connection("foley_library".to_string(), "FOLEY LIBRARY".to_string());
        self.foley_lawn.add_connection("hemmingson".to_string(), "HEMMINGSON".to_string());
        self.foley_lawn.add_connection("pathways".to_string(), "PATHWAYS".to_string());
        self.foley_lawn.add_connection("bulldog_alley_east".to_string(), "EAST".to_string());

        self.foley_library = Default::default();
        self.foley_library.description = "You are inside Foley Library. You can see the door out to (Foley Lawn). There's a (Dusty Laptop) sitting on a desk. The lights are dim. You can make out the faint sound of whirring from above, coming from the (Staircase).".to_string();
        self.foley_library.address = "foley_library".to_string();
        self.foley_library.add_connection("foley_lawn".to_string(), "FOLEY LAWN".to_string());
        self.foley_library.add_object(&"DUSTY LAPTOP".to_string(), &"An old, dusty laptop. The power light is on.".to_string(), &"Dusty Laptop".to_string());
        self.foley_library.add_object(&"STAIRCASE".to_string(), &"You don't want to go up those steps.".to_string(), &"Staircase".to_string());

        self.hemmingson = Default::default();
        self.hemmingson.description = "You are outside of Hemmingson, but the doors are locked. A scanner lies waiting. Behind you is (East) Bulldog Alley, and (Foley Lawn).".to_string();
        self.hemmingson.address = "hemmingson".to_string();
        self.hemmingson.add_connection("bulldog_alley_east".to_string(), "EAST".to_string());
        self.hemmingson.add_connection("foley_lawn".to_string(), "FOLEY LAWN".to_string());

        self.herak_quad = Default::default();
        self.herak_quad.description = "You are on the Herak Quad; from here you can see (Crosby), (Hughes), and (Central) Bulldog Alley.".to_string();
        self.herak_quad.address = "herak_quad".to_string();
        self.herak_quad.add_connection("crosby".to_string(), "CROSBY".to_string());
        self.herak_quad.add_connection("hughes".to_string(), "HUGHES".to_string());
        self.herak_quad.add_connection("bulldog_alley_central".to_string(), "CENTRAL".to_string());

        self.hughes = Default::default();
        self.hughes.description = "You are inside of hughes, you can see the door back out to (Herak Quad)".to_string();
        self.hughes.address = "hughes".to_string();
        self.hughes.add_connection("herak_quad".to_string(), "HERAK QUAD".to_string());

        self.pathways = Default::default();
        self.pathways.description = "You are in the pathways between (Desmet) and (Welch). From here you can see those buildings in addition to (Foley Lawn) and the (East) side of Bulldog Alley.".to_string();
        self.pathways.address = "pathways".to_string();
        self.pathways.add_connection("desmet".to_string(), "DESMET".to_string());
        self.pathways.add_connection("foley_lawn".to_string(), "FOLEY LAWN".to_string());
        self.pathways.add_connection("welch".to_string(), "WELCH".to_string());
        self.pathways.add_connection("bulldog_alley_east".to_string(), "EAST".to_string());

        self.rosauer = Default::default();
        self.rosauer.description = "You are inside of rosauer, you can see the door back out to the (East) side of Bulldog Alley.".to_string();
        self.rosauer.address = "rosauer".to_string();
        self.rosauer.add_connection("bulldog_alley_east".to_string(), "EAST".to_string());

        self.welch = Default::default();
        self.welch.description = "You are inside of welch, you can see the doors back out to the (East) side of Bulldog Alley and to the (Pathways) between Welch and Desmet. Nearby there is a (Tree).".to_string();
        self.welch.address = "welch".to_string();
        self.welch.add_connection("bulldog_alley_east".to_string(), "EAST".to_string());
        self.welch.add_connection("pathways".to_string(), "PATHWAYS".to_string());
        self.welch.add_item(&"SQUIRREL FOOD".to_string(), &"A few acorns. Maybe squribos would enjoy it.".to_string(), &"Squirrel Food".to_string());
        self.welch.add_object(&"TREE".to_string(), &"There's a few acorns on the ground. Perhaps... (Squirrel Food)?".to_string(), &"Tree".to_string());
    }

    pub fn change_room(&self, destination: String) -> &Room {
        return match destination.as_str() {
            "bulldog_alley_central" => {
                &self.bulldog_alley_central
            }
            "bulldog_alley_east" => {
                &self.bulldog_alley_east
            }
            "college_hall" => {
                &self.college_hall
            }
            "crosby" => {
                &self.crosby
            }
            "desmet" => {
                &self.desmet
            }
            "foley_lawn" => {
                &self.foley_lawn
            }
            "foley_library" => {
                &self.foley_library
            }
            "hemmingson" => {
                &self.hemmingson
            }
            "herak_quad" => {
                &self.herak_quad
            }
            "hughes" => {
                &self.hughes
            }
            "pathways" => {
                &self.pathways
            }
            "rosauer" => {
                &self.rosauer
            }
            "welch" => {
                &self.welch
            }
            _ => {
                &self.evil_bad_error_room
            }
        }
    }

    //always input tag, not name
    pub fn use_thing(&self, room : String, tag : String) {
        let tag = tag.as_str();
        match room.as_str() {
            "bulldog_alley_central" => {
                match tag {
                    _ => {
                        println!("Unable to use that.");
                    }
                }
            }
            "bulldog_alley_east" => {
                match tag {
                    _ => {
                        println!("Unable to use that.");
                    }
                }
            }
            "college_hall" => {
                match tag {
                    "UNLOCKED DOOR" => {
                        println!("You push the door open.");
                        println!("It slowly creaks, revealing a desolate room ahead. The walls are caked in blood, and a (Sodexo Corpse) lies slumped in a chair.");
                    }
                    _ => {
                        println!("Unable to use that.");
                    }
                }
            }
            "crosby" => {
                match tag {
                    "SMALL NOTECARD" => {
                        println!("You look around and enter the password found on the notecard into the safe. Doing so, you find a (Hacker Laptop).");
                    }
                    _ => {
                        println!("Unable to use that.");
                    }
                }
            }
            "desmet" => {
                match tag {
                    _ => {
                        println!("Unable to use that.");
                    }
                }
            }
            "foley_lawn" => {
                match tag {
                    _ => {
                        println!("Unable to use that.");
                    }
                }
            }
            "foley_library" => {
                match tag {
                    "DUSTY LAPTOP" => {
                        println!("You shake the mouse to wake up the laptop. As it boots up, you scroll through an article.");
                        println!("BREAKING - SODEXO FOOD ROBOTS MALFUNCTION");
                        println!("Published December 20, 2023");
                        println!("Although many enjoy going home over the holidays, not everyone gets to do so. Some students remain on-campus, enjoying their holidays at the wonderful GU.");
                        println!("However, some never get the luxury of going home. Corion Ilstess was found dead this morning in Foley.");
                        println!("Early this morning, Corion ordered a sandwhich from Iggy's, delivered by the Sodexo food robots. However, as the robot approached, disaster struck. The robot approached Corion, and exploded. ");
                    }
                    "ROBOT SCRAP" => {
                        println!("You sit down at one of the PCs in the library. Curious, you Google for information about the Robot Scrap, looking to find any information you can about it.");
                        println!("You don't find much, but you notice that the last user was logged into their email. You open it. It reads:");
                        println!("Subject : Recent Happenings");
                        println!("Date : Dec 19, 2023");
                        println!("To all Sodexo employees,");
                        println!("It seems the robots have gone rogue. Please come to College Hall for an immediate meeting.");
                        println!("Best,");
                        println!("- Soda Dexter");
                    }
                    _ => {
                        println!("Unable to use that.");
                    }
                }
            }
            "hemmingson" => {
                match tag {
                    "HACKER LAPTOP" => {
                        println!("You open the laptop and boot it up. As you finish entering the code, you hear a slow buzz. The door opens. All around you are Sodexo food robots. They have heart eyes.");
                        println!();
                        println!("The end.");
                        println!();
                        println!("You have now unlocked Luigi... and entered freeplay.");
                    }

                    "ID CARD" => {
                        println!("You try to scan yourself in to get further, but the card is declined. It seems the Sodexo Security system has been hacked... You'll need better tech to get in.");
                    }
                    _ => {
                        println!("Unable to use that.");
                    }
                }
            }
            "herak_quad" => {
                match tag {
                    _ => {
                        println!("Unable to use that.");
                    }
                }
            }
            "hughes" => {
                match tag {
                    _ => {
                        println!("Unable to use that.");
                    }
                }
            }
            "pathways" => {
                match tag {
                    "SQUIRREL FOOD" => {
                        println!("A squirrel approaches you and takes the food.");
                        println!("You are filled with determination.");
                    }
                    _ => {
                        println!("Unable to use that.");
                    }
                }
            }
            "rosauer" => {
                match tag {
                    _ => {
                        println!("Unable to use that.");
                    }
                }
            }
            "welch" => {
                match tag {
                    _ => {
                        println!("Unable to use that.");
                    }
                }
            }
            _ => {

            }
        }

    }
}



impl Default for World {
    fn default() -> Self {
        // Default Constructor my beloved.
        World {
            evil_bad_error_room: Default::default(),
            bulldog_alley_central: Default::default(),
            bulldog_alley_east: Default::default(),
            college_hall: Default::default(),
            crosby: Default::default(),
            desmet: Default::default(),
            foley_lawn: Default::default(),
            foley_library: Default::default(),
            hemmingson: Default::default(),
            herak_quad: Default::default(),
            hughes: Default::default(),
            pathways: Default::default(),
            rosauer: Default::default(),
            welch: Default::default(),
        }
    }
}    