import { calculateDifferences, calculateSimilarities } from "./day1";

const sampleInput = `3   4
4   3
2   5
1   3
3   9
3   3`;

describe("day1", () => {
	test("should calculate differences", () => {
		expect(calculateDifferences(sampleInput)).toBe(11);
	});
	test("should calculate similarity", () => {
		expect(calculateSimilarities(sampleInput)).toBe(31);
	});
});
