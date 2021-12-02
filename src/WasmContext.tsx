import React from "react";

type Awaited<T> = T extends PromiseLike<infer U> ? U : T;
export type WasmModule = Awaited<typeof import("../build/wasm")>;

const WasmContext: React.Context<WasmModule | undefined> =
  React.createContext(null);

export default WasmContext;
