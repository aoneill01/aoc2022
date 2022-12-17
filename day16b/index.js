const fs = require("fs");

const [map, hasRate] = parseInput("input.txt");

const result = pressure(map, ["AA", 0], ["AA", 0], hasRate, 26);
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

function pressure(map, a, b, unseen, remainingDays) {
  a = [...a];
  b = [...b];

  if (remainingDays <= 0) return 0;

  const sub = Math.min(a[1], b[1]);
  remainingDays -= sub;
  a[1] -= sub;
  b[1] -= sub;

  const [locationA, distanceA] = a;
  const [locationB, distanceB] = b;

  let result = 0;

  const valveA = map[locationA];
  const valveB = map[locationB];

  let nextAs = [a];
  let nextBs = [b];

  if (distanceA === 0) {
    result += valveA.rate * remainingDays;
    nextAs = [];
    for (const dest of unseen) {
      const remaining = remainingDays - (valveA.distance[dest] + 1);
      if (remaining > 0) {
        nextAs.push([dest, valveA.distance[dest] + 1]);
      }
    }
  }
  if (distanceB === 0) {
    result += valveB.rate * remainingDays;
    nextBs = [];
    for (const dest of unseen) {
      const remaining = remainingDays - (valveB.distance[dest] + 1);
      if (remaining > 0) {
        nextBs.push([dest, valveB.distance[dest] + 1]);
      }
    }
  }

  const candidates = [];

  if (nextAs.length === 0) {
    nextAs.push(["AA", 999]);
  }
  if (nextBs.length === 0) {
    nextBs.push(["AA", 999]);
  }

  for (const nextA of nextAs) {
    for (let nextB of nextBs) {
      if (nextB[0] === nextA[0]) {
        if (nextBs.length === 1) {
          nextB = ["AA", 999];
        } else {
          continue;
        }
      }
      const nextUnseen = unseen.filter((u) => u !== nextA[0] && u !== nextB[0]);
      candidates.push(pressure(map, nextA, nextB, nextUnseen, remainingDays));
    }
  }


  if (candidates.length === 0) return result;
  return result + Math.max(...candidates);
}
