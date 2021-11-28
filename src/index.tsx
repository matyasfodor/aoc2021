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
import Day01 from "./solutions/day01.tsx";
import Day02 from "./solutions/day02.tsx";
import Day03 from "./solutions/day03.tsx";
import Day04 from "./solutions/day04.tsx";
import Day05 from "./solutions/day05.tsx";

const MainContainer = styled.div`
  display: flex;
  flex-direction: row;
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
    // const [name, setName] = useState("");
    // const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    //   setName(e.target.value);
    // };
    const handleClick = () => {
      alert("Length is " + m.solver1("asd"));
    };

    return (
      <>
        <header>Advent of Code solutions 2021</header>
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
            <button onClick={handleClick}>Click me!</button>
            <MainRouter />
          </Main>
        </MainContainer>
        <footer></footer>
      </>
    );
  };

  ReactDOM.render(
    <BrowserRouter>
      <App />
    </BrowserRouter>,
    document.getElementById("root")
  );
});
