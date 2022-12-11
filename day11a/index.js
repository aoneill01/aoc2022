const fs = require("fs");

function parseInput() {
  const opRegex = /new = old (.) (.+)/;
  const input = fs.readFileSync("input.txt", "utf8").split("\n");
  const monkeys = [];

  for (let i = 0; i < input.length; i += 7) {
    const id = parseInt(input[i].split(" ")[1].split(":")[0], 10);
    const starting = input[i + 1]
      .split(": ")[1]
      .split(", ")
      .map((worry) => parseInt(worry, 10));
    let operation = undefined;
    let lastValue = undefined;
    const opMatch = opRegex.exec(input[i + 2]);
    switch (opMatch[2]) {
      case "old":
        lastValue = (old) => old;
        break;
      default:
        lastValue = (old) => parseInt(opMatch[2]);
        break;
    }
    switch (opMatch[1]) {
      case "+":
        operation = (old) => old + lastValue(old);
        break;
      case "*":
        operation = (old) => old * lastValue(old);
        break;
    }
    const testDivisible = parseInt(input[i + 3].split("by ")[1]);
    const testTrue = parseInt(input[i + 4].split("monkey ")[1]);
    const testFalse = parseInt(input[i + 5].split("monkey ")[1]);

    monkeys.push({
      inspectionCount: 0,
      worries: starting,
      operation,
      nextMonkey: (worry) =>
        worry % testDivisible === 0 ? testTrue : testFalse,
    });
  }

  return monkeys;
}

function processMonkey(monkeys, id) {
  const monkey = monkeys[id];

  while (monkey.worries.length) {
    const old = monkey.worries.shift();
    let updated = monkey.operation(old);
    updated = Math.floor(updated / 3);
    const throwTo = monkey.nextMonkey(updated);
    monkeys[throwTo].worries.push(updated);
    monkey.inspectionCount++;
  }
}

const monkeys = parseInput();
for (let i = 0; i < 20; i++) {
  for (const id in monkeys) {
    processMonkey(monkeys, id);
  }
}

const [m1, m2] = monkeys
  .map((monkey) => monkey.inspectionCount)
  .sort((a, b) => b - a);
console.log(m1 * m2);
