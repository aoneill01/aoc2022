const fs = require("fs");

const numbers = fs
  .readFileSync("input.txt", "utf-8")
  .trim()
  .split("\n")
  .map((n) => +n);

let zero = null;
let originalOrder = [];
let first = null;
let previous = null;

for (const number of numbers) {
  if (first === null) {
    first = previous = {
      value: number,
      previous: null,
      next: null,
    };
    originalOrder.push(first);
  } else {
    const curr = {
      value: number,
      previous,
      next: null,
    };
    previous.next = curr;
    originalOrder.push(curr);
    previous = curr;
  }
  if (number === 0) zero = previous;
}
first.previous = previous;
previous.next = first;

const length = originalOrder.length;

for (const pointer of originalOrder) {
  pointer.previous.next = pointer.next;
  pointer.next.previous = pointer.previous;

  let insertAfter = pointer.previous;

  for (let i = 0; i < Math.abs(pointer.value) % (length - 1); i++) {
    if (pointer.value < 0) {
      insertAfter = insertAfter.previous;
    } else {
      insertAfter = insertAfter.next;
    }
  }

  insertAfter.next.previous = pointer;
  pointer.next = insertAfter.next;
  pointer.previous = insertAfter;
  insertAfter.next = pointer;
}

console.log(nthValue(1000) + nthValue(2000) + nthValue(3000));

function nthValue(n) {
  let pointer = zero;
  for (let i = 0; i < n % length; i++) {
    pointer = pointer.next;
  }
  return pointer.value;
}
