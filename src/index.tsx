import React, { useState } from "react";
import ReactDOM from "react-dom";
import styled from "styled-components";

import {
  BrowserRouter,
  NavLink,
  Outlet,
  Routes,
  useRoutes,
} from "react-router-dom";
import Day01 from "./solutions/day01";
import Day02 from "./solutions/day02";
import Day03 from "./solutions/day03";
import Day04 from "./solutions/day04";
import Day05 from "./solutions/day05";
import WasmContext from "./WasmContext";

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
];

wasm.then((m) => {
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
    return (
      <WasmContext.Provider value={m}>
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
          with 🩸 🥵 😢
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
});