const fs = require("fs");

const scans = fs
  .readFileSync("input.txt", "utf-8")
  .split("\n")
  .filter(Boolean)
  .map((line) => {
    const [x, y, z] = line.split(",");
    return [+x, +y, +z];
  });

const minX = Math.min(...scans.map(([x, ,]) => x));
const maxX = Math.max(...scans.map(([x, ,]) => x));
const minY = Math.min(...scans.map(([, y]) => y));
const maxY = Math.max(...scans.map(([, y]) => y));
const minZ = Math.min(...scans.map(([, , z]) => z));
const maxZ = Math.max(...scans.map(([, , z]) => z));

const map = [];
for (let i = 0; i < maxX - minX + 3; i++) {
  map[i] = [];
  for (let j = 0; j < maxY - minY + 3; j++) {
    map[i][j] = [];
    for (let k = 0; k < maxZ - minZ + 3; k++) {
      map[i][j][k] = 0;
    }
  }
}

for (const [x, y, z] of scans) {
  map[x - minX + 1][y - minY + 1][z - minZ + 1] = 1;
}

let result = fill(0, 0, 0);

console.log(result);

function fill(i0, j0, k0) {
  let result = 0;
  let stack = [[i0, j0, k0]];

  while (stack.length) {
    const [i, j, k] = stack.pop();
    if (map[i][j][k]) continue;
    map[i][j][k] = 2;

    if (i > 0) {
      const neighbor = map[i - 1][j][k];
      if (neighbor === 1) {
        result++;
      }
      if (neighbor === 0) {
        stack.push([i - 1, j, k]);
      }
    }
    if (i < maxX - minX + 2) {
      const neighbor = map[i + 1][j][k];
      if (neighbor === 1) {
        result++;
      }
      if (neighbor === 0) {
        stack.push([i + 1, j, k]);
      }
    }

    if (j > 0) {
      const neighbor = map[i][j - 1][k];
      if (neighbor === 1) {
        result++;
      }
      if (neighbor === 0) {
        stack.push([i, j - 1, k]);
      }
    }
    if (j < maxY - minY + 2) {
      const neighbor = map[i][j + 1][k];
      if (neighbor === 1) {
        result++;
      }
      if (neighbor === 0) {
        stack.push([i, j + 1, k]);
      }
    }

    if (k > 0) {
      const neighbor = map[i][j][k - 1];
      if (neighbor === 1) {
        result++;
      }
      if (neighbor === 0) {
        stack.push([i, j, k - 1]);
      }
    }
    if (k < maxZ - minZ + 2) {
      const neighbor = map[i][j][k + 1];
      if (neighbor === 1) {
        result++;
      }
      if (neighbor === 0) {
        stack.push([i, j, k + 1]);
      }
    }
  }

  return result;
}
