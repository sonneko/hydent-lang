import { runTests, testParserGeneration } from "./bnf/test";

(() => {
    runTests();
    testParserGeneration();
})();
