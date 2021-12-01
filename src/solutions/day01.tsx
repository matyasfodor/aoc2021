import React, { ChangeEvent, useContext, useState } from "react";
import WasmContext from "../WasmContext";

const solution = () => {
  const [input, setInput] = useState("");
  const [answer, setAnswer] = useState("");
  const [isSecond, setSecond] = useState(false);
  const wasmModule = useContext(WasmContext);
  const clickHandler = () => {
    setAnswer("" + wasmModule.day01(input, isSecond));
  };
  const checkboxHandler = (e: ChangeEvent<HTMLInputElement>) => {
    setSecond(e.target.checked);
  };
  return (
    <div>
      <label>
        Use second
        <input type="checkbox" checked={isSecond} onChange={checkboxHandler} />
      </label>
      <textarea
        value={input}
        onChange={(e) => setInput(e.target.value)}
      ></textarea>
      <button onClick={clickHandler}>Solve it!</button>
      <pre>{answer}</pre>
    </div>
  );
};

export default solution;
