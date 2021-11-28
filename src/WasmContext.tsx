import React from "react";

type Awaited<T> = T extends PromiseLike<infer U> ? U : T;
type WasmModule = Awaited<typeof import("../build/wasm")>;

const WasmContext: React.Context<WasmModule> = React.createContext(null);

export default WasmContext;
