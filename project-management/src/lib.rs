// Example of importing crate dependency
use rand::Rng;

// Example of module structure
mod take_away_restaurant {
    pub mod food_menu {
        fn customer_queue() {}

        fn customer_selection() {}

        pub fn customer_decision() {}

        fn customer_payment() {}
    }
}

pub fn eat_food() {
    // Absolute path
    crate::take_away_restaurant::food_menu::customer_decision();
    
    // Relative path
    take_away_restaurant::food_menu::customer_decision();
}