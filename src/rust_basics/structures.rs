pub mod pet {

   #[derive(Debug)] 
    pub struct Pet {
        pub name: String,
        pub owner: String,
        pub deserves_treat: bool,
    }

    pub struct Insect {
        pub name: String,
        pub owner: String,
        pub deserves_treat: bool,
    }
    impl Pet {
        pub fn new(pet_name: &str, pet_owner: &str, pet_deserves_treat: bool) -> Pet {
            let pet = Pet {
                name: pet_name.to_string(),
                owner: pet_owner.to_string(),
                deserves_treat: pet_deserves_treat,
            };

            pet
        }

        pub fn give_treat(&self) {
            if self.deserves_treat {
                println! {"Thank you for the treat, {}", self.owner};
            } else if !self.deserves_treat {
                println! {"{} doesn't deserve a treat..", self.name};
            }
        }

        pub fn do_trick(&mut self) {
            println!("doing cool trick for.. {}", self.owner);
            self.deserves_treat = true;
        }

    }

}


pub mod rect {
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
    }

    impl Rectangle {
        pub fn new(width: u32, height: u32) -> Rectangle{
           Rectangle { width: width, height: height}
    }

        pub fn calculate_area(&self) -> u32{
            self.width * self.height
        }
    }
}

