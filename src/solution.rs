use std::{arch::x86_64, collections::HashMap, result};

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
    let mut storage: HashMap<i32, i32> = HashMap::new();
    storage.insert(0, 0);
    let mut d = vec![0; n as usize];

    for i in 1 .. n{
        d[i as usize] = i32::MAX;
        storage.insert(i, i32::MAX);
    }
    let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut edge_weights: HashMap<(i32,i32), i32> = HashMap::new();

    for tuple in trails{
        if is_further_euclidean(tuple.1, tuple.0, &landmarks){
            if !adj.contains_key(&tuple.0){
                adj.insert(tuple.0, Vec::new());
            }
            adj.entry(tuple.0).and_modify(|v: &mut Vec<i32>| v.push(tuple.1));
            edge_weights.insert((tuple.0, tuple.1), tuple.2);
        }
    }

    let mut min: i32 = i32::MAX;
    let mut result: i32 = i32::MAX;

    while storage.len() > 0{
        let mut u : i32 = -1;

        for key in storage.keys(){

            if let Some(&v) = storage.get(key){
                if v < min{
                    u = *key;
                    min = v;
                }
            }
        }
        if u == -1{
            break;
        }

        min = i32::MAX;
        storage.remove(&u);

        if adj.contains_key(&u){
            if let Some(v) = adj.get(&u){
                for &val in v{
                    if let Some(&edge_val) = edge_weights.get(&(u,val)){
                        if d[u as usize] + edge_val < d[val as usize]{
                            storage.entry(val).and_modify(|res:&mut i32| *res = d[u as usize] + edge_val);
                            d[val as usize] = d[u as usize] + edge_val;

                            if d[val as usize] < result{
                                result = d[val as usize];
                            }
                        }
                    }       
                } 
            }        
        }
    }

    result
}

fn is_further_euclidean(vert1:i32, vert2:i32, landmarks: &Vec<Landmark>) -> bool{

    let x1 = landmarks[vert1 as usize].x as f64;
    let y1 = landmarks[vert1 as usize].y as f64;
    let x2 = landmarks[vert2 as usize].x as f64;
    let y2 = landmarks[vert2 as usize].y as f64;
    let x  = landmarks[0].x as f64;
    let y  = landmarks[0].x as f64;

    let dist1: f64 = f64::sqrt((x1 - x).powf(2.0f64) + (y1 - y).powf(2.0f64));
    let dist2: f64 = f64::sqrt((x2 - x).powf(2.0f64) + (y2 - y).powf(2.0f64));

    dist2 < dist1

}