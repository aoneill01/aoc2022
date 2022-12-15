const fs = require("fs");

const sensors = parseInput("input.txt");

for (let y = 0; y <= 4_000_000; y++) {
  const ranges = sensors
    .map((sensor) => rangeForRow(sensor, y))
    .filter(Boolean);
  const c = coverage(ranges);
  if (c.length > 1) {
    console.log(4_000_000 * (c[1].start - 1) + y);
  }
}

function parseInput(filename) {
  const regex = /Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)/;
  const data = fs.readFileSync(filename, "utf-8");

  return data
    .split("\n")
    .filter((line) => line != "")
    .map((line) => {
      const match = regex.exec(line);

      const result = {
        x: +match[1],
        y: +match[2],
        beacon: {
          x: +match[3],
          y: +match[4],
        },
      };

      result.distance =
        Math.abs(result.x - result.beacon.x) +
        Math.abs(result.y - result.beacon.y);

      return result;
    });
}

function rangeForRow(sensor, y) {
  const distance = Math.abs(sensor.y - y);

  if (distance > sensor.distance) return null;

  const result = {
    start: sensor.x - (sensor.distance - distance),
    distance: 2 * (sensor.distance - distance) + 1,
  };

  if (result.start > 4_000_000 || result.start + result.distance < 0)
    return null;

  return result;
}

function coverage(ranges) {
  ranges.sort((a, b) => a.start - b.start);
  const processed = [];
  let last = null;
  for (range of ranges) {
    if (last == null) {
      last = range;
      continue;
    }
    const comb = combined(last, range);
    if (comb) {
      last = comb;
    } else {
      processed.push(last);
      last = range;
    }
  }
  if (last) processed.push(last);

  return processed;
}

function combined(range1, range2) {
  if (range1.start > range2.start) {
    const tmp = range1;
    range1 = range2;
    range2 = tmp;
  }

  if (range1.start + range1.distance >= range2.start) {
    return {
      start: range1.start,
      distance: Math.max(
        range1.distance,
        range2.distance + range2.start - range1.start - 1
      ),
    };
  }

  return null;
}
