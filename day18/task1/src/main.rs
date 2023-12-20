fn main() {
    let contents = std::fs::read_to_string("input.txt")
    .expect("Should have been able to read file");
    
    let lines:Vec<Vec<&str>> = contents.lines()
        .map(|l| l.split(" ")
            .collect())
        .collect();
    

    let mut plans:Vec<(char,i32)> = Vec::new();
    for line in lines {
        plans.push((line[0].chars().next().unwrap() , line[1].parse::<i32>().unwrap()));
    }

    let mut vertices: Vec<(i32,i32)> = Vec::new();
    let mut border_length = 0;
    vertices.push((0,0));
    for (i, plan) in plans.iter().enumerate() {

        let dir = match plan.0 {
            'U' => (0,-1 * plan.1),
            'R' => (1 * plan.1, 0),
            'D' => (0,1 * plan.1) ,
            'L' => (-1 * plan.1, 0),
            _ => panic!("Invalid direction")
        };
        vertices.push((vertices[i].0 + dir.0, vertices[i].1 + dir.1));
        border_length += plan.1;
    }
    let area = calc_area(&vertices) + border_length / 2 + 1;
    dbg!(area);

}

fn calc_area(vertices: &Vec<(i32,i32)>)-> i32 {
    let mut area = 0;
    for i in 0..vertices.len()-1 {
        area += (vertices[i].1 + vertices[i + 1].1) * (vertices[i].0 - vertices[i + 1].0); 
    }
    area / 2
}