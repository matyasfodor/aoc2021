import React, { useContext } from "react";
import Solution from "../components/Solution";
import WasmContext from "../WasmContext";

const Day = () => {
  const wasmModule = useContext(WasmContext);
  return <Solution solution={(...args) => wasmModule.day20(...args)} />;
};

export default Day;
