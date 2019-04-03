/* globals bench suite */
'use strict';
const levenshteinEditDistance = require('levenshtein-edit-distance');
const fastLevenshtein = require('fast-levenshtein').get;
const levenshteinComponent = require('levenshtein-component');
const ld = require('ld').computeDistance;
const levdist = require('levdist');
const natural = require('natural').LevenshteinDistance;
const levenshtein = require('levenshtein');
const talisman = require('talisman/metrics/distance/levenshtein');
const leven = require('leven');
const levenRs = require('.');

function run(fn) {
	fn('a', 'b');
	fn('ab', 'ac');
	fn('ac', 'bc');
	fn('abc', 'axc');
	fn('kitten', 'sitting');
	fn('xabxcdxxefxgx', '1ab2cd34ef5g6');
	fn('cat', 'cow');
	fn('xabxcdxxefxgx', 'abcdefg');
	fn('javawasneat', 'scalaisgreat');
	fn('example', 'samples');
	fn('sturgeon', 'urgently');
	fn('levenshtein', 'frankenstein');
	fn('distance', 'difference');
	fn('因為我是中國人所以我會說中文', '因為我是英國人所以我會說英文');
	fn('Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim.', 'Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et dis parturient montes, nascetur ridiculus mus. Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim.');
}

suite('leven', () => {
	bench('leven', () => {
		run(leven);
	});

	bench('talisman', () => {
		run(talisman);
  });
  
  bench('leven-rs', () => {
    run(levenRs);
  })

	bench('levenshtein-edit-distance', () => {
		run(levenshteinEditDistance);
	});

	bench('fast-levenshtein', () => {
		run(fastLevenshtein);
	});

	bench('levenshtein-component', () => {
		run(levenshteinComponent);
	});

	bench('ld', () => {
		run(ld);
	});

	bench('levenshtein', () => {
		run(levenshtein);
	});

	bench('levdist', () => {
		run(levdist);
	});

	bench('natural', () => {
		run(natural);
	});
});