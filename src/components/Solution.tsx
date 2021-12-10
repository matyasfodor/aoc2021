import React, { ChangeEvent, useContext, useState } from "react";
import styled from "styled-components";

import WasmContext from "../WasmContext";

const Container = styled.div`
  display: flex;
  flex-direction: column;
`;

type SolutionProps = {
  solution: (str: string, isSecond: boolean) => number | BigInt;
};

const Solution = ({ solution }: SolutionProps) => {
  const [input, setInput] = useState("");
  const [answer, setAnswer] = useState("");
  const [isSecond, setSecond] = useState(false);
  const clickHandler = () => {
    setAnswer(`${solution(input, isSecond)}`);
  };
  const checkboxHandler = (e: ChangeEvent<HTMLInputElement>) => {
    setSecond(e.target.checked);
  };
  return (
    <Container>
      <textarea
        value={input}
        onChange={(e) => setInput(e.target.value)}
      ></textarea>
      <label>
        Second solution
        <input type="checkbox" checked={isSecond} onChange={checkboxHandler} />
      </label>
      <button onClick={clickHandler}>Solve it!</button>
      <pre>{answer}</pre>
    </Container>
  );
};

export default Solution;
