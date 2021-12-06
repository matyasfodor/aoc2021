import { solution } from "./day06";

describe(solution.name, () => {
  const testlifeTimes = [3, 4, 3, 1, 2];

  it("should give the correct result after 18 days", () => {
    let result = solution(testlifeTimes, 18);
    expect(result).toBe(26);
  });

  it("should give the correct result after 80 days", () => {
    const testString = "3,4,3,1,2";
    let result = solution(testlifeTimes, 80);
    expect(result).toBe(5934);
  });

  it("should give the correct result after 80 days", () => {
    const testString = "3,4,3,1,2";
    let result = solution(testlifeTimes, 256);
    expect(result).toBe(26984457539);
  });
});
