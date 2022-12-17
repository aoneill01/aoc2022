const fs = require("fs");

const map = parseInput("input.txt");

const result = pressure(map, "AA", null, new Set(), 30);
console.log(result);

function parseInput(filename) {
  const regex =
    /Valve (\S+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (.*)/;

  const map = {};

  for (const line of fs
    .readFileSync(filename, "utf-8")
    .split("\n")
    .filter(Boolean)) {
    const match = regex.exec(line);

    const connectedTo = match[3]
      .split(", ")
      .map((valve) => ({ valve, distance: 1 }));

    map[match[1]] = {
      rate: +match[2],
      connectedTo,
    };
  }

  for (const valve of Object.keys(map)) {
    if (map[valve].rate === 0 && valve !== "AA") {
      for (const connection of map[valve].connectedTo) {
        const sourceValve = connection.valve;
        const tmp = map[sourceValve].connectedTo.find((v) => v.valve === valve);
        map[sourceValve].connectedTo = map[sourceValve].connectedTo
          .filter((v) => v.valve !== valve)
          .concat(
            map[valve].connectedTo
              .filter((v) => v.valve !== sourceValve)
              .map((v) => ({
                valve: v.valve,
                distance: tmp.distance + v.distance,
              }))
          );
      }
      delete map[valve];
    }
  }

  return map;
}

function pressure(map, location, previous, seen, remainingDays) {
  if (remainingDays <= 0) return 0;

  const curr = map[location];

  const candidates = [];
  if (curr.rate && !seen.has(location)) {
    const newSeen = new Set(seen);
    newSeen.add(location);
    candidates.push(
      curr.rate * (remainingDays - 1) +
        pressure(map, location, null, newSeen, remainingDays - 1)
    );
  }
  for (const connection of curr.connectedTo.filter(
    (c) => c.valve !== previous
  )) {
    candidates.push(
      pressure(
        map,
        connection.valve,
        location,
        seen,
        remainingDays - connection.distance
      )
    );
  }

  return Math.max(...candidates);
}
