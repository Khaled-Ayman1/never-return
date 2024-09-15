#[derive(Copy, Clone)]
pub struct Landmark{
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub is_inside: bool,
}

impl Landmark {
    pub fn new(id: i32, x: i32, y:i32, is_inside: bool) -> Landmark{
        Landmark{
            id,
            x,
            y,
            is_inside
        }
    }
}

/// <summary>
/// Find the shortest path from "goerge" to any of the landmarks that is outside the Honor Stone 
/// </summary>
/// <param name="landmarks">list of Landmarks, each with Id, x, y, IsInside </param>
/// <param name="trails">list of all trails, each consists of landmark1, landmark2, length</param>
/// <param name="N">number of landmarks</param>
/// <returns>value of the shortest path from goerge to outside </returns>
pub fn required_function(landmarks: Vec<Landmark>, trails: Vec<(i32, i32, i32)>, n: i32) -> i32{
    unimplemented!()

}