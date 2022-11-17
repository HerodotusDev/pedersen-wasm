import {
  pedersen,
  starknet_pedersen,
  starknet_pedersen_cairo,
} from "./pkg/pedersen_wasm.js";
import { toBufferLE } from "bigint-buffer";
import { pedersen as starknetJsPedersen } from "starknet/dist/utils/hash";

enum HashType {
  PEDERSEN = "pedersen",
  KECCAK = "keccak",
}

type Candidate = {
  hashFn: any;
  name: string;
  type: HashType;
};

function randomUint32(): number {
  return Math.floor(Math.random() * 4294967295);
}

function checkOutputs(candidate: Candidate) {
  const { hashFn, name, type } = candidate;

  const output = hashFn.call(null, "17", "71");

  console.log(`${name} ${type} hash(17, 71): "${output}"`);
}

function doBenchmark(candidate: Candidate, inputs: number[][]) {
  const { hashFn, name, type } = candidate;

  const before = Date.now();
  for (let i = 0; i < inputs.length; ++i) {
    hashFn.call(null, inputs[i][0], inputs[i][1]);
  }
  const after = Date.now();

  console.log(`${name} ${type} hash -> elapsed time: ${after - before}ms`);
}

function main() {
  const candidates: Candidate[] = [
    {
      hashFn: (x: string, y: string) => starknetJsPedersen([x, y]),
      name: "Starknet (JavaScript)  ",
      type: HashType.PEDERSEN,
    },
    {
      hashFn: (x: string, y: string) =>
        starknet_pedersen(toBufferLE(BigInt(x), 32), toBufferLE(BigInt(y), 32)),
      type: HashType.PEDERSEN,
      name: "Starknet-rs (Rust/WASM)",
    },
    {
      hashFn: (x: string, y: string) =>
        starknet_pedersen_cairo(
          toBufferLE(BigInt(x), 32),
          toBufferLE(BigInt(y), 32)
        ),
      type: HashType.PEDERSEN,
      name: "Starknet Rust (Cairo)  ",
    },
    {
      hashFn: (x: string, y: string) => pedersen(x, y),
      type: HashType.PEDERSEN,
      name: "Geometry (Rust/WASM)   ",
    },
  ];

  candidates.forEach((candidate) => checkOutputs(candidate));

  const iterations = [10, 100, 1000];

  for (const iteration of iterations) {
    const inputs = [];
    for (let i = 0; i < iteration; ++i) {
      inputs.push(randomUint32().toString(), randomUint32().toString());
    }
    console.log(`--> ${iteration} invocations <--`);
    for (const candidate of candidates) {
      doBenchmark(candidate, inputs);
    }
  }
}

main();
