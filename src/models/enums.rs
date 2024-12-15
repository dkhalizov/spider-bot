use serde::Serialize;

#[derive(Debug, Clone, Copy)]
pub enum HealthStatus {
    Healthy = 1,
    Monitor = 2,
    Critical = 3,
}

impl HealthStatus {
    pub fn to_db_name(&self) -> &'static str {
        match self {
            HealthStatus::Healthy => "Healthy",
            HealthStatus::Monitor => "Monitor",
            HealthStatus::Critical => "Critical",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            HealthStatus::Healthy => "Normal health status with no concerns",
            HealthStatus::Monitor => "Requires extra attention and monitoring",
            HealthStatus::Critical => "Immediate attention required",
        }
    }
    pub fn from_id(id: i64) -> HealthStatus {
        match id {
            1 => HealthStatus::Healthy,
            2 => HealthStatus::Monitor,
            3 => HealthStatus::Critical,
            _ => HealthStatus::Healthy,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FeedingStatus {
    Accepted = 1,
    Rejected = 2,
    Partial = 3,
    PreMolt = 4,   
    Dead = 5,      
    Overflow = 6,   
}

impl FeedingStatus {
    pub fn to_db_name(&self) -> &'static str {
        match self {
            FeedingStatus::Accepted => "Accepted",
            FeedingStatus::Rejected => "Rejected",
            FeedingStatus::Partial => "Partial",
            FeedingStatus::PreMolt => "Pre-molt",
            FeedingStatus::Dead => "Dead",
            FeedingStatus::Overflow => "Overflow",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            FeedingStatus::Accepted => "Food was accepted normally",
            FeedingStatus::Rejected => "Food was rejected",
            FeedingStatus::Partial => "Only part of the food was consumed",
            FeedingStatus::PreMolt => "Refused food due to pre-molt state",
            FeedingStatus::Dead => "Prey died without being eaten",
            FeedingStatus::Overflow => "Too many prey items left in enclosure",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MoltStage {
    Normal = 1,
    PreMolt = 2,
    Molting = 3,
    PostMolt = 4,
    Failed = 5,     // New: Molt complications
}

impl MoltStage {
    pub fn to_db_name(&self) -> &'static str {
        match self {
            MoltStage::Normal => "Normal",
            MoltStage::PreMolt => "Pre-molt",
            MoltStage::Molting => "Molting",
            MoltStage::PostMolt => "Post-molt",
            MoltStage::Failed => "Failed",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            MoltStage::Normal => "Regular feeding/activity cycle",
            MoltStage::PreMolt => "Showing signs of upcoming molt",
            MoltStage::Molting => "Currently in molt",
            MoltStage::PostMolt => "Recently molted, needs time to harden",
            MoltStage::Failed => "Experiencing molt complications",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum CricketSize {
    Pinhead = 1,
    Small = 2,
    Medium = 3,
    Large = 4,
    Adult = 5,
    Unknown = 6,
}

impl CricketSize {
    pub fn to_db_name(&self) -> &'static str {
        match self {
            CricketSize::Pinhead => "Pinhead",
            CricketSize::Small => "Small",
            CricketSize::Medium => "Medium",
            CricketSize::Large => "Large",
            CricketSize::Adult => "Adult",
            CricketSize::Unknown => "Unknown",
        }
    }

    pub fn length_mm(&self) -> f32 {
        match self {
            CricketSize::Pinhead => 2.0,
            CricketSize::Small => 5.0,
            CricketSize::Medium => 10.0,
            CricketSize::Large => 15.0,
            CricketSize::Adult => 20.0,
            CricketSize::Unknown => 0.0,
        }
    }
}