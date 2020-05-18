use read_input::prelude::*;

// Meter to other 11

pub fn meter_kilometer() -> f64 {
    
    let x = input::<f64>().get();
    let y = x/1000 as f64;

    y    
}

pub fn meter_centimeter() -> i64 {
    let x = input::<i64>().get();
    let y = x * 100;

    y 
}

pub fn meter_milimeter() -> i64 {
    let x = input::<i64>().get();
    let y = x * 1000;

    y     
}

pub fn meter_micrometer() -> i64 {
    let x = input::<i64>().get();
    let y = x * 1000000;

    y
}

pub fn meter_nanometer() -> i64 {
    let x = input::<i64>().get();
    let y = x * 1000000000;

    y 
}

pub fn meter_mile() -> f64 {
    let x = input::<f64>().get();
    let y = x / 1609.344 as f64;

    y
}

pub fn meter_yard() -> f64 {
    let x = input::<f64>().get();
    let y = x * 1.094 as f64;

    y
}

pub fn meter_foot() -> f64 {
    let x = input::<f64>().get();
    let y = x * 3.281 as f64;

    y
}

pub fn meter_inch() -> f64 {
    let x = input::<f64>().get();
    let y = x * 39.37 as f64;

    y
}

pub fn meter_nauticalmile() -> f64 {
    let x = input::<f64>().get();
    let y = x / 1852 as f64;

    y
}


// kilometer to 11 others

pub fn kilometer_centimeter() -> i64 {
    let x = input::<i64>().get();
    let y = x * 100000;

    y
}

pub fn kilometer_meter() -> i64 {
    let x = input::<i64>().get();
    let y = x * 1000;

    y
}

pub fn kilometer_milimeter() -> i64 {
    let x = input::<i64>().get();
    let y = x * 1000000;

    y
}

pub fn kilometer_micrometer() -> i64 {
    let x = input::<i64>().get();
    let y = x * 1000000000;

    y
}

pub fn kilometer_nanometer() -> i64 {
    let x = input::<i64>().get();
    let y = x * 1000000000000;

    y
}

pub fn kilometer_mile() -> f64 {
    let x = input::<f64>().get();
    let y = x / 1.609 as f64;

    y
}

pub fn kilometer_yard() -> f64 {
    let x = input::<f64>().get();
    let y = x * 1093.613 as f64;

    y
}

pub fn kilometer_foot() -> f64 {
    let x = input::<f64>().get();
    let y = x * 3280.84 as f64;

    y
}

pub fn kilometer_inch() -> f64 {
    let x = input::<f64>().get();
    let y = x * 39370.079 as f64;

    y
}

pub fn kilometer_nauticalmile() -> f64 {
    let x = input::<f64>().get();
    let y = x / 1.852 as f64;

    y
}


// centimeter

pub fn centimeter_meter() -> i64 {
    let x = input::<i64>().get();
    let y = x / 100 as i64;

    y
}

pub fn centimeter_kilometer() -> f64 {
    let x = input::<f64>().get();
    let y = x / 100000 as f64;

    y
}

pub fn centimeter_milimeter() -> i64 {
    let x = input::<i64>().get();
    let y = x * 100 as i64;

    y
}

pub fn centimeter_micrometer() -> i64 {
    let x = input::<i64>().get();
    let y = x * 10000 as i64;

    y
}

pub fn centimeter_nanometer() -> i64 {
    let x = input::<i64>().get();
    let y = x * 10000000 as i64;

    y
}

pub fn centimeter_mile() -> f64 {
    let x = input::<f64>().get();
    let y = x / 160934.4 as f64;

    y
}

pub fn centimeter_yard() -> f64 {
    let x = input::<f64>().get();
    let y = x / 91.44 as f64;

    y
}

pub fn centimeter_foot() -> f64 {
    let x = input::<f64>().get();
    let y = x / 30.48 as f64;

    y
}

pub fn centimeter_inch() -> f64 {
    let x = input::<f64>().get();
    let y = x / 2.54 as f64;

    y
}

pub fn centimeter_nauticalmile() -> f64 {
    let x = input::<f64>().get();
    let y = x / 185200 as f64;

    y
}


// milimeter

pub fn milimeter_meter() -> f64 {
    let x = input::<f64>().get();
    let y = x / 1000 as f64;

    y
}

pub fn milimeter_centimeter() -> f64 {
    let x = input::<f64>().get();
    let y = x / 10 as f64;

    y
}

