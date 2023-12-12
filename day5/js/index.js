const fs = require('fs');
const contents = fs.readFileSync("./input.txt", { encoding: 'utf8', flag: 'r' }).split(/\r?\n/);

const Mapping = (dst, src, len) => {
    return {
        dst,
        src,
        len
    }    
}

const Interval = (start, end) => {
    return {
        start,
        end
    }
}

let seeds = contents[0].split(" ");
seeds.shift(); // remove string
seeds = seeds.map(num => parseInt(num));
let seedIntervals = [];
for (let i = 0; i < seeds.length; i+=2) {
    seedIntervals.push(Interval(seeds[i], seeds[i+1]))
}
seedIntervals.sort((a,b) => a.start - b.start);

let mappings = [];
let currentLevel = []
let currentLevelIndex = 0;
for(let i = 3; i < contents.length; i++) {
    let line = contents[i];
    if(line == "")  {
        mappings.push(currentLevel);
        currentLevelIndex++;
        currentLevel = [];
        i++;
        continue;
    }
    line = line.split(" ").map(num => parseInt(num))
    currentLevel.push(Mapping(...line));
}

mappings.forEach(level => {
    level.sort((a,b) => a.src - b.src);

    // Add 1 to 1 mapping to unmapped areas 
    // This did nothing.. there were no unmapped areas.. 
    let lastEndIndex = 0;
    for (let i = 0; i < level.length; i++) {
        let mapping = level[i];
        if(mapping.start - lastEndIndex > 0) {
            mappings.splice(i, 0, Mapping(lastEndIndex, lastEndIndex, mapping.start - lastEndIndex))
            lastEndIndex = mapping.start;
        } 
    }

});



let locationIntervals = [];
function _findLocationIntervals(interval, level) {
    if(level == mappings.length) {
        locationIntervals.push(interval)
        return
    }

    // Find an interval containing s
    let s = interval.start;
    let currentInterval;
    
    while (s != interval.end) {
        // Find an interval containing s, 
        let mapping = mappings[level].find(map => s >= map.src && s < map.src + map.len);
        if(mapping == undefined) mapping = Mapping(s, s, interval.end) // ??
        let m = Math.min(mapping.src + mapping.len, interval.end);
        currentInterval = Interval(s, m);
        
        // we need to map the values of currentInterval to next levl
        let offset = s - mapping.src;
        currentInterval.start = mapping.dst + offset;  
        currentInterval.end = mapping.dst + offset + mapping.len; 
        _findLocationIntervals(currentInterval, level + 1); 
         
        s = m;    
    }

}

function findLocationIntervals(seedInterval) {
    _findLocationIntervals(seedInterval, 0);
}

seedIntervals.forEach(seedInterval => {
    findLocationIntervals(seedInterval);
})

locationIntervals.sort((a,b) => a.start - b.start)
console.log(locationIntervals)

// 11554135 worked as a solution, but why?