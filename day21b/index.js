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

let left = calculate(map["root"].left);
let right = calculate(map["root"].right);

if (typeof left === "number") {
  const tmp = right;
  right = left;
  left = tmp;
}

while (left !== "x") {
  switch (left.op) {
    case "+":
      if (typeof left.left === "number") {
        right -= left.left;
        left = left.right;
      } else {
        right -= left.right;
        left = left.left;
      }
      break;
    case "-":
      if (typeof left.left === "number") {
        right = left.left - right;
        left = left.right;
      } else {
        right += left.right;
        left = left.left;
      }
      break;
    case "*":
      if (typeof left.left === "number") {
        right /= left.left;
        left = left.right;
      } else {
        right /= left.right;
        left = left.left;
      }
      break;
    case "/":
      if (typeof left.left === "number") {
        right = left.left / right;
        left = left.right;
      } else {
        right *= left.right;
        left = left.left;
      }
      break;
  }
}

console.log(right);

function calculate(monkey) {
  if (monkey === "humn") return "x";

  const job = map[monkey];
  if ("value" in job) return job.value;

  const left = calculate(job.left);
  const right = calculate(job.right);

  if (typeof left === "number" && typeof right === "number") {
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
  } else {
    return { left, right, op: job.op };
  }
}
