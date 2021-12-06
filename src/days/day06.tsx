import React from "react";
import Solution from "../components/Solution";
import { day06solution } from "../solutions/day06";

const Day01 = () => {
  return (
    <Solution solution={(input, second) => day06solution(input, second)} />
  );
};

export default Day01;
