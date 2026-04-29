use std::error::Error;
use std::fmt;
use std::num::NonZeroUsize;

pub type ActionID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArmCount(NonZeroUsize);

impl ArmCount {
    pub const MAX_ARMS: usize = 1 << 20;

    fn positive(k: usize) -> Result<Self, ArmCountError> {
        let Some(k) = NonZeroUsize::new(k) else {
            return Err(ArmCountError::Zero);
        };

        if k.get() > Self::MAX_ARMS {
            return Err(ArmCountError::TooLarge {
                actual: k.get(),
                max: Self::MAX_ARMS,
            });
        }

        Ok(Self(k))
    }

    pub fn non_trivial(k: usize) -> Result<Self, ArmCountError> {
        let count = Self::positive(k)?;

        if count.get() < 2 {
            return Err(ArmCountError::TooSmall {
                actual: count.get(),
                min: 2,
            });
        }

        Ok(count)
    }

    pub fn get(self) -> usize {
        self.0.get()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmCountError {
    Zero,
    TooSmall { actual: usize, min: usize },
    TooLarge { actual: usize, max: usize },
}

impl fmt::Display for ArmCountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => {
                write!(f, "invalid arm count: must be positive, actual count is 0")
            }
            Self::TooSmall { actual, min } => {
                write!(
                    f,
                    "invalid arm count: non-trivial k-armed bandit requires count >= {min}, actual count is {actual}"
                )
            }
            Self::TooLarge { actual, max } => {
                write!(
                    f,
                    "invalid arm count: exceeds safety limit (supported max is {max}, actual count is {actual})"
                )
            }
        }
    }
}

impl Error for ArmCountError {}
