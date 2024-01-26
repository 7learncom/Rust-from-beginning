use resturant::hosting::{add_to_waitlist, add_to_waitlist1};

fn main() {
    eat_at_restaurant()
}

mod resturant {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn add_to_waitlist1() {}
    }

    mod cooking {}

    mod payment {
        pub fn pay() {
            calculate_factor()
        }
        
        fn calculate_factor() {
            calculate_discount()
        }

        fn calculate_discount() {}
    }
}

fn eat_at_restaurant() {
    add_to_waitlist()
}

fn eat_at_restaurant1() {
    add_to_waitlist1()
}
