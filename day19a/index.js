const fs = require("fs");

const blueprints = fs
  .readFileSync("input.txt", "utf-8")
  .split("\n")
  .filter(Boolean)
  .map(parseBlueprint);

let qualityLevels = 0;
for (const blueprint of blueprints) {
  const result = collectGeodes(
    blueprint,
    {
      ore: 0,
      clay: 0,
      obsidian: 0,
      geodes: 0,
      oreRobots: 1,
      clayRobots: 0,
      obsidianRobots: 0,
      geodeRobots: 0,
    },
    24
  );
  console.log(result);
  qualityLevels += result.geodes * blueprint.id;
}
console.log(qualityLevels);

function parseBlueprint(line) {
  const regex =
    /Blueprint (.+): Each ore robot costs (.+) ore. Each clay robot costs (.+) ore. Each obsidian robot costs (.+) ore and (.+) clay. Each geode robot costs (.+) ore and (.+) obsidian./;
  const match = regex.exec(line);
  return {
    id: +match[1],
    oreRobot: {
      ore: +match[2],
    },
    clayRobot: {
      ore: +match[3],
    },
    obsidianRobot: {
      ore: +match[4],
      clay: +match[5],
    },
    geodeRobot: {
      ore: +match[6],
      obsidian: +match[7],
    },
  };
}

function collectGeodes(blueprint, resources, daysLeft) {
  const nextResources = { ...resources };
  const candidates = [];

  let purchasedOre = resources.ore > 25;
  let purchasedClay = resources.clay > 25;
  let purchasedObsidian = resources.obsidian > 25;
  let purchasedGeodes = false;
  for (let nextDaysLeft = daysLeft; nextDaysLeft > 0; nextDaysLeft--) {
    if (!purchasedOre) {
      const x = next(nextResources, blueprint.oreRobot);
      if (x) {
        x.oreRobots++;
        candidates.push(collectGeodes(blueprint, x, nextDaysLeft - 1));
        purchasedOre = true;
      }
    }
    if (!purchasedClay) {
      const x = next(nextResources, blueprint.clayRobot);
      if (x) {
        x.clayRobots++;
        candidates.push(collectGeodes(blueprint, x, nextDaysLeft - 1));
        purchasedClay = true;
      }
    }
    if (!purchasedObsidian) {
      const x = next(nextResources, blueprint.obsidianRobot);
      if (x) {
        x.obsidianRobots++;
        candidates.push(collectGeodes(blueprint, x, nextDaysLeft - 1));
        purchasedObsidian = true;
      }
    }
    if (!purchasedGeodes) {
      const x = next(nextResources, blueprint.geodeRobot);
      if (x) {
        x.geodeRobots++;
        candidates.push(collectGeodes(blueprint, x, nextDaysLeft - 1));
        purchasedGeodes = true;
      }
    }

    nextResources.ore += resources.oreRobots;
    nextResources.clay += resources.clayRobots;
    nextResources.obsidian += resources.obsidianRobots;
    nextResources.geodes += resources.geodeRobots;
  }

  candidates.push(nextResources);

  return candidates.reduce((max, curr) =>
    curr.geodes > max.geodes ? curr : max
  );
}

function next(resources, cost) {
  if (
    resources.ore >= (cost?.ore ?? 0) &&
    resources.clay >= (cost?.clay ?? 0) &&
    resources.obsidian >= (cost?.obsidian ?? 0)
  ) {
    const spent = { ...resources };
    spent.ore -= cost?.ore ?? 0;
    spent.clay -= cost?.clay ?? 0;
    spent.obsidian -= cost?.obsidian ?? 0;
    spent.ore += resources.oreRobots;
    spent.clay += resources.clayRobots;
    spent.obsidian += resources.obsidianRobots;
    spent.geodes += resources.geodeRobots;
    return spent;
  }
}
