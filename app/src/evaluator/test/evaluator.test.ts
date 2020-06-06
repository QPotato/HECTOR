import { evaluatorDependenciesFactory } from './testUtils';
import { testExpectedValues } from '../../utils/utils';

test('returnNumber program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('returnNumber.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('returnVariable program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('returnVariable.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('callIdentity program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('callIdentity.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('callFactorial program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('callFactorial.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('callAddone program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('callAddone.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('escape program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('escape.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('basicFor program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('basicFor.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('localHideGlobal program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('localHideGlobal.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('whileWithBreak program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('whileWithBreak.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});

test('basicWhile program works', async () => {
    const { evaluator, expectedValues } = await evaluatorDependenciesFactory('basicWhile.tig');
    const returnValue = await evaluator.run();

    testExpectedValues(returnValue, expectedValues);
});