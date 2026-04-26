use cucumber::{World, Parameter};
use std::{str::FromStr};

#[derive(Debug, Default, Parameter, PartialEq, Eq)]
#[param(name="state",regex="(?ism)(hungry|happy|annoyed|curious|satiated)")]
pub enum Feeling {
    Happy,
    Annoyed,
    #[default]
    Hungry,
    Curious,
    Satiated,
}

impl FromStr for Feeling {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "hungry" => Feeling::Hungry,
            "happy" => Feeling::Happy,
            "annoyed" => Feeling::Annoyed,
            "curious" => Feeling::Curious,
            "satiated" => Feeling::Satiated,
            invalid => return Err(format!("Invalid `feeling`: {invalid}"))
        })  
    }
}

#[derive(Debug, Default, Parameter)]
#[param(name="action",regex="(?ims)(pet|call|feed)")]
pub enum Action {
    #[default]
    Pet,
    Call,
    Feed
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "pet" => Self::Pet,
            "call" => Self::Call,
            "feed" => Self::Feed,
            invalid => return Err(format!("invalid `action`: {invalid}"))
        })
    }
}

// These `Cat` definitions would normally be inside your project's code, 
// not test code, but we create them here for the show case.
#[derive(Debug, Default)]
pub struct Cat {
    pub feeling:Feeling 
}

impl Cat {
    pub fn feed(&mut self,food:&Food) {
        self.feeling = match self.feeling {
            Feeling::Hungry => 
                match food {
                    Food::CatFood => Feeling::Happy,
                    Food::Tuna => Feeling::Happy,
                    Food::Beans => Feeling::Annoyed,
                }
            _ => Feeling::Curious 
        }
    }

    pub fn pet(&mut self) {
        self.feeling = match self.feeling {
            Feeling::Hungry => Feeling::Annoyed,
            Feeling::Satiated => Feeling::Happy,
            Feeling::Curious => Feeling::Happy,
            _ => Feeling::Curious
        }
    }

    pub fn call(&mut self) {
        self.feeling = Feeling::Curious
    }
}

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario. 
#[derive(Debug, Default, World)]
pub struct AnimalWorld {
    pub cat: Cat,
    pub food: Food
}

#[derive(Debug, Default, Parameter, PartialEq, Eq)]
#[param(name="food",regex=r"(?ims)(?P<food>[\w\s]+)")]
pub enum Food {
    #[default]
    CatFood,
    Tuna,
    Beans
}

impl FromStr for Food {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "tuna" => Food::Tuna,
            "beans" => Food::Beans,
            "cat food" => Food::CatFood,
            invalid => return Err(format!("invalid `food`: {invalid}")),
        })
    }
}