pub fn milimeter_kilometer() -> f64 {
    let x = input::<f64>().get();
    let y = x / 1000000 as f64;
    
    y
}

pub fn milimeter_micrometer() -> i64 {
    let x = input::<i64>().get();
    let y = x * 1000;

    y
}

pub fn milimeter_nanometer() -> i64 {
    let x = input::<i64>().get();
    let y = x * 1000000;

    y
}

pub fn milimeter_mile() -> f64 {
    let x = input::<f64>().get();
    let y = x / 1.609000000 as f64;
    
    y
}

pub fn milimeter_yard() -> f64 {
    let x = input::<f64>().get();
    let y = x / 914.4 as f64;
    
    y
}

pub fn milimeter_foot() -> f64 {
    let x = input::<f64>().get();
    let y = x / 308.8 as f64;
    
    y
}

pub fn milimeter_inch() -> f64 {
    let x = input::<f64>().get();
    let y = x / 25.4 as f64;
    
    y
}

pub fn milimeter_nauticalmile() -> f64 {
    let x = input::<f64>().get();
    let y = x / 25.4 as f64;
    
    y
}


// micrometer

// pub fn micrometer_centimeter() -> {}

// pub fn micrometer_kilometer() -> {}

// pub fn micrometer_milimeter() -> {}

// pub fn micrometer_meter() -> {}

// pub fn micrometer_nanometer() -> {}

// pub fn micrometer_mile() -> {}

// pub fn micrometer_yard() -> {}

// pub fn micrometer_foot() -> {}

// pub fn micrometer_inch() -> {}

// pub fn micrometer_nauticalmile() -> {}


// // nanometer

// pub fn nanometer_centimeter() -> {}

// pub fn nanometer_kilometer() -> {}

// pub fn nanometer_milimeter() -> {}

// pub fn nanometer_micrometer() -> {}

// pub fn nanometer_nanometer() -> {}

// pub fn nanometer_mile() -> {}

// pub fn nanometer_yard() -> {}

// pub fn nanometer_foot() -> {}

// pub fn nanometer_inch() -> {}

// pub fn nanometer_nauticalmile() -> {}


// // mile

// pub fn mile_centimeter() -> {}

// pub fn mile_kilometer() -> {}

// pub fn mile_milimeter() -> {}

// pub fn mile_micrometer() -> {}

// pub fn mile_nanometer() -> {}

// pub fn mile_mile() -> {}

// pub fn mile_yard() -> {}

// pub fn mile_foot() -> {}

// pub fn mile_inch() -> {}

// pub fn mile_nauticalmile() -> {}


// // yard

// pub fn yard_centimeter() -> {}

// pub fn yard_kilometer() -> {}

// pub fn yard_milimeter() -> {}

// pub fn yard_micrometer() -> {}

// pub fn yard_nanometer() -> {}

// pub fn yard_mile() -> {}

// pub fn yard_yard() -> {}

// pub fn yard_foot() -> {}

// pub fn yard_inch() -> {}

// pub fn yard_nauticalmile() -> {}


// // foot

// pub fn foot_centimeter() -> {}

// pub fn foot_kilometer() -> {}

// pub fn foot_milimeter() -> {}

// pub fn foot_micrometer() -> {}

// pub fn foot_nanometer() -> {}

// pub fn foot_mile() -> {}

// pub fn foot_yard() -> {}

// pub fn foot_foot() -> {}

// pub fn foot_inch() -> {}

// pub fn foot_nauticalmile() -> {}


// // inch

// pub fn inch_centimeter() -> {}

// pub fn inch_kilometer() -> {}

// pub fn inch_milimeter() -> {}

// pub fn inch_micrometer() -> {}

// pub fn inch_nanometer() -> {}

// pub fn inch_mile() -> {}

// pub fn inch_yard() -> {}

// pub fn inch_foot() -> {}

// pub fn inch_inch() -> {}

// pub fn inch_nauticalmile() -> {}


// // nauticalmile

// pub fn nauticalmile_centimeter() -> {}

// pub fn nauticalmile_kilometer() -> {}

// pub fn nauticalmile_milimeter() -> {}

// pub fn nauticalmile_micrometer() -> {}

// pub fn nauticalmile_nanometer() -> {}

// pub fn nauticalmile_mile() -> {}

// pub fn nauticalmile_yard() -> {}

// pub fn nauticalmile_foot() -> {}

// pub fn nauticalmile_inch() -> {}

// pub fn nauticalmile_nauticalmile() -> {}