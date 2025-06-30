#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}
use Antigen::*;
use RhFactor::*;
#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use core::fmt;
use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Ok(A),
            "AB" => Ok(AB),
            "B" => Ok(B),
            "O" => Ok(O),
            _ => Err("Error parse blood type".to_string()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Negative),
            "+" => Ok(Self::Positive),
            _ => Err("".to_string()),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rh_factor.cmp(&other.rh_factor) {
            Ordering::Equal => self.antigen.cmp(&other.antigen),
            ord => ord,
        }
    }
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        if min > max {
            min
        } else if min < max {
            max
        } else {
            max
        }
    }
}

impl FromStr for BloodType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Input string is empty".to_string());
        }
        let rh_part: &str;
        let antigen_part: &str;
        if s.contains("AB") {
            (antigen_part, rh_part) = s.split_at(2);
        } else {
            (antigen_part, rh_part) = s.split_at(1);
        }

        let rh_factor = RhFactor::from_str(rh_part)?;
        let antigen = Antigen::from_str(antigen_part)?;

        Ok(BloodType { rh_factor, antigen })
    }
}

use std::fmt::Debug;

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = match self.antigen {
            AB => write!(f, "AB"),
            A => write!(f, "A"),
            O => write!(f, "O"),
            B => write!(f, "B"),
        };
        match self.rh_factor {
            Negative => write!(f, "-"),
            _ => write!(f, "+"),
        }
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        Self::can_donate_to(other, self)
    }

    pub fn donors(&self) -> Vec<Self> {
        let all_types = vec![
            BloodType {
                antigen: O,
                rh_factor: Negative,
            },
            BloodType {
                antigen: O,
                rh_factor: Positive,
            },
            BloodType {
                antigen: A,
                rh_factor: Negative,
            },
            BloodType {
                antigen: A,
                rh_factor: Positive,
            },
            BloodType {
                antigen: B,
                rh_factor: Negative,
            },
            BloodType {
                antigen: B,
                rh_factor: Positive,
            },
            BloodType {
                antigen: AB,
                rh_factor: Negative,
            },
            BloodType {
                antigen: AB,
                rh_factor: Positive,
            },
        ];

        all_types
            .into_iter()
            .filter(|donor| self.can_receive_from(donor))
            .collect()
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let all_types = vec![
            BloodType {
                antigen: O,
                rh_factor: Negative,
            },
            BloodType {
                antigen: O,
                rh_factor: Positive,
            },
            BloodType {
                antigen: A,
                rh_factor: Negative,
            },
            BloodType {
                antigen: A,
                rh_factor: Positive,
            },
            BloodType {
                antigen: B,
                rh_factor: Negative,
            },
            BloodType {
                antigen: B,
                rh_factor: Positive,
            },
            BloodType {
                antigen: AB,
                rh_factor: Negative,
            },
            BloodType {
                antigen: AB,
                rh_factor: Positive,
            },
        ];

        all_types
            .into_iter()
            .filter(|recipient| recipient.can_receive_from(self))
            .collect()
    }

    fn can_donate_to(donor: &BloodType, recipient: &BloodType) -> bool {
        let antigen_ok = match (&donor.antigen, &recipient.antigen) {
            (O, _) => true,
            (A, A) | (A, AB) => true,
            (B, B) | (B, AB) => true,
            (AB, AB) => true,
            _ => false,
        };

        // Rh compatibility
        let rh_ok = match (&donor.rh_factor, &recipient.rh_factor) {
            (Negative, _) => true,
            (Positive, Positive) => true,
            _ => false,
        };

        antigen_ok && rh_ok
    }
}