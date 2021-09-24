use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

#[derive(Clone, Copy)]
pub enum Region {
    // guangzhou
    APGuangzhou1, // "ap-guangzhou-1"
    APGuangzhou2, // "ap-guangzhou-2"
    APGuangzhou3, // "ap-guangzhou-3"
    APGuangzhou4, // "ap-guangzhou-4"
    APGuangzhou6, // "ap-guangzhou-6"

    // shanghai
    APShanghai1, // "ap-shanghai-1"
    APShanghai2, // "ap-shanghai-2"
    APShanghai3, // "ap-shanghai-3"
    APShanghai4, // "ap-shanghai-4"
    APShanghai5, // "ap-shanghai-5"

    // nanjing
    APNanjing1, // "ap-nanjing-1"
    APNanjing2, // "ap-nanjing-2"

    // beijing
    APBeijing1, // "ap-beijing-1"
    APBeijing2, // "ap-beijing-2"
    APBeijing3, // "ap-beijing-3"
    APBeijing4, // "ap-beijing-4"
    APBeijing5, // "ap-beijing-5"
    APBeijing6, // "ap-beijing-6"
    APBeijing7, // "ap-beijing-7"

    // chengdu
    APChengdu1, // "ap-chengdu-1"
    APChengdu2, // "ap-chengdu-2"

    // chongqing
    APChongqing1, // "ap-chongqing-1"

    // hongkong
    APHongkong1, // "ap-hongkong-1"
    APHongkong2, // "ap-hongkong-2"
    APHongkong3, // "ap-hongkong-3"
}

impl Debug for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::APGuangzhou1 => write!(f, "ap-guangzhou-1"),
            Self::APGuangzhou2 => write!(f, "ap-guangzhou-2"),
            Self::APGuangzhou3 => write!(f, "ap-guangzhou-3"),
            Self::APGuangzhou4 => write!(f, "ap-guangzhou-4"),
            Self::APGuangzhou6 => write!(f, "ap-guangzhou-6"),
            Self::APShanghai1 => write!(f, "ap-shanghai-1"),
            Self::APShanghai2 => write!(f, "ap-shanghai-2"),
            Self::APShanghai3 => write!(f, "ap-shanghai-3"),
            Self::APShanghai4 => write!(f, "ap-shanghai-4"),
            Self::APShanghai5 => write!(f, "ap-shanghai-5"),
            Self::APNanjing1 => write!(f, "ap-nanjing-1"),
            Self::APNanjing2 => write!(f, "ap-nanjing-2"),
            Self::APBeijing1 => write!(f, "ap-beijing-1"),
            Self::APBeijing2 => write!(f, "ap-beijing-2"),
            Self::APBeijing3 => write!(f, "ap-beijing-3"),
            Self::APBeijing4 => write!(f, "ap-beijing-4"),
            Self::APBeijing5 => write!(f, "ap-beijing-5"),
            Self::APBeijing6 => write!(f, "ap-beijing-6"),
            Self::APBeijing7 => write!(f, "ap-beijing-7"),
            Self::APChengdu1 => write!(f, "ap-chengdu-1"),
            Self::APChengdu2 => write!(f, "ap-chengdu-2"),
            Self::APChongqing1 => write!(f, "ap-chongqing-1"),
            Self::APHongkong1 => write!(f, "ap-hongkong-1"),
            Self::APHongkong2 => write!(f, "ap-hongkong-2"),
            Self::APHongkong3 => write!(f, "ap-hongkong-3"),
        }
    }
}

impl AsRef<str> for Region {
    fn as_ref(&self) -> &str {
        match self {
            Region::APGuangzhou1 => "ap-guangzhou-1",
            Region::APGuangzhou2 => "ap-guangzhou-2",
            Region::APGuangzhou3 => "ap-guangzhou-3",
            Region::APGuangzhou4 => "ap-guangzhou-4",
            Region::APGuangzhou6 => "ap-guangzhou-6",
            Region::APShanghai1 => "ap-shanghai-1",
            Region::APShanghai2 => "ap-shanghai-2",
            Region::APShanghai3 => "ap-shanghai-3",
            Region::APShanghai4 => "ap-shanghai-4",
            Region::APShanghai5 => "ap-shanghai-5",
            Region::APNanjing1 => "ap-nanjing-1",
            Region::APNanjing2 => "ap-nanjing-2",
            Region::APBeijing1 => "ap-beijing-1",
            Region::APBeijing2 => "ap-beijing-2",
            Region::APBeijing3 => "ap-beijing-3",
            Region::APBeijing4 => "ap-beijing-4",
            Region::APBeijing5 => "ap-beijing-5",
            Region::APBeijing6 => "ap-beijing-6",
            Region::APBeijing7 => "ap-beijing-7",
            Region::APChengdu1 => "ap-chengdu-1",
            Region::APChengdu2 => "ap-chengdu-2",
            Region::APChongqing1 => "ap-chongqing-1",
            Region::APHongkong1 => "ap-hongkong-1",
            Region::APHongkong2 => "ap-hongkong-2",
            Region::APHongkong3 => "ap-hongkong-3",
        }
    }
}

impl FromStr for Region {
    type Err = String;

    fn from_str(region: &str) -> Result<Self, Self::Err> {
        match region {
            "ap-guangzhou-1" => Ok(Region::APGuangzhou1),
            "ap-guangzhou-2" => Ok(Region::APGuangzhou2),
            "ap-guangzhou-3" => Ok(Region::APGuangzhou3),
            "ap-guangzhou-4" => Ok(Region::APGuangzhou4),
            "ap-guangzhou-6" => Ok(Region::APGuangzhou6),
            "ap-shanghai-1" => Ok(Region::APShanghai1),
            "ap-shanghai-2" => Ok(Region::APShanghai2),
            "ap-shanghai-3" => Ok(Region::APShanghai3),
            "ap-shanghai-4" => Ok(Region::APShanghai4),
            "ap-shanghai-5" => Ok(Region::APShanghai5),
            "ap-nanjing-1" => Ok(Region::APNanjing1),
            "ap-nanjing-2" => Ok(Region::APNanjing2),
            "ap-beijing-1" => Ok(Region::APBeijing1),
            "ap-beijing-2" => Ok(Region::APBeijing2),
            "ap-beijing-3" => Ok(Region::APBeijing3),
            "ap-beijing-4" => Ok(Region::APBeijing4),
            "ap-beijing-5" => Ok(Region::APBeijing5),
            "ap-beijing-6" => Ok(Region::APBeijing6),
            "ap-beijing-7" => Ok(Region::APBeijing7),
            "ap-chengdu-1" => Ok(Region::APChengdu1),
            "ap-chengdu-2" => Ok(Region::APChengdu2),
            "ap-chongqing-1" => Ok(Region::APChongqing1),
            "ap-hongkong-1" => Ok(Region::APHongkong1),
            "ap-hongkong-2" => Ok(Region::APHongkong2),
            "ap-hongkong-3" => Ok(Region::APHongkong3),
            _ => Err(format!("unknown region: {}", region)),
        }
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn Debug).fmt(f)
    }
}

impl Default for Region {
    fn default() -> Self {
        Region::APBeijing1
    }
}
