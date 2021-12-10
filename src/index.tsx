import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom";
import styled from "styled-components";

import { BrowserRouter, NavLink, useRoutes } from "react-router-dom";
import Day01 from "./days/day01";
import Day02 from "./days/day02";
import Day03 from "./days/day03";
import Day04 from "./days/day04";
import Day05 from "./days/day05";
import Day06 from "./days/day06";
import Day07 from "./days/day07";
import Day08 from "./days/day08";
import Day09 from "./days/day09";
import Day10 from "./days/day10";
import WasmContext, { WasmModule } from "./WasmContext";

const Header = styled.header`
  height: 50px;
`;

const MainContainer = styled.div`
  display: flex;
  flex-direction: row;
  min-height: calc(100vh - 100px);
`;

const Footer = styled.footer`
  height: 50px;
`;

const Main = styled.main`
  padding: 20px;
`;

const Aside = styled.aside`
  min-width: 200px;
`;

const UnorderedList = styled.ul`
  list-style-type: none;
  margin: 20px;
  padding: 0;
`;

const wasm = import("../build/wasm");

const solutions: {
  name: string;
  path: string;
  element: JSX.Element;
}[] = [
  {
    name: "Day 01",
    path: "day/01",
    element: <Day01 />,
  },
  {
    name: "Day 02",
    path: "day/02",
    element: <Day02 />,
  },
  {
    name: "Day 03",
    path: "day/03",
    element: <Day03 />,
  },
  {
    name: "Day 04",
    path: "day/04",
    element: <Day04 />,
  },
  {
    name: "Day 05",
    path: "day/05",
    element: <Day05 />,
  },
  {
    name: "Day 06",
    path: "day/06",
    element: <Day06 />,
  },
  {
    name: "Day 07",
    path: "day/07",
    element: <Day07 />,
  },
  {
    name: "Day 08",
    path: "day/08",
    element: <Day08 />,
  },
  {
    name: "Day 09",
    path: "day/09",
    element: <Day09 />,
  },
  {
    name: "Day 10",
    path: "day/10",
    element: <Day10 />,
  },
];

const MainRouter = () => {
  const element = useRoutes([
    {
      path: "/",
      element: <div>This is home</div>,
    },
    ...solutions.map(({ path, element }) => ({ path, element })),
  ]);

  return element;
};

const App = () => {
  const [wasmModule, setWasmModule] = useState<WasmModule | undefined>();

  useEffect(() => {
    wasm.then((m) => {
      setWasmModule(m);
    });
  }, []);

  return (
    <WasmContext.Provider value={wasmModule}>
      <Header>Advent of Code solutions 2021</Header>
      <MainContainer>
        <Aside>
          <UnorderedList>
            {solutions.map((solution) => (
              <li key={solution.path}>
                <NavLink to={solution.path}>{solution.name}</NavLink>
              </li>
            ))}
          </UnorderedList>
        </Aside>
        <Main>
          <MainRouter />
        </Main>
      </MainContainer>
      <Footer>
        Made by{" "}
        <a
          href="https://matyasfodor.com"
          target="_blank"
          aria-label="Read more about Matyas"
        >
          Matyas
        </a>{" "}
        with 🩸 🥵 and 😢.
      </Footer>
    </WasmContext.Provider>
  );
};

ReactDOM.render(
  <BrowserRouter>
    <App />
  </BrowserRouter>,
  document.getElementById("root")
);
