const { dir } = require("console");
const fs = require("fs");

const [mapInput, pathInput] = fs
  .readFileSync("input.txt", "utf-8")
  .split("\n\n");

const map = mapInput.split("\n");
const path = parsePath(pathInput.trim());

const right = {
  dCol: 1,
  dRow: 0,
  value: 0,
};
const down = {
  dCol: 0,
  dRow: 1,
  value: 1,
};
const left = {
  dCol: -1,
  dRow: 0,
  value: 2,
};
const up = {
  dCol: 0,
  dRow: -1,
  value: 3,
};
right.R = down;
right.L = up;
down.R = left;
down.L = right;
left.R = up;
left.L = down;
up.R = right;
up.L = left;

let row = 1;
let col = 1;
let direction = right;
while (map[row - 1].charAt(col - 1) !== ".") col++;

for (const step of path) {
  if (typeof step === "number") {
    for (let i = 0; i < step; i++) {
      const [nextRow, nextCol] = nextInDirection(row, col, direction);
      if (map[nextRow - 1].charAt(nextCol - 1) == "#") break;
      row = nextRow;
      col = nextCol;
    }
  } else {
    direction = direction[step];
  }
}

console.log(1000 * row + 4 * col + direction.value);

function parsePath(pathInput) {
  const numbers = pathInput.split(/[RL]/).map((n) => +n);
  const turns = pathInput.split(/\d+/).filter(Boolean);

  const result = [];

  for (let i = 0; i < numbers.length; i++) {
    result.push(numbers[i]);
    if (i < turns.length) result.push(turns[i]);
  }

  return result;
}

function nextInDirection(row, col, direction) {
  while (true) {
    let nextRow = row + direction.dRow;
    let nextCol = col + direction.dCol;

    if (nextRow > map.length) {
      nextRow = 1;
    }
    if (nextRow < 1) {
      nextRow = map.length;
    }
    if (nextCol > map[nextRow - 1].length) {
      nextCol = 1;
    }
    if (nextCol < 1) {
      nextCol = map[nextRow - 1].length;
    }

    const value = map[nextRow - 1].charAt(nextCol - 1);
    if (value === "." || value === "#") return [nextRow, nextCol];
    row = nextRow;
    col = nextCol;
  }
}
