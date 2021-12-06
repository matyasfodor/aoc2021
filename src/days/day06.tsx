import React from "react";
import Solution from "../components/Solution";
import { solution } from "../solutions/day06";

const Day01 = () => {
  return <Solution solution={(input, second) => solution(input, 80, second)} />;
};

export default Day01;
