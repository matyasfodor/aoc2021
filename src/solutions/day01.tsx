import React, { useContext, useState } from "react";
import WasmContext from "../WasmContext";

const solution = () => {
  const [input, setInput] = useState("");
  const [answer, setAnswer] = useState("");
  const wasmModule = useContext(WasmContext);
  const clickHandler = () => {
    setAnswer("" + wasmModule.day01(input));
  };
  return (
    <div>
      <input value={input} onChange={(e) => setInput(e.target.value)}></input>
      <button onClick={clickHandler}>Solve it!</button>
      <pre>{answer}</pre>
    </div>
  );
};

export default solution;
