const fs = require("fs");

const [map, hasRate] = parseInput("input.txt");

const result = pressure(map, "AA", hasRate, 30);
console.log(result);

function parseInput(filename) {
  const regex =
    /Valve (\S+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (.*)/;

  const map = {};
  const hasRate = [];

  for (const line of fs
    .readFileSync(filename, "utf-8")
    .split("\n")
    .filter(Boolean)) {
    const match = regex.exec(line);

    const connectedTo = match[3].split(", ");

    map[match[1]] = {
      rate: +match[2],
      connectedTo,
      distance: {}
    };

    if (map[match[1]].rate) {
      hasRate.push(match[1]);
    }
  }

  for (const valve of hasRate) {
    setDistance(map, valve, valve, 0);
  }

  return [map, hasRate];
}

function setDistance(map, current, source, distance) {
  const c = map[current];

  if (!(source in c.distance) || distance < c.distance[source]) {
    c.distance[source] = distance;

    for (const connection of c.connectedTo) {
      setDistance(map, connection, source, distance + 1);
    }
  }
}

function pressure(map, location, unseen, remainingDays) {
  if (remainingDays <= 0) return 0;

  const curr = map[location];

  const candidates = [];

  for (const dest of unseen) {
    if (curr.distance[dest] + 1 >= remainingDays) continue;
    const rate = map[dest].rate;
    const updatedUnseen = unseen.filter((s) => s !== dest);
    const updatedRemaining = remainingDays - (curr.distance[dest] + 1);
    candidates.push(rate * updatedRemaining + pressure(map, dest, updatedUnseen, updatedRemaining));
  }

  if (candidates.length === 0) return 0;
  return Math.max(...candidates);
}
