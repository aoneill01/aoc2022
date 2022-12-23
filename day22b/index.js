const { dir } = require("console");
const fs = require("fs");

const [mapInput, pathInput] = fs.readFileSync("input.txt", "utf-8").split("\n\n");

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

const a = {
  origin: [0, 50],
  map: side(0, 50),
};
const b = {
  origin: [0, 100],
  map: side(0, 100),
};
const c = {
  origin: [50, 50],
  map: side(50, 50),
};
const d = {
  origin: [100, 0],
  map: side(100, 0),
};
const e = {
  origin: [100, 50],
  map: side(100, 50),
};
const f = {
  origin: [150, 0],
  map: side(150, 0),
};

a.up = {
  square: f,
  rotate: 3,
};
a.right = {
  square: b,
  rotate: 0,
};
a.down = {
  square: c,
  rotate: 0,
};
a.left = {
  square: d,
  rotate: 2,
};

b.up = {
  square: f,
  rotate: 0,
};
b.right = {
  square: e,
  rotate: 2,
};
b.down = {
  square: c,
  rotate: 3,
};
b.left = {
  square: a,
  rotate: 0,
};

c.up = {
  square: a,
  rotate: 0,
};
c.right = {
  square: b,
  rotate: 1,
};
c.down = {
  square: e,
  rotate: 0,
};
c.left = {
  square: d,
  rotate: 1,
};

d.up = {
  square: c,
  rotate: 3,
};
d.right = {
  square: e,
  rotate: 0,
};
d.down = {
  square: f,
  rotate: 0,
};
d.left = {
  square: a,
  rotate: 2,
};

e.up = {
  square: c,
  rotate: 0,
};
e.right = {
  square: b,
  rotate: 2,
};
e.down = {
  square: f,
  rotate: 3,
};
e.left = {
  square: d,
  rotate: 0,
};

f.up = {
  square: d,
  rotate: 0,
};
f.right = {
  square: e,
  rotate: 1,
};
f.down = {
  square: b,
  rotate: 0,
};
f.left = {
  square: a,
  rotate: 1,
};

let face = a;
let row = 0;
let col = 0;
let direction = right;

let rotationTransform = 0;

for (const step of path) {
  if (typeof step === "number") {
    for (let i = 0; i < step; i++) {
      const relativeDirection = getRelativeDirection(direction, rotationTransform);
      let nextRow = row + relativeDirection.dRow;
      let nextCol = col + relativeDirection.dCol;
      let nextRotationTransformation = rotationTransform;
      let nextFace = face;
      if (nextRow < 0) {
        nextRotationTransformation += face.up.rotate;
        const [newRow, newCol] = getRelativeCoordinates(49, nextCol, face.up.rotate);
        nextFace = face.up.square;
        nextRow = newRow;
        nextCol = newCol;
      }
      if (nextRow > 49) {
        nextRotationTransformation += face.down.rotate;
        const [newRow, newCol] = getRelativeCoordinates(0, nextCol, face.down.rotate);
        nextFace = face.down.square;
        nextRow = newRow;
        nextCol = newCol;
      }
      if (nextCol < 0) {
        nextRotationTransformation += face.left.rotate;
        const [newRow, newCol] = getRelativeCoordinates(nextRow, 49, face.left.rotate);
        nextFace = face.left.square;
        nextRow = newRow;
        nextCol = newCol;
      }
      if (nextCol > 49) {
        nextRotationTransformation += face.right.rotate;
        const [newRow, newCol] = getRelativeCoordinates(nextRow, 0, face.right.rotate);
        nextFace = face.right.square;
        nextRow = newRow;
        nextCol = newCol;
      }
      if (nextFace.map[nextRow].charAt(nextCol) === "#") {
        break;
      } else {
        row = nextRow;
        col = nextCol;
        rotationTransform = nextRotationTransformation;
        face = nextFace;
      }
    }
  } else {
    direction = direction[step];
  }
}

console.log(
  1000 * (row + face.origin[0] + 1) +
    4 * (col + face.origin[1] + 1) +
    getRelativeDirection(direction, rotationTransform).value
);

function getRelativeDirection(direction, rotationTransform) {
  let result = direction;

  for (let i = 0; i < rotationTransform % 4; i++) {
    result = result.L;
  }

  return result;
}

function getRelativeCoordinates(row, col, rotationTransform) {
  let result = [row, col];

  for (let i = 0; i < rotationTransform % 4; i++) {
    result = rotateCoordinatesClockwise(result[0], result[1]);
  }

  return result;
}

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

function rotateCoordinatesClockwise(row, col) {
  return [49 - col, row];
}

function side(row, col) {
  const result = [];
  for (let r = 0; r < 50; r++) {
    result.push(map[row + r].substring(col, col + 50));
  }
  return result;
}
