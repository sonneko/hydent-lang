import { BranchParserFunction, ProductParserFunction, HookParserFunction, IR } from "./new_analyzer"

export class Generator {
    public generate(ir: IR) {
        for (const func of ir) {
            switch (func.kind) {
                case "branch":
                    this.generateBranchParseFunction(func);
                    break;
                case "product":
                    this.generateProductParserFunction(func);
                    break;
                case "hook":
                    this.generateHookParserFunction(func);
                    break;
            }
        }
    }

    private generateBranchParseFunction(func: BranchParserFunction): string {
        return "";
    }

    private generateProductParserFunction(func: ProductParserFunction): string {
        let ret = "";
        ret += `\tpub fn ${func.functionName}(&mut self) -> Result<${func.astTypeName}, Self::Error> {\n`;
        for (const element of func.elements) {
            switch (element.kind) {
                case "normal":
                    ret += `\t\tlet ${element.astTypeName} = self.parse_${element.astTypeName}()?;\n`;
                    break;
                case "boxed":
                    ret += `\t\tlet ${element.astTypeName} = self.alloc(self.parse_${element.astTypeName}()?);\n`;
                    break;
                case "option":
                    ret += `\t\tlet ${element.astTypeName} = self.parse_${element.astTypeName}().is_ok();\n`;
                    break;
                case "repeat":
                    ret += `\t\tlet ${element.astTypeName} = self.repeat(Self::parse_${element.astTypeName});\n`;
                    break;
            }
        }
        ret += `\t${func.astTypeName} {\n`;
        for (const element of func.elements) {
            ret += `\t\t${element.astTypeName},\n`;
        }
        return ret;
    }

    private generateHookParserFunction(func: HookParserFunction): string {
        return "";
    }
}