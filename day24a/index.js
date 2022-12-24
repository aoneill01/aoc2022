const fs = require("fs");

const lines = fs.readFileSync("input.txt", "utf-8").trim().split("\n");
const height = lines.length - 2;
const width = lines[0].length - 2;
const blizzards = [];

for (let row = 0; row < height; row++) {
  for (let col = 0; col < width; col++) {
    const cell = lines[row + 1][col + 1];
    switch (cell) {
      case ">":
        blizzards.push({
          row,
          col,
          direction: {
            dRow: 0,
            dCol: 1,
          },
        });
        break;
      case "v":
        blizzards.push({
          row,
          col,
          direction: {
            dRow: 1,
            dCol: 0,
          },
        });
        break;
      case "<":
        blizzards.push({
          row,
          col,
          direction: {
            dRow: 0,
            dCol: -1,
          },
        });
        break;
      case "^":
        blizzards.push({
          row,
          col,
          direction: {
            dRow: -1,
            dCol: 0,
          },
        });
        break;
    }
  }
}

const magic = 1_000;

function locationToNumber(row, col) {
  return row * magic + col;
}

function numberToLocation(n) {
  return [Math.floor(n / magic), n % magic];
}

let possibleLocations = new Set();
possibleLocations.add(locationToNumber(-1, 0));
const moves = [
  { dRow: 0, dCol: 0 },
  { dRow: 1, dCol: 0 },
  { dRow: -1, dCol: 0 },
  { dRow: 0, dCol: 1 },
  { dRow: 0, dCol: -1 },
];

for (let minute = 1; ; minute++) {
  let blizzardLocations = new Set();
  // Move blizzards
  for (const blizzard of blizzards) {
    let row = blizzard.row + blizzard.direction.dRow;
    let col = blizzard.col + blizzard.direction.dCol;
    if (row < 0) row = height - 1;
    if (row >= height) row = 0;
    if (col < 0) col = width - 1;
    if (col >= width) col = 0;
    blizzard.row = row;
    blizzard.col = col;
    blizzardLocations.add(locationToNumber(row, col));
  }

  const nextPossibleLocations = new Set();
  // move possible locations
  for (const n of possibleLocations) {
    const [row, col] = numberToLocation(n);

    for (const move of moves) {
      const newRow = row + move.dRow;
      const newCol = col + move.dCol;

      if (canMove(newRow, newCol, blizzardLocations)) {
        nextPossibleLocations.add(locationToNumber(newRow, newCol));
      }
    }
  }

  possibleLocations = nextPossibleLocations;

  if (possibleLocations.has(locationToNumber(height, width - 1))) {
    console.log(minute);
    break;
  }
}

function canMove(row, col, blizzardLocations) {
  if (row < 0) {
    return row === -1 && col === 0;
  }
  if (row >= height) {
    return row === height && col === width - 1;
  }
  if (col < 0 || col >= width) return false;
  return !blizzardLocations.has(locationToNumber(row, col));
}
