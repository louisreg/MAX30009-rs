#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub enum LoffImag {
    Off = 0,
    N50 = 1,
    N100 = 2,
    N200 = 3,
    N500 = 4,
    N1000 = 5,
    N1000Alt = 6,
    N100Alt = 7,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub enum LoffThresh {
    Th215 = 0,
    Th245 = 1,
    Th275 = 2,
    Th305 = 3,
    Th335 = 4,
    Th365 = 5,
    Th395 = 6,
    Th425 = 7,
    Th455 = 8,
    Th485 = 9,
    Th515 = 10,
    Th545 = 11,
    Th575 = 12,
    Th605 = 13,
    Th635 = 14,
    Th665 = 15,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub enum RbiasValue {
    R500M = 0,
    R1G = 1,
    R2G = 2,
}
