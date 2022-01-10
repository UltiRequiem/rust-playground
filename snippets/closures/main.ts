const multiplier = (factor: number) => (num: number) => num * factor;

const multiplyByTwo = multiplier(2);

for (let i = 2; i < 8; i++) {
  console.log(`The double of ${i} is ${multiplyByTwo(i)}.`);
}
