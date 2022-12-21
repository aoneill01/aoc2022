const fs = require("fs");

const map = fs
  .readFileSync("input.txt", "utf-8")
  .trim()
  .split("\n")
  .reduce((map, line) => {
    const [key, value] = line.split(": ");
    const match = /(.+) (.) (.+)/.exec(value);
    if (match) {
      map[key] = {
        left: match[1],
        right: match[3],
        op: match[2],
      };
    } else {
      map[key] = {
        value: +value,
      };
    }
    return map;
  }, {});

console.log(calculate("root"));

function calculate(monkey) {
  const job = map[monkey];
  if ("value" in job) return job.value;

  const left = calculate(job.left);
  const right = calculate(job.right);

  switch (job.op) {
    case "+":
      return left + right;
    case "-":
      return left - right;
    case "*":
      return left * right;
    case "/":
      return left / right;
  }
}
