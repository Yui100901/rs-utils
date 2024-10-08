use std::f64::consts::PI;
use crate::geo_utils::coordinate::{AzimuthOffset, Coordinate, CoordinateOffset};

/// 地球平均半径
const EARTH_RADIUS: f64 = 6371_393.0;

/// 传入点p，方向角正北为0（以度为单位），移动的距离（以米为单位）
pub fn exec_offset(p: &Coordinate, cf: &CoordinateOffset) -> Coordinate {
    println!("当前位置：{}, {}", p.longitude, p.latitude);

    let new_longitude = p.longitude + cf.longitude_offset;
    let new_latitude = p.latitude + cf.latitude_offset;
    println!("偏移位置：{}, {}", new_longitude, new_latitude);
    Coordinate::new(new_longitude, new_latitude)
}

/// 计算坐标偏移
pub fn calc_coordinate_offset(c: &Coordinate, of: &AzimuthOffset) -> CoordinateOffset {
    // 方向角度转换为弧度
    let azimuth_radians = degree_to_radians(of.azimuth);
    // 纬度转换成弧度
    let latitude_radians = degree_to_radians(c.latitude);
    // 单位经度距离
    let unit_longitude_distance = 2.0 * PI * latitude_radians.cos() * EARTH_RADIUS / 360.0;
    // 单位纬度距离
    let unit_latitude_distance = 2.0 * PI * EARTH_RADIUS / 360.0;
    // 单位距离的经度变化量
    let unit_delta_longitude = azimuth_radians.sin() / unit_longitude_distance;
    // 单位距离的纬度变化量
    let unit_delta_latitude = azimuth_radians.cos() / unit_latitude_distance;

    // 经度偏移
    let delta_longitude = unit_delta_longitude * of.distance;
    // 纬度偏移
    let delta_latitude = unit_delta_latitude * of.distance;

    CoordinateOffset::new(delta_longitude, delta_latitude)
}

/// 将度数转换为弧度
pub fn degree_to_radians(degree: f64) -> f64 {
    degree * PI / 180.0
}

/// 将弧度转换为度数
pub fn radians_to_degree(radians: f64) -> f64 {
    radians * 180.0 / PI
}

/// Haversine公式，计算两点间距离
pub fn haversine(p1: &Coordinate, p2: &Coordinate) -> f64 {
    // 经度变化量
    let d_longitude_radians = p2.longitude_radians - p1.longitude_radians;
    // 纬度变化量
    let d_latitude_radians = p2.latitude_radians - p1.latitude_radians;

    let a = (d_latitude_radians / 2.0).sin().powi(2)
        + degree_to_radians(p1.latitude).cos()
        * degree_to_radians(p2.latitude).cos()
        * (d_longitude_radians / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS * c
}