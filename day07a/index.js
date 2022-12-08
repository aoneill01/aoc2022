const fs = require("fs");

const sum = (a, s) => a + s;

const root = {
  name: null,
  files: [],
  directories: [],
  parent: null,
};

let cwd = root;

const input = fs.readFileSync("input.txt", "utf8");
for (let line of input.split("\n")) {
  const match = /\$ cd (.*)/.exec(line);
  if (match) {
    const dirName = match[1];
    if (dirName === "..") {
      cwd = cwd.parent;
    } else if (dirName === "/") {
      cwd = root;
    } else {
      cwd = cwd.directories.find((d) => d.name === dirName);
    }
    continue;
  }

  if (line === "$ ls") {
    continue;
  }

  const [a, b] = line.split(" ");

  if (a === "dir") {
    cwd.directories.push({
      name: b,
      files: [],
      directories: [],
      parent: cwd,
    });
  } else {
    cwd.files.push({
      name: b,
      size: +a,
    });
  }
}

console.log(
  [...directories(root), root]
    .map((d) => size(d))
    .filter((s) => s < 100000)
    .reduce(sum, 0)
);

function* directories(directory) {
  for (const d1 of directory.directories) {
    for (const d2 of directories(d1)) {
      yield d2;
    }
    yield d1;
  }
}

function size(directory) {
  return (
    directory.directories.map((d) => size(d)).reduce(sum, 0) +
    directory.files.map((f) => f.size).reduce(sum, 0)
  );
}
