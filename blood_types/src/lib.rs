#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive( PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum RhFactor {
	Positive,
	Negative,
}

impl Debug for RhFactor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{}",
            match self {
                Self::Positive => '+',
                Self::Negative => '-',            
            }
        )
    }
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => 	Ok(Self::A),
            "AB" => Ok(Self::AB),
            "B" => 	Ok(Self::B),
            "O" => 	Ok(Self::O),
            &_ => 	Err(()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => 	Ok(Self::Positive),
            "-" => 	Ok(Self::Negative),
            _ => 	Err(()),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.antigen == other.antigen {
            self.rh_factor.cmp(&other.rh_factor)
        } else {
            self.antigen.cmp(&other.antigen)
        }
    }
}

impl FromStr for BloodType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !(s.len() == 2 || s.len() == 3) {
            Err(())?
        };
        Ok(
            BloodType { 
                antigen:   s[0..s.len()-1].parse()?,
                rh_factor: s[s.len()-1..].parse()?,
            }
        )
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{:?}",self.antigen, self.rh_factor)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        if self.antigen == Antigen::AB && self.rh_factor == RhFactor::Positive {
            true
        } else if other.antigen == Antigen::O && other.rh_factor == RhFactor::Negative {
            true
        } else if self.antigen == other.antigen && self.rh_factor == other.rh_factor {
            true
        } else if self.antigen == Antigen::AB && self.rh_factor == RhFactor::Negative && other.rh_factor == RhFactor::Negative {
            true
        } else if other.antigen == Antigen::O && other.rh_factor == RhFactor::Positive && self.rh_factor == RhFactor::Positive {
            true
        } else if self.antigen == other.antigen && other.rh_factor == RhFactor::Negative {
            true
        } else {
            false
        }
    }

    pub fn donors(&self) -> Vec<BloodType> {
        let mut donors = Vec::new();
        let antigen_values = [
            Antigen::A,
            Antigen::B,
            Antigen::AB,
            Antigen::O,
        ];
        let rh_factor_values = [
            RhFactor::Positive,
            RhFactor::Negative,
        ];
    
        for antigen in antigen_values.iter() {
            for rh_factor in rh_factor_values.iter() {
                let blood_type = BloodType {
                    antigen: *antigen,
                    rh_factor: *rh_factor,
                };
    
                if self.can_receive_from(&blood_type) {
                    donors.push(blood_type);
                }
            }
        }
    
        donors
    }
    

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut recipients = Vec::new();
        let antigen_values = [
            Antigen::A,
            Antigen::B,
            Antigen::AB,
            Antigen::O,
        ];
        let rh_factor_values = [
            RhFactor::Positive,
            RhFactor::Negative,
        ];
    
        for antigen in antigen_values.iter() {
            for rh_factor in rh_factor_values.iter() {
                let blood_type = BloodType {
                    antigen: *antigen,
                    rh_factor: *rh_factor,
                };
    
                if blood_type.can_receive_from(self) {
                    recipients.push(blood_type);
                }
            }
        }
    
        recipients
    }
}