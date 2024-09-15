use solution::Landmark;

mod solution;

fn print_case(landmarks: Vec<Landmark>, trails: Vec<(i32, i32, i32)>, expected: i32, output: i32){
    println!("Landmarks: ");

    for i in 0..landmarks.len(){
        println!("ID= {}, X= {}, Y= {}, Inside= {}", landmarks[i].id, landmarks[i].x, landmarks[i].y, landmarks[i].is_inside);
    }

    for i in 0..trails.len(){
        println!("{} {} {}", trails[i].0, trails[i].1, trails[i].2)
    }
    println!("Output: {}", output);
    println!("Expected: {}", expected);
    if expected == output{
        println!("CORRECT");
    }else{
        println!("WRONG");
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import functions from outer scope
    use std::fs::File;
    use std::io::{self, BufReader, BufRead};
    use std::time::Instant;
    #[test]
    fn run_all_tests_sequentially() {
        trial_tests();
        sample_tests().unwrap();
        complete_tests().unwrap();
    }

    fn trial_tests(){
        {
            let n = 5;
            let mut landmarks: Vec<solution::Landmark> = Vec::new();

            landmarks.push(Landmark::new(0, -54, 4, true));
            landmarks.push(Landmark::new(1, 1885, -2334, true));
            landmarks.push(Landmark::new(2, -524, 3047, true));
            landmarks.push(Landmark::new(3, -1179, 6405, false));
            landmarks.push(Landmark::new(4, -31, 4127, false));

            let mut trails: Vec<(i32, i32, i32)> = Vec::new();

            trails.push((0, 1, 3245));
            trails.push((1, 2, 5614));
            trails.push((0, 2, 2367));
            trails.push((2, 4, 9259));
            trails.push((2, 3, 8561));
            trails.push((1, 3, 1792));
            trails.push((3, 4, 4241));

            let expected = 5037;
            let output = solution::required_function(landmarks.clone(), trails.clone(), n);
            print_case(landmarks, trails, expected, output);
        }

        {
            let n = 6;
            let mut landmarks: Vec<solution::Landmark> = Vec::new();

            landmarks.push(Landmark::new(0, 0, 97, true));
            landmarks.push(Landmark::new(1, 1738, 8252, true));
            landmarks.push(Landmark::new(2, -6434, -2962, true));
            landmarks.push(Landmark::new(3, -1033, 3411, true));
            landmarks.push(Landmark::new(4, 15110, 12691, false));
            landmarks.push(Landmark::new(5, -14106, -16055, false));

            let mut trails: Vec<(i32, i32, i32)> = Vec::new();

            trails.push((0, 3, 233));
            trails.push((0, 2, 185));
            trails.push((2, 3, 8838));
            trails.push((1, 2, 6402));
            trails.push((0, 1, 843));
            trails.push((2, 4, 913));
            trails.push((3, 5, 9041));
            trails.push((4, 5, 1370));

            let expected = 1098;
            let output = solution::required_function(landmarks.clone(), trails.clone(), n);
            print_case(landmarks, trails, expected, output);
        }

        {
            let n = 4;
            let mut landmarks: Vec<solution::Landmark> = Vec::new();

            landmarks.push(Landmark::new(0, -60, -1, true));
            landmarks.push(Landmark::new(1, 3499, 5087, true));
            landmarks.push(Landmark::new(2, -1112, -4096, true));
            landmarks.push(Landmark::new(3, 8898, 4355, false));
            
            let mut trails: Vec<(i32, i32, i32)> = Vec::new();

            trails.push((0, 2, 9536));
            trails.push((1, 2, 1526));
            trails.push((0, 1, 2557));
            trails.push((0, 3, 9877));
            trails.push((1, 3, 3187));
            trails.push((2, 3, 5762));

            let expected = 5744;
            let output = solution::required_function(landmarks.clone(), trails.clone(), n);
            print_case(landmarks, trails, expected, output);
        }
        
    }

    fn sample_tests() -> io::Result<()> {
        let file = File::open("tests\\NeverRetrun_Easy.txt")?;
        let mut reader = BufReader::new(file);
    
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let test_cases= line[0..1].parse::<i32>().unwrap();
    
        let mut tl = 0;
        let mut corr = 0;
        let mut wng = 0;
    
        for i in 0..test_cases {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            let line_parts: Vec<&str> = line.trim().split('\r').collect();
            let vertecies = line_parts[0].parse::<i32>().unwrap();
    
            let mut line = String::new();
            reader.read_line(&mut line)?;
            let line_parts: Vec<&str> = line.trim().split('\r').collect();
            let edges = line_parts[0].parse::<i32>().unwrap();
    
            let mut landmarks: Vec<Landmark> = Vec::new();
            
            for _v in 0..vertecies {
                let mut line = String::new();
                reader.read_line(&mut line)?;
                let line_parts: Vec<&str> = line.trim().split(',').collect();
                
                let landmark = match (
                    line_parts[0].parse::<i32>().unwrap(),
                    line_parts[1].parse::<i32>().unwrap(),
                    line_parts[2].parse::<i32>().unwrap(),
                    line_parts[3].to_string()
                ) {
                    (x, y, z, is_active) => if is_active == "True"{ Landmark::new(x, y, z, true) }else{Landmark::new(x, y, z, false)},
                };
                landmarks.push(landmark);
            }
    
            let mut trails: Vec<(i32, i32, i32)> = Vec::new();
    
            for _e in 0..edges {
                let mut line = String::new();
                reader.read_line(&mut line)?;
                let line_parts: Vec<&str> = line.trim().split(',').collect();
                let trail = (
                    line_parts[0].parse::<i32>().unwrap(),
                    line_parts[1].parse::<i32>().unwrap(),
                    line_parts[2].parse::<i32>().unwrap()
                );
                trails.push(trail);
            }
    
            let mut line = String::new();
            reader.read_line(&mut line)?;
            let line_parts: Vec<&str> = line.trim().split('\r').collect();
            let actual_result = line_parts[0].parse::<i32>().unwrap();
    
            let mut line = String::new();
            reader.read_line(&mut line)?;
            let line_parts: Vec<&str> = line.trim().split(':').collect();
            let time_limit = line_parts[1].parse::<i32>().unwrap();
                
    
            let start = Instant::now();
            let result = solution::required_function(landmarks, trails, vertecies);
            let end = Instant::now();
            let elapsed_time = end - start;
    
            if elapsed_time.as_secs() > time_limit as u64 {
                tl += 1;
                println!("Time Limit Exceeded");
            } else if result == actual_result {
                corr += 1;
                println!("Test Case {} Passed!", i + 1);
            } else {
                wng += 1;
                println!("Wrong Answer in Case {}", i + 1);
            }
        }
    
        println!("# correct = {}", corr);
        println!("# time limit = {}", tl);
        println!("# wrong = {}", wng);
        println!("\nFINAL EVALUATION (%) = {}", 100 * (corr / test_cases));
    
        Ok(())
    }

    fn complete_tests() -> io::Result<()>{
        let file = File::open("tests\\NeverRetrun_Hard.txt")?;
        let mut reader = BufReader::new(file);
    
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let test_cases= line[0..1].parse::<i32>().unwrap();
    
        let mut tl = 0;
        let mut corr = 0;
        let mut wng = 0;
    
        for i in 0..test_cases {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            let line_parts: Vec<&str> = line.trim().split('\r').collect();
            let vertecies = line_parts[0].parse::<i32>().unwrap();
    
            let mut line = String::new();
            reader.read_line(&mut line)?;
            let line_parts: Vec<&str> = line.trim().split('\r').collect();
            let edges = line_parts[0].parse::<i32>().unwrap();
    
            let mut landmarks: Vec<Landmark> = Vec::new();
            
            for _v in 0..vertecies {
                let mut line = String::new();
                reader.read_line(&mut line)?;
                let line_parts: Vec<&str> = line.trim().split(',').collect();
                
                let landmark = match (
                    line_parts[0].parse::<i32>().unwrap(),
                    line_parts[1].parse::<i32>().unwrap(),
                    line_parts[2].parse::<i32>().unwrap(),
                    line_parts[3].to_string()
                ) {
                    (x, y, z, is_active) => if is_active == "True"{ Landmark::new(x, y, z, true) }else{Landmark::new(x, y, z, false)},
                };
                landmarks.push(landmark);
            }
    
            let mut trails: Vec<(i32, i32, i32)> = Vec::new();
    
            for _e in 0..edges {
                let mut line = String::new();
                reader.read_line(&mut line)?;
                let line_parts: Vec<&str> = line.trim().split(',').collect();
                let trail = (
                    line_parts[0].parse::<i32>().unwrap(),
                    line_parts[1].parse::<i32>().unwrap(),
                    line_parts[2].parse::<i32>().unwrap()
                );
                trails.push(trail);
            }
    
            let mut line = String::new();
            reader.read_line(&mut line)?;
            let line_parts: Vec<&str> = line.trim().split('\r').collect();
            let actual_result = line_parts[0].parse::<i32>().unwrap();
    
            let mut line = String::new();
            reader.read_line(&mut line)?;
            let line_parts: Vec<&str> = line.trim().split(':').collect();
            let time_limit = line_parts[1].parse::<i32>().unwrap();
    
            let start = Instant::now();
            let result = solution::required_function(landmarks, trails, vertecies);
            let end = Instant::now();
            let elapsed_time = end - start;
    
            if elapsed_time.as_secs() > time_limit as u64 {
                tl += 1;
                println!("Time Limit Exceeded");
            } else if result == actual_result {
                corr += 1;
                println!("Test Case {} Passed!", i + 1);
            } else {
                wng += 1;
                println!("Wrong Answer in Case {}", i + 1);
            }
        }
    
        println!("# correct = {}", corr);
        println!("# time limit = {}", tl);
        println!("# wrong = {}", wng);
        println!("\nFINAL EVALUATION (%) = {}", 100 * (corr / test_cases));
    
        Ok(())
    }
}


fn main() {
    println!("hello world")
}
