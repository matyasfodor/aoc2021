import { solution } from "./day06";

describe(solution.name, () => {
  it("should give the correct result after 18 days", () => {
    const testString = "3,4,3,1,2";
    let result = solution(testString, 18, false);
    expect(result).toBe(26);
  });

  it("should give the correct result after 80 days", () => {
    const testString = "3,4,3,1,2";
    let result = solution(testString, 80, false);
    expect(result).toBe(5934);
  });
});
