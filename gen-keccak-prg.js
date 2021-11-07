const fs = require('fs');
const SEP = '================================';
let t = fs
  .readFileSync('keccak-prg-output.txt', 'utf8')
  .split(SEP)
  .map((i) => i.trim());
//console.log('T', t);
const res = [];
for (let i of t) {
  i = i
    .split('\n\n')
    .filter((i) => !i.includes('Checksum'))
    .filter((i) => !i.includes('* Keccak with width'))
    .filter((i) => !i.includes('    - OK'));
  if (!i.length) continue;
  i = i[0].split('\n');
  const [init, input, output] = i;
  if (!init.startsWith('Keccak[')) console.log('WTF', i);
  const capacity = /Keccak\[r=\d+, c=(\d+)\]/.exec(init)[1];
  if (!input.startsWith('Input of')) console.log('WTF', i);
  const input2 = input.split(':')[1].trim().replace(/ /g, '');
  if (!output.startsWith('Output of')) console.log('WTF', i);
  const output2 = output.split(':')[1].trim().replace(/ /g, '');
  res.push({ capacity, input: input2, output: output2 });
}
console.log(JSON.stringify(res));
