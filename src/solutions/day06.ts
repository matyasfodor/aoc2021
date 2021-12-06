export function progressState(
  state: Record<number, number>
): Record<number, number> {
  const ret: Record<number, number> = {};
  const numberOfReproducingFish = state[0] ?? 0;
  ret[6] = numberOfReproducingFish;
  // Set offsprings
  ret[8] = numberOfReproducingFish;
  for (let i = 1; i < 9; i++) {
    ret[i - 1] = (ret[i - 1] ?? 0) + (state[i] ?? 0);
  }
  return ret;
}

export function solution(input: string, days: number, second: boolean): number {
  let lifetimes = input.split(",").map((e) => {
    return parseInt(e);
  });
  // State holds a histogram of states
  let state = lifetimes.reduce((acc: Record<number, number>, next) => {
    acc[next] = (acc[next] ?? 0) + 1;
    return acc;
  }, {});

  for (let i = 0; i < days; i++) {
    state = progressState(state);
  }

  // Might have to decrease 9 to 8
  const result = Object.values(state).reduce((a, b) => a + b);

  return result;
}
