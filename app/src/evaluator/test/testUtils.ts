import { promises as fs } from 'fs';
import { MockConsole } from '../../interpreter/test/mockConsole';
import { WasmEvaluator, Evaluator } from '../evaluator';
import { CustomConsole } from '../../utils/console';
import { ExpectedValues } from '../../utils/expectedValues';

interface EvaluatorTestDependencies {
    evaluator: Evaluator;
    customConsole: CustomConsole;
    expectedValues: ExpectedValues;
}

export const evaluatorDependenciesFactory = async (
    file: string
): Promise<EvaluatorTestDependencies> => {
    const baseName = file.split('.tig')[0];

    const wasmPath = `test/inputs/wasm/${baseName}.wasm`;
    const expectedValuesPath = `test/inputs/expectedValues/${baseName}.json`;
    const [binaryWasm, rawExpectedValues] = await Promise.all([
        fs.readFile(wasmPath),
        fs.readFile(expectedValuesPath, 'utf8'),
    ]);

    const expectedValues = JSON.parse(rawExpectedValues);

    const customConsole = new MockConsole();
    const evaluator = new WasmEvaluator(binaryWasm, customConsole);

    return {
        evaluator,
        customConsole,
        expectedValues,
    };
};