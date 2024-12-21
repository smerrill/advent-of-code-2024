import { zip, isNumber, toNumber, sum } from "lodash";
import { readFile } from "node:fs/promises";

const getParsedLists = (listsText: string): number[][] => {
	const listRegex = /(\d+)\s+(\d+)/;
	const listMatches = listsText
		.split("\n")
		.map((j) => j.match(listRegex))
		.filter((j) => j !== null)
		.map((j) => j.slice(1, 3).map((k) => toNumber(k)));

	const [leftList, rightList] = zip<number>(...listMatches);
	const left = leftList.filter(isNumber).sort();
	const right = rightList.filter(isNumber).sort();

	if (left.length !== right.length) {
		throw new Error("Lists are not of the same length");
	}
	return [left, right];
};

export const calculateDifferences = (listsText: string): number => {
	const [left, right] = getParsedLists(listsText);
	return sum(left.map((leftValue, idx) => Math.abs(right[idx] - leftValue)));
};

export const calculateSimilarities = (listsText: string): number => {
	const [left, right] = getParsedLists(listsText);

	const rightOccurences: Record<number, number> = {};
	for (const j of right) {
		rightOccurences[j] = (rightOccurences[j] || 0) + 1;
	}

	return sum(left.map((k) => k * (rightOccurences[k] || 0)));
};

if (require.main === module) {
	const input = await readFile("./day1.txt", "utf8");
	console.log(calculateDifferences(input));
	console.log(calculateSimilarities(input));
}
