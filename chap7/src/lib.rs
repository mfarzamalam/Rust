pub mod restaurant {
    pub mod front_of_restaurant {
        pub mod waiter {
                pub fn welcome_costomer() {
                    println!("waiter has welcome the customer");
                    super::super::back_of_restaurant::cheff::receive_order();
                }
                pub fn take_order() {
                    println!("waiter has taken the order");
                }

                pub fn take_payment() {
                    println!("waiter has taken the payment");
                }
        }
    }

    mod back_of_restaurant {
            pub mod cheff {
                pub fn receive_order() {
                        println!("chef received order from waiter") 
                }

                    fn cook_food() {
                        println!("chef started cooking");
                }

                pub fn handover() {
                        println!("chef handover the food to waiter");                    
                }
            }
    }
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

