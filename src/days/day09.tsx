import React, { useContext } from "react";
import Solution from "../components/Solution";
import WasmContext from "../WasmContext";

const Day01 = () => {
  const wasmModule = useContext(WasmContext);
  return <Solution solution={(...args) => wasmModule.day09(...args)} />;
};

export default Day01;
