use std::f64::consts::PI;

/// 坐标
#[derive(Debug)]
pub struct Coordinate {
    /// 经度
    pub longitude: f64,
    /// 纬度
    pub latitude: f64,
    /// 弧度制经度
    pub longitude_radians: f64,
    /// 弧度制纬度
    pub latitude_radians: f64,
}

impl Coordinate {
    /// 创建一个新的 `Coordinate` 实例
    pub fn new(longitude: f64, latitude: f64) -> Self {
        Coordinate {
            longitude,
            latitude,
            longitude_radians: degree_to_radians(longitude),
            latitude_radians: degree_to_radians(latitude),
        }
    }
}

/// 方位偏移
#[derive(Debug)]
pub struct AzimuthOffset {
    /// 方位角（以度为单位）
    pub azimuth: f64,
    /// 距离（以米为单位）
    pub distance: f64,
}

impl AzimuthOffset {
    /// 创建一个新的 `AzimuthOffset` 实例
    pub fn new(azimuth: f64, distance: f64) -> Self {
        AzimuthOffset { azimuth, distance }
    }
}

/// 坐标偏移
#[derive(Debug)]
pub struct CoordinateOffset {
    /// 经度偏移
    pub longitude_offset: f64,
    /// 纬度偏移
    pub latitude_offset: f64,
}

impl CoordinateOffset {
    /// 创建一个新的 `CoordinateOffset` 实例
    pub fn new(longitude_offset: f64, latitude_offset: f64) -> Self {
        CoordinateOffset {
            longitude_offset,
            latitude_offset,
        }
    }
}

