let times = [51699878]
let records = [377117112241505]
// ugly brute force
let res = 1;
function findRemainingRecordPossibilities(time, record) {
    // button_press_time [0..time] (=speed)
    // remaining_time = time - button_press_time
    // distance_traveled = speed * remaining_time = button_press_time * (time - button_press_time) =
    // = - button_press_time^2 + button_press_time * time  
    let distances = [];
    for (let button_press_time = 0; button_press_time <= time; button_press_time++) {
        distances.push(button_press_time* (time - button_press_time));       
    }
    distances = distances.filter(d => d > record)

    return distances.length;
}

for (let i = 0; i < times.length; i++) {
    res *= findRemainingRecordPossibilities(times[i], records[i])
}
console.log(res)